use crate::{
    data::u64_zipfian::U64ZipfianDataGen,
    workload::{design::WorkloadDesign, item::WorkItem, op::WorkloadOp},
};
use rand::RngExt;

#[derive(Clone)]
pub struct ThreadWorkload {
    pub items: Vec<WorkItem>,
}

#[derive(Clone, Copy)]
pub enum KeyDistribution {
    Uniform,
    Zipfian(f64),
}

impl std::fmt::Debug for KeyDistribution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyDistribution::Uniform => write!(f, "Uniform"),
            KeyDistribution::Zipfian(exponent) => f.debug_tuple("Zipfian").field(exponent).finish(),
        }
    }
}

impl std::fmt::Display for KeyDistribution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyDistribution::Uniform => write!(f, "Uniform"),
            KeyDistribution::Zipfian(exponent) => f.debug_tuple("Zipfian").field(exponent).finish(),
        }
    }
}

impl KeyDistribution {
    /// Builds a thread workload whose keys are drawn from the provided key slices
    pub fn thread_workload(
        &self,
        design: &WorkloadDesign,
        existing_keys: &[u64],
        missing_keys: &[u64],
        rng: &mut impl RngExt,
    ) -> ThreadWorkload {
        let total = design.total_ops();
        let mut items = Vec::with_capacity(total);

        for _ in 0..design.lookup_hits {
            items.push(WorkItem {
                op: WorkloadOp::Lookup,
                key: self.pick_key(existing_keys, rng),
            });
        }
        for _ in 0..design.lookup_misses {
            items.push(WorkItem {
                op: WorkloadOp::Lookup,
                key: self.pick_key(missing_keys, rng),
            });
        }
        for _ in 0..design.inserts {
            items.push(WorkItem {
                op: WorkloadOp::Insert,
                key: self.pick_key(missing_keys, rng),
            });
        }
        for _ in 0..design.updates {
            items.push(WorkItem {
                op: WorkloadOp::Insert,
                key: self.pick_key(existing_keys, rng),
            });
        }
        for _ in 0..design.removes {
            items.push(WorkItem {
                op: WorkloadOp::Remove,
                key: self.pick_key(existing_keys, rng),
            });
        }
        ThreadWorkload { items }
    }

    fn pick_key(&self, keys: &[u64], rng: &mut impl RngExt) -> u64 {
        match self {
            Self::Uniform => keys[rng.random_range(0..keys.len())],
            Self::Zipfian(exponent) => {
                // Sample a Zipfian rank over the slice: index 0 (the hottest key
                // when the slice is sorted ascending) is drawn far more often than
                // the cold tail.
                let zipf = U64ZipfianDataGen::new(keys.len() as u64, *exponent);
                keys[zipf.sample_key(rng) as usize]
            }
        }
    }
}
