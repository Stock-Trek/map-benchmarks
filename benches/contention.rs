use bench_map::{
    concurrent_workers::ConcurrentWorkers,
    config::*,
    constants::*,
    data::{u64_dense::U64DenseDataGen, u64_sparse::U64SparseDataGen},
    map_data::MapData,
    map_gen::MapGen,
    maps::*,
    number_formatter::format_n,
    workload::{design::WorkloadDesign, op::WorkloadOp, thread_workload::ThreadWorkload},
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

/// Builds the map data used by every contention test: a dense set of
/// `entry_count` consecutive u64 keys (a compact addressable range), so all
/// three tests measure contention on the same table shape. The query key
/// distributions (uniform / Zipfian) are drawn from this dense key set when
/// the per-thread workloads are generated, not from the map population itself.
fn generate_contention_map_data(entry_count: usize, missing_key_count: usize) -> MapData<u64, u64> {
    MapGen::generate(
        U64DenseDataGen,
        U64SparseDataGen,
        entry_count,
        entry_count,
        missing_key_count,
        true,
    )
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

fn contention(c: &mut Criterion) {
    let design = WorkloadDesign::contention(CONTENTION_OP_COUNT);
    let missing_key_count = CONTENTION_THREAD_COUNT * CONTENTION_OP_COUNT;
    let map_data = generate_contention_map_data(CONTENTION_ENTRY_COUNT, missing_key_count);

    // The three query key distributions, all drawn from the dense map key set:
    // `None` selects keys uniformly (the low-contention baseline), while
    // `Some(exponent)` selects keys from a Zipfian distribution with that
    // exponent, with higher exponents concentrating more traffic on the
    // hottest keys.
    let tests: &[(&str, Option<f64>)] = &[
        ("uniform", None),
        ("zipfian-exp-1", Some(1.0)),
        ("zipfian-exp-2", Some(2.0)),
    ];

    for &(name, zipfian_exponent) in tests {
        let mut rng = rand::rng();
        let total_ops = CONTENTION_THREAD_COUNT * CONTENTION_OP_COUNT;
        let workloads = (0..CONTENTION_THREAD_COUNT)
            .map(|_| match zipfian_exponent {
                None => ThreadWorkload::new(
                    &design,
                    map_data.existing_keys(),
                    map_data.missing_keys(),
                    &mut rng,
                ),
                Some(exponent) => ThreadWorkload::new_zipfian(
                    &design,
                    map_data.existing_keys(),
                    map_data.missing_keys(),
                    &mut rng,
                    exponent,
                ),
            })
            .collect::<Vec<_>>();

        let mut group = c.benchmark_group(format!(
            "contention/{OUT_OF_THE_BOX_GROUP_NAME}/{}/map-size-{}/threads-{}",
            name,
            format_n(CONTENTION_ENTRY_COUNT),
            CONTENTION_THREAD_COUNT
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
            CONTENTION_THREAD_COUNT,
            &workloads,
            "crossbeam-skiplist",
        );
        bench::<DashMapBenchMap<u64, u64>>(
            &mut group,
            &map_data,
            CONTENTION_THREAD_COUNT,
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
            CONTENTION_THREAD_COUNT,
            &workloads,
            "leapfrog",
        );
        bench::<PapayaBenchMap<u64, u64>>(
            &mut group,
            &map_data,
            CONTENTION_THREAD_COUNT,
            &workloads,
            "papaya",
        );
        // bench::<RpdsHashTrieMapBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "rpds-hash-trie-map"); // mutation returns a new map; requires &mut or storing the result, cannot mutate through a shared reference (and the default Rc pointer is not Send/Sync)
        // bench::<RustCHashBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "rustc-hash"); // not concurrent
        bench::<SccBenchMap<u64, u64>>(
            &mut group,
            &map_data,
            CONTENTION_THREAD_COUNT,
            &workloads,
            "scc",
        );
        bench::<StarshardBenchMap<u64, u64>>(
            &mut group,
            &map_data,
            CONTENTION_THREAD_COUNT,
            &workloads,
            "starshard",
        );
        // bench::<StdBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "std"); // not concurrent
        bench::<TxMapBenchMap<u64, u64>>(
            &mut group,
            &map_data,
            CONTENTION_THREAD_COUNT,
            &workloads,
            "txmap",
        );
    }
}

criterion_group!(group, contention);
criterion_main!(group);
