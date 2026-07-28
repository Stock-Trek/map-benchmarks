use crate::data::data_gen::DataGen;
use hashbrown::HashSet;

pub struct U64DenseDataGen;

impl DataGen for U64DenseDataGen {
    type Output = u64;

    fn generate_unique(&self, count: usize, avoid: &HashSet<Self::Output>) -> Vec<Self::Output> {
        let mut vec = Vec::with_capacity(count);
        let mut i = 0;
        while vec.len() < count {
            if !avoid.contains(&i) {
                vec.push(i as u64);
            }
            i += 1;
        }
        vec
    }
}
