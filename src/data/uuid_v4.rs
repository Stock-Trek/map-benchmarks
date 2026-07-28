use crate::data::data_gen::DataGen;
use hashbrown::HashSet;
use rand::rngs::ThreadRng;
use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct UuidV4DataGen;

impl DataGen for UuidV4DataGen {
    type Output = Uuid;

    fn generate_with(&self, count: usize, _rng: &mut ThreadRng) -> HashSet<Self::Output> {
        let mut result = HashSet::new();
        while result.len() < count {
            let candidate = uuid::Uuid::new_v4();
            result.insert(candidate);
        }
        result
    }
}
