use crate::data::u64_zipfian::U64ZipfianDataGen;
use crate::workload::{design::WorkloadDesign, item::WorkItem, op::WorkloadOp};
use rand::RngExt;

#[derive(Clone)]
pub struct ThreadWorkload {
    pub items: Vec<WorkItem>,
}

impl ThreadWorkload {
    /// Builds a workload whose keys are drawn uniformly from the provided key
    /// slices (the default, uniform-access workload).
    pub fn new(
        design: &WorkloadDesign,
        existing_keys: &[u64],
        missing_keys: &[u64],
        rng: &mut impl RngExt,
    ) -> Self {
        Self::generate(
            design,
            existing_keys,
            missing_keys,
            rng,
            KeySelector::Uniform,
        )
    }

    /// Builds a workload whose keys are drawn from a Zipfian (hot-key)
    /// distribution over the provided key slices, so the key at the lowest
    /// index of each slice is accessed far more often than the rest.
    ///
    /// Pass the slices sorted ascending so that index `0` holds the hottest
    /// key and higher indices are increasingly cold, matching the skew of
    /// `exponent`.
    pub fn new_zipfian(
        design: &WorkloadDesign,
        existing_keys: &[u64],
        missing_keys: &[u64],
        rng: &mut impl RngExt,
        exponent: f64,
    ) -> Self {
        Self::generate(
            design,
            existing_keys,
            missing_keys,
            rng,
            KeySelector::Zipfian(exponent),
        )
    }

    fn generate(
        design: &WorkloadDesign,
        existing_keys: &[u64],
        missing_keys: &[u64],
        rng: &mut impl RngExt,
        selector: KeySelector,
    ) -> Self {
        let total = design.total_ops();
        let mut items = Vec::with_capacity(total);

        for _ in 0..design.lookup_hits {
            items.push(WorkItem {
                op: WorkloadOp::Lookup,
                key: pick_key(existing_keys, rng, selector),
            });
        }
        for _ in 0..design.lookup_misses {
            items.push(WorkItem {
                op: WorkloadOp::Lookup,
                key: pick_key(missing_keys, rng, selector),
            });
        }
        for _ in 0..design.inserts {
            items.push(WorkItem {
                op: WorkloadOp::Insert,
                key: pick_key(missing_keys, rng, selector),
            });
        }
        for _ in 0..design.updates {
            items.push(WorkItem {
                op: WorkloadOp::Insert,
                key: pick_key(existing_keys, rng, selector),
            });
        }
        for _ in 0..design.removes {
            items.push(WorkItem {
                op: WorkloadOp::Remove,
                key: pick_key(existing_keys, rng, selector),
            });
        }
        Self { items }
    }
}

#[derive(Clone, Copy)]
enum KeySelector {
    Uniform,
    Zipfian(f64),
}

fn pick_key(keys: &[u64], rng: &mut impl RngExt, selector: KeySelector) -> u64 {
    match selector {
        KeySelector::Uniform => keys[rng.random_range(0..keys.len())],
        KeySelector::Zipfian(exponent) => {
            // Sample a Zipfian rank over the slice: index 0 (the hottest key
            // when the slice is sorted ascending) is drawn far more often than
            // the cold tail.
            let zipf = U64ZipfianDataGen::new(keys.len() as u64, exponent);
            keys[zipf.sample_key(rng) as usize]
        }
    }
}
