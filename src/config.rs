use std::time::Duration;

pub const WARM_UP_TIME: Duration = Duration::from_secs(1);
pub const MEASUREMENT_TIME: Duration = Duration::from_secs(2);

pub const OUT_OF_THE_BOX_ENTRY_COUNT: &[usize] = &[100, 10_000, 1_000_000];

pub const SAME_HASHER_MAP_COUNT: usize = 10_000;

pub const CONCURRENCY_OPS_PER_THREAD: usize = 10_000;
pub const CONCURRENCY_THREAD_COUNTS: &[usize] = &[1, 2, 4];

pub const BULK_CLEAR_ENTRY_COUNT: &[usize] = &[100, 10_000];

pub const KEY_SENSITIVITY_ENTRY_COUNT: usize = 10_000;

pub const MIXED_ENTRY_COUNT: &[usize] = &[100, 10_000, 1_000_000];
pub const MIXED_OPS_PER_DESIGN: usize = 10_000;
pub const MIXED_MISSING_KEY_COUNT: usize = 10_000;
