use std::time::Duration;

pub const WARM_UP_TIME: Duration = Duration::from_secs(1);
pub const MEASUREMENT_TIME: Duration = Duration::from_secs(2);
pub const ENTRY_COUNT: &[usize] = &[10_000, 100_000];
