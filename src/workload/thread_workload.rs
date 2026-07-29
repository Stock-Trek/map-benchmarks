use crate::workload::{design::WorkloadDesign, item::WorkItem, op::WorkloadOp};
use rand::RngExt;

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
}
