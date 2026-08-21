// How well does it cope when threads compete for the same keys? Tests synchronization granularity, how sharding/locking handles hot spots under uniform and skewed (Zipfian) key distributions.
use bench_map::{
    concurrent_workers::ConcurrentWorkers,
    config::*,
    constants::*,
    data::{u64_dense::U64DenseDataGen, u64_sparse::U64SparseDataGen},
    expand_bench_concurrent,
    map_data::MapData,
    map_gen::MapGen,
    maps::*,
    number_formatter::format_n,
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
use std::{cell::Cell, hint::black_box, sync::Arc};

fn run_workload<M>(workload: &ThreadWorkload, map: &M)
where
    M: BenchMapGetCloned<u64, u64>,
    M: BenchMapInsert<u64, u64>,
    M: BenchMapRemove<u64, u64>,
{
    for item in &workload.items {
        match item.op {
            WorkloadOp::Lookup => {
                let key = black_box(&item.key);
                black_box(map.get_cloned(key));
            }
            WorkloadOp::Insert => {
                let key = black_box(item.key);
                map.insert(key, 42u64);
            }
            WorkloadOp::Remove => {
                let key = black_box(&item.key);
                black_box(map.remove(key));
            }
        }
    }
}

fn bench<Map>(
    name: &str,
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<u64, u64>,
    thread_count: usize,
    workloads: &[ThreadWorkload],
) where
    Map: BenchMapNew<u64, u64>
        + BenchMapMutInsert<u64, u64>
        + BenchMapGetCloned<u64, u64>
        + BenchMapInsert<u64, u64>
        + BenchMapRemove<u64, u64>
        + Send
        + Sync
        + 'static,
{
    group.bench_function(name, move |b| {
        // Spawn and pin the worker threads once per sample, outside the timed
        // region, so thread spawn/join and CPU-pinning costs are amortized
        // instead of being measured on every iteration.
        let workers = ConcurrentWorkers::<ThreadWorkload, Map>::new(
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

        let mut group = c.benchmark_group(format!(
            "contention/{OUT_OF_THE_BOX_GROUP_NAME}/{}/map-size-{}/threads-{}",
            key_distribution,
            format_n(DEFAULT_ENTRY_COUNT),
            DEFAULT_THREAD_COUNT
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(total_ops as u64));

        expand_bench_concurrent!(bench, &mut group, &map_data, DEFAULT_THREAD_COUNT, &workloads,
            // AhashBenchMap<u64, u64>, // not concurrent
            // BTreeMapBenchMap<u64, u64>, // not concurrent
            // ConcreadBenchMap<u64, u64>, // too slow
            // ConcurrentMapBenchMap<u64, u64>, // Send but not Sync; cannot share &ConcurrentMap across threads
            CrossbeamSkiplistBenchMap<u64, u64>,
            DashMapBenchMap<u64, u64>,
            // FlurryBenchMap<u64, u64>, // too slow
            // HashbrownBenchMap<u64, u64>, // not concurrent
            // HashlinkBenchMap<u64, u64>, // mutation requires &mut, cannot mutate through a shared reference
            // HordeBenchMap<u64, u64>, // mutation requires &mut, cannot mutate through a shared reference
            // ImmutableChunkMapBenchMap<u64, u64>,; // mutation returns a new map; requires &mut or storing the result, cannot mutate through a shared reference
            // ImblBenchMap<u64, u64>, // mutation requires &mut, cannot mutate through a shared reference
            // IndexMapBenchMap<u64, u64>, // not concurrent
            LeapfrogBenchMap<u64, u64>,
            PapayaBenchMap<u64, u64>,
            // RpdsHashTrieMapBenchMap<u64, u64> // mutation returns a new map; requires &mut or storing the result, cannot mutate through a shared reference (and the default Rc pointer is not Send/Sync)
            // RustCHashBenchMap<u64, u64> // not concurrent
            SccBenchMap<u64, u64>,
            StarshardBenchMap<u64, u64>,
            // StdBenchMap<u64, u64>, // not concurrent
            TxMapBenchMap<u64, u64>,
        );
    }
}

criterion_group!(group, contention);
criterion_main!(group);
