use bench_map::{
    config::*,
    data::u64_sparse::U64SparseDataGen,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BTreeMapBenchMap, BenchMapMutInsert, ConcreadBenchMap, DashMapBenchMap,
        HashbrownBenchMap, ImmutableChunkMapBenchMap, IndexMapBenchMap, RustCHashBenchMap,
        StarshardBenchMap, StdBenchMap, TxMapBenchMap,
    },
    thousands_format::format_with_underscores,
};
use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
use std::rc::Rc;

macro_rules! bench_insert_missing {
    ($group:ident, $map_data:expr, $map_type:path, $name:expr) => {
        let map_data = $map_data;
        $group.bench_function($name, move |b| {
            let map_data_ref = &map_data;
            b.iter_batched(
                move || {
                    let map = map_data_ref.create_map::<$map_type>();
                    let keys = map_data_ref.missing_keys().clone();
                    (map, keys)
                },
                |(mut map, mut keys)| {
                    for key in keys.drain(..) {
                        let key = std::hint::black_box(key);
                        std::hint::black_box(map.insert(key, 42));
                    }
                    std::hint::black_box(map)
                },
                BatchSize::PerIteration,
            );
        });
    };
}

fn baseline_insert(c: &mut Criterion) {
    let entry_count = 0;
    let existing_key_count = 0;
    let sort_keys = false;
    for missing_key_count in BASELINE_ENTRY_COUNT {
        let map_data = Rc::new(MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            entry_count,
            existing_key_count,
            *missing_key_count,
            sort_keys,
        ));
        let mut group = c.benchmark_group(format!(
            "baseline/insert/map-size-{}",
            format_with_underscores(*missing_key_count)
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(*missing_key_count as u64));
        bench_insert_missing!(group, map_data.clone(), AhashBenchMap<_, _>, "ahash");
        bench_insert_missing!(group, map_data.clone(), BTreeMapBenchMap<_, _>, "btreemap");
        bench_insert_missing!(group, map_data.clone(), ConcreadBenchMap<_, _>, "concread");
        bench_insert_missing!(group, map_data.clone(), DashMapBenchMap<_, _>, "dashmap");
        bench_insert_missing!(group, map_data.clone(), HashbrownBenchMap<_, _>, "hashbrown");
        bench_insert_missing!(group, map_data.clone(), ImmutableChunkMapBenchMap<_, _>, "immutable-chunkmap");
        bench_insert_missing!(group, map_data.clone(), IndexMapBenchMap<_, _>, "indexmap");
        bench_insert_missing!(group, map_data.clone(), RustCHashBenchMap<_, _>, "rustc-hash");
        bench_insert_missing!(group, map_data.clone(), StarshardBenchMap<_, _>, "starshard");
        bench_insert_missing!(group, map_data.clone(), StdBenchMap<_, _>, "std");
        bench_insert_missing!(group, map_data.clone(), TxMapBenchMap<_, _>, "txmap");
    }
}

criterion_group!(group, baseline_insert);
criterion_main!(group);
