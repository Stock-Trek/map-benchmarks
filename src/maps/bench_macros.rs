/// Prints the start and end wall-clock time of a benchmark along with how long
/// it took to run. Used internally by the `expand_bench*` macros.
#[doc(hidden)]
#[macro_export]
macro_rules! __bench_timed {
    ($name:expr, $body:expr) => {{
        let start_wall = ::std::time::SystemTime::now()
            .duration_since(::std::time::UNIX_EPOCH)
            .map(|d| d.as_secs_f64())
            .unwrap_or_default();
        let start = ::std::time::Instant::now();
        println!("[bench] start {} at {:.3}", $name, start_wall);
        let result = $body;
        let duration = start.elapsed();
        let end_wall = ::std::time::SystemTime::now()
            .duration_since(::std::time::UNIX_EPOCH)
            .map(|d| d.as_secs_f64())
            .unwrap_or_default();
        println!(
            "[bench] end   {} at {:.3} (duration {:.3}s)",
            $name,
            end_wall,
            duration.as_secs_f64()
        );
        result
    }};
}

#[macro_export]
macro_rules! expand_bench {
    ($bench_fn:ident, $key_type:ty, $group:expr, $($bench_type:ty),* $(,)?) => {
        $(
            let name = <$bench_type as $crate::maps::BenchMapName>::NAME;
            $crate::__bench_timed!(name, $bench_fn::<$bench_type, $key_type>(name, $group));
        )*
    };
}

#[macro_export]
macro_rules! expand_bench_with_map_data {
    ($bench_fn:ident, $key_type:ty, $group:expr, $map_data:expr, $($bench_type:ty),* $(,)?) => {
        $(
            let name = <$bench_type as $crate::maps::BenchMapName>::NAME;
            $crate::__bench_timed!(
                name,
                $bench_fn::<$bench_type, $key_type>(name, $group, $map_data)
            );
        )*
    };
}

#[macro_export]
macro_rules! expand_bench_with_map_data_and_common_hasher {
    ($bench_fn:ident, $key_type:ty, $group:expr, $map_data:expr, $($bench_type:ty),* $(,)?) => {
        let hasher = $crate::common_hasher::CommonHasher::new();
        $(
            let name = <$bench_type as $crate::maps::BenchMapName>::NAME;
            $crate::__bench_timed!(
                name,
                $bench_fn::<$bench_type, $key_type>(name, $group, $map_data, hasher.clone())
            );
        )*
    };
}

#[macro_export]
macro_rules! expand_bench_concurrent {
    ($bench_fn:ident, $key_type:ty, $group:expr, $map_data:expr, $thread_count:expr, $workload:expr, $($bench_type:ty),* $(,)?) => {
        $(
            let name = <$bench_type as $crate::maps::BenchMapName>::NAME;
            $crate::__bench_timed!(
                name,
                $bench_fn::<$bench_type, $key_type>(
                    name, $group, $map_data, $thread_count, $workload
                )
            );
        )*
    };
}

#[macro_export]
macro_rules! expand_bench_concurrent_with_common_hasher {
    ($bench_fn:ident, $key_type:ty, $group:expr, $map_data:expr, $thread_count:expr, $workload:expr, $($bench_type:ty),* $(,)?) => {
        let hasher = $crate::common_hasher::CommonHasher::new();
        $(
            let name = <$bench_type as $crate::maps::BenchMapName>::NAME;
            $crate::__bench_timed!(
                name,
                $bench_fn::<$bench_type, $key_type>(
                    name, $group, $map_data, $thread_count, $workload, hasher.clone()
                )
            );
        )*
    };
}
