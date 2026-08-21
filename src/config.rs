use std::time::Duration;

pub const WARM_UP_TIME: Duration = Duration::from_secs(1);
pub const MEASUREMENT_TIME: Duration = Duration::from_secs(2);

pub const OUT_OF_THE_BOX_ENTRY_COUNT: &[usize] = &[1_000, 10_000, 100_000];

/// Entry counts for the growth benchmark: the map starts empty and is grown
/// by inserting this many u64 keys on a single thread.
pub const GROWTH_ENTRY_COUNT: &[usize] = &[1_000, 10_000, 100_000, 1_000_000];

pub const CREATE_MAP_COUNT: usize = 10_000;

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

pub const SYNC_THREAD_COUNT: usize = 3;
pub const SYNC_OP_COUNT: usize = 10_000;
/// The single key every thread of the synchronization benchmark contends on.
pub const SYNC_KEY: u64 = 0;

pub const CONTENTION_THREAD_COUNT: usize = 3;
/// Size of the dense key set held by the contention map: a compact range of
/// consecutive keys, so the map data is the same for every contention test
/// and only the query key distribution (uniform / Zipfian) varies. The set
/// is large enough to exercise the hash table broadly while still letting
/// the hot-key query distributions concentrate traffic on a small subset.
pub const CONTENTION_ENTRY_COUNT: usize = 10_000;
pub const CONTENTION_OP_COUNT: usize = 10_000;

pub const GET_OR_INSERT_EXISTING_KEY_COUNT: usize = 100;
pub const GET_OR_INSERT_MISSING_KEY_COUNT: usize = 100;
/// Fraction of each concurrent worker's get-or-insert operations that target
/// keys already present in the map (the "get-or-create cache entry" pattern);
/// the remainder target missing keys and insert them.
pub const GET_OR_INSERT_CONCURRENT_HIT_RATIO: f64 = 0.90;
