use bench_map::{
    config::*,
    data::u64_sparse::U64SparseDataGen,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BTreeMapBenchMap, BenchMapGetCloned, ConcreadBenchMap, DashMapBenchMap,
        HashbrownBenchMap, ImmutableChunkMapBenchMap, IndexMapBenchMap, RustCHashBenchMap,
        StarshardBenchMap, StdBenchMap, TxMapBenchMap,
    },
    thousands_format::format_with_underscores,
};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::rc::Rc;

macro_rules! bench_lookup_miss {
    ($group:ident, $map_data:expr, $map_type:path, $name:expr) => {
        let map_data = $map_data;
        $group.bench_function($name, move |b| {
            let map = map_data.create_map::<$map_type>();
            let keys = map_data.missing_keys();
            b.iter(|| {
                for key in keys {
                    let key = std::hint::black_box(key);
                    std::hint::black_box(map.get_cloned(key));
                }
            });
        });
    };
}

fn baseline_lookup_miss(c: &mut Criterion) {
    let existing_key_count = 0;
    let missing_key_count = 100;
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
            "baseline/lookup-miss/map-size-{}",
            format_with_underscores(*entry_count)
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(missing_key_count as u64));
        bench_lookup_miss!(group, map_data.clone(), AhashBenchMap<_, _>, "ahash");
        bench_lookup_miss!(group, map_data.clone(), BTreeMapBenchMap<_, _>, "btreemap");
        bench_lookup_miss!(group, map_data.clone(), ConcreadBenchMap<_, _>, "concread");
        bench_lookup_miss!(group, map_data.clone(), DashMapBenchMap<_, _>, "dashmap");
        bench_lookup_miss!(group, map_data.clone(), HashbrownBenchMap<_, _>, "hashbrown");
        bench_lookup_miss!(group, map_data.clone(), ImmutableChunkMapBenchMap<_, _>, "immutable-chunkmap");
        bench_lookup_miss!(group, map_data.clone(), IndexMapBenchMap<_, _>, "indexmap");
        bench_lookup_miss!(group, map_data.clone(), RustCHashBenchMap<_, _>, "rustc-hash");
        bench_lookup_miss!(group, map_data.clone(), StarshardBenchMap<_, _>, "starshard");
        bench_lookup_miss!(group, map_data.clone(), StdBenchMap<_, _>, "std");
        bench_lookup_miss!(group, map_data.clone(), TxMapBenchMap<_, _>, "txmap");
    }
}

criterion_group!(group, baseline_lookup_miss);
criterion_main!(group);
