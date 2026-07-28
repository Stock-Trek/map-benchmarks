use crate::data::data_gen::DataGen;
use hashbrown::HashSet;
use rand::{RngExt, rngs::ThreadRng};

#[derive(Clone, Copy)]
pub struct StringDataGen<const LENGTH: usize>;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                         abcdefghijklmnopqrstuvwxyz\
                         0123456789";

impl<const LENGTH: usize> DataGen for StringDataGen<LENGTH> {
    type Output = String;

    fn generate_with(&self, count: usize, rng: &mut ThreadRng) -> HashSet<Self::Output> {
        let mut result = HashSet::new();
        while result.len() < count {
            let candidate = (0..LENGTH)
                .map(|_| {
                    let idx = rng.random_range(0..CHARSET.len());
                    CHARSET[idx] as char
                })
                .collect();
            result.insert(candidate);
        }
        result
    }
}
