use crate::data::data_gen::DataGen;
use hashbrown::HashSet;
use rand::{RngExt, rngs::ThreadRng};

#[derive(Clone, Copy)]
pub struct U64SparseDataGen;

impl DataGen for U64SparseDataGen {
    type Output = u64;

    fn generate_with(&self, count: usize, rng: &mut ThreadRng) -> HashSet<Self::Output> {
        let mut result = HashSet::new();
        while result.len() < count {
            let candidate = rng.random_range(u64::MIN..u64::MAX);
            result.insert(candidate as u64);
        }
        result
    }
}
