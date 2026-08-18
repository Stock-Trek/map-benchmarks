use bench_map::{
    config::*,
    constants::*,
    data::{
        byte_array::ByteArrayDataGen, string::StringDataGen, u64_dense::U64DenseDataGen,
        u64_sparse::U64SparseDataGen, u64_zipfian::U64ZipfianDataGen, uuid_v4::UuidV4DataGen,
    },
    map_data::MapData,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BenchMapGetCloned, BenchMapMutInsert, BenchMapNewWithHasher,
        DashMapBenchMap, FlurryBenchMap, HashbrownBenchMap, IndexMapBenchMap, LeapfrogBenchMap,
        PapayaBenchMap, SccBenchMap, StarshardBenchMap, StdBenchMap, TxMapBenchMap,
        horde_benchmap::HordeBenchMap,
    },
};
use criterion::{
    BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main, measurement::WallTime,
};
use std::{hash::Hash, hint::black_box, rc::Rc};
use uuid::Uuid;

type CommonHasher = ahash::RandomState;

fn bench<Map, K, V>(
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<K, V>,
    name: &str,
    hasher: CommonHasher,
) where
    Map: BenchMapNewWithHasher<K, V, CommonHasher>
        + BenchMapMutInsert<K, V>
        + BenchMapGetCloned<K, V>,
    K: Clone + Hash + Eq,
    V: Clone,
{
    group.bench_function(name, move |b| {
        let map = map_data.create_map_with_hasher::<Map, CommonHasher>(hasher.clone());
        let keys = map_data.existing_keys();
        b.iter(|| {
            for key in keys {
                let key = black_box(key);
                black_box(map.get_cloned(key));
            }
        });
    });
}

