use crate::data::data_gen::DataGen;
use hashbrown::HashSet;
use rand::RngExt;

pub struct StringDataGen<const LENGTH: usize>;

impl<const LENGTH: usize> DataGen for StringDataGen<LENGTH> {
    type Output = String;

    fn generate_unique(&self, count: usize, avoid: &HashSet<Self::Output>) -> Vec<Self::Output> {
        if count == 0 {
            return Vec::new();
        }

        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                  abcdefghijklmnopqrstuvwxyz\
                                  0123456789";

        let mut rng = rand::rng();
        let mut result = Vec::with_capacity(count);
        let mut gen_string = || -> String {
            (0..LENGTH)
                .map(|_| {
                    let idx = rng.random_range(0..CHARSET.len());
                    CHARSET[idx] as char
                })
                .collect()
        };
        while result.len() < count {
            let mut candidate = gen_string();
            while avoid.contains(&candidate) || result.contains(&candidate) {
                candidate = gen_string();
            }
            result.push(candidate);
        }
        result
    }
}
