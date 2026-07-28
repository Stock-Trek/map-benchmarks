use bench_map::{
    config::*,
    data::u64_sparse::U64SparseDataGen,
    map_gen::MapGen,
    pin_thread::PinThread,
    workload::{WorkloadDesign, generate_workloads, run_workload},
};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::sync::Arc;

/// Number of operations per thread per benchmark iteration
const OPS_PER_THREAD: usize = 10_000;

/// Thread counts to benchmark
const THREAD_COUNTS: &[usize] = &[1, 2, 4];

macro_rules! bench_concurrent {
    ($group:ident, $map_data:expr, $thread_count:expr, $workloads:expr, $map_type:path, $name:expr) => {
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
                        PinThread::pin(thread_id);
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

    let map_data = std::rc::Rc::new(MapGen::generate(
        U64SparseDataGen,
        U64SparseDataGen,
        entry_count,
        entry_count,
        max_threads * OPS_PER_THREAD,
        false,
    ));

    let design = WorkloadDesign::balanced(OPS_PER_THREAD);

    for &thread_count in THREAD_COUNTS {
        let total_ops = thread_count * OPS_PER_THREAD;
        let workloads = generate_workloads(
            &design,
            map_data.existing_keys(),
            map_data.missing_keys(),
            thread_count,
        );

        let mut group = c.benchmark_group(format!("concurrency/{}_threads", thread_count));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(total_ops as u64));

        bench_concurrent!(
            group,
            map_data,
            thread_count,
            workloads,
            bench_map::maps::sync_dashmap_benchmap::SyncDashMapBenchMap<_, _>,
            "dashmap"
        );
        bench_concurrent!(
            group,
            map_data,
            thread_count,
            workloads,
            bench_map::maps::sync_concread_benchmap::SyncConcreadBenchMap<_, _>,
            "concread"
        );
        bench_concurrent!(
            group,
            map_data,
            thread_count,
            workloads,
            bench_map::maps::sync_starshard_benchmap::SyncStarshardBenchMap<_, _>,
            "starshard"
        );
        bench_concurrent!(
            group,
            map_data,
            thread_count,
            workloads,
            bench_map::maps::sync_txmap_benchmap::SyncTxMapBenchMap<_, _>,
            "txmap"
        );
    }
}

criterion_group!(group, concurrency);
criterion_main!(group);
