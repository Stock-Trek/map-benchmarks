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
    ($bench_fn:ident, $group:expr, $map_data:expr, $thread_count:expr, $workload:expr, $($bench_type:ty),* $(,)?) => {
        $(
            $bench_fn::<$bench_type>(
                <$bench_type as $crate::maps::BenchMapName>::NAME, $group, $map_data, $thread_count, $workload
            );
        )*
    };
}

#[macro_export]
macro_rules! expand_bench_concurrent_with_common_hasher {
    ($bench_fn:ident, $group:expr, $map_data:expr, $thread_count:expr, $workload:expr, $($bench_type:ty),* $(,)?) => {
        let hasher = $crate::common_hasher::CommonHasher::new();
        $(
            $bench_fn::<$bench_type>(
                <$bench_type as $crate::maps::BenchMapName>::NAME, $group, $map_data, $thread_count, $workload, hasher.clone()
            );
        )*
    };
}
