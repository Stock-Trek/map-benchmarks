use crate::data::data_gen::DataGen;
use hashbrown::HashSet;
use rand::{Rng, RngExt};

/// Generates u64 keys drawn from a Zipfian distribution over `0..num_items`.
///
/// A Zipfian distribution is a discrete power-law distribution in which the
/// probability of drawing an item is inversely proportional to a power of its
/// rank: rank 0 (the "hottest" key) is the most likely to be generated and
/// higher ranks become increasingly unlikely. The result is a skewed key set
/// in which hot keys dominate.
///
/// The skew is controlled by [`exponent`](U64ZipfianDataGen::exponent) (`s`):
///
/// - `s = 0` produces a uniform distribution over `0..num_items`
/// - larger values of `s` concentrate probability mass on the lowest ranks
#[derive(Clone, Copy, Debug)]
pub struct U64ZipfianDataGen {
    /// The number of items in the distribution; keys are drawn from `0..num_items`.
    pub num_items: u64,
    /// The skew exponent `s >= 0`; larger values produce more skew.
    pub exponent: f64,
}

impl U64ZipfianDataGen {
    /// Creates a new Zipfian generator over `0..num_items` with the given skew exponent.
    ///
    /// # Panics
    ///
    /// Panics if `num_items` is zero or `exponent` is negative.
    pub fn new(num_items: u64, exponent: f64) -> Self {
        assert!(num_items > 0, "num_items must be greater than zero");
        assert!(exponent >= 0.0, "exponent must be non-negative");
        Self {
            num_items,
            exponent,
        }
    }
}

impl DataGen for U64ZipfianDataGen {
    type Output = u64;

    fn generate_with(
        &self,
        count: usize,
        rng: &mut rand::rngs::ThreadRng,
    ) -> HashSet<Self::Output> {
        assert!(
            (self.num_items as usize) >= count,
            "cannot generate {count} distinct keys from a Zipfian distribution over only {} items",
            self.num_items,
        );

        let mut result = HashSet::new();
        while result.len() < count {
            let candidate = zipf_sample(rng, self.num_items, self.exponent);
            result.insert(candidate);
        }
        result
    }
}

/// Samples a rank `1..=num_items` from a Zipfian distribution via rejection
/// sampling (algorithm by Jason Crease, also used by `rand_distr::Zipf`) and
/// maps it to a zero-based key in `0..num_items`.
fn zipf_sample<R: Rng>(rng: &mut R, num_items: u64, exponent: f64) -> u64 {
    debug_assert!(num_items > 0);
    let n = num_items as f64;
    let q = 1.0 / (1.0 - exponent);
    let t = if exponent.is_infinite() {
        1.0
    } else if exponent == 1.0 {
        1.0 + n.ln()
    } else {
        (n.powf(1.0 - exponent) - exponent) * q
    };

    loop {
        let pt = rng.random::<f64>() * t;
        let inv_b = if pt <= 1.0 {
            pt
        } else if exponent == 1.0 {
            (pt - 1.0).exp()
        } else {
            (pt * (1.0 - exponent) + exponent).powf(q)
        };

        // Clamp guards against floating-point rounding pushing a sample past n.
        let x = (inv_b + 1.0).floor().clamp(1.0, n);
        let mut ratio = x.powf(-exponent);
        if x > 1.0 {
            ratio *= inv_b.powf(exponent);
        }

        if rng.random::<f64>() < ratio {
            return x as u64 - 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, rngs::StdRng};

    fn seeded_rng() -> StdRng {
        StdRng::seed_from_u64(42)
    }

    fn zipf_harmonic(num_items: u64, exponent: f64) -> f64 {
        (1..=num_items).map(|k| (k as f64).powf(-exponent)).sum()
    }

    #[test]
    fn new_rejects_zero_items() {
        let result = std::panic::catch_unwind(|| U64ZipfianDataGen::new(0, 1.5));
        assert!(result.is_err());
    }

    #[test]
    fn new_rejects_negative_exponent() {
        let result = std::panic::catch_unwind(|| U64ZipfianDataGen::new(100, -0.1));
        assert!(result.is_err());
    }

    #[test]
    fn generate_rejects_count_exceeding_items() {
        let generator = U64ZipfianDataGen::new(10, 1.5);
        let result = std::panic::catch_unwind(|| generator.generate(11));
        assert!(result.is_err());
    }

    #[test]
    fn samples_stay_within_range() {
        let mut rng = seeded_rng();
        for _ in 0..10_000 {
            let key = zipf_sample(&mut rng, 1_000, 1.5);
            assert!(key < 1_000);
        }
    }

    #[test]
    fn generate_yields_distinct_skewed_keys() {
        let generator = U64ZipfianDataGen::new(10_000, 1.5);
        let keys = generator.generate(200);
        assert_eq!(keys.len(), 200);
        assert!(keys.iter().all(|&k| k < 10_000));
        // The hottest key is overwhelmingly likely to be present.
        assert!(keys.contains(&0));
    }

    #[test]
    fn distribution_matches_zipfian_probabilities() {
        let num_items = 1_000u64;
        let exponent = 1.5;
        let mut rng = seeded_rng();
        let sample_count = 200_000;
        let mut rank_one = 0u64;
        let mut max_key = 0u64;
        for _ in 0..sample_count {
            let key = zipf_sample(&mut rng, num_items, exponent);
            assert!(key < num_items);
            max_key = max_key.max(key);
            if key == 0 {
                rank_one += 1;
            }
        }

        let expected = 1.0 / zipf_harmonic(num_items, exponent);
        let empirical = rank_one as f64 / sample_count as f64;
        assert!(
            (empirical - expected).abs() < 0.02,
            "rank-one frequency {empirical} differs from expected {expected}"
        );

        // The tail should be rare but present: ~8% of samples land at rank >= 100.
        assert!(max_key >= 100, "cold keys were never generated");
    }

    #[test]
    fn uniform_when_exponent_is_zero() {
        let num_items = 1_000u64;
        let mut rng = seeded_rng();
        let sample_count = 100_000;
        let mut counts = vec![0u64; num_items as usize];
        for _ in 0..sample_count {
            let key = zipf_sample(&mut rng, num_items, 0.0);
            assert!(key < num_items);
            counts[key as usize] += 1;
        }

        let mean = sample_count as f64 / num_items as f64;
        let min = *counts.iter().min().unwrap() as f64;
        let max = *counts.iter().max().unwrap() as f64;
        assert!(
            counts.iter().all(|&c| c > 0),
            "some keys were never generated"
        );
        assert!(
            max - min < 4.0 * mean.sqrt() * 4.0,
            "uniform counts spread too wide: min {min}, max {max}, mean {mean}"
        );
    }
}
