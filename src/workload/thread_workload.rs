use crate::maps::benchmap::{
    BenchMapGetCloned, BenchMapInsert, BenchMapMutInsert, BenchMapMutRemove, BenchMapRemove,
};
use crate::workload::{design::WorkloadDesign, item::WorkItem, op::WorkloadOp};
use rand::RngExt;
use std::sync::Mutex;

#[derive(Clone)]
pub struct ThreadWorkload {
    pub items: Vec<WorkItem>,
}

impl ThreadWorkload {
    pub fn new(
        design: &WorkloadDesign,
        existing_keys: &[u64],
        missing_keys: &[u64],
        rng: &mut impl RngExt,
    ) -> Self {
        let total = design.total_ops();
        let mut items = Vec::with_capacity(total);

        for _ in 0..design.lookup_hits {
            items.push(WorkItem {
                op: WorkloadOp::Lookup,
                key: existing_keys[rng.random_range(0..existing_keys.len())],
            });
        }
        for _ in 0..design.lookup_misses {
            items.push(WorkItem {
                op: WorkloadOp::Lookup,
                key: missing_keys[rng.random_range(0..missing_keys.len())],
            });
        }
        for _ in 0..design.inserts {
            items.push(WorkItem {
                op: WorkloadOp::Insert,
                key: missing_keys[rng.random_range(0..missing_keys.len())],
            });
        }
        for _ in 0..design.updates {
            items.push(WorkItem {
                op: WorkloadOp::Insert,
                key: existing_keys[rng.random_range(0..existing_keys.len())],
            });
        }
        for _ in 0..design.removes {
            items.push(WorkItem {
                op: WorkloadOp::Remove,
                key: existing_keys[rng.random_range(0..existing_keys.len())],
            });
        }
        Self { items }
    }

    /// Run this workload on a concurrent map that supports `&self` for all operations.
    pub fn run_shared(
        &self,
        map: &(impl BenchMapGetCloned<u64, u64> + BenchMapInsert<u64, u64> + BenchMapRemove<u64, u64>),
    ) {
        for item in &self.items {
            match item.op {
                WorkloadOp::Lookup => {
                    let _ = map.get_cloned(&item.key);
                }
                WorkloadOp::Insert => {
                    map.insert(item.key, 42u64);
                }
                WorkloadOp::Remove => {
                    map.remove(&item.key);
                }
            }
        }
    }

    /// Run this workload on a non-concurrent map protected by a Mutex.
    pub fn run_mutex(
        &self,
        map: &Mutex<
            impl BenchMapGetCloned<u64, u64> + BenchMapMutInsert<u64, u64> + BenchMapMutRemove<u64, u64>,
        >,
    ) {
        for item in &self.items {
            match item.op {
                WorkloadOp::Lookup => {
                    let guard = map.lock().unwrap();
                    let _ = guard.get_cloned(&item.key);
                }
                WorkloadOp::Insert => {
                    let mut guard = map.lock().unwrap();
                    guard.insert(item.key, 42u64);
                }
                WorkloadOp::Remove => {
                    let mut guard = map.lock().unwrap();
                    guard.remove(&item.key);
                }
            }
        }
    }
}
