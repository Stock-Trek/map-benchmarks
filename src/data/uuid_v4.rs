use crate::data::data_gen::DataGen;
use hashbrown::HashSet;
use uuid::Uuid;

pub struct UuidV4DataGen;

impl DataGen for UuidV4DataGen {
    type Output = Uuid;

    fn generate_unique(&self, count: usize, avoid: &HashSet<Self::Output>) -> Vec<Self::Output> {
        let mut vec = Vec::with_capacity(count);
        while vec.len() < count {
            let uuid = uuid::Uuid::new_v4();
            if !avoid.contains(&uuid) {
                vec.push(uuid);
            }
        }
        vec
    }
}
