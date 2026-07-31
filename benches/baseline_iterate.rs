use bench_map::{
    config::*,
    data::u64_sparse::U64SparseDataGen,
    map_data::MapData,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BenchMapIter, BenchMapMutInsert, BenchMapNew, DashMapBenchMap,
        HashbrownBenchMap, IndexMapBenchMap, RustCHashBenchMap, StarshardBenchMap, StdBenchMap,
        TxMapBenchMap,
    },
    thousands_format::format_with_underscores,
};
use criterion::{
    BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main, measurement::WallTime,
};
use std::rc::Rc;

fn bench_iterate<Map>(
    group: &mut BenchmarkGroup<WallTime>,
    map_data: Rc<MapData<u64, u64>>,
    name: &str,
) where
    Map: BenchMapNew<u64, u64> + BenchMapMutInsert<u64, u64> + BenchMapIter<u64, u64>,
{
    group.bench_function(name, move |b| {
        let map = map_data.create_map::<Map>();
        b.iter(|| {
            let mut sum = 0u64;
            for entry in map.iter() {
                let value_ref = map.item_value_ref(&entry);
                sum = sum.wrapping_add(*value_ref);
            }
            std::hint::black_box(sum);
        });
    });
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
        bench_iterate::<AhashBenchMap<u64, u64>>(&mut group, map_data.clone(), "ahash");
        // bench_iterate::<BTreeMapBenchMap<u64, u64>>(&mut group, map_data.clone(), "btreemap"); // too slow
        // bench_iterate::<ConcreadBenchMap<u64, u64>>(&mut group, map_data.clone(), "concread"); // read guard prevents storing iterator
        bench_iterate::<DashMapBenchMap<u64, u64>>(&mut group, map_data.clone(), "dashmap");
        bench_iterate::<HashbrownBenchMap<u64, u64>>(&mut group, map_data.clone(), "hashbrown");
        // bench_iterate::<ImmutableChunkMapBenchMap<u64, u64>>(&mut group, map_data.clone(), "immutable-chunkmap"); // no immutable iter
        bench_iterate::<IndexMapBenchMap<u64, u64>>(&mut group, map_data.clone(), "indexmap");
        bench_iterate::<RustCHashBenchMap<u64, u64>>(&mut group, map_data.clone(), "rustc-hash");
        bench_iterate::<StarshardBenchMap<u64, u64>>(&mut group, map_data.clone(), "starshard");
        bench_iterate::<StdBenchMap<u64, u64>>(&mut group, map_data.clone(), "std");
        bench_iterate::<TxMapBenchMap<u64, u64>>(&mut group, map_data.clone(), "txmap");
    }
}

criterion_group!(group, baseline_iterate);
criterion_main!(group);
