// How does it scale from empty to its target size? Tests the capacity growth design — resizing/rehashing strategy and the amortized cost of expansion.
use bench_map::{
    config::*, constants::*, data::u64_sparse::U64SparseDataGen, expand_bench_with_map_data,
    expand_bench_with_map_data_and_hasher, map_data::MapData, map_gen::MapGen, maps::*,
    number_formatter::format_n,
};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main,
    measurement::WallTime,
};
use std::hint::black_box;

type CommonHasher = ahash::RandomState;

fn bench_out_of_the_box<Map>(
    name: &str,
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<u64, u64>,
) where
    Map: BenchMapNew<u64, u64> + BenchMapMutInsert<u64, u64>,
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

fn bench_same_hasher<Map>(
    name: &str,
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<u64, u64>,
    hasher: CommonHasher,
) where
    Map: BenchMapNewWithHasher<u64, u64, CommonHasher> + BenchMapMutInsert<u64, u64>,
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
    for missing_key_count in GROWTH_ENTRY_COUNTS {
        let map_data = MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            entry_count,
            existing_key_count,
            *missing_key_count,
            sort_keys,
        );

        // Each map uses its default hasher
        {
            let mut group = c.benchmark_group(format!(
                "growth/{OUT_OF_THE_BOX_GROUP_NAME}/map-size-{}/threads-1",
                format_n(*missing_key_count)
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(*missing_key_count as u64));

            expand_bench_with_map_data!(bench_out_of_the_box, &mut group, &map_data,
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

        // Every map that supports a custom hasher uses the same CommonHasher
        {
            let hasher = CommonHasher::new();
            let mut group = c.benchmark_group(format!(
                "growth/{SAME_HASHER_GROUP_NAME}/map-size-{}/threads-1",
                format_n(*missing_key_count)
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(*missing_key_count as u64));

            expand_bench_with_map_data_and_hasher!(
                bench_same_hasher,
                &mut group,
                &map_data,
                hasher,
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
    }
}

criterion_group!(group, growth);
criterion_main!(group);
