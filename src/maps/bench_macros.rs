#[macro_export]
macro_rules! expand_bench {
    ($bench_fn:ident, $group:expr, $($bench_type:ty),* $(,)?) => {
        $(
            $bench_fn::<$bench_type>(
                $group, <$bench_type as crate::benchmap::BenchMapName>::NAME
            );
        )*
    };
}

#[macro_export]
macro_rules! expand_bench_with_map_data {
    ($bench_fn:ident, $group:expr, $map_data:expr, $($bench_type:ty),* $(,)?) => {
        $(
            $bench_fn::<$bench_type>(
                $group, $map_data, <$bench_type as crate::benchmap::BenchMapName>::NAME
            );
        )*
    };
}
