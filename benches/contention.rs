// How well does it cope when threads compete for the same keys? Tests synchronization granularity, how sharding/locking handles hot spots under uniform and skewed (Zipfian) key distributions.
use bench_map::{
    common_hasher::CommonHasher,
    concurrent_workers::ConcurrentWorkers,
    config::*,
    constants::*,
    data::{u64_dense::U64DenseDataGen, u64_sparse::U64SparseDataGen},
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
use std::{cell::Cell, hash::Hash, hint::black_box, sync::Arc};

fn run_workload<K, M>(workload: &ThreadWorkload<K>, map: &M)
where
    M: BenchMapGetCloned<K, u64>,
    M: BenchMapInsert<K, u64>,
    M: BenchMapRemove<K, u64>,
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
    thread_count: usize,
    workloads: &[ThreadWorkload<K>],
) where
    Map: BenchMapNew<K, u64>
        + BenchMapMutInsert<K, u64>
        + BenchMapGetCloned<K, u64>
        + BenchMapInsert<K, u64>
        + BenchMapRemove<K, u64>
        + Send
        + Sync
        + 'static,
    K: Clone + Hash + Eq + Send + Sync + 'static,
{
    group.bench_function(name, move |b| {
        // Spawn and pin the worker threads once per sample, outside the timed
        // region, so thread spawn/join and CPU-pinning costs are amortized
        // instead of being measured on every iteration.
        let workers = ConcurrentWorkers::<ThreadWorkload<K>, Map>::new(
            thread_count,
            workloads,
            |workload, map| run_workload(workload, map),
        );
        // 1-based index of the iteration about to be timed; used to derive the
        // cumulative `done` target for the worker pool.
        let iteration = Cell::new(0usize);
        b.iter_batched(
            || {
                // Untimed setup: publish a fresh map for this iteration.
                let map = Arc::new(map_data.create_map::<Map>());
                for slot in workers.slots() {
                    *slot.lock().unwrap() = Some(map.clone());
                }
                iteration.set(iteration.get() + 1);
                map
            },
            |map| {
                // Timed region: release the workers and wait for all of them
                // to finish their workload.
                let target = iteration.get() * thread_count;
                workers.run(target);
                map
            },
            BatchSize::PerIteration,
        );
        // The worker pool is shut down and joined here, outside the timed
        // region (its `Drop` implementation).
    });
}

fn bench_same_hasher<Map, K>(
    name: &str,
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<K, u64>,
    thread_count: usize,
    workloads: &[ThreadWorkload<K>],
    hasher: CommonHasher,
) where
    Map: BenchMapNewWithHasher<K, u64, CommonHasher>
        + BenchMapMutInsert<K, u64>
        + BenchMapGetCloned<K, u64>
        + BenchMapInsert<K, u64>
        + BenchMapRemove<K, u64>
        + Send
        + Sync
        + 'static,
    K: Clone + Hash + Eq + Send + Sync + 'static,
{
    group.bench_function(name, move |b| {
        // Spawn and pin the worker threads once per sample, outside the timed
        // region, so thread spawn/join and CPU-pinning costs are amortized
        // instead of being measured on every iteration.
        let workers = ConcurrentWorkers::<ThreadWorkload<K>, Map>::new(
            thread_count,
            workloads,
            |workload, map| run_workload(workload, map),
        );
        // 1-based index of the iteration about to be timed; used to derive the
        // cumulative `done` target for the worker pool.
        let iteration = Cell::new(0usize);
        b.iter_batched(
            || {
                // Untimed setup: publish a fresh map for this iteration.
                let map =
                    Arc::new(map_data.create_map_with_hasher::<Map, CommonHasher>(hasher.clone()));
                for slot in workers.slots() {
                    *slot.lock().unwrap() = Some(map.clone());
                }
                iteration.set(iteration.get() + 1);
                map
            },
            |map| {
                // Timed region: release the workers and wait for all of them
                // to finish their workload.
                let target = iteration.get() * thread_count;
                workers.run(target);
                map
            },
            BatchSize::PerIteration,
        );
        // The worker pool is shut down and joined here, outside the timed
        // region (its `Drop` implementation).
    });
}

