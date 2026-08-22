use std::time::Duration;

pub const WARM_UP_TIME: Duration = Duration::from_secs(1);
pub const MEASUREMENT_TIME: Duration = Duration::from_secs(2);

pub const DEFAULT_ENTRY_COUNT: usize = 10_000;
pub const DEFAULT_ENTRY_COUNTS: &[(usize, &str)] =
    &[(1_000, "1K"), (10_000, "10K"), (100_000, "100K")];
pub const DEFAULT_OP_COUNT: usize = 10_000;
pub const DEFAULT_THREAD_COUNT: usize = 3;
pub const DEFAULT_THREAD_COUNTS: &[usize] = &[2, 3];

pub const GROWTH_ENTRY_COUNTS: &[(usize, &str)] = &[
    (1_000, "1K"),
    (10_000, "10K"),
    (100_000, "100K"),
    (1_000_000, "1M"),
];
