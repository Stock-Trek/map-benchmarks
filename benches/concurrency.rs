use bench_map::{
    config::*,
    data::u64_sparse::U64SparseDataGen,
    map_data::MapData,
    map_gen::MapGen,
    maps::{
        BenchMapGetCloned, BenchMapInsert, BenchMapMutInsert, BenchMapNew, BenchMapRemove,
        ConcreadBenchMap, DashMapBenchMap, StarshardBenchMap, TxMapBenchMap,
    },
    pin_thread::PinThread,
    workload::{design::WorkloadDesign, op::WorkloadOp, thread_workload::ThreadWorkload},
};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main,
    measurement::WallTime,
};
use std::{rc::Rc, sync::Arc};

fn run_workload<M>(workload: &ThreadWorkload, map: &M)
where
    M: BenchMapGetCloned<u64, u64>,
    M: BenchMapInsert<u64, u64>,
    M: BenchMapRemove<u64, u64>,
{
    for item in &workload.items {
        match item.op {
            WorkloadOp::Lookup => {
                let key = std::hint::black_box(&item.key);
                std::hint::black_box(map.get_cloned(key));
            }
            WorkloadOp::Insert => {
                let key = std::hint::black_box(item.key);
                map.insert(key, 42u64);
            }
            WorkloadOp::Remove => {
                let key = std::hint::black_box(&item.key);
                std::hint::black_box(map.remove(key));
            }
        }
    }
}

fn bench_concurrent_shared<Map>(
    group: &mut BenchmarkGroup<WallTime>,
    map_data: Rc<MapData<u64, u64>>,
    thread_count: usize,
    workloads: Vec<ThreadWorkload>,
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
        b.iter_batched(
            || {
                let map = map_data.create_map::<Map>();
                let map = Arc::new(map);
                let workloads = workloads.clone();
                (map, workloads)
            },
            |(map, workloads)| {
                let mut handles = Vec::with_capacity(thread_count);
                for (thread_id, workload) in
                    workloads.iter().cloned().enumerate().take(thread_count)
                {
                    let map = Arc::clone(&map);
                    handles.push(std::thread::spawn(move || {
                        PinThread::try_pin(thread_id).expect("failed to pin thread to CPU");
                        run_workload(&workload, &*map);
                    }));
                }
                for handle in handles {
                    handle.join().unwrap();
                }
            },
            BatchSize::PerIteration,
        );
    });
}

fn concurrency(c: &mut Criterion) {
    let max_threads = CONCURRENCY_THREAD_COUNTS.last().unwrap();
    let entry_count = 1_000_000;
    let existing_key_count = entry_count;
    let missing_key_count = max_threads * CONCURRENCY_OPS_PER_THREAD;
    let sort_keys = false;
    let map_data = Rc::new(MapGen::generate(
        U64SparseDataGen,
        U64SparseDataGen,
        entry_count,
        existing_key_count,
        missing_key_count,
        sort_keys,
    ));

    let design = WorkloadDesign::balanced(CONCURRENCY_OPS_PER_THREAD);
    let mut rng = rand::rng();

    for &thread_count in CONCURRENCY_THREAD_COUNTS {
        let total_ops = thread_count * CONCURRENCY_OPS_PER_THREAD;
        let workloads = (0..thread_count)
            .map(|_| {
                ThreadWorkload::new(
                    &design,
                    map_data.existing_keys(),
                    map_data.missing_keys(),
                    &mut rng,
                )
            })
            .collect::<Vec<_>>();

        let mut group = c.benchmark_group(format!(
            "concurrency/balanced-workload/{}_threads",
            thread_count
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(total_ops as u64));

        // bench_concurrent_shared::<AhashBenchMap<_, _>>(&mut group, map_data.clone(), thread_count, workloads.clone(), "ahash");
        // bench_concurrent_shared::<BTreeMapBenchMap<_, _>>(&mut group, map_data.clone(), thread_count, workloads.clone(), "btreemap");
        bench_concurrent_shared::<ConcreadBenchMap<u64, u64>>(
            &mut group,
            map_data.clone(),
            thread_count,
            workloads.clone(),
            "concread",
        );
        bench_concurrent_shared::<DashMapBenchMap<u64, u64>>(
            &mut group,
            map_data.clone(),
            thread_count,
            workloads.clone(),
            "dashmap",
        );
        // bench_concurrent_shared::<HashbrownBenchMap<_, _>>(&mut group, map_data.clone(), thread_count, workloads.clone(), "hashbrown");
        // bench_concurrent_shared::<ImmutableChunkMapBenchMap<_, _>>(&mut group, map_data.clone(), thread_count, workloads.clone(), "immutable-chunkmap"); // mutation returns a new map; requires &mut or storing the result, cannot mutate through a shared reference
        // bench_concurrent_shared::<IndexMapBenchMap<_, _>>(&mut group, map_data.clone(), thread_count, workloads.clone(), "indexmap");
        // bench_concurrent_shared::<RustCHashBenchMap<_, _>>(&mut group, map_data.clone(), thread_count, workloads.clone(), "rustc-hash");
        bench_concurrent_shared::<StarshardBenchMap<u64, u64>>(
            &mut group,
            map_data.clone(),
            thread_count,
            workloads.clone(),
            "starshard",
        );
        // bench_concurrent_shared::<StdBenchMap<_, _>>(&mut group, map_data.clone(), thread_count, workloads.clone(), "std");
        bench_concurrent_shared::<TxMapBenchMap<u64, u64>>(
            &mut group,
            map_data.clone(),
            thread_count,
            workloads.clone(),
            "txmap",
        );
    }
}

criterion_group!(group, concurrency);
criterion_main!(group);
