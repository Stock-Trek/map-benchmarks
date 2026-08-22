// Can a map be recycled as a reusable pool without paying rebuild costs? Tests capacity-retention semantics, whether clearing preserves the underlying allocation so refilling avoids reallocation and re-growth.
use bench_map::{
    common_hasher::CommonHasher, config::*, constants::*, data::u64_sparse::U64SparseDataGen,
    expand_bench_with_map_data, expand_bench_with_map_data_and_common_hasher, map_data::MapData,
    map_gen::MapGen, maps::*,
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
    Map: BenchMapNew<K, u64> + BenchMapMutInsert<K, u64> + BenchMapMutClear<K, u64>,
    K: Clone + Hash + Eq,
{
    group.bench_function(name, move |b| {
        let map_data_ref = &map_data;
        b.iter_batched(
            move || {
                // "Pool" setup: a fully populated map.
                let map = map_data_ref.create_map::<Map>();
                let keys = map_data_ref.missing_keys().clone();
                (map, keys)
            },
            // Measured cycle: empty the map but keep it alive, then refill it.
            |(mut map, mut keys)| {
                map.clear();
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
    Map: BenchMapNewWithHasher<K, u64, CommonHasher>
        + BenchMapMutInsert<K, u64>
        + BenchMapMutClear<K, u64>,
    K: Clone + Hash + Eq,
{
    group.bench_function(name, move |b| {
        let map_data_ref = &map_data;
        let hasher = hasher.clone();
        b.iter_batched(
            move || {
                // "Pool" setup: a fully populated map.
                let map = map_data_ref.create_map_with_hasher::<Map, CommonHasher>(hasher.clone());
                let keys = map_data_ref.missing_keys().clone();
                (map, keys)
            },
            // Measured cycle: empty the map but keep it alive, then refill it.
            |(mut map, mut keys)| {
                map.clear();
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

fn clear_and_reuse(c: &mut Criterion) {
    let existing_key_count = 0;
    let sort_keys = false;
    for (entry_count, entry_count_name) in DEFAULT_ENTRY_COUNTS {
        let missing_key_count = *entry_count;
        let map_data = MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            *entry_count,
            existing_key_count,
            missing_key_count,
            sort_keys,
        );

        // Each map uses its default hasher
        {
            let mut group = c.benchmark_group(format!(
                "clear-and-reuse/map-size-{}/{OUT_OF_THE_BOX_GROUP_NAME}",
                entry_count_name
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(*entry_count as u64));

            expand_bench_with_map_data!(bench_out_of_the_box, u64, &mut group, &map_data,
                AhashBenchMap<u64, u64>,
                BTreeMapBenchMap<u64, u64>,
                // ConcreadBenchMap<u64, u64>, // too slow
                // ConcurrentMapBenchMap<u64, u64>, // no clear
                CrossbeamSkiplistBenchMap<u64, u64>,
                DashMapBenchMap<u64, u64>,
                // FlurryBenchMap<u64, u64>, // too slow
                HashbrownBenchMap<u64, u64>,
                HashlinkBenchMap<u64, u64>,
                HordeBenchMap<u64, u64>,
                // ImmutableChunkMapBenchMap<u64, u64>, // no clear
                ImblBenchMap<u64, u64>,
                IndexMapBenchMap<u64, u64>,
                IntMapBenchMap<u64, u64>,
                // LeapfrogBenchMap<u64, u64>, // no clear
                PapayaBenchMap<u64, u64>,
                // RpdsHashTrieMapBenchMap<u64, u64>, // no clear
                RustCHashBenchMap<u64, u64>,
                SccBenchMap<u64, u64>,
                StarshardBenchMap<u64, u64>,
                StdBenchMap<u64, u64>,
                TxMapBenchMap<u64, u64>,
            );
        }

        // Every map that supports a custom hasher uses the same CommonHasher
        {
            let mut group = c.benchmark_group(format!(
                "clear-and-reuse/map-size-{}/{SAME_HASHER_GROUP_NAME}",
                entry_count_name
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(*entry_count as u64));

            expand_bench_with_map_data_and_common_hasher!(bench_same_hasher, u64, &mut group, &map_data,
                AhashBenchMap<u64, u64, CommonHasher>,
                // BTreeMapBenchMap<u64, u64, CommonHasher>, // doesn't allow setting hasher
                // ConcreadBenchMap<u64, u64, CommonHasher>, // doesn't allow setting hasher
                // ConcurrentMapBenchMap<u64, u64, CommonHasher>, // doesn't allow setting hasher
                DashMapBenchMap<u64, u64, CommonHasher>,
                // FlurryBenchMap<u64, u64, CommonHasher>, // too slow
                HashbrownBenchMap<u64, u64, CommonHasher>,
                HashlinkBenchMap<u64, u64, CommonHasher>,
                HordeBenchMap<u64, u64, CommonHasher>,
                // ImmutableChunkMapBenchMap<u64, u64, CommonHasher>, // doesn't allow setting hasher
                ImblBenchMap<u64, u64, CommonHasher>,
                IndexMapBenchMap<u64, u64, CommonHasher>,
                // LeapfrogBenchMap<u64, u64, CommonHasher>, // doesn't allow setting hasher
                PapayaBenchMap<u64, u64, CommonHasher>,
                // RpdsHashTrieMapBenchMap<u64, u64, CommonHasher>, // doesn't allow setting hasher
                // RustCHashBenchMap<u64, u64, CommonHasher>, // doesn't allow setting hasher
                SccBenchMap<u64, u64, CommonHasher>,
                StarshardBenchMap<u64, u64, CommonHasher>,
                StdBenchMap<u64, u64, CommonHasher>,
                TxMapBenchMap<u64, u64, CommonHasher>,
            );
        }
    }
}

criterion_group!(group, clear_and_reuse);
criterion_main!(group);
