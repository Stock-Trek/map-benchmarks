use bench_map::{
    config::*,
    data::u64_sparse::U64SparseDataGen,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BTreeMapBenchMap, BenchMapMutRemove, ConcreadBenchMap, DashMapBenchMap,
        HashbrownBenchMap, ImmutableChunkMapBenchMap, IndexMapBenchMap, RustCHashBenchMap,
        StarshardBenchMap, StdBenchMap, TxMapBenchMap,
    },
    thousands_format::format_with_underscores,
};
use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
use std::rc::Rc;

macro_rules! bench_remove {
    ($group:ident, $map_data:expr, $map_type:path, $name:expr) => {
        let map_data = $map_data;
        $group.bench_function($name, move |b| {
            let map_data_ref = &map_data;
            let removal_keys = map_data_ref.existing_keys();
            b.iter_batched(
                move || {
                    let map = map_data_ref.create_map::<$map_type>();
                    let keys_to_remove = removal_keys.clone();
                    (map, keys_to_remove)
                },
                |(mut map, mut keys_to_remove)| {
                    for key in keys_to_remove.drain(..) {
                        let key = std::hint::black_box(key);
                        std::hint::black_box(map.remove(&key));
                    }
                    std::hint::black_box(map)
                },
                BatchSize::PerIteration,
            );
        });
    };
}

fn baseline_remove(c: &mut Criterion) {
    let existing_key_count = 100;
    let missing_key_count = 0;
    let sort_keys = false;
    for entry_count in BASELINE_ENTRY_COUNT {
        let map_data = Rc::new(MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            *entry_count,
            existing_key_count,
            missing_key_count,
            sort_keys,
        ));
        let mut group = c.benchmark_group(format!(
            "baseline/remove/map-size-{}",
            format_with_underscores(*entry_count)
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count as u64));
        bench_remove!(group, map_data.clone(), AhashBenchMap<_, _>, "ahash");
        bench_remove!(group, map_data.clone(), BTreeMapBenchMap<_, _>, "btreemap");
        bench_remove!(group, map_data.clone(), ConcreadBenchMap<_, _>, "concread");
        bench_remove!(group, map_data.clone(), DashMapBenchMap<_, _>, "dashmap");
        bench_remove!(group, map_data.clone(), HashbrownBenchMap<_, _>, "hashbrown");
        bench_remove!(group, map_data.clone(), ImmutableChunkMapBenchMap<_, _>, "immutable-chunkmap");
        bench_remove!(group, map_data.clone(), IndexMapBenchMap<_, _>, "indexmap");
        bench_remove!(group, map_data.clone(), RustCHashBenchMap<_, _>, "rustc-hash");
        bench_remove!(group, map_data.clone(), StarshardBenchMap<_, _>, "starshard");
        bench_remove!(group, map_data.clone(), StdBenchMap<_, _>, "std");
        bench_remove!(group, map_data.clone(), TxMapBenchMap<_, _>, "txmap");
    }
}

criterion_group!(group, baseline_remove);
criterion_main!(group);
