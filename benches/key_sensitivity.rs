// How much does the choice of key type matter? Tests hashing and key-handling design, the cost of hashing/comparing keys of different sizes.
use bench_map::{
    common_hasher::CommonHasher,
    config::*,
    constants::*,
    data::{string::StringDataGen, u64_sparse::U64SparseDataGen},
    expand_bench_with_map_data_and_common_hasher,
    map_data::MapData,
    map_gen::MapGen,
    maps::*,
};
use criterion::{
    BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main, measurement::WallTime,
};
use std::{hash::Hash, hint::black_box, rc::Rc};

fn bench<Map, K>(
    name: &str,
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<K, u64>,
    hasher: CommonHasher,
) where
    Map: BenchMapNewWithHasher<K, u64, CommonHasher>
        + BenchMapMutInsert<K, u64>
        + BenchMapGetCloned<K, u64>,
    K: Clone + Hash + Eq,
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
        let mut group =
            c.benchmark_group(format!("{KEY_SENSITIVITY_GROUP_NAME}/{COMMON_HASHER}/u64"));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));

        expand_bench_with_map_data_and_common_hasher!(bench, u64, &mut group, &map_data,
            AhashBenchMap<u64, u64, CommonHasher>,
            // BTreeMapBenchMap<u64, u64, CommonHasher>, // doesn't allow setting hasher
            // ConcreadBenchMap<u64, u64, CommonHasher>, // too slow
            // ConcurrentMapBenchMap<u64, u64, CommonHasher>, // doesn't allow setting hasher
            // CrossbeamSkiplistBenchMap<u64, u64, CommonHasher>, // doesn't allow setting hasher
            DashMapBenchMap<u64, u64, CommonHasher>,
            // FlurryBenchMap<u64, u64, CommonHasher>, // too slow
            HashbrownBenchMap<u64, u64, CommonHasher>,
            HashlinkBenchMap<u64, u64, CommonHasher>,
            HordeBenchMap<u64, u64, CommonHasher>,
            // ImmutableChunkMapBenchMap<u64, u64, CommonHasher>, // doesn't allow setting hasher
            ImblBenchMap<u64, u64, CommonHasher>,
            IndexMapBenchMap<u64, u64, CommonHasher>,
            // IntMapBenchMap<u64, u64, CommonHasher>, // doesn't allow setting hasher
            LeapfrogBenchMap<u64, u64, CommonHasher>,
            PapayaBenchMap<u64, u64, CommonHasher>,
            RpdsHashTrieMapBenchMap<u64, u64, CommonHasher>,
            // RustCHashBenchMap<u64, u64, CommonHasher>, // doesn't allow setting hasher
            SccBenchMap<u64, u64, CommonHasher>,
            StarshardBenchMap<u64, u64, CommonHasher>,
            StdBenchMap<u64, u64, CommonHasher>,
            TxMapBenchMap<u64, u64, CommonHasher>,
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
            "{KEY_SENSITIVITY_GROUP_NAME}/{COMMON_HASHER}/String<16>"
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));

        expand_bench_with_map_data_and_common_hasher!(bench, String, &mut group, &map_data,
            AhashBenchMap<String, u64, CommonHasher>,
            // BTreeMapBenchMap<String, u64, CommonHasher>, // doesn't allow setting hasher
            // ConcreadBenchMap<String, u64, CommonHasher>, // too slow
            // ConcurrentMapBenchMap<String, u64, CommonHasher>, // doesn't allow setting hasher
            // CrossbeamSkiplistBenchMap<String, u64, CommonHasher>, // doesn't allow setting hasher
            DashMapBenchMap<String, u64, CommonHasher>,
            // FlurryBenchMap<String, u64, CommonHasher>, // too slow
            HashbrownBenchMap<String, u64, CommonHasher>,
            HashlinkBenchMap<String, u64, CommonHasher>,
            HordeBenchMap<String, u64, CommonHasher>,
            // ImmutableChunkMapBenchMap<String, u64, CommonHasher>, // doesn't allow setting hasher
            ImblBenchMap<String, u64, CommonHasher>,
            IndexMapBenchMap<String, u64, CommonHasher>,
            // IntMapBenchMap<String, u64, CommonHasher>, // keys require IntKey
            // LeapfrogBenchMap<String, u64, CommonHasher>, // keys require Copy
            PapayaBenchMap<String, u64, CommonHasher>,
            RpdsHashTrieMapBenchMap<String, u64, CommonHasher>,
            // RustCHashBenchMap<String, u64, CommonHasher>, // doesn't allow setting hasher
            SccBenchMap<String, u64, CommonHasher>,
            StarshardBenchMap<String, u64, CommonHasher>,
            StdBenchMap<String, u64, CommonHasher>,
            TxMapBenchMap<String, u64, CommonHasher>,
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
            "{KEY_SENSITIVITY_GROUP_NAME}/{COMMON_HASHER}/String<128>"
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));

        expand_bench_with_map_data_and_common_hasher!(bench, String, &mut group, &map_data,
            AhashBenchMap<String, u64, CommonHasher>,
            // BTreeMapBenchMap<String, u64, CommonHasher>, // doesn't allow setting hasher
            // ConcreadBenchMap<String, u64, CommonHasher>, // too slow
            // ConcurrentMapBenchMap<String, u64, CommonHasher>, // doesn't allow setting hasher
            // CrossbeamSkiplistBenchMap<String, u64, CommonHasher>, // doesn't allow setting hasher
            DashMapBenchMap<String, u64, CommonHasher>,
            // FlurryBenchMap<String, u64, CommonHasher>, // too slow
            HashbrownBenchMap<String, u64, CommonHasher>,
            HashlinkBenchMap<String, u64, CommonHasher>,
            HordeBenchMap<String, u64, CommonHasher>,
            // ImmutableChunkMapBenchMap<String, u64, CommonHasher>, // doesn't allow setting hasher
            ImblBenchMap<String, u64, CommonHasher>,
            IndexMapBenchMap<String, u64, CommonHasher>,
            // IntMapBenchMap<String, u64, CommonHasher>, // keys require IntKey
            // LeapfrogBenchMap<String, u64, CommonHasher>, // keys require Copy
            PapayaBenchMap<String, u64, CommonHasher>,
            RpdsHashTrieMapBenchMap<String, u64, CommonHasher>,
            // RustCHashBenchMap<String, u64, CommonHasher>, // doesn't allow setting hasher
            SccBenchMap<String, u64, CommonHasher>,
            StarshardBenchMap<String, u64, CommonHasher>,
            StdBenchMap<String, u64, CommonHasher>,
            TxMapBenchMap<String, u64, CommonHasher>,
        );
    }
}

criterion_group!(group, key_sensitivity);
criterion_main!(group);
