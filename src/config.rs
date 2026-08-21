use std::time::Duration;

pub const WARM_UP_TIME: Duration = Duration::from_secs(1);
pub const MEASUREMENT_TIME: Duration = Duration::from_secs(2);

pub const OUT_OF_THE_BOX_ENTRY_COUNT: &[usize] = &[1_000, 10_000, 100_000];

pub const SAME_HASHER_MAP_COUNT: usize = 10_000;

pub const KEY_SENSITIVITY_ENTRY_COUNT: usize = 10_000;

pub const WORKLOAD_ENTRY_COUNT: &[usize] = &[1_000, 10_000, 100_000];
pub const WORKLOAD_OP_COUNT: usize = 10_000;
pub const WORKLOAD_MISSING_KEY_COUNT: usize = 10_000;
pub const WORKLOAD_CONCURRENT_THREAD_COUNTS: &[usize] = &[2, 3];
/// Skew exponent `s` of the Zipfian (hot-key) workload: larger values
/// concentrate more accesses on the hottest keys (`s = 0` is uniform).
/// `s = 1.0` is classic Zipf's law, the standard model for real-world
/// hot-key workloads (caches, trending items).
pub const WORKLOAD_ZIPFIAN_EXPONENT: f64 = 1.0;
/// The Zipfian key space is `entry_count * WORKLOAD_ZIPFIAN_KEY_SPACE_MULTIPLIER`,
/// so the map holds the hottest `entry_count` keys of a larger domain while
/// leaving room for a cold tail of missing keys.
pub const WORKLOAD_ZIPFIAN_KEY_SPACE_MULTIPLIER: usize = 2;

pub const GET_OR_INSERT_EXISTING_KEY_COUNT: usize = 100;
pub const GET_OR_INSERT_MISSING_KEY_COUNT: usize = 100;
/// Fraction of each concurrent worker's get-or-insert operations that target
/// keys already present in the map (the "get-or-create cache entry" pattern);
/// the remainder target missing keys and insert them.
pub const GET_OR_INSERT_CONCURRENT_HIT_RATIO: f64 = 0.90;
