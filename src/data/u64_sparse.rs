use crate::data::data_gen::DataGen;
use hashbrown::HashSet;
use rand::RngExt;

pub struct U64SparseDataGen;

impl DataGen for U64SparseDataGen {
    type Output = u64;

    fn generate_unique(&self, count: usize, avoid: &HashSet<Self::Output>) -> Vec<Self::Output> {
        let mut rng = rand::rng();
        let mut vec = Vec::with_capacity(count);
        while vec.len() < count {
            let i = rng.random_range(u64::MIN..u64::MAX);
            if !avoid.contains(&i) {
                vec.push(i as u64);
            }
        }
        vec
    }
}
