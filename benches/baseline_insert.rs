use bench_map::{
    config::*,
    data::u64_sparse::U64SparseDataGen,
    map_data::MapData,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BenchMapMutInsert, BenchMapNew, ConcreadBenchMap, DashMapBenchMap,
        HashbrownBenchMap, ImmutableChunkMapBenchMap, IndexMapBenchMap, RustCHashBenchMap,
        StarshardBenchMap, StdBenchMap, TxMapBenchMap,
    },
    thousands_format::format_with_underscores,
};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main,
    measurement::WallTime,
};
use std::rc::Rc;

fn bench_insert_missing<Map>(
    group: &mut BenchmarkGroup<WallTime>,
    map_data: Rc<MapData<u64, u64>>,
    name: &str,
) where
    Map: BenchMapNew<u64, u64> + BenchMapMutInsert<u64, u64>,
{
    group.bench_function(name, move |b| {
        let map_data_ref = &map_data;
        b.iter_batched(
            move || {
                let map = map_data_ref.create_map::<Map>();
                let keys = map_data_ref.missing_keys().clone();
                (map, keys)
            },
            |(mut map, mut keys)| {
                for key in keys.drain(..) {
                    let key = std::hint::black_box(key);
                    map.insert(key, 42);
                }
                std::hint::black_box(map)
            },
            BatchSize::PerIteration,
        );
    });
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
        bench_insert_missing::<AhashBenchMap<u64, u64>>(&mut group, map_data.clone(), "ahash");
        // bench_insert_missing::<BTreeMapBenchMap<u64, u64>>(&mut group, map_data.clone(), "btreemap"); // too slow
        bench_insert_missing::<ConcreadBenchMap<u64, u64>>(
            &mut group,
            map_data.clone(),
            "concread",
        );
        bench_insert_missing::<DashMapBenchMap<u64, u64>>(&mut group, map_data.clone(), "dashmap");
        bench_insert_missing::<HashbrownBenchMap<u64, u64>>(
            &mut group,
            map_data.clone(),
            "hashbrown",
        );
        bench_insert_missing::<ImmutableChunkMapBenchMap<u64, u64>>(
            &mut group,
            map_data.clone(),
            "immutable-chunkmap",
        );
        bench_insert_missing::<IndexMapBenchMap<u64, u64>>(
            &mut group,
            map_data.clone(),
            "indexmap",
        );
        bench_insert_missing::<RustCHashBenchMap<u64, u64>>(
            &mut group,
            map_data.clone(),
            "rustc-hash",
        );
        bench_insert_missing::<StarshardBenchMap<u64, u64>>(
            &mut group,
            map_data.clone(),
            "starshard",
        );
        bench_insert_missing::<StdBenchMap<u64, u64>>(&mut group, map_data.clone(), "std");
        bench_insert_missing::<TxMapBenchMap<u64, u64>>(&mut group, map_data.clone(), "txmap");
    }
}

criterion_group!(group, baseline_insert);
criterion_main!(group);
