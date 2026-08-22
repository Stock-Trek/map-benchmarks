// How does it handle synchronization? Tests locking/synchronization mechanisms when all threads hammer a single key under varying read/write mixes.
use bench_map::{
    concurrent_workers::ConcurrentWorkers, config::*, expand_bench_concurrent, map_data::MapData,
    maps::*,
};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main,
    measurement::WallTime,
};
use rand::RngExt;
use std::{cell::Cell, hint::black_box, sync::Arc};

/// A single operation of the synchronization benchmark, performed on the one
/// contended key.
#[derive(Clone, Copy, Debug)]
enum SyncOp {
    Read,
    Write,
}

const SYNCHRONIZATION_HIT_KEY: u64 = 0;

/// The 6 synchronization workloads: the fraction of operations that are
/// reads (lookups of the single key); the remainder are writes (inserts /
/// updates of the same key). Every thread hammers the same key, so this measures
/// how well the map can synchronize operations under the worst case scenario.
const SYNC_WORKLOADS: &[(&str, f64)] = &[
    ("read-only", 1.0),
    ("read-mostly", 0.80),
    ("read-majority", 0.60),
    ("write-majority", 0.40),
    ("write-mostly", 0.20),
    ("write-only", 0.0),
];

/// Generates one worker's operations: `op_count` operations on the single
/// synchronization key, each a read with probability `read_ratio` and a write
/// otherwise.
fn generate_sync_workload(op_count: usize, read_ratio: f64, rng: &mut impl RngExt) -> Vec<SyncOp> {
    let mut ops = Vec::with_capacity(op_count);
    for _ in 0..op_count {
        ops.push(if rng.random_bool(read_ratio) {
            SyncOp::Read
        } else {
            SyncOp::Write
        });
    }
    ops
}

fn run_sync_workload<M>(ops: &[SyncOp], map: &M)
where
    M: BenchMapGetCloned<u64, u64> + BenchMapInsert<u64, u64>,
{
    for op in ops {
        match op {
            SyncOp::Read => {
                black_box(map.get_cloned(&SYNCHRONIZATION_HIT_KEY));
            }
            SyncOp::Write => {
                map.insert(SYNCHRONIZATION_HIT_KEY, 42u64);
            }
        }
    }
}

fn bench<Map>(
    name: &str,
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<u64, u64>,
    thread_count: usize,
    workloads: &[Vec<SyncOp>],
) where
    Map: BenchMapNew<u64, u64>
        + BenchMapMutInsert<u64, u64>
        + BenchMapGetCloned<u64, u64>
        + BenchMapInsert<u64, u64>
        + Send
        + Sync
        + 'static,
{
    group.bench_function(name, move |b| {
        // Spawn and pin the worker threads once per sample, outside the timed
        // region, so thread spawn/join and CPU-pinning costs are amortized
        // instead of being measured on every iteration.
        let workers =
            ConcurrentWorkers::<Vec<SyncOp>, Map>::new(thread_count, workloads, |ops, map| {
                run_sync_workload(ops, map)
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

fn synchronization(c: &mut Criterion) {
    // The map holds exactly one entry: the single key all threads contend on.
    let map_data = MapData::new(
        vec![(SYNCHRONIZATION_HIT_KEY, 42u64)],
        vec![SYNCHRONIZATION_HIT_KEY],
        vec![],
    );

    let mut rng = rand::rng();
    for &(name, read_ratio) in SYNC_WORKLOADS {
        let total_ops = DEFAULT_THREAD_COUNT * DEFAULT_OP_COUNT;
        let workloads = (0..DEFAULT_THREAD_COUNT)
            .map(|_| generate_sync_workload(DEFAULT_OP_COUNT, read_ratio, &mut rng))
            .collect::<Vec<_>>();

        let mut group = c.benchmark_group(format!(
            "synchronization-{DEFAULT_THREAD_COUNT}-threads/{}",
            name
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(total_ops as u64));

        expand_bench_concurrent!(bench, &mut group, &map_data, DEFAULT_THREAD_COUNT, &workloads,
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
            LeapfrogBenchMap<u64, u64>,
            PapayaBenchMap<u64, u64>,
            // RpdsHashTrieMapBenchMap<u64, u64>, // mutation returns a new map; requires &mut or storing the result, cannot mutate through a shared reference (and the default Rc pointer is not Send/Sync)
            // RustCHashBenchMap<u64, u64>, // not concurrent
            SccBenchMap<u64, u64>,
            StarshardBenchMap<u64, u64>,
            // StdBenchMap<u64, u64>, // not concurrent
            TxMapBenchMap<u64, u64>,
        );
    }
}

criterion_group!(group, synchronization);
criterion_main!(group);
