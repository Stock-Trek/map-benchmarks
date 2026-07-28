use crate::maps::sync_benchmap::SyncBenchMap;
use rand::{RngExt, SeedableRng};

/// Operation types for workloads.
#[derive(Clone, Copy, Debug)]
pub enum Op {
    LookupHit,
    LookupMiss,
    Insert,
    Update,
    Remove,
}

/// A workload design specifies the exact number of each operation type
/// to include in a generated workload.
///
/// This allows benchmarks to precisely control the composition of operations
/// for reproducibility and cross-map comparability.
#[derive(Clone, Copy, Debug)]
pub struct WorkloadDesign {
    pub lookup_hits: usize,
    pub lookup_misses: usize,
    pub inserts: usize,
    pub updates: usize,
    pub removes: usize,
}

impl WorkloadDesign {
    /// Total number of operations in this design.
    pub fn total_ops(&self) -> usize {
        self.lookup_hits + self.lookup_misses + self.inserts + self.updates + self.removes
    }

    /// A balanced workload: 80% lookup hit, 5% insert, 10% update, 5% remove.
    pub fn balanced(total_ops: usize) -> Self {
        Self {
            lookup_hits: (total_ops as f64 * 0.80) as usize,
            lookup_misses: 0,
            inserts: (total_ops as f64 * 0.05) as usize,
            updates: (total_ops as f64 * 0.10) as usize,
            removes: (total_ops as f64 * 0.05) as usize,
        }
    }
}

/// A single operation with its associated key.
#[derive(Clone, Copy, Debug)]
pub struct WorkItem {
    pub op: Op,
    pub key: u64,
}

/// A pre-generated workload for a single thread.
#[derive(Clone)]
pub struct ThreadWorkload {
    pub items: Vec<WorkItem>,
}

/// Generate a workload for a single thread from a design.
///
/// `existing_keys` are used for lookup hits, updates, and removes.
/// `missing_keys` are used for lookup misses and inserts.
pub fn generate_workload(
    design: &WorkloadDesign,
    existing_keys: &[u64],
    missing_keys: &[u64],
    rng: &mut impl RngExt,
) -> ThreadWorkload {
    let total = design.total_ops();
    let mut items = Vec::with_capacity(total);

    for _ in 0..design.lookup_hits {
        items.push(WorkItem {
            op: Op::LookupHit,
            key: existing_keys[rng.random_range(0..existing_keys.len())],
        });
    }
    for _ in 0..design.lookup_misses {
        items.push(WorkItem {
            op: Op::LookupMiss,
            key: missing_keys[rng.random_range(0..missing_keys.len())],
        });
    }
    for _ in 0..design.inserts {
        items.push(WorkItem {
            op: Op::Insert,
            key: missing_keys[rng.random_range(0..missing_keys.len())],
        });
    }
    for _ in 0..design.updates {
        items.push(WorkItem {
            op: Op::Update,
            key: existing_keys[rng.random_range(0..existing_keys.len())],
        });
    }
    for _ in 0..design.removes {
        items.push(WorkItem {
            op: Op::Remove,
            key: existing_keys[rng.random_range(0..existing_keys.len())],
        });
    }

    ThreadWorkload { items }
}

/// Generate workloads for all threads using the same design.
pub fn generate_workloads(
    design: &WorkloadDesign,
    existing_keys: &[u64],
    missing_keys: &[u64],
    thread_count: usize,
) -> Vec<ThreadWorkload> {
    let mut workloads = Vec::with_capacity(thread_count);
    for i in 0..thread_count {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42 + i as u64);
        workloads.push(generate_workload(
            design,
            existing_keys,
            missing_keys,
            &mut rng,
        ));
    }
    workloads
}

/// Run a single thread's workload against a map.
pub fn run_workload<M, K, V>(map: &M, workload: &ThreadWorkload)
where
    M: SyncBenchMap<K, V>,
    K: From<u64> + Copy,
    V: From<u64> + Copy,
{
    for item in &workload.items {
        let key = K::from(item.key);
        match item.op {
            Op::LookupHit => {
                std::hint::black_box(map.get_cloned(std::hint::black_box(&key)));
            }
            Op::LookupMiss => {
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
