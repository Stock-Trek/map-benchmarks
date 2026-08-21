use bench_map::{
    config::*,
    constants::*,
    data::{string::StringDataGen, u64_sparse::U64SparseDataGen},
    map_data::MapData,
    map_gen::MapGen,
    maps::*,
};
use criterion::{
    BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main, measurement::WallTime,
};
use std::{hash::Hash, hint::black_box, rc::Rc};

type CommonHasher = ahash::RandomState;

fn bench<Map, K, V>(
    name: &str,
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<K, V>,
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
    let entry_count = DEFAULT_ENTRY_COUNT;
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
            "ahash",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        // bench::<BTreeMapBenchMap<u64, u64, CommonHasher>, u64, u64>("btreemap", &mut group, &map_data); // doesn't allow setting hasher
        // bench::<ConcreadBenchMap<u64, u64, CommonHasher>, u64, u64>("concread", &mut group, &map_data); // doesn't allow setting hasher
        // bench::<ConcurrentMapBenchMap<u64, u64>, u64, u64>("concurrent-map", &mut group, &map_data); // doesn't allow setting hasher
        bench::<DashMapBenchMap<u64, u64, CommonHasher>, u64, u64>(
            "dashmap",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        // bench::<FlurryBenchMap<u64, u64, CommonHasher>, u64, u64>("flurry", &mut group, &map_data, hasher.clone()); // too slow
        bench::<HashbrownBenchMap<u64, u64, CommonHasher>, u64, u64>(
            "hashbrown",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<HashlinkBenchMap<u64, u64, CommonHasher>, u64, u64>(
            "hashlink",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<HordeBenchMap<u64, u64, CommonHasher>, u64, u64>(
            "horde",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        // bench::<ImmutableChunkMapBenchMap<u64, u64, CommonHasher>, u64, u64>("immutable-chunkmap", &mut group, &map_data); // doesn't allow setting hasher
        bench::<ImblBenchMap<u64, u64, CommonHasher>, u64, u64>(
            "imbl",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<IndexMapBenchMap<u64, u64, CommonHasher>, u64, u64>(
            "indexmap",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<LeapfrogBenchMap<u64, u64, CommonHasher>, u64, u64>(
            "leapfrog",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<PapayaBenchMap<u64, u64, CommonHasher>, u64, u64>(
            "papaya",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<RpdsHashTrieMapBenchMap<u64, u64, CommonHasher>, u64, u64>(
            "rpds-hash-trie-map",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        // bench::<RustCHashBenchMap<u64, u64, CommonHasher>, u64, u64>("rustc-hash", &mut group, &map_data); // doesn't allow setting hasher
        bench::<SccBenchMap<u64, u64, CommonHasher>, u64, u64>(
            "scc",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<StarshardBenchMap<u64, u64, CommonHasher>, u64, u64>(
            "starshard",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<StdBenchMap<u64, u64, CommonHasher>, u64, u64>(
            "std",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<TxMapBenchMap<u64, u64, CommonHasher>, u64, u64>(
            "txmap",
            &mut group,
            &map_data,
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
            "ahash",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        // bench::<BTreeMapBenchMap<String, u64, CommonHasher>, String, u64>("btreemap", &mut group, &map_data); // doesn't allow setting hasher
        // bench::<ConcreadBenchMap<String, u64, CommonHasher>, String, u64>("concread", &mut group, &map_data); // doesn't allow setting hasher
        // bench::<ConcurrentMapBenchMap<String, u64>, String, u64>("concurrent-map", &mut group, &map_data); // doesn't allow setting hasher
        bench::<DashMapBenchMap<String, u64, CommonHasher>, String, u64>(
            "dashmap",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        // bench::<FlurryBenchMap<String, u64, CommonHasher>, String, u64>("flurry", &mut group, &map_data, hasher.clone()); // too slow
        bench::<HashbrownBenchMap<String, u64, CommonHasher>, String, u64>(
            "hashbrown",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<HashlinkBenchMap<String, u64, CommonHasher>, String, u64>(
            "hashlink",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<HordeBenchMap<String, u64, CommonHasher>, String, u64>(
            "horde",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        // bench::<ImmutableChunkMapBenchMap<String, u64, CommonHasher>, String, u64>("immutable-chunkmap", &mut group, &map_data); // doesn't allow setting hasher
        // bench::<LeapfrogBenchMap<String, u64, CommonHasher>, String, u64>("leapfrog", &mut group, &map_data, hasher.clone()); // keys must be Copy
        bench::<ImblBenchMap<String, u64, CommonHasher>, String, u64>(
            "imbl",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<IndexMapBenchMap<String, u64, CommonHasher>, String, u64>(
            "indexmap",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<PapayaBenchMap<String, u64, CommonHasher>, String, u64>(
            "papaya",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<RpdsHashTrieMapBenchMap<String, u64, CommonHasher>, String, u64>(
            "rpds-hash-trie-map",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        // bench::<RustCHashBenchMap<String, u64, CommonHasher>, String, u64>("rustc-hash", &mut group, &map_data); // doesn't allow setting hasher
        bench::<SccBenchMap<String, u64, CommonHasher>, String, u64>(
            "scc",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<StarshardBenchMap<String, u64, CommonHasher>, String, u64>(
            "starshard",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<StdBenchMap<String, u64, CommonHasher>, String, u64>(
            "std",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<TxMapBenchMap<String, u64, CommonHasher>, String, u64>(
            "txmap",
            &mut group,
            &map_data,
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
            "ahash",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        // bench::<BTreeMapBenchMap<String, u64, CommonHasher>, String, u64>("btreemap", &mut group, &map_data); // doesn't allow setting hasher
        // bench::<ConcreadBenchMap<String, u64, CommonHasher>, String, u64>("concread", &mut group, &map_data); // doesn't allow setting hasher
        // bench::<ConcurrentMapBenchMap<String, u64>, String, u64>("concurrent-map", &mut group, &map_data); // doesn't allow setting hasher
        bench::<DashMapBenchMap<String, u64, CommonHasher>, String, u64>(
            "dashmap",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        // bench::<FlurryBenchMap<String, u64, CommonHasher>, String, u64>("flurry", &mut group, &map_data, hasher.clone()); // too slow
        bench::<HashbrownBenchMap<String, u64, CommonHasher>, String, u64>(
            "hashbrown",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<HashlinkBenchMap<String, u64, CommonHasher>, String, u64>(
            "hashlink",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<HordeBenchMap<String, u64, CommonHasher>, String, u64>(
            "horde",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        // bench::<ImmutableChunkMapBenchMap<String, u64, CommonHasher>, String, u64>("immutable-chunkmap", &mut group, &map_data); // doesn't allow setting hasher
        bench::<ImblBenchMap<String, u64, CommonHasher>, String, u64>(
            "imbl",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<IndexMapBenchMap<String, u64, CommonHasher>, String, u64>(
            "indexmap",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        // bench::<LeapfrogBenchMap<String, u64, CommonHasher>, String, u64>("leapfrog", &mut group, &map_data, hasher.clone()); // keys must be Copy
        bench::<PapayaBenchMap<String, u64, CommonHasher>, String, u64>(
            "papaya",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<RpdsHashTrieMapBenchMap<String, u64, CommonHasher>, String, u64>(
            "rpds-hash-trie-map",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        // bench::<RustCHashBenchMap<String, u64, CommonHasher>, String, u64>("rustc-hash", &mut group, &map_data); // doesn't allow setting hasher
        bench::<SccBenchMap<String, u64, CommonHasher>, String, u64>(
            "scc",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<StarshardBenchMap<String, u64, CommonHasher>, String, u64>(
            "starshard",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<StdBenchMap<String, u64, CommonHasher>, String, u64>(
            "std",
            &mut group,
            &map_data,
            hasher.clone(),
        );
        bench::<TxMapBenchMap<String, u64, CommonHasher>, String, u64>(
            "txmap",
            &mut group,
            &map_data,
            hasher.clone(),
        );
    }
}

criterion_group!(group, key_sensitivity);
criterion_main!(group);
