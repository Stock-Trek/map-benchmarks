use crate::pin_thread::PinThread;
use std::sync::{
    Arc, Condvar, Mutex,
    atomic::{AtomicBool, Ordering},
};

/// A pool of pinned worker threads reused across all timed iterations of one
/// benchmark sample.
///
/// The threads are spawned and pinned to dedicated CPUs once per sample,
/// outside the timed region, so the per-iteration time measured by Criterion
/// contains only the parallel workload execution plus nanosecond-scale
/// start/done signalling - not thread spawn/join or CPU-pinning overhead.
///
/// `W` is the immutable per-worker workload type and `M` the shared map type;
/// the per-iteration work is supplied as a `run(&workload, &map)` closure.
pub struct ConcurrentWorkers<W, M> {
    state: Arc<WorkerPoolState<W, M>>,
    handles: Vec<std::thread::JoinHandle<()>>,
}

/// Signature of the per-iteration workload runner invoked as
/// `run(&workload, &map)` by each worker thread.
type WorkerRunner<W, M> = dyn Fn(&W, &M) + Send + Sync;

/// State shared between the benchmark thread and the worker pool: the
/// start/done signalling, panic propagation, per-worker map slots, and
/// immutable per-worker workloads.
struct WorkerPoolState<W, M> {
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
    workloads: Vec<W>,
    /// Per-iteration workload runner, invoked as `run(&workload, &map)`.
    run: Arc<WorkerRunner<W, M>>,
}

impl<W, M> ConcurrentWorkers<W, M> {
    /// Spawns and pins `thread_count` worker threads, each repeatedly
    /// executing `run(workload, map)` on its own workload once per release.
    pub fn new<F>(thread_count: usize, workloads: &[W], run: F) -> Self
    where
        W: Clone + Send + Sync + 'static,
        M: Send + Sync + 'static,
        F: Fn(&W, &M) + Send + Sync + 'static,
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
            run: Arc::new(run),
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
                            (state.run)(workload, &*map);
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
    pub fn run(&self, target: usize) {
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

    /// Per-worker slots into which the (untimed) setup closure publishes the
    /// fresh map for the upcoming iteration before calling [`Self::run`].
    pub fn slots(&self) -> &[Mutex<Option<Arc<M>>>] {
        &self.state.slots
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

impl<W, M> Drop for ConcurrentWorkers<W, M> {
    fn drop(&mut self) {
        self.state.shutdown.store(true, Ordering::SeqCst);
        for handle in self.handles.drain(..) {
            handle.join().unwrap();
        }
    }
}
