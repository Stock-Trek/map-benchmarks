// How well does the get-or-create pattern hold up under contention? Tests the atomic read-modify-write / entry-API design when multiple threads race to get-or-insert (possibly overlapping) keys: 90% of operations hit existing keys, the remaining 10% insert missing keys.
use bench_map::{
    common_hasher::CommonHasher, concurrent_workers::ConcurrentWorkers, config::*, constants::*,
    data::u64_sparse::U64SparseDataGen, expand_bench_concurrent,
    expand_bench_concurrent_with_common_hasher, map_data::MapData, map_gen::MapGen, maps::*,
};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main,
    measurement::WallTime,
};
use rand::RngExt;
use std::{cell::Cell, hash::Hash, hint::black_box, sync::Arc};

/// Generates one worker's operations: `op_count` keys, each an existing key
/// with probability `hit_ratio` and a missing key otherwise.
fn generate_workload<K>(
    op_count: usize,
    hit_ratio: f64,
    existing_keys: &[K],
    missing_keys: &[K],
    rng: &mut impl RngExt,
) -> Vec<K>
where
    K: Clone,
{
    let mut keys = Vec::with_capacity(op_count);
    for _ in 0..op_count {
        let key = if rng.random_bool(hit_ratio) {
            existing_keys[rng.random_range(0..existing_keys.len())].clone()
        } else {
            missing_keys[rng.random_range(0..missing_keys.len())].clone()
        };
        keys.push(key);
    }
    keys
}

fn run_get_or_insert<M, K>(keys: &[K], map: &M)
where
    M: BenchMapGetOrInsert<K, u64>,
    K: Clone,
{
    for key in keys {
        let key = black_box(key);
        black_box(map.get_or_insert(key.clone(), 42));
    }
}

fn bench_out_of_the_box<Map, K>(
    name: &str,
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<K, u64>,
    thread_count: usize,
    workloads: &[Vec<K>],
) where
    Map: BenchMapNew<K, u64>
        + BenchMapMutInsert<K, u64>
        + BenchMapGetOrInsert<K, u64>
        + Send
        + Sync
        + 'static,
    K: Clone + Hash + Eq + Send + Sync + 'static,
{
    group.bench_function(name, move |b| {
        // Spawn and pin the worker threads once per sample, outside the timed
        // region, so thread spawn/join and CPU-pinning costs are amortized
        // instead of being measured on every iteration.
        let workers =
            ConcurrentWorkers::<Vec<K>, Map>::new(thread_count, workloads, |keys, map| {
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

fn bench_same_hasher<Map, K>(
    name: &str,
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<K, u64>,
    thread_count: usize,
    workloads: &[Vec<K>],
    hasher: CommonHasher,
) where
    Map: BenchMapNewWithHasher<K, u64, CommonHasher>
        + BenchMapMutInsert<K, u64>
        + BenchMapGetOrInsert<K, u64>
        + Send
        + Sync
        + 'static,
    K: Clone + Hash + Eq + Send + Sync + 'static,
{
    group.bench_function(name, move |b| {
        // Spawn and pin the worker threads once per sample, outside the timed
        // region, so thread spawn/join and CPU-pinning costs are amortized
        // instead of being measured on every iteration.
        let workers =
            ConcurrentWorkers::<Vec<K>, Map>::new(thread_count, workloads, |keys, map| {
                run_get_or_insert(keys, map)
            });
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

fn get_or_insert_concurrent(c: &mut Criterion) {
    let max_threads = DEFAULT_THREAD_COUNTS.last().unwrap();
    let entry_count = DEFAULT_ENTRY_COUNT;
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

    let mut rng = rand::rng();
    for &thread_count in DEFAULT_THREAD_COUNTS {
        let total_ops = thread_count * DEFAULT_OP_COUNT;
        let workloads = (0..thread_count)
            .map(|_| {
                generate_workload(
                    DEFAULT_OP_COUNT,
                    GET_OR_INSERT_HIT_RATIO,
                    map_data.existing_keys(),
                    map_data.missing_keys(),
                    &mut rng,
                )
            })
            .collect::<Vec<_>>();

        // default hashers
        {
            let mut group = c.benchmark_group(format!(
                "get-or-insert/threads-{}/{DEFAULT_HASHER}",
                thread_count
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(total_ops as u64));

            expand_bench_concurrent!(bench_out_of_the_box, u64, &mut group, &map_data, thread_count, &workloads,
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
                // RpdsHashTrieMapBenchMap<u64, u64>, // mutation returns a new map; requires &mut or storing the result, cannot mutate through a shared reference
                // RustCHashBenchMap<u64, u64>, // not concurrent
                SccBenchMap<u64, u64>,
                StarshardBenchMap<u64, u64>,
                // StdBenchMap<u64, u64>, // not concurrent
                TxMapBenchMap<u64, u64>,
            );
        }

        // CommonHasher
        {
            let mut group = c.benchmark_group(format!(
                "get-or-insert/threads-{}/{COMMON_HASHER}",
                thread_count
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(total_ops as u64));

            expand_bench_concurrent_with_common_hasher!(bench_same_hasher, u64, &mut group, &map_data, thread_count, &workloads,
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

criterion_group!(group, get_or_insert_concurrent);
criterion_main!(group);
