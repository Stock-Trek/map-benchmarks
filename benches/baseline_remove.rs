use bench_map::{
    config::*,
    data::u64_sparse::U64SparseDataGen,
    map_data::MapData,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BenchMapMutInsert, BenchMapMutRemove, BenchMapNew, ConcreadBenchMap,
        DashMapBenchMap, HashbrownBenchMap, ImmutableChunkMapBenchMap, IndexMapBenchMap,
        RustCHashBenchMap, StarshardBenchMap, StdBenchMap, TxMapBenchMap,
    },
    thousands_format::format_with_underscores,
};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main,
    measurement::WallTime,
};
use std::rc::Rc;

fn bench_remove<Map>(
    group: &mut BenchmarkGroup<WallTime>,
    map_data: Rc<MapData<u64, u64>>,
    name: &str,
) where
    Map: BenchMapNew<u64, u64> + BenchMapMutInsert<u64, u64> + BenchMapMutRemove<u64, u64>,
{
    group.bench_function(name, move |b| {
        let map_data_ref = &map_data;
        let removal_keys = map_data_ref.existing_keys();
        b.iter_batched(
            move || {
                let map = map_data_ref.create_map::<Map>();
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
        bench_remove::<AhashBenchMap<u64, u64>>(&mut group, map_data.clone(), "ahash");
        // bench_remove::<BTreeMapBenchMap<u64, u64>>(&mut group, map_data.clone(), "btreemap"); // too slow
        bench_remove::<ConcreadBenchMap<u64, u64>>(&mut group, map_data.clone(), "concread");
        bench_remove::<DashMapBenchMap<u64, u64>>(&mut group, map_data.clone(), "dashmap");
        bench_remove::<HashbrownBenchMap<u64, u64>>(&mut group, map_data.clone(), "hashbrown");
        bench_remove::<ImmutableChunkMapBenchMap<u64, u64>>(
            &mut group,
            map_data.clone(),
            "immutable-chunkmap",
        );
        bench_remove::<IndexMapBenchMap<u64, u64>>(&mut group, map_data.clone(), "indexmap");
        bench_remove::<RustCHashBenchMap<u64, u64>>(&mut group, map_data.clone(), "rustc-hash");
        bench_remove::<StarshardBenchMap<u64, u64>>(&mut group, map_data.clone(), "starshard");
        bench_remove::<StdBenchMap<u64, u64>>(&mut group, map_data.clone(), "std");
        bench_remove::<TxMapBenchMap<u64, u64>>(&mut group, map_data.clone(), "txmap");
    }
}

criterion_group!(group, baseline_remove);
criterion_main!(group);