fn contention(c: &mut Criterion) {
    let design = WorkloadDesign::contention(DEFAULT_OP_COUNT);
    let missing_key_count = DEFAULT_THREAD_COUNT * DEFAULT_OP_COUNT;
    let map_data = MapGen::generate(
        U64DenseDataGen,
        U64SparseDataGen,
        DEFAULT_ENTRY_COUNT,
        DEFAULT_ENTRY_COUNT,
        missing_key_count,
        true,
    );

    let key_distributions = vec![
        KeyDistribution::Uniform,
        KeyDistribution::Zipfian(1.0),
        KeyDistribution::Zipfian(2.0),
    ];

    for key_distribution in key_distributions {
        let mut rng = rand::rng();
        let total_ops = DEFAULT_THREAD_COUNT * DEFAULT_OP_COUNT;
        let workloads = (0..DEFAULT_THREAD_COUNT)
            .map(|_| {
                key_distribution.thread_workload(
                    &design,
                    map_data.existing_keys(),
                    map_data.missing_keys(),
                    &mut rng,
                )
            })
            .collect::<Vec<_>>();

        // default hashers
        {
            let mut group = c.benchmark_group(format!(
                "contention-{DEFAULT_THREAD_COUNT}-threads/{OUT_OF_THE_BOX_GROUP_NAME}/{}",
                key_distribution
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(total_ops as u64));

            expand_bench_concurrent!(bench_out_of_the_box, u64, &mut group, &map_data, DEFAULT_THREAD_COUNT, &workloads,
                // AhashBenchMap<u64, u64>, // not concurrent
                // BTreeMapBenchMap<u64, u64>, // not concurrent
                // ConcreadBenchMap<u64, u64>, // too slow
                ConcurrentMapBenchMap<u64, u64>,
                CrossbeamSkiplistBenchMap<u64, u64>,
                DashMapBenchMap<u64, u64>,
                // FlurryBenchMap<u64, u64>, // too slow
                // HashbrownBenchMap<u64, u64>, // not concurrent
                // HashlinkBenchMap<u64, u64>, // mutation requires &mut, cannot mutate through a shared reference
                // HordeBenchMap<u64, u64>, // mutation requires &mut, cannot mutate through a shared reference
                // ImmutableChunkMapBenchMap<u64, u64>, // mutation returns a new map; requires &mut or storing the result, cannot mutate through a shared reference
                // ImblBenchMap<u64, u64>, // mutation requires &mut, cannot mutate through a shared reference
                // IndexMapBenchMap<u64, u64>, // not concurrent
                // IntMapBenchMap<u64, u64>, // mutation requires &mut, cannot mutate through a shared reference
                LeapfrogBenchMap<u64, u64>,
                PapayaBenchMap<u64, u64>,
                // RpdsHashTrieMapBenchMap<u64, u64> // mutation returns a new map; requires &mut or storing the result, cannot mutate through a shared reference
                // RustCHashBenchMap<u64, u64> // not concurrent
                SccBenchMap<u64, u64>,
                StarshardBenchMap<u64, u64>,
                // StdBenchMap<u64, u64>, // not concurrent
                TxMapBenchMap<u64, u64>,
            );
        }

        // CommonHasher
        {
            let mut group = c.benchmark_group(format!(
                "contention-{DEFAULT_THREAD_COUNT}-threads/{SAME_HASHER_GROUP_NAME}/{}",
                key_distribution
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(total_ops as u64));

            expand_bench_concurrent_with_common_hasher!(bench_same_hasher, u64, &mut group, &map_data, DEFAULT_THREAD_COUNT, &workloads,
                // AhashBenchMap<u64, u64, CommonHasher>, // not concurrent
                // BTreeMapBenchMap<u64, u64, CommonHasher>, // not concurrent
                // ConcreadBenchMap<u64, u64, CommonHasher>, // too slow
                // ConcurrentMapBenchMap<u64, u64, CommonHasher>, // doesn't allow setting hasher
                // CrossbeamSkiplistBenchMap<u64, u64, CommonHasher>, // doesn't allow setting hasher
                DashMapBenchMap<u64, u64, CommonHasher>,
                // FlurryBenchMap<u64, u64, CommonHasher>, // too slow
                // HashbrownBenchMap<u64, u64, CommonHasher>, // not concurrent
                // HashlinkBenchMap<u64, u64, CommonHasher>, // mutation requires &mut, cannot mutate through a shared reference
                // HordeBenchMap<u64, u64, CommonHasher>, // mutation requires &mut, cannot mutate through a shared reference
                // ImmutableChunkMapBenchMap<u64, u64, CommonHasher>, // mutation returns a new map; requires &mut or storing the result, cannot mutate through a shared reference
                // ImblBenchMap<u64, u64, CommonHasher>, // mutation requires &mut, cannot mutate through a shared reference
                // IndexMapBenchMap<u64, u64, CommonHasher>, // not concurrent
                // IntMapBenchMap<u64, u64, CommonHasher>, // mutation requires &mut, cannot mutate through a shared reference
                LeapfrogBenchMap<u64, u64, CommonHasher>,
                PapayaBenchMap<u64, u64, CommonHasher>,
                // RpdsHashTrieMapBenchMap<u64, u64, CommonHasher>, // mutation returns a new map; requires &mut or storing the result, cannot mutate through a shared reference
                // RustCHashBenchMap<u64, u64, CommonHasher>, // not concurrent
                SccBenchMap<u64, u64, CommonHasher>,
                StarshardBenchMap<u64, u64, CommonHasher>,
                // StdBenchMap<u64, u64, CommonHasher>, // not concurrent
                TxMapBenchMap<u64, u64, CommonHasher>,
            );
        }
    }
}

criterion_group!(group, contention);
criterion_main!(group);
