use bench_map::{
    config::*,
    data::u64_sparse::U64SparseDataGen,
    map_data::MapData,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BenchMapMutInsert, BenchMapMutRemove, BenchMapNew, DashMapBenchMap,
        HashbrownBenchMap, ImmutableChunkMapBenchMap, IndexMapBenchMap, RustCHashBenchMap,
        StarshardBenchMap, StdBenchMap, TxMapBenchMap,
    },
    number_formatter::format_n,
};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main,
    measurement::WallTime,
};
use std::hint::black_box;

fn bench<Map>(group: &mut BenchmarkGroup<WallTime>, map_data: &MapData<u64, u64>, name: &str)
where
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
                    let key = black_box(key);
                    black_box(map.remove(&key));
                }
                black_box(map)
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
        let map_data = MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            *entry_count,
            existing_key_count,
            missing_key_count,
            sort_keys,
        );
        let mut group = c.benchmark_group(format!(
            "baseline/remove/map-size-{}",
            format_n(*entry_count)
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count as u64));

        bench::<AhashBenchMap<u64, u64>>(&mut group, &map_data, "ahash");
        // bench::<BTreeMapBenchMap<u64, u64>>(&mut group, &map_data, "btreemap"); // too slow
        // bench::<ConcreadBenchMap<u64, u64>>(&mut group, &map_data, "concread"); // too slow
        bench::<DashMapBenchMap<u64, u64>>(&mut group, &map_data, "dashmap");
        bench::<HashbrownBenchMap<u64, u64>>(&mut group, &map_data, "hashbrown");
        bench::<ImmutableChunkMapBenchMap<u64, u64>>(&mut group, &map_data, "immutable-chunkmap");
        bench::<IndexMapBenchMap<u64, u64>>(&mut group, &map_data, "indexmap");
        bench::<RustCHashBenchMap<u64, u64>>(&mut group, &map_data, "rustc-hash");
        bench::<StarshardBenchMap<u64, u64>>(&mut group, &map_data, "starshard");
        bench::<StdBenchMap<u64, u64>>(&mut group, &map_data, "std");
        bench::<TxMapBenchMap<u64, u64>>(&mut group, &map_data, "txmap");
    }
}

criterion_group!(group, baseline_remove);
criterion_main!(group);
