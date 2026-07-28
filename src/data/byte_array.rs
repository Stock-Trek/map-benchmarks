use crate::data::data_gen::DataGen;
use hashbrown::HashSet;
use rand::RngExt;

pub struct ByteArrayDataGen<const LENGTH: usize>;

impl<const LENGTH: usize> DataGen for ByteArrayDataGen<LENGTH> {
    type Output = [u8; LENGTH];

    fn generate_unique(&self, count: usize, avoid: &HashSet<Self::Output>) -> Vec<Self::Output> {
        let mut rng = rand::rng();
        let unique = (0..count)
            .map(|_| {
                let mut arr = [0u8; LENGTH];
                rng.fill(&mut arr);
                while avoid.contains(&arr) {
                    rng.fill(&mut arr);
                }
                arr
            })
            .collect();
        unique
    }
}
