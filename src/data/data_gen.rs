use hashbrown::HashSet;
use rand::rngs::ThreadRng;
use std::hash::Hash;

pub trait DataGen: Clone + Copy {
    type Output: Clone + Ord + Hash + Eq;

    fn generate(&self, count: usize) -> HashSet<Self::Output> {
        self.generate_avoiding(count, &HashSet::new())
    }
    fn generate_with(&self, count: usize, rng: &mut ThreadRng) -> HashSet<Self::Output>;
    fn generate_avoiding(
        &self,
        count: usize,
        avoid: &HashSet<Self::Output>,
    ) -> HashSet<Self::Output> {
        if count == 0 {
            return HashSet::new();
        }

        let mut rng = rand::rng();
        let mut result = HashSet::new();

        while result.len() < count {
            let required_count = count - result.len();
            let mut candidates = self.generate_with(required_count, &mut rng);
            for candidate in candidates.drain() {
                if !avoid.contains(&candidate) {
                    result.insert(candidate);
                }
            }
        }
        result
    }
}
