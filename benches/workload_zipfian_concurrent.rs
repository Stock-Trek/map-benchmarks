use bench_map::{
    concurrent_workers::ConcurrentWorkers,
    config::*,
    constants::*,
    data::{data_gen::DataGen, u64_sparse::U64SparseDataGen, u64_zipfian::U64ZipfianDataGen},
    map_data::MapData,
    maps::*,
    number_formatter::format_n,
    workload::{design::WorkloadDesign, op::WorkloadOp, thread_workload::ThreadWorkload},
};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main,
    measurement::WallTime,
};
use hashbrown::HashSet;
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
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<u64, u64>,
    thread_count: usize,
    workloads: &[ThreadWorkload],
    name: &str,
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

/// Builds the skewed map data for the Zipfian workload.
///
/// The entry keys are the hottest `entry_count` keys of a Zipfian distribution
/// over a larger key space (`entry_count * 2`), so the sorted existing-key
/// slice ranks keys from hottest (index 0) to coldest (index len - 1) and the
/// hot entries dominate the map. Missing keys are random u64s, which with
/// overwhelming probability are not present in the map.
fn generate_zipfian_map_data(
    entry_count: usize,
    missing_key_count: usize,
    exponent: f64,
) -> MapData<u64, u64> {
    let key_space = entry_count * WORKLOAD_ZIPFIAN_KEY_SPACE_MULTIPLIER;
    let mut entry_keys = U64ZipfianDataGen::new(key_space as u64, exponent)
        .generate(entry_count)
        .into_iter()
        .collect::<Vec<_>>();
    entry_keys.sort_unstable();
    let entry_set = entry_keys.iter().copied().collect::<HashSet<_>>();
    let mut missing_keys = U64SparseDataGen
        .generate_avoiding(missing_key_count, &entry_set)
        .into_iter()
        .collect::<Vec<_>>();
    missing_keys.sort_unstable();

    let values = U64SparseDataGen.generate(entry_count);
    let entries = entry_keys.iter().copied().zip(values).collect();
    MapData::new(entries, entry_keys, missing_keys)
}

fn workload_zipfian_concurrent(c: &mut Criterion) {
    let max_threads = WORKLOAD_CONCURRENT_THREAD_COUNTS.last().unwrap();
    for &entry_count in WORKLOAD_ENTRY_COUNT {
        let missing_key_count = max_threads * WORKLOAD_OP_COUNT;
        let map_data =
            generate_zipfian_map_data(entry_count, missing_key_count, WORKLOAD_ZIPFIAN_EXPONENT);

        let designs: &[(&str, WorkloadDesign)] = &[
            (
                "write-heavy",
                WorkloadDesign::write_heavy(WORKLOAD_OP_COUNT),
            ),
            ("balanced", WorkloadDesign::balanced(WORKLOAD_OP_COUNT)),
            ("read-heavy", WorkloadDesign::read_heavy(WORKLOAD_OP_COUNT)),
        ];

        for &(name, design) in designs {
            let mut rng = rand::rng();
            for &thread_count in WORKLOAD_CONCURRENT_THREAD_COUNTS {
                let total_ops = thread_count * WORKLOAD_OP_COUNT;
                let workloads = (0..thread_count)
                    .map(|_| {
                        ThreadWorkload::new_zipfian(
                            &design,
                            map_data.existing_keys(),
                            map_data.missing_keys(),
                            &mut rng,
                            WORKLOAD_ZIPFIAN_EXPONENT,
                        )
                    })
                    .collect::<Vec<_>>();

                let mut group = c.benchmark_group(format!(
                    "workload/{OUT_OF_THE_BOX_GROUP_NAME}/zipfian/{}/map-size-{}/threads-{}",
                    name,
                    format_n(entry_count),
                    thread_count
                ));
                group.warm_up_time(WARM_UP_TIME);
                group.measurement_time(MEASUREMENT_TIME);
                group.throughput(Throughput::Elements(total_ops as u64));

                // bench::<AhashBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "ahash"); // not concurrent
                // bench::<BTreeMapBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "btreemap"); // not concurrent
                // bench::<ConcreadBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "concread"); // too slow
                // bench::<ConcurrentMapBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "concurrent-map"); // Send but not Sync; cannot share &ConcurrentMap across threads
                bench::<CrossbeamSkiplistBenchMap<u64, u64>>(
                    &mut group,
                    &map_data,
                    thread_count,
                    &workloads,
                    "crossbeam-skiplist",
                );
                bench::<DashMapBenchMap<u64, u64>>(
                    &mut group,
                    &map_data,
                    thread_count,
                    &workloads,
                    "dashmap",
                );
                // bench::<FlurryBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "flurry"); // too slow
                // bench::<HashbrownBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "hashbrown"); // not concurrent
                // bench::<HashlinkBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "hashlink"); // mutation requires &mut, cannot mutate through a shared reference
                // bench::<HordeBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "horde"); // mutation requires &mut, cannot mutate through a shared reference
                // bench::<ImmutableChunkMapBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "immutable-chunkmap"); // mutation returns a new map; requires &mut or storing the result, cannot mutate through a shared reference
                // bench::<ImblBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "imbl"); // mutation requires &mut, cannot mutate through a shared reference
                // bench::<IndexMapBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "indexmap"); // not concurrent
                bench::<LeapfrogBenchMap<u64, u64>>(
                    &mut group,
                    &map_data,
                    thread_count,
                    &workloads,
                    "leapfrog",
                );
                bench::<PapayaBenchMap<u64, u64>>(
                    &mut group,
                    &map_data,
                    thread_count,
                    &workloads,
                    "papaya",
                );
                // bench::<RpdsHashTrieMapBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "rpds-hash-trie-map"); // mutation returns a new map; requires &mut or storing the result, cannot mutate through a shared reference (and the default Rc pointer is not Send/Sync)
                // bench::<RustCHashBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "rustc-hash"); // not concurrent
                bench::<SccBenchMap<u64, u64>>(
                    &mut group,
                    &map_data,
                    thread_count,
                    &workloads,
                    "scc",
                );
                bench::<StarshardBenchMap<u64, u64>>(
                    &mut group,
                    &map_data,
                    thread_count,
                    &workloads,
                    "starshard",
                );
                // bench::<StdBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "std"); // not concurrent
                bench::<TxMapBenchMap<u64, u64>>(
                    &mut group,
                    &map_data,
                    thread_count,
                    &workloads,
                    "txmap",
                );
            }
        }
    }
}

criterion_group!(group, workload_zipfian_concurrent);
criterion_main!(group);
