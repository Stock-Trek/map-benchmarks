use bench_map::{
    config::*,
    constants::*,
    data::u64_sparse::U64SparseDataGen,
    map_data::MapData,
    map_gen::MapGen,
    maps::{
        BenchMapGetCloned, BenchMapInsert, BenchMapMutInsert, BenchMapNew, BenchMapRemove,
        DashMapBenchMap, LeapfrogBenchMap, PapayaBenchMap, SccBenchMap, StarshardBenchMap,
        TxMapBenchMap,
    },
    number_formatter::format_n,
    pin_thread::PinThread,
    workload::{design::WorkloadDesign, op::WorkloadOp, thread_workload::ThreadWorkload},
};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main,
    measurement::WallTime,
};
use std::{
    cell::Cell,
    hint::black_box,
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicBool, Ordering},
    },
};

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

/// A pool of pinned worker threads reused across all timed iterations of one
/// benchmark sample.
///
/// The threads are spawned and pinned to dedicated CPUs once per sample,
/// outside the timed region, so the per-iteration time measured by Criterion
/// contains only the parallel workload execution plus nanosecond-scale
/// start/done signalling - not thread spawn/join or CPU-pinning overhead.
struct ConcurrentWorkers<M> {
    state: Arc<WorkerPoolState<M>>,
    handles: Vec<std::thread::JoinHandle<()>>,
}

/// State shared between the benchmark thread and the worker pool: the
/// start/done signalling, panic propagation, per-worker map slots, and
/// immutable per-worker workloads.
struct WorkerPoolState<M> {
    /// Raised by the timed region to release the workers; cleared once all
    /// workers have reported done for the current iteration.
    start: AtomicBool,
    /// Cumulative number of completed work items (monotonic across
    /// iterations, so no per-iteration reset is needed). Each worker adds one
    /// per iteration, so after the Nth iteration the counter equals
    /// N * thread_count. Guarded by `done_cond` so the main thread can block
    /// instead of busy-waiting (a busy-wait on the main thread would steal
    /// cycles from the pinned workers on machines where every core is
    /// occupied).
    done: Mutex<usize>,
    /// Signalled whenever a worker completes a work item or panics.
    done_cond: Condvar,
    /// Set when a worker thread panics so the main thread aborts the
    /// benchmark instead of spinning forever waiting for a dead worker.
    panicked: AtomicBool,
    /// The first worker panic payload, re-raised on the main thread.
    panic: Mutex<Option<Box<dyn std::any::Any + Send>>>,
    /// Set when the pool is dropped so idle workers exit and can be joined.
    shutdown: AtomicBool,
    /// Per-worker slot holding the current iteration's map. The (untimed)
    /// setup closure publishes a fresh map here before raising `start`.
    slots: Vec<Mutex<Option<Arc<M>>>>,
    /// Immutable per-worker workloads.
    workloads: Vec<ThreadWorkload>,
}

