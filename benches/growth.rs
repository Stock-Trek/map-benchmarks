// How does it scale from empty to its target size? Tests the capacity growth design, resizing/rehashing strategy and the amortized cost of expansion.
use bench_map::{
    common_hasher::CommonHasher,
    config::*,
    constants::*,
    data::{string::StringDataGen, u64_sparse::U64SparseDataGen},
    expand_bench_with_map_data, expand_bench_with_map_data_and_common_hasher,
    map_data::MapData,
    map_gen::MapGen,
    maps::*,
};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main,
    measurement::WallTime,
};
use std::{hash::Hash, hint::black_box};

fn bench_out_of_the_box<Map, K>(
    name: &str,
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<K, u64>,
) where
    Map: BenchMapNew<K, u64> + BenchMapMutInsert<K, u64>,
    K: Clone + Hash + Eq,
{
    group.bench_function(name, move |b| {
        let map_data_ref = &map_data;
        b.iter_batched(
            move || {
                let map = map_data_ref.create_map::<Map>();
                let keys = map_data_ref.missing_keys().clone();
                (map, keys)
            },
            |(mut map, mut keys)| {
                for key in keys.drain(..) {
                    let key = black_box(key);
                    map.insert(key, 42);
                }
                black_box(map)
            },
            BatchSize::PerIteration,
        );
    });
}

fn bench_same_hasher<Map, K>(
    name: &str,
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<K, u64>,
    hasher: CommonHasher,
) where
    Map: BenchMapNewWithHasher<K, u64, CommonHasher> + BenchMapMutInsert<K, u64>,
    K: Clone + Hash + Eq,
{
    group.bench_function(name, move |b| {
        let map_data_ref = &map_data;
        let hasher = hasher.clone();
        b.iter_batched(
            move || {
                let map = map_data_ref.create_map_with_hasher::<Map, CommonHasher>(hasher.clone());
                let keys = map_data_ref.missing_keys().clone();
                (map, keys)
            },
            |(mut map, mut keys)| {
                for key in keys.drain(..) {
                    let key = black_box(key);
                    map.insert(key, 42);
                }
                black_box(map)
            },
            BatchSize::PerIteration,
        );
    });
}

fn growth(c: &mut Criterion) {
    let entry_count = 0;
    let existing_key_count = 0;
    let sort_keys = false;
    for (missing_key_count, entry_count_name) in GROWTH_ENTRY_COUNTS {
        let map_data_u64 = MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            entry_count,
            existing_key_count,
            *missing_key_count,
            sort_keys,
        );
        let map_data_string_32 = MapGen::generate(
            StringDataGen::<32>,
            U64SparseDataGen,
            entry_count,
            existing_key_count,
            *missing_key_count,
            sort_keys,
        );

        // out of the box, u64 keys
        {
            let mut group = c.benchmark_group(format!(
                "growth/map-size-{entry_count_name}/{OUT_OF_THE_BOX}/u64"
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(*missing_key_count as u64));

            expand_bench_with_map_data!(bench_out_of_the_box, u64, &mut group, &map_data_u64,
                AhashBenchMap<u64, u64>,
                BTreeMapBenchMap<u64, u64>,
                // ConcreadBenchMap<u64, u64>, // too slow
                ConcurrentMapBenchMap<u64, u64>,
                CrossbeamSkiplistBenchMap<u64, u64>,
                DashMapBenchMap<u64, u64>,
                // FlurryBenchMap<u64, u64>, // too slow
                HashbrownBenchMap<u64, u64>,
                HashlinkBenchMap<u64, u64>,
                HordeBenchMap<u64, u64>,
                ImmutableChunkMapBenchMap<u64, u64>,
                ImblBenchMap<u64, u64>,
                IndexMapBenchMap<u64, u64>,
                IntMapBenchMap<u64, u64>,
                LeapfrogBenchMap<u64, u64>,
                PapayaBenchMap<u64, u64>,
                RpdsHashTrieMapBenchMap<u64, u64>,
                RustCHashBenchMap<u64, u64>,
                SccBenchMap<u64, u64>,
                StarshardBenchMap<u64, u64>,
                StdBenchMap<u64, u64>,
                TxMapBenchMap<u64, u64>,
            );
        }

        // CommonHasher, u64 keys
        {
            let mut group = c.benchmark_group(format!(
                "growth/map-size-{entry_count_name}/{SAME_HASHER}/u64"
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(*missing_key_count as u64));

            expand_bench_with_map_data_and_common_hasher!(bench_same_hasher, u64, &mut group, &map_data_u64,
                AhashBenchMap<u64, u64, CommonHasher>,
                // BTreeMapBenchMap<u64, u64, CommonHasher>, // doesn't allow setting hasher
                // ConcreadBenchMap<u64, u64, CommonHasher>, // doesn't allow setting hasher
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

        // out of the box, String<32> keys
        {
            let mut group = c.benchmark_group(format!(
                "growth/map-size-{entry_count_name}/{OUT_OF_THE_BOX}/String<32>"
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(*missing_key_count as u64));

            expand_bench_with_map_data!(bench_out_of_the_box, String, &mut group, &map_data_string_32,
                AhashBenchMap<String, u64>,
                BTreeMapBenchMap<String, u64>,
                // ConcreadBenchMap<String, u64>, // too slow
                ConcurrentMapBenchMap<String, u64>,
                CrossbeamSkiplistBenchMap<String, u64>,
                DashMapBenchMap<String, u64>,
                // FlurryBenchMap<String, u64>, // too slow
                HashbrownBenchMap<String, u64>,
                HashlinkBenchMap<String, u64>,
                HordeBenchMap<String, u64>,
                ImmutableChunkMapBenchMap<String, u64>,
                ImblBenchMap<String, u64>,
                IndexMapBenchMap<String, u64>,
                // IntMapBenchMap<String, u64>, // keys require IntKey
                // LeapfrogBenchMap<String, u64>, // keys require Copy
                PapayaBenchMap<String, u64>,
                RpdsHashTrieMapBenchMap<String, u64>,
                RustCHashBenchMap<String, u64>,
                SccBenchMap<String, u64>,
                StarshardBenchMap<String, u64>,
                StdBenchMap<String, u64>,
                TxMapBenchMap<String, u64>,
            );
        }

        // CommonHasher, String<32> keys
        {
            let mut group = c.benchmark_group(format!(
                "growth/map-size-{entry_count_name}/{SAME_HASHER}/String<32>"
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(*missing_key_count as u64));

            expand_bench_with_map_data_and_common_hasher!(bench_same_hasher, String, &mut group, &map_data_string_32,
                AhashBenchMap<String, u64, CommonHasher>,
                // BTreeMapBenchMap<String, u64, CommonHasher>, // doesn't allow setting hasher
                // ConcreadBenchMap<String, u64, CommonHasher>, // doesn't allow setting hasher
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
}

criterion_group!(group, growth);
criterion_main!(group);
