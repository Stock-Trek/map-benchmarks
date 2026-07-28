use crate::data::data_gen::DataGen;
use hashbrown::HashSet;
use rand::{RngExt, rngs::ThreadRng};

#[derive(Clone, Copy)]
pub struct ByteArrayDataGen<const LENGTH: usize>;

impl<const LENGTH: usize> DataGen for ByteArrayDataGen<LENGTH> {
    type Output = [u8; LENGTH];

    fn generate_with(&self, count: usize, rng: &mut ThreadRng) -> HashSet<Self::Output> {
        let mut result = HashSet::new();
        while result.len() < count {
            let mut candidate = [0u8; LENGTH];
            rng.fill(&mut candidate);
            result.insert(candidate);
        }
        result
    }
}
