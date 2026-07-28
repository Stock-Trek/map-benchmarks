use hashbrown::HashSet;
use rand::{RngExt, seq::SliceRandom};

pub trait DataGen {
    type Output: Clone + Ord;

    fn generate_unique(&self, count: usize, avoid: &HashSet<Self::Output>) -> Vec<Self::Output>;
    fn generate(&self, count: usize, unique_proportion: f32, sorted: bool) -> Vec<Self::Output> {
        if count == 0 {
            return Vec::new();
        }
        let mut rng = rand::rng();
        let unique_count = ((unique_proportion * count as f32).round() as usize).clamp(1, count);
        let uniques = self.generate_unique(unique_count, &HashSet::new());
        if unique_count == count {
            return uniques;
        }
        let mut result = Vec::with_capacity(count);
        result.extend_from_slice(&uniques);
        while result.len() < count {
            let idx = rng.random_range(0..uniques.len());
            result.push(uniques[idx].clone());
        }
        if sorted {
            result.sort();
        } else {
            result.shuffle(&mut rng);
        }
        result
    }
}
