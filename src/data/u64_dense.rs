use crate::data::data_gen::DataGen;
use hashbrown::HashSet;
use rand::{RngExt, rngs::ThreadRng};

#[derive(Clone, Copy)]
pub struct U64DenseDataGen;

impl DataGen for U64DenseDataGen {
    type Output = u64;

    fn generate_with(&self, count: usize, rng: &mut ThreadRng) -> HashSet<Self::Output> {
        let start_max = u64::MAX - (count as u64);
        let start = rng.random_range(u64::MIN..start_max);
        let max = start + (count as u64);
        let mut result = HashSet::new();
        for candidate in start..max {
            result.insert(candidate);
        }
        result
    }
}
