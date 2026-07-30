use bench_map::{
    config::*,
    data::u64_sparse::U64SparseDataGen,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BenchMapIter, DashMapBenchMap, HashbrownBenchMap, IndexMapBenchMap,
        RustCHashBenchMap, StarshardBenchMap, StdBenchMap, TxMapBenchMap,
    },
    thousands_format::format_with_underscores,
};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::rc::Rc;

macro_rules! bench_iterate {
    ($group:ident, $map_data:expr, $map_type:path, $name:expr) => {
        let map_data = $map_data;
        $group.bench_function($name, move |b| {
            let map = map_data.create_map::<$map_type>();
            b.iter(|| {
                let mut sum = 0u64;
                for entry in map.iter() {
                    let value_ref = map.item_value_ref(&entry);
                    sum = sum.wrapping_add(*value_ref);
                }
                std::hint::black_box(sum);
            });
        });
    };
}

fn baseline_iterate(c: &mut Criterion) {
    let existing_key_count = 0;
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
            "baseline/iterate/map-size-{}",
            format_with_underscores(*entry_count)
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(*entry_count as u64));
        bench_iterate!(group, map_data.clone(), AhashBenchMap<u64, u64>, "ahash");
        // bench_iterate!(group, map_data.clone(), BTreeMapBenchMap<u64, u64>, "btreemap"); // too slow
        // bench_iterate!(group, map_data.clone(), ConcreadBenchMap<u64, u64>, "concread"); // read guard prevents storing iterator
        bench_iterate!(group, map_data.clone(), DashMapBenchMap<u64, u64>, "dashmap");
        bench_iterate!(group, map_data.clone(), HashbrownBenchMap<u64, u64>, "hashbrown");
        // bench_iterate!(group, map_data.clone(), ImmutableChunkMapBenchMap<u64, u64>, "immutable-chunkmap"); // no immutable iter
        bench_iterate!(group, map_data.clone(), IndexMapBenchMap<u64, u64>, "indexmap");
        bench_iterate!(group, map_data.clone(), RustCHashBenchMap<u64, u64>, "rustc-hash");
        bench_iterate!(group, map_data.clone(), StarshardBenchMap<u64, u64>, "starshard");
        bench_iterate!(group, map_data.clone(), StdBenchMap<u64, u64>, "std");
        bench_iterate!(group, map_data.clone(), TxMapBenchMap<u64, u64>, "txmap");
    }
}

criterion_group!(group, baseline_iterate);
criterion_main!(group);
