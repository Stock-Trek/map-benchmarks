#[macro_export]
macro_rules! expand_bench {
    ($bench_fn:ident, $group:expr, $($bench_type:ty),* $(,)?) => {
        $(
            $bench_fn::<$bench_type>(
                <$bench_type as $crate::maps::BenchMapName>::NAME, $group
            );
        )*
    };
}

#[macro_export]
macro_rules! expand_bench_with_map_data {
    ($bench_fn:ident, $group:expr, $map_data:expr, $($bench_type:ty),* $(,)?) => {
        $(
            $bench_fn::<$bench_type>(
                <$bench_type as $crate::maps::BenchMapName>::NAME, $group, $map_data
            );
        )*
    };
}

#[macro_export]
macro_rules! expand_bench_with_map_data_and_hasher {
    ($bench_fn:ident, $group:expr, $map_data:expr, $hasher:expr, $($bench_type:ty),* $(,)?) => {
        $(
            $bench_fn::<$bench_type>(
                <$bench_type as $crate::maps::BenchMapName>::NAME, $group, $map_data, $hasher.clone()
            );
        )*
    };
}

#[macro_export]
macro_rules! expand_bench_with_kv_map_data_and_hasher {
    ($bench_fn:ident, $key_type:ty, $value_type:ty, $group:expr, $map_data:expr, $hasher:expr, $($bench_type:ty),* $(,)?) => {
        $(
            $bench_fn::<$bench_type, $key_type, $value_type>(
                <$bench_type as $crate::maps::BenchMapName>::NAME, $group, $map_data, $hasher.clone()
            );
        )*
    };
}
