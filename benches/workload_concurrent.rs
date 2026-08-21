use bench_map::{
    concurrent_workers::ConcurrentWorkers,
    config::*,
    constants::*,
    data::u64_sparse::U64SparseDataGen,
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

fn workload_concurrent(c: &mut Criterion) {
    let max_threads = DEFAULT_THREAD_COUNTS.last().unwrap();
    for &entry_count in DEFAULT_ENTRY_COUNTS {
        let existing_key_count = entry_count;
        let missing_key_count = max_threads * DEFAULT_OP_COUNT;
        let sort_keys = false;
        let map_data = MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            entry_count,
            existing_key_count,
            missing_key_count,
            sort_keys,
        );

        let designs: &[(&str, WorkloadDesign)] = &[
            ("write-heavy", WorkloadDesign::write_heavy(DEFAULT_OP_COUNT)),
            ("balanced", WorkloadDesign::balanced(DEFAULT_OP_COUNT)),
            ("read-heavy", WorkloadDesign::read_heavy(DEFAULT_OP_COUNT)),
        ];

        for &(name, design) in designs {
            let mut rng = rand::rng();
            for &thread_count in DEFAULT_THREAD_COUNTS {
                let total_ops = thread_count * DEFAULT_OP_COUNT;
                let workloads = (0..thread_count)
                    .map(|_| {
                        KeyDistribution::Uniform.thread_workload(
                            &design,
                            map_data.existing_keys(),
                            map_data.missing_keys(),
                            &mut rng,
                        )
                    })
                    .collect::<Vec<_>>();

                let mut group = c.benchmark_group(format!(
                    "workload/{OUT_OF_THE_BOX_GROUP_NAME}/{}/map-size-{}/threads-{}",
                    name,
                    format_n(entry_count),
                    thread_count
                ));
                group.warm_up_time(WARM_UP_TIME);
                group.measurement_time(MEASUREMENT_TIME);
                group.throughput(Throughput::Elements(total_ops as u64));

                // bench::<AhashBenchMap<u64, u64>>("ahash", &mut group, &map_data, thread_count, &workloads); // not concurrent
                // bench::<BTreeMapBenchMap<u64, u64>>("btreemap", &mut group, &map_data, thread_count, &workloads); // not concurrent
                // bench::<ConcreadBenchMap<u64, u64>>("concread", &mut group, &map_data, thread_count, &workloads); // too slow
                // bench::<ConcurrentMapBenchMap<u64, u64>>("concurrent-map", &mut group, &map_data, thread_count, &workloads); // Send but not Sync; cannot share &ConcurrentMap across threads
                bench::<CrossbeamSkiplistBenchMap<u64, u64>>(
                    "crossbeam-skiplist",
                    &mut group,
                    &map_data,
                    thread_count,
                    &workloads,
                );
                bench::<DashMapBenchMap<u64, u64>>(
                    "dashmap",
                    &mut group,
                    &map_data,
                    thread_count,
                    &workloads,
                );
                // bench::<FlurryBenchMap<u64, u64>>("flurry", &mut group, &map_data, thread_count, &workloads); // too slow
                // bench::<HashbrownBenchMap<u64, u64>>("hashbrown", &mut group, &map_data, thread_count, &workloads); // not concurrent
                // bench::<HashlinkBenchMap<u64, u64>>("hashlink", &mut group, &map_data, thread_count, &workloads); // mutation requires &mut, cannot mutate through a shared reference
                // bench::<HordeBenchMap<u64, u64>>("horde", &mut group, &map_data, thread_count, &workloads); // mutation requires &mut, cannot mutate through a shared reference
                // bench::<ImmutableChunkMapBenchMap<u64, u64>>("immutable-chunkmap", &mut group, &map_data, thread_count, &workloads); // mutation returns a new map; requires &mut or storing the result, cannot mutate through a shared reference
                // bench::<ImblBenchMap<u64, u64>>("imbl", &mut group, &map_data, thread_count, &workloads); // mutation requires &mut, cannot mutate through a shared reference
                // bench::<IndexMapBenchMap<u64, u64>>("indexmap", &mut group, &map_data, thread_count, &workloads); // not concurrent
                bench::<LeapfrogBenchMap<u64, u64>>(
                    "leapfrog",
                    &mut group,
                    &map_data,
                    thread_count,
                    &workloads,
                );
                bench::<PapayaBenchMap<u64, u64>>(
                    "papaya",
                    &mut group,
                    &map_data,
                    thread_count,
                    &workloads,
                );
                // bench::<RpdsHashTrieMapBenchMap<u64, u64>>("rpds-hash-trie-map", &mut group, &map_data, thread_count, &workloads); // mutation returns a new map; requires &mut or storing the result, cannot mutate through a shared reference (and the default Rc pointer is not Send/Sync)
                // bench::<RustCHashBenchMap<u64, u64>>("rustc-hash", &mut group, &map_data, thread_count, &workloads); // not concurrent
                bench::<SccBenchMap<u64, u64>>(
                    "scc",
                    &mut group,
                    &map_data,
                    thread_count,
                    &workloads,
                );
                bench::<StarshardBenchMap<u64, u64>>(
                    "starshard",
                    &mut group,
                    &map_data,
                    thread_count,
                    &workloads,
                );
                // bench::<StdBenchMap<u64, u64>>("std", &mut group, &map_data, thread_count, &workloads); // not concurrent
                bench::<TxMapBenchMap<u64, u64>>(
                    "txmap",
                    &mut group,
                    &map_data,
                    thread_count,
                    &workloads,
                );
            }
        }
    }
}

criterion_group!(group, workload_concurrent);
criterion_main!(group);
