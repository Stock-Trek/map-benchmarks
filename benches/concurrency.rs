use bench_map::{
    config::*, data::u64_sparse::U64SparseDataGen, map_gen::MapGen,
    maps::sync_benchmap::SyncBenchMap, pin_thread::pin_thread,
};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use rand::{RngExt, SeedableRng};
use std::sync::Arc;

/// Number of operations per thread per benchmark iteration
const OPS_PER_THREAD: usize = 10_000;

/// Thread counts to benchmark
const THREAD_COUNTS: &[usize] = &[1, 2, 4];

/// Operation types for the balanced workload
#[derive(Clone, Copy, Debug)]
enum Op {
    Lookup,
    Insert,
    Update,
    Remove,
}

/// Pre-generated operations for a single thread
#[derive(Clone)]
struct ThreadWorkload {
    ops: Vec<Op>,
    keys: Vec<u64>,
}

/// Generate a balanced workload for a single thread.
///
/// Distribution: 80% lookup, 5% insert, 10% update, 5% remove.
/// Existing keys are used for lookup/update/remove.
/// Missing keys are used for inserts.
fn generate_workload(
    existing_keys: &[u64],
    missing_keys: &[u64],
    ops_per_thread: usize,
    rng: &mut impl RngExt,
) -> ThreadWorkload {
    let mut ops = Vec::with_capacity(ops_per_thread);
    let mut keys = Vec::with_capacity(ops_per_thread);

    for _ in 0..ops_per_thread {
        let roll: f64 = rng.random();
        if roll < 0.80 {
            // Lookup
            ops.push(Op::Lookup);
            keys.push(existing_keys[rng.random_range(0..existing_keys.len())]);
        } else if roll < 0.85 {
            // Insert
            ops.push(Op::Insert);
            keys.push(missing_keys[rng.random_range(0..missing_keys.len())]);
        } else if roll < 0.95 {
            // Update
            ops.push(Op::Update);
            keys.push(existing_keys[rng.random_range(0..existing_keys.len())]);
        } else {
            // Remove
            ops.push(Op::Remove);
            keys.push(existing_keys[rng.random_range(0..existing_keys.len())]);
        }
    }

    ThreadWorkload { ops, keys }
}

/// Run a single thread's workload against a map.
fn run_workload<M, K, V>(map: &M, workload: &ThreadWorkload)
where
    M: SyncBenchMap<K, V>,
    K: From<u64> + Copy,
    V: From<u64> + Copy,
{
    for (op, key) in workload.ops.iter().zip(workload.keys.iter()) {
        let key = K::from(*key);
        match op {
            Op::Lookup => {
                std::hint::black_box(map.get_cloned(std::hint::black_box(&key)));
            }
            Op::Insert => {
                map.insert(std::hint::black_box(key), std::hint::black_box(V::from(42)));
            }
            Op::Update => {
                map.insert(std::hint::black_box(key), std::hint::black_box(V::from(99)));
            }
            Op::Remove => {
                std::hint::black_box(map.remove(std::hint::black_box(&key)));
            }
        }
    }
}

/// Generate workloads for all threads.
fn generate_workloads(
    existing_keys: &[u64],
    missing_keys: &[u64],
    thread_count: usize,
    ops_per_thread: usize,
) -> Vec<ThreadWorkload> {
    let mut workloads = Vec::with_capacity(thread_count);
    for i in 0..thread_count {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42 + i as u64);
        workloads.push(generate_workload(
            existing_keys,
            missing_keys,
            ops_per_thread,
            &mut rng,
        ));
    }
    workloads
}

macro_rules! bench_concurrent {
    ($group:ident, $map_data:expr, $thread_count:expr, $ops_per_thread:expr, $workloads:expr, $map_type:path, $name:expr) => {
        let map_data = $map_data.clone();
        let workloads = $workloads.clone();
        $group.bench_function($name, move |b| {
            b.iter(|| {
                let map = map_data.create_map_sync::<$map_type>();
                let map = Arc::new(map);
                let mut handles = Vec::with_capacity($thread_count);
                for thread_id in 0..$thread_count {
                    let map = Arc::clone(&map);
                    let workload = workloads[thread_id].clone();
                    handles.push(std::thread::spawn(move || {
                        pin_thread(thread_id);
                        run_workload::<$map_type, u64, u64>(&*map, &workload);
                    }));
                }
                for handle in handles {
                    handle.join().unwrap();
                }
            });
        });
    };
}

fn concurrency(c: &mut Criterion) {
    let entry_count = 1_000_000;
    let max_threads = *THREAD_COUNTS.last().unwrap();
    let missing_key_count = max_threads * OPS_PER_THREAD;

    let map_data = std::rc::Rc::new(MapGen::generate(
        U64SparseDataGen,
        U64SparseDataGen,
        entry_count,
        entry_count,
        missing_key_count,
        false,
    ));

    for &thread_count in THREAD_COUNTS {
        let total_ops = thread_count * OPS_PER_THREAD;
        let workloads = generate_workloads(
            map_data.existing_keys(),
            map_data.missing_keys(),
            thread_count,
            OPS_PER_THREAD,
        );

        let mut group = c.benchmark_group(format!("concurrency/{}_threads", thread_count));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(total_ops as u64));

        bench_concurrent!(
            group,
            map_data,
            thread_count,
            OPS_PER_THREAD,
            workloads,
            bench_map::maps::sync_dashmap_benchmap::SyncDashMapBenchMap<_, _>,
            "dashmap"
        );
        bench_concurrent!(
            group,
            map_data,
            thread_count,
            OPS_PER_THREAD,
            workloads,
            bench_map::maps::sync_concread_benchmap::SyncConcreadBenchMap<_, _>,
            "concread"
        );
        bench_concurrent!(
            group,
            map_data,
            thread_count,
            OPS_PER_THREAD,
            workloads,
            bench_map::maps::sync_starshard_benchmap::SyncStarshardBenchMap<_, _>,
            "starshard"
        );
        bench_concurrent!(
            group,
            map_data,
            thread_count,
            OPS_PER_THREAD,
            workloads,
            bench_map::maps::sync_txmap_benchmap::SyncTxMapBenchMap<_, _>,
            "txmap"
        );
    }
}

criterion_group!(group, concurrency);
criterion_main!(group);