impl<M> ConcurrentWorkers<M> {
    fn new(thread_count: usize, workloads: &[ThreadWorkload]) -> Self
    where
        M: BenchMapGetCloned<u64, u64>
            + BenchMapInsert<u64, u64>
            + BenchMapRemove<u64, u64>
            + Send
            + Sync
            + 'static,
    {
        let workloads = workloads
            .iter()
            .take(thread_count)
            .cloned()
            .collect::<Vec<_>>();
        let slots = (0..thread_count).map(|_| Mutex::new(None)).collect();
        let state = Arc::new(WorkerPoolState {
            start: AtomicBool::new(false),
            done: Mutex::new(0),
            done_cond: Condvar::new(),
            panicked: AtomicBool::new(false),
            panic: Mutex::new(None),
            shutdown: AtomicBool::new(false),
            slots,
            workloads,
        });

        let handles = (0..thread_count)
            .map(|thread_id| {
                let state = state.clone();
                std::thread::spawn(move || {
                    // Swallow panics just long enough to record them so the
                    // main thread's done-counter wait cannot hang; the panic
                    // is re-raised on the main thread from `run`.
                    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        PinThread::try_pin(thread_id).expect("failed to pin thread to CPU");
                        let workload = &state.workloads[thread_id];
                        loop {
                            // Wait for the timed region to release us (or for
                            // the pool to shut down).
                            while !state.start.load(Ordering::Acquire) {
                                if state.shutdown.load(Ordering::Acquire) {
                                    return;
                                }
                                std::hint::spin_loop();
                            }
                            // The setup closure publishes the current map
                            // before raising `start`, so the slot is normally
                            // populated already; spin briefly in case the flag
                            // races ahead of the publish.
                            let map = loop {
                                let mut slot = state.slots[thread_id].lock().unwrap();
                                if let Some(map) = slot.take() {
                                    break map;
                                }
                                drop(slot);
                                if state.shutdown.load(Ordering::Acquire) {
                                    return;
                                }
                                std::hint::spin_loop();
                            };
                            run_workload(workload, &*map);
                            *state.done.lock().unwrap() += 1;
                            state.done_cond.notify_one();
                        }
                    }));
                    if let Err(payload) = result {
                        let mut guard = state.panic.lock().unwrap();
                        if guard.is_none() {
                            *guard = Some(payload);
                        }
                        drop(guard);
                        state.panicked.store(true, Ordering::SeqCst);
                        // Account for this worker so the main thread's wait
                        // terminates even if the panic happened before the
                        // per-iteration `done` increment.
                        *state.done.lock().unwrap() += 1;
                        state.done_cond.notify_one();
                    }
                })
            })
            .collect::<Vec<_>>();

        Self { state, handles }
    }

    /// Releases the workers and waits until all of them have finished their
    /// current work item. `target` is the cumulative number of completed work
    /// items expected after this iteration (iteration_index * thread_count).
    fn run(&self, target: usize) {
        self.state.start.store(true, Ordering::Release);
        let mut done = self.state.done.lock().unwrap();
        while *done < target {
            if self.state.panicked.load(Ordering::SeqCst) {
                drop(done);
                self.resume_worker_panic();
            }
            done = self.state.done_cond.wait(done).unwrap();
        }
        drop(done);
        if self.state.panicked.load(Ordering::SeqCst) {
            self.resume_worker_panic();
        }
        self.state.start.store(false, Ordering::SeqCst);
    }

    fn resume_worker_panic(&self) -> ! {
        let payload = self.state.panic.lock().unwrap().take();
        if let Some(payload) = payload {
            std::panic::resume_unwind(payload);
        } else {
            std::panic::panic_any("a worker thread panicked");
        }
    }
}

impl<M> Drop for ConcurrentWorkers<M> {
    fn drop(&mut self) {
        self.state.shutdown.store(true, Ordering::SeqCst);
        for handle in self.handles.drain(..) {
            handle.join().unwrap();
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
        let workers = ConcurrentWorkers::<Map>::new(thread_count, workloads);
        // 1-based index of the iteration about to be timed; used to derive the
        // cumulative `done` target for the worker pool.
        let iteration = Cell::new(0usize);
        b.iter_batched(
            || {
                // Untimed setup: publish a fresh map for this iteration.
                let map = Arc::new(map_data.create_map::<Map>());
                for slot in &workers.state.slots {
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
                        ThreadWorkload::new(
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

                // bench::<AhashBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "ahash"); // not concurrent
                // bench::<BTreeMapBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "btreemap"); // not concurrent
                // bench::<ConcreadBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "concread"); // too slow
                bench::<DashMapBenchMap<u64, u64>>(
                    &mut group,
                    &map_data,
                    thread_count,
                    &workloads,
                    "dashmap",
                );
                // bench::<FlurryBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "flurry"); // too slow
                // bench::<HashbrownBenchMap<u64, u64>>(&mut group, &map_data, thread_count, &workloads, "hashbrown"); // not concurrent
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

criterion_group!(group, workload_concurrent);
criterion_main!(group);
