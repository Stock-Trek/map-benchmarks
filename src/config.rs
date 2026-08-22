use std::time::Duration;

pub const WARM_UP_TIME: Duration = Duration::from_secs(1);
pub const MEASUREMENT_TIME: Duration = Duration::from_secs(2);

pub const DEFAULT_ENTRY_COUNT: usize = 10_000;
pub const DEFAULT_ENTRY_COUNTS: &[(usize, &str)] =
    &[(1_000, "1K"), (10_000, "10K"), (100_000, "100K")];
pub const DEFAULT_OP_COUNT: usize = 10_000;
pub const DEFAULT_THREAD_COUNT: usize = 3;
pub const DEFAULT_THREAD_COUNTS: &[usize] = &[2, 3];

/// The fraction of get-or-insert operations that hit keys already in the map;
/// the remainder are missing keys that get inserted (the "get-or-create cache
/// entry" pattern).
pub const GET_OR_INSERT_HIT_RATIO: f64 = 0.90;

pub const GROWTH_ENTRY_COUNTS: &[(usize, &str)] = &[
    (1_000, "1K"),
    (10_000, "10K"),
    (100_000, "100K"),
    (1_000_000, "1M"),
];
