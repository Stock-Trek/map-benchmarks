/// Creates a Criterion [`BenchmarkGroup`](criterion::BenchmarkGroup) with the
/// default configuration shared by every benchmark applied. Configuration that
/// depends on a specific benchmark (such as `throughput`, or a longer
/// measurement time for concurrent benchmarks) is still set on the returned
/// group.
#[macro_export]
macro_rules! bench_group {
    ($criterion:expr, $name:expr) => {{
        let mut group = $criterion.benchmark_group($name);
        group.warm_up_time($crate::config::WARM_UP_TIME);
        group.measurement_time($crate::config::MEASUREMENT_TIME);
        group.sampling_mode($crate::config::SAMPLING_MODE);
        group
    }};
}

#[macro_export]
macro_rules! expand_bench {
    ($bench_fn:ident, $key_type:ty, $group:expr, $($bench_type:ty),* $(,)?) => {
        $(
            $bench_fn::<$bench_type, $key_type>(
                <$bench_type as $crate::maps::BenchMapName>::NAME, $group
            );
        )*
    };
}

#[macro_export]
macro_rules! expand_bench_with_map_data {
    ($bench_fn:ident, $key_type:ty, $group:expr, $map_data:expr, $($bench_type:ty),* $(,)?) => {
        $(
            $bench_fn::<$bench_type, $key_type>(
                <$bench_type as $crate::maps::BenchMapName>::NAME, $group, $map_data
            );
        )*
    };
}

#[macro_export]
macro_rules! expand_bench_with_map_data_and_common_hasher {
    ($bench_fn:ident, $key_type:ty, $group:expr, $map_data:expr, $($bench_type:ty),* $(,)?) => {
        let hasher = $crate::common_hasher::CommonHasher::new();
        $(
            $bench_fn::<$bench_type, $key_type>(
                <$bench_type as $crate::maps::BenchMapName>::NAME, $group, $map_data, hasher.clone()
            );
        )*
    };
}

#[macro_export]
macro_rules! expand_bench_concurrent {
    ($bench_fn:ident, $key_type:ty, $group:expr, $map_data:expr, $thread_count:expr, $workload:expr, $($bench_type:ty),* $(,)?) => {
        $(
            $bench_fn::<$bench_type, $key_type>(
                <$bench_type as $crate::maps::BenchMapName>::NAME, $group, $map_data, $thread_count, $workload
            );
        )*
    };
}

#[macro_export]
macro_rules! expand_bench_concurrent_with_common_hasher {
    ($bench_fn:ident, $key_type:ty, $group:expr, $map_data:expr, $thread_count:expr, $workload:expr, $($bench_type:ty),* $(,)?) => {
        let hasher = $crate::common_hasher::CommonHasher::new();
        $(
            $bench_fn::<$bench_type, $key_type>(
                <$bench_type as $crate::maps::BenchMapName>::NAME, $group, $map_data, $thread_count, $workload, hasher.clone()
            );
        )*
    };
}
