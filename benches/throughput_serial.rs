// How does it perform on realistic single-threaded use? Tests the combined read/write/remove design, the interplay of all operations in one pass without concurrency overhead.
use bench_map::{
    common_hasher::CommonHasher,
    config::*,
    constants::*,
    data::u64_sparse::U64SparseDataGen,
    expand_bench_concurrent, expand_bench_concurrent_with_common_hasher,
    map_data::MapData,
    map_gen::MapGen,
    maps::*,
    workload::{
        design::WorkloadDesign,
        op::WorkloadOp,
        thread_workload::{KeyDistribution, ThreadWorkload},
    },
};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main,
    measurement::WallTime,
};
use std::{hash::Hash, hint::black_box};

fn run_workload<M, K>(workload: &ThreadWorkload<K>, map: &mut M)
where
    M: BenchMapGetCloned<K, u64>,
    M: BenchMapMutInsert<K, u64>,
    M: BenchMapMutRemove<K, u64>,
    K: Clone,
{
    for item in &workload.items {
        match item.op {
            WorkloadOp::Lookup => {
                let key = black_box(&item.key);
                black_box(map.get_cloned(key));
            }
            WorkloadOp::Insert => {
                let key = black_box(item.key.clone());
                map.insert(key, 42u64);
            }
            WorkloadOp::Remove => {
                let key = black_box(&item.key);
                black_box(map.remove(key));
            }
        }
    }
}

fn bench_out_of_the_box<Map, K>(
    name: &str,
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<K, u64>,
    _thread_count_1: usize,
    workload: &ThreadWorkload<K>,
) where
    Map: BenchMapNew<K, u64>
        + BenchMapMutInsert<K, u64>
        + BenchMapMutRemove<K, u64>
        + BenchMapGetCloned<K, u64>,
    K: Clone + Hash + Eq,
{
    group.bench_function(name, move |b| {
        b.iter_batched(
            || map_data.create_map::<Map>(),
            |mut map| {
                run_workload(workload, &mut map);
            },
            BatchSize::PerIteration,
        );
    });
}

fn bench_same_hasher<Map, K>(
    name: &str,
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<K, u64>,
    _thread_count_1: usize,
    workload: &ThreadWorkload<K>,
    hasher: CommonHasher,
) where
    Map: BenchMapNewWithHasher<K, u64, CommonHasher>
        + BenchMapMutInsert<K, u64>
        + BenchMapMutRemove<K, u64>
        + BenchMapGetCloned<K, u64>,
    K: Clone + Hash + Eq,
{
    group.bench_function(name, move |b| {
        b.iter_batched(
            || map_data.create_map_with_hasher::<Map, CommonHasher>(hasher.clone()),
            |mut map| {
                run_workload(workload, &mut map);
            },
            BatchSize::PerIteration,
        );
    });
}

fn throughput_serial(c: &mut Criterion) {
    let entry_count = DEFAULT_ENTRY_COUNT;
    let existing_key_count = entry_count;
    let missing_key_count = DEFAULT_OP_COUNT;
    let sort_keys = false;
    let map_data = MapGen::generate(
        U64SparseDataGen,
        U64SparseDataGen,
        entry_count,
        existing_key_count,
        missing_key_count,
        sort_keys,
    );

    let mut rng = rand::rng();
    let designs: &[(&str, WorkloadDesign)] = &[
        ("write-heavy", WorkloadDesign::write_heavy(DEFAULT_OP_COUNT)),
        ("balanced", WorkloadDesign::balanced(DEFAULT_OP_COUNT)),
        ("read-heavy", WorkloadDesign::read_heavy(DEFAULT_OP_COUNT)),
    ];

    for &(name, design) in designs {
        let workload = KeyDistribution::Uniform.thread_workload(
            &design,
            map_data.existing_keys(),
            map_data.missing_keys(),
            &mut rng,
        );

        // default hashers
        {
            let mut group =
                c.benchmark_group(format!("throughput/threads-1/{DEFAULT_HASHER}/{}", name));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(DEFAULT_OP_COUNT as u64));

            expand_bench_concurrent!(bench_out_of_the_box, u64, &mut group, &map_data, 1, &workload,
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

        // CommonHasher
        {
            let mut group =
                c.benchmark_group(format!("throughput/threads-1/{SAME_HASHER}/{}", name));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(DEFAULT_OP_COUNT as u64));

            expand_bench_concurrent_with_common_hasher!(bench_same_hasher, u64, &mut group, &map_data, 1, &workload,
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
    }
}

criterion_group!(group, throughput_serial);
criterion_main!(group);
