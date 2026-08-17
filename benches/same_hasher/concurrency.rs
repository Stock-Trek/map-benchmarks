use bench_map::{
    config::*,
    data::u64_sparse::U64SparseDataGen,
    map_data::MapData,
    map_gen::MapGen,
    maps::{
        BenchMapGetCloned, BenchMapInsert, BenchMapMutInsert, BenchMapNewWithHasher,
        BenchMapRemove, DashMapBenchMap, StarshardBenchMap, TxMapBenchMap,
    },
    pin_thread::PinThread,
    workload::{design::WorkloadDesign, op::WorkloadOp, thread_workload::ThreadWorkload},
};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main,
    measurement::WallTime,
};
use std::{hint::black_box, sync::Arc};

/// The hasher shared by every map implementation that supports a custom hasher,
/// so map implementations are compared against each other on a level playing
/// field rather than each using its own default hasher.
type CommonHasher = ahash::RandomState;

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

fn bench<Map, H>(
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<u64, u64>,
    thread_count: usize,
    workloads: &[ThreadWorkload],
    name: &str,
    hasher: H,
) where
    Map: BenchMapNewWithHasher<u64, u64, H>
        + BenchMapMutInsert<u64, u64>
        + BenchMapGetCloned<u64, u64>
        + BenchMapInsert<u64, u64>
        + BenchMapRemove<u64, u64>
        + Send
        + Sync
        + 'static,
    H: std::hash::BuildHasher + Clone,
{
    group.bench_function(name, move |b| {
        b.iter_batched(
            || {
                let map = Arc::new(map_data.create_map_with_hasher::<Map, H>(hasher.clone()));
                let workloads = workloads
                    .iter()
                    .take(thread_count)
                    .cloned()
                    .collect::<Vec<_>>();
                (map, workloads)
            },
            |(map, workloads)| {
                let mut handles = Vec::with_capacity(thread_count);
                for (thread_id, workload) in workloads.into_iter().enumerate() {
                    let map = map.clone();
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
    let map_data = MapGen::generate(
        U64SparseDataGen,
        U64SparseDataGen,
        entry_count,
        existing_key_count,
        missing_key_count,
        sort_keys,
    );

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
            "same-hasher/concurrency/balanced-workload/{}_threads",
            thread_count
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(total_ops as u64));

        let hasher = CommonHasher::new();

        // bench::<AhashBenchMap<u64, u6>>(&mut group, &map_data, thread_count, &workloads, "ahash"); // not concurrent
        // bench::<BTreeMapBenchMap<u64, u6>>(&mut group, &map_data, thread_count, &workloads, "btreemap"); // not concurrent
        // bench::<ConcreadBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "concread"); // too slow
        bench::<DashMapBenchMap<u64, u64, CommonHasher>, CommonHasher>(
            &mut group,
            &map_data,
            thread_count,
            &workloads,
            "dashmap",
            hasher.clone(),
        );
        // bench::<HashbrownBenchMap<u64, u6>>(&mut group, &map_data, thread_count, &workloads, "hashbrown"); // not concurrent
        // bench::<HordeBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "horde"); // not concurrent
        // bench::<ImmutableChunkMapBenchMap<u64, u6>>(&mut group, &map_data, thread_count, &workloads, "immutable-chunkmap"); // mutation returns a new map; requires &mut or storing the result, cannot mutate through a shared reference
        // bench::<IndexMapBenchMap<u64, u6>>(&mut group, &map_data, thread_count, &workloads, "indexmap"); // not concurrent
        // bench::<RustCHashBenchMap<u64, u6>>(&mut group, &map_data, thread_count, &workloads, "rustc-hash"); // not concurrent
        bench::<StarshardBenchMap<u64, u64, CommonHasher>, CommonHasher>(
            &mut group,
            &map_data,
            thread_count,
            &workloads,
            "starshard",
            hasher.clone(),
        );
        // bench::<StdBenchMap<u64, u6>>(&mut group, &map_data, thread_count, &workloads, "std"); // not concurrent
        bench::<TxMapBenchMap<u64, u64, CommonHasher>, CommonHasher>(
            &mut group,
            &map_data,
            thread_count,
            &workloads,
            "txmap",
            hasher.clone(),
        );
    }
}

criterion_group!(group, concurrency);
criterion_main!(group);
