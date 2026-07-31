use bench_map::{
    config::*,
    data::u64_sparse::U64SparseDataGen,
    map_data::MapData,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BenchMapGetCloned, BenchMapMutInsert, BenchMapNew, ConcreadBenchMap,
        DashMapBenchMap, HashbrownBenchMap, ImmutableChunkMapBenchMap, IndexMapBenchMap,
        RustCHashBenchMap, StarshardBenchMap, StdBenchMap, TxMapBenchMap,
    },
    thousands_format::format_with_underscores,
};
use criterion::{
    BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main, measurement::WallTime,
};
use std::rc::Rc;

fn bench_lookup_hit<Map>(
    group: &mut BenchmarkGroup<WallTime>,
    map_data: Rc<MapData<u64, u64>>,
    name: &str,
) where
    Map: BenchMapNew<u64, u64> + BenchMapMutInsert<u64, u64> + BenchMapGetCloned<u64, u64>,
{
    group.bench_function(name, move |b| {
        let map = map_data.create_map::<Map>();
        let keys = map_data.existing_keys();
        b.iter(|| {
            for key in keys {
                let key = std::hint::black_box(key);
                std::hint::black_box(map.get_cloned(key));
            }
        });
    });
}

fn baseline_lookup_hit(c: &mut Criterion) {
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
            "baseline/lookup-hit/map-size-{}",
            format_with_underscores(*entry_count)
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count as u64));
        bench_lookup_hit::<AhashBenchMap<u64, u64>>(&mut group, map_data.clone(), "ahash");
        // bench_lookup_hit::<BTreeMapBenchMap<u64, u64>>(&mut group, map_data.clone(), "btreemap"); // too slow
        bench_lookup_hit::<ConcreadBenchMap<u64, u64>>(&mut group, map_data.clone(), "concread");
        bench_lookup_hit::<DashMapBenchMap<u64, u64>>(&mut group, map_data.clone(), "dashmap");
        bench_lookup_hit::<HashbrownBenchMap<u64, u64>>(&mut group, map_data.clone(), "hashbrown");
        bench_lookup_hit::<ImmutableChunkMapBenchMap<u64, u64>>(
            &mut group,
            map_data.clone(),
            "immutable-chunkmap",
        );
        bench_lookup_hit::<IndexMapBenchMap<u64, u64>>(&mut group, map_data.clone(), "indexmap");
        bench_lookup_hit::<RustCHashBenchMap<u64, u64>>(&mut group, map_data.clone(), "rustc-hash");
        bench_lookup_hit::<StarshardBenchMap<u64, u64>>(&mut group, map_data.clone(), "starshard");
        bench_lookup_hit::<StdBenchMap<u64, u64>>(&mut group, map_data.clone(), "std");
        bench_lookup_hit::<TxMapBenchMap<u64, u64>>(&mut group, map_data.clone(), "txmap");
    }
}

criterion_group!(group, baseline_lookup_hit);
criterion_main!(group);
