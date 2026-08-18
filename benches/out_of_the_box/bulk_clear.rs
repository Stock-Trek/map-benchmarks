use bench_map::{
    config::*,
    constants::*,
    data::u64_sparse::U64SparseDataGen,
    map_data::MapData,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BTreeMapBenchMap, BenchMapMutClear, BenchMapMutInsert, BenchMapNew,
        ConcreadBenchMap, DashMapBenchMap, HashbrownBenchMap, HordeBenchMap, IndexMapBenchMap,
        RustCHashBenchMap, StarshardBenchMap, StdBenchMap, TxMapBenchMap,
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
    Map: BenchMapNew<u64, u64> + BenchMapMutInsert<u64, u64> + BenchMapMutClear<u64, u64>,
{
    group.bench_function(name, move |b| {
        b.iter_batched(
            || map_data.create_map::<Map>(),
            |mut map| {
                map.clear();
                black_box(map)
            },
            BatchSize::PerIteration,
        );
    });
}

fn data_bulk_clear(c: &mut Criterion) {
    let existing_key_count = 0;
    let missing_key_count = 0;
    let sort_keys = false;
    for entry_count in OUT_OF_THE_BOX_ENTRY_COUNT {
        let map_data = MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            *entry_count,
            existing_key_count,
            missing_key_count,
            sort_keys,
        );
        let mut group = c.benchmark_group(format!(
            "{OUT_OF_THE_BOX_GROUP_NAME}/bulk-clear/map-size-{}",
            format_n(*entry_count)
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(*entry_count as u64));

        bench::<AhashBenchMap<u64, u64>>(&mut group, &map_data, "ahash");
        bench::<BTreeMapBenchMap<u64, u64>>(&mut group, &map_data, "btreemap");
        bench::<ConcreadBenchMap<u64, u64>>(&mut group, &map_data, "concread");
        bench::<DashMapBenchMap<u64, u64>>(&mut group, &map_data, "dashmap");
        bench::<HashbrownBenchMap<u64, u64>>(&mut group, &map_data, "hashbrown");
        bench::<HordeBenchMap<u64, u64>>(&mut group, &map_data, "horde");
        bench::<IndexMapBenchMap<u64, u64>>(&mut group, &map_data, "indexmap");
        bench::<RustCHashBenchMap<u64, u64>>(&mut group, &map_data, "rustc-hash");
        bench::<StarshardBenchMap<u64, u64>>(&mut group, &map_data, "starshard");
        bench::<StdBenchMap<u64, u64>>(&mut group, &map_data, "std");
        bench::<TxMapBenchMap<u64, u64>>(&mut group, &map_data, "txmap");
    }
}

criterion_group!(group, data_bulk_clear);
criterion_main!(group);
