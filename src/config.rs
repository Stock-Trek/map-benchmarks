use std::time::Duration;

pub const WARM_UP_TIME: Duration = Duration::from_secs(1);
pub const MEASUREMENT_TIME: Duration = Duration::from_secs(2);

pub const OUT_OF_THE_BOX_ENTRY_COUNT: &[usize] = &[1_000, 10_000, 100_000];

pub const SAME_HASHER_MAP_COUNT: usize = 10_000;

pub const KEY_SENSITIVITY_ENTRY_COUNT: usize = 10_000;

pub const WORKLOAD_ENTRY_COUNT: &[usize] = &[1_000, 10_000, 100_000];
pub const WORKLOAD_OP_COUNT: usize = 10_000;
pub const WORKLOAD_MISSING_KEY_COUNT: usize = 10_000;
pub const WORKLOAD_CONCURRENT_THREAD_COUNTS: &[usize] = &[2, 4];
