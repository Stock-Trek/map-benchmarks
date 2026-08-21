use std::time::Duration;

pub const WARM_UP_TIME: Duration = Duration::from_secs(1);
pub const MEASUREMENT_TIME: Duration = Duration::from_secs(2);

pub const DEFAULT_ENTRY_COUNT: usize = 10_000;
pub const DEFAULT_ENTRY_COUNTS: &[usize] = &[1_000, 10_000, 100_000];
pub const DEFAULT_OP_COUNT: usize = 10_000;
pub const DEFAULT_THREAD_COUNT: usize = 3;
pub const DEFAULT_THREAD_COUNTS: &[usize] = &[2, 3];

pub const GROWTH_ENTRY_COUNTS: &[usize] = &[1_000, 10_000, 100_000, 1_000_000];