fn key_sensitivity(c: &mut Criterion) {
    const KEY_SENSITIVITY_GROUP_NAME: &str = "key-sensitivity";
    let entry_count = KEY_SENSITIVITY_ENTRY_COUNT;
    let existing_key_count: u64 = 100;
    let missing_key_count: u64 = 0;
    let sort_keys = false;
    // u64 keys
    {
        let map_data = MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            entry_count,
            existing_key_count as usize,
            missing_key_count as usize,
            sort_keys,
        );
        let mut group = c.benchmark_group(format!(
            "{KEY_SENSITIVITY_GROUP_NAME}/{SAME_HASHER_GROUP_NAME}/u64"
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));

        let hasher = CommonHasher::new();
        bench::<AhashBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "ahash",
            hasher.clone(),
        );
        // bench::<BTreeMapBenchMap<u64, u64, CommonHasher>, u64, u64>(&mut group, &map_data, "btreemap"); // doesn't allow setting hasher
        // bench::<ConcreadBenchMap<u64, u64, CommonHasher>, u64, u64>(&mut group, &map_data, "concread"); // doesn't allow setting hasher
        bench::<DashMapBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "dashmap",
            hasher.clone(),
        );
        bench::<FlurryBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "flurry",
            hasher.clone(),
        );
        bench::<HashbrownBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "hashbrown",
            hasher.clone(),
        );
        bench::<HordeBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "horde",
            hasher.clone(),
        );
        // bench::<ImmutableChunkMapBenchMap<u64, u64, CommonHasher>, u64, u64>(&mut group, &map_data, "immutable-chunkmap"); // doesn't allow setting hasher
        bench::<IndexMapBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "indexmap",
            hasher.clone(),
        );
        bench::<LeapfrogBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "leapfrog",
            hasher.clone(),
        );
        bench::<PapayaBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "papaya",
            hasher.clone(),
        );
        // bench::<RustCHashBenchMap<u64, u64, CommonHasher>, u64, u64>(&mut group, &map_data, "rustc-hash"); // doesn't allow setting hasher
        bench::<SccBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "scc",
            hasher.clone(),
        );
        bench::<StarshardBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "starshard",
            hasher.clone(),
        );
        bench::<StdBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "std",
            hasher.clone(),
        );
        bench::<TxMapBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "txmap",
            hasher.clone(),
        );
    }
    // UUID v4 keys
    {
        let map_data = Rc::new(MapGen::generate(
            UuidV4DataGen,
            U64SparseDataGen,
            entry_count,
            existing_key_count as usize,
            missing_key_count as usize,
            sort_keys,
        ));
        let mut group = c.benchmark_group(format!(
            "{KEY_SENSITIVITY_GROUP_NAME}/{SAME_HASHER_GROUP_NAME}/UUID"
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));

        let hasher = CommonHasher::new();
        bench::<AhashBenchMap<Uuid, u64, CommonHasher>, Uuid, u64>(
            &mut group,
            &map_data,
            "ahash",
            hasher.clone(),
        );
        // bench::<BTreeMapBenchMap<Uuid, u64, CommonHasher>, Uuid, u64>(&mut group, &map_data, "btreemap"); // doesn't allow setting hasher
        // bench::<ConcreadBenchMap<Uuid, u64, CommonHasher>, Uuid, u64>(&mut group, &map_data, "concread"); // doesn't allow setting hasher
        bench::<DashMapBenchMap<Uuid, u64, CommonHasher>, Uuid, u64>(
            &mut group,
            &map_data,
            "dashmap",
            hasher.clone(),
        );
        bench::<FlurryBenchMap<Uuid, u64, CommonHasher>, Uuid, u64>(
            &mut group,
            &map_data,
            "flurry",
            hasher.clone(),
        );
        bench::<HashbrownBenchMap<Uuid, u64, CommonHasher>, Uuid, u64>(
            &mut group,
            &map_data,
            "hashbrown",
            hasher.clone(),
        );
        bench::<HordeBenchMap<Uuid, u64, CommonHasher>, Uuid, u64>(
            &mut group,
            &map_data,
            "horde",
            hasher.clone(),
        );
        // bench::<ImmutableChunkMapBenchMap<Uuid, u64, CommonHasher>, Uuid, u64>(&mut group, &map_data, "immutable-chunkmap"); // doesn't allow setting hasher
        bench::<IndexMapBenchMap<Uuid, u64, CommonHasher>, Uuid, u64>(
            &mut group,
            &map_data,
            "indexmap",
            hasher.clone(),
        );
        bench::<LeapfrogBenchMap<Uuid, u64, CommonHasher>, Uuid, u64>(
            &mut group,
            &map_data,
            "leapfrog",
            hasher.clone(),
        );
        bench::<PapayaBenchMap<Uuid, u64, CommonHasher>, Uuid, u64>(
            &mut group,
            &map_data,
            "papaya",
            hasher.clone(),
        );
        // bench::<RustCHashBenchMap<Uuid, u64, CommonHasher>, Uuid, u64>(&mut group, &map_data, "rustc-hash"); // doesn't allow setting hasher
        bench::<SccBenchMap<Uuid, u64, CommonHasher>, Uuid, u64>(
            &mut group,
            &map_data,
            "scc",
            hasher.clone(),
        );
        bench::<StarshardBenchMap<Uuid, u64, CommonHasher>, Uuid, u64>(
            &mut group,
            &map_data,
            "starshard",
            hasher.clone(),
        );
        bench::<StdBenchMap<Uuid, u64, CommonHasher>, Uuid, u64>(
            &mut group,
            &map_data,
            "std",
            hasher.clone(),
        );
        bench::<TxMapBenchMap<Uuid, u64, CommonHasher>, Uuid, u64>(
            &mut group,
            &map_data,
            "txmap",
            hasher.clone(),
        );
    }
    // Byte(32) keys
    {
        let map_data = Rc::new(MapGen::generate(
            ByteArrayDataGen::<32>,
            U64SparseDataGen,
            entry_count,
            existing_key_count as usize,
            missing_key_count as usize,
            sort_keys,
        ));
        let mut group = c.benchmark_group(format!(
            "{KEY_SENSITIVITY_GROUP_NAME}/{SAME_HASHER_GROUP_NAME}/Byte<32>"
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));

        let hasher = CommonHasher::new();
        bench::<AhashBenchMap<[u8; 32], u64, CommonHasher>, [u8; 32], u64>(
            &mut group,
            &map_data,
            "ahash",
            hasher.clone(),
        );
        // bench::<BTreeMapBenchMap<[u8; 32], u64, CommonHasher>, [u8; 32], u64>(&mut group, &map_data, "btreemap"); // doesn't allow setting hasher
        // bench::<ConcreadBenchMap<[u8; 32], u64, CommonHasher>, [u8; 32], u64>(&mut group, &map_data, "concread"); // doesn't allow setting hasher
        bench::<DashMapBenchMap<[u8; 32], u64, CommonHasher>, [u8; 32], u64>(
            &mut group,
            &map_data,
            "dashmap",
            hasher.clone(),
        );
        bench::<FlurryBenchMap<[u8; 32], u64, CommonHasher>, [u8; 32], u64>(
            &mut group,
            &map_data,
            "flurry",
            hasher.clone(),
        );
        bench::<HashbrownBenchMap<[u8; 32], u64, CommonHasher>, [u8; 32], u64>(
            &mut group,
            &map_data,
            "hashbrown",
            hasher.clone(),
        );
        bench::<HordeBenchMap<[u8; 32], u64, CommonHasher>, [u8; 32], u64>(
            &mut group,
            &map_data,
            "horde",
            hasher.clone(),
        );
        // bench::<ImmutableChunkMapBenchMap<[u8; 32], u64, CommonHasher>, [u8; 32], u64>(&mut group, &map_data, "immutable-chunkmap"); // doesn't allow setting hasher
        bench::<IndexMapBenchMap<[u8; 32], u64, CommonHasher>, [u8; 32], u64>(
            &mut group,
            &map_data,
            "indexmap",
            hasher.clone(),
        );
        bench::<LeapfrogBenchMap<[u8; 32], u64, CommonHasher>, [u8; 32], u64>(
            &mut group,
            &map_data,
            "leapfrog",
            hasher.clone(),
        );
        bench::<PapayaBenchMap<[u8; 32], u64, CommonHasher>, [u8; 32], u64>(
            &mut group,
            &map_data,
            "papaya",
            hasher.clone(),
        );
        // bench::<RustCHashBenchMap<[u8; 32], u64, CommonHasher>, [u8; 32], u64>(&mut group, &map_data, "rustc-hash"); // doesn't allow setting hasher
        bench::<SccBenchMap<[u8; 32], u64, CommonHasher>, [u8; 32], u64>(
            &mut group,
            &map_data,
            "scc",
            hasher.clone(),
        );
        bench::<StarshardBenchMap<[u8; 32], u64, CommonHasher>, [u8; 32], u64>(
            &mut group,
            &map_data,
            "starshard",
            hasher.clone(),
        );
        bench::<StdBenchMap<[u8; 32], u64, CommonHasher>, [u8; 32], u64>(
            &mut group,
            &map_data,
            "std",
            hasher.clone(),
        );
        bench::<TxMapBenchMap<[u8; 32], u64, CommonHasher>, [u8; 32], u64>(
            &mut group,
            &map_data,
            "txmap",
            hasher.clone(),
        );
    }
    // String<16> keys
    {
        let map_data = Rc::new(MapGen::generate(
            StringDataGen::<16>,
            U64SparseDataGen,
            entry_count,
            existing_key_count as usize,
            missing_key_count as usize,
            sort_keys,
        ));
        let mut group = c.benchmark_group(format!(
            "{KEY_SENSITIVITY_GROUP_NAME}/{SAME_HASHER_GROUP_NAME}/String<16>"
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));

        let hasher = CommonHasher::new();
        bench::<AhashBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "ahash",
            hasher.clone(),
        );
        // bench::<BTreeMapBenchMap<String, u64, CommonHasher>, String, u64>(&mut group, &map_data, "btreemap"); // doesn't allow setting hasher
        // bench::<ConcreadBenchMap<String, u64, CommonHasher>, String, u64>(&mut group, &map_data, "concread"); // doesn't allow setting hasher
        bench::<DashMapBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "dashmap",
            hasher.clone(),
        );
        bench::<FlurryBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "flurry",
            hasher.clone(),
        );
        bench::<HashbrownBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "hashbrown",
            hasher.clone(),
        );
        bench::<HordeBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "horde",
            hasher.clone(),
        );
        // bench::<ImmutableChunkMapBenchMap<String, u64, CommonHasher>, String, u64>(&mut group, &map_data, "immutable-chunkmap"); // doesn't allow setting hasher
        // bench::<LeapfrogBenchMap<String, u64, CommonHasher>, String, u64>(&mut group, &map_data, "leapfrog", hasher.clone()); // keys must be Copy
        bench::<IndexMapBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "indexmap",
            hasher.clone(),
        );
        bench::<PapayaBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "papaya",
            hasher.clone(),
        );
        // bench::<RustCHashBenchMap<String, u64, CommonHasher>, String, u64>(&mut group, &map_data, "rustc-hash"); // doesn't allow setting hasher
        bench::<SccBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "scc",
            hasher.clone(),
        );
        bench::<StarshardBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "starshard",
            hasher.clone(),
        );
        bench::<StdBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "std",
            hasher.clone(),
        );
        bench::<TxMapBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "txmap",
            hasher.clone(),
        );
    }
    // String<128> keys
    {
        let map_data = Rc::new(MapGen::generate(
            StringDataGen::<128>,
            U64SparseDataGen,
            entry_count,
            existing_key_count as usize,
            missing_key_count as usize,
            sort_keys,
        ));
        let mut group = c.benchmark_group(format!(
            "{KEY_SENSITIVITY_GROUP_NAME}/{SAME_HASHER_GROUP_NAME}/String<128>"
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));

        let hasher = CommonHasher::new();
        bench::<AhashBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "ahash",
            hasher.clone(),
        );
        // bench::<BTreeMapBenchMap<String, u64, CommonHasher>, String, u64>(&mut group, &map_data, "btreemap"); // doesn't allow setting hasher
        // bench::<ConcreadBenchMap<String, u64, CommonHasher>, String, u64>(&mut group, &map_data, "concread"); // doesn't allow setting hasher
        bench::<DashMapBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "dashmap",
            hasher.clone(),
        );
        bench::<FlurryBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "flurry",
            hasher.clone(),
        );
        bench::<HashbrownBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "hashbrown",
            hasher.clone(),
        );
        bench::<HordeBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "horde",
            hasher.clone(),
        );
        // bench::<ImmutableChunkMapBenchMap<String, u64, CommonHasher>, String, u64>(&mut group, &map_data, "immutable-chunkmap"); // doesn't allow setting hasher
        bench::<IndexMapBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "indexmap",
            hasher.clone(),
        );
        // bench::<LeapfrogBenchMap<String, u64, CommonHasher>, String, u64>(&mut group, &map_data, "leapfrog", hasher.clone()); // keys must be Copy
        bench::<PapayaBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "papaya",
            hasher.clone(),
        );
        // bench::<RustCHashBenchMap<String, u64, CommonHasher>, String, u64>(&mut group, &map_data, "rustc-hash"); // doesn't allow setting hasher
        bench::<SccBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "scc",
            hasher.clone(),
        );
        bench::<StarshardBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "starshard",
            hasher.clone(),
        );
        bench::<StdBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "std",
            hasher.clone(),
        );
        bench::<TxMapBenchMap<String, u64, CommonHasher>, String, u64>(
            &mut group,
            &map_data,
            "txmap",
            hasher.clone(),
        );
    }
    // Adversarial keys (dense)
    {
        let map_data = Rc::new(MapGen::generate(
            U64DenseDataGen,
            U64SparseDataGen,
            entry_count,
            existing_key_count as usize,
            missing_key_count as usize,
            sort_keys,
        ));
        let mut group = c.benchmark_group(format!(
            "{KEY_SENSITIVITY_GROUP_NAME}/{SAME_HASHER_GROUP_NAME}/u64-dense"
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));

        let hasher = CommonHasher::new();
        bench::<AhashBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "ahash",
            hasher.clone(),
        );
        // bench::<BTreeMapBenchMap<u64, u64, CommonHasher>, u64, u64>(&mut group, &map_data, "btreemap"); // doesn't allow setting hasher
        // bench::<ConcreadBenchMap<u64, u64, CommonHasher>, u64, u64>(&mut group, &map_data, "concread"); // doesn't allow setting hasher
        bench::<DashMapBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "dashmap",
            hasher.clone(),
        );
        bench::<FlurryBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "flurry",
            hasher.clone(),
        );
        bench::<HashbrownBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "hashbrown",
            hasher.clone(),
        );
        bench::<HordeBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "horde",
            hasher.clone(),
        );
        // bench::<ImmutableChunkMapBenchMap<u64, u64, CommonHasher>, u64, u64>(&mut group, &map_data, "immutable-chunkmap"); // doesn't allow setting hasher
        bench::<IndexMapBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "indexmap",
            hasher.clone(),
        );
        bench::<LeapfrogBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "leapfrog",
            hasher.clone(),
        );
        bench::<PapayaBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "papaya",
            hasher.clone(),
        );
        // bench::<RustCHashBenchMap<u64, u64, CommonHasher>, u64, u64>(&mut group, &map_data, "rustc-hash"); // doesn't allow setting hasher
        bench::<SccBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "scc",
            hasher.clone(),
        );
        bench::<StarshardBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "starshard",
            hasher.clone(),
        );
        bench::<StdBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "std",
            hasher.clone(),
        );
        bench::<TxMapBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "txmap",
            hasher.clone(),
        );
    }
    // Adversarial keys (zipfian)
    {
        let map_data = Rc::new(MapGen::generate(
            U64ZipfianDataGen {
                num_items: (entry_count * 1_000) as u64,
                exponent: 1.0,
            },
            U64SparseDataGen,
            entry_count,
            existing_key_count as usize,
            missing_key_count as usize,
            sort_keys,
        ));
        let mut group = c.benchmark_group(format!(
            "{KEY_SENSITIVITY_GROUP_NAME}/{SAME_HASHER_GROUP_NAME}/u64-zipfian"
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));

        let hasher = CommonHasher::new();
        bench::<AhashBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "ahash",
            hasher.clone(),
        );
        // bench::<BTreeMapBenchMap<u64, u64, CommonHasher>, u64, u64>(&mut group, &map_data, "btreemap"); // doesn't allow setting hasher
        // bench::<ConcreadBenchMap<u64, u64, CommonHasher>, u64, u64>(&mut group, &map_data, "concread"); // doesn't allow setting hasher
        bench::<DashMapBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "dashmap",
            hasher.clone(),
        );
        bench::<FlurryBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "flurry",
            hasher.clone(),
        );
        bench::<HashbrownBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "hashbrown",
            hasher.clone(),
        );
        bench::<HordeBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "horde",
            hasher.clone(),
        );
        // bench::<ImmutableChunkMapBenchMap<u64, u64, CommonHasher>, u64, u64>(&mut group, &map_data, "immutable-chunkmap"); // doesn't allow setting hasher
        bench::<IndexMapBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "indexmap",
            hasher.clone(),
        );
        bench::<LeapfrogBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "leapfrog",
            hasher.clone(),
        );
        bench::<PapayaBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "papaya",
            hasher.clone(),
        );
        // bench::<RustCHashBenchMap<u64, u64, CommonHasher>, u64, u64>(&mut group, &map_data, "rustc-hash"); // doesn't allow setting hasher
        bench::<SccBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "scc",
            hasher.clone(),
        );
        bench::<StarshardBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "starshard",
            hasher.clone(),
        );
        bench::<StdBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "std",
            hasher.clone(),
        );
        bench::<TxMapBenchMap<u64, u64, CommonHasher>, u64, u64>(
            &mut group,
            &map_data,
            "txmap",
            hasher.clone(),
        );
    }
}

criterion_group!(group, key_sensitivity);
criterion_main!(group);
