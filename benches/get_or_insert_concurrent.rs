use bench_map::{
    concurrent_workers::ConcurrentWorkers, config::*, constants::*,
    data::u64_sparse::U64SparseDataGen, map_data::MapData, map_gen::MapGen, maps::*,
    number_formatter::format_n,
};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main,
    measurement::WallTime,
};
use rand::RngExt;
use std::{cell::Cell, hint::black_box, sync::Arc};

/// The "get-or-create cache entry" pattern: each operation reads a key and, if
/// absent, inserts the default value. `hit_ratio` of the keys already exist in
/// the map (pure reads); the rest are missing keys that get inserted.
fn generate_workload(
    op_count: usize,
    hit_ratio: f64,
    existing_keys: &[u64],
    missing_keys: &[u64],
    rng: &mut impl RngExt,
) -> Vec<u64> {
    let mut keys = Vec::with_capacity(op_count);
    for _ in 0..op_count {
        let key = if rng.random_bool(hit_ratio) {
            existing_keys[rng.random_range(0..existing_keys.len())]
        } else {
            missing_keys[rng.random_range(0..missing_keys.len())]
        };
        keys.push(key);
    }
    keys
}

fn run_get_or_insert<M>(keys: &[u64], map: &M)
where
    M: BenchMapGetOrInsert<u64, u64>,
{
    for key in keys {
        let key = black_box(key);
        black_box(map.get_or_insert(*key, 42));
    }
}

fn bench<Map>(
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<u64, u64>,
    thread_count: usize,
    workloads: &[Vec<u64>],
    name: &str,
) where
    Map: BenchMapNew<u64, u64>
        + BenchMapMutInsert<u64, u64>
        + BenchMapGetOrInsert<u64, u64>
        + Send
        + Sync
        + 'static,
{
    group.bench_function(name, move |b| {
        // Spawn and pin the worker threads once per sample, outside the timed
        // region, so thread spawn/join and CPU-pinning costs are amortized
        // instead of being measured on every iteration.
        let workers =
            ConcurrentWorkers::<Vec<u64>, Map>::new(thread_count, workloads, |keys, map| {
                run_get_or_insert(keys, map)
            });
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

fn get_or_insert_concurrent(c: &mut Criterion) {
    let max_threads = WORKLOAD_CONCURRENT_THREAD_COUNTS.last().unwrap();
    for &entry_count in WORKLOAD_ENTRY_COUNT {
        let existing_key_count = entry_count;
        let missing_key_count = max_threads * WORKLOAD_OP_COUNT;
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
        for &thread_count in WORKLOAD_CONCURRENT_THREAD_COUNTS {
            let total_ops = thread_count * WORKLOAD_OP_COUNT;
            let workloads = (0..thread_count)
                .map(|_| {
                    generate_workload(
                        WORKLOAD_OP_COUNT,
                        GET_OR_INSERT_CONCURRENT_HIT_RATIO,
                        map_data.existing_keys(),
                        map_data.missing_keys(),
                        &mut rng,
                    )
                })
                .collect::<Vec<_>>();

            let mut group = c.benchmark_group(format!(
                "get-or-insert/{OUT_OF_THE_BOX_GROUP_NAME}/map-size-{}/threads-{}",
                format_n(entry_count),
                thread_count
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(total_ops as u64));

            // bench::<ConcreadBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "concread"); // too slow
            bench::<DashMapBenchMap<u64, u64>>(
                &mut group,
                &map_data,
                thread_count,
                &workloads,
                "dashmap",
            );
            // bench::<FlurryBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "flurry"); // too slow
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
            bench::<SccBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "scc");
            bench::<StarshardBenchMap<u64, u64>>(
                &mut group,
                &map_data,
                thread_count,
                &workloads,
                "starshard",
            );
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

criterion_group!(group, get_or_insert_concurrent);
criterion_main!(group);
