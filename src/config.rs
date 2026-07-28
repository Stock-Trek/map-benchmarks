use std::time::Duration;

pub const WARM_UP_TIME: Duration = Duration::from_secs(1);
pub const MEASUREMENT_TIME: Duration = Duration::from_secs(2);

pub const BASELINE_ENTRY_COUNT: &[usize] = &[10_000, 100_000, 1_000_000];
pub const KEY_SENSITIVITY_ENTRY_COUNT: &[usize] = &[100_000];

pub const CONCURRENCY_OPS_PER_THREAD: usize = 10_000;
pub const CONCURRENCY_THREAD_COUNTS: &[usize] = &[1, 2, 4];
