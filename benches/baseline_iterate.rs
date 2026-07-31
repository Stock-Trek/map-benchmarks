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
    number_formatter::format_n,
};
use criterion::{
    BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main, measurement::WallTime,
};
use std::hint::black_box;

fn bench<Map>(group: &mut BenchmarkGroup<WallTime>, map_data: &MapData<u64, u64>, name: &str)
where
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
            black_box(sum);
        });
    });
}

fn baseline_iterate(c: &mut Criterion) {
    let existing_key_count = 0;
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
            "baseline/iterate/map-size-{}",
            format_n(*entry_count)
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(*entry_count as u64));

        bench::<AhashBenchMap<u64, u64>>(&mut group, &map_data, "ahash");
        // bench::<BTreeMapBenchMap<u64, u64>>(&mut group, &map_data, "btreemap"); // too slow
        // bench::<ConcreadBenchMap<u64, u64>>(&mut group, &map_data, "concread"); // read guard prevents storing iterator
        bench::<DashMapBenchMap<u64, u64>>(&mut group, &map_data, "dashmap");
        bench::<HashbrownBenchMap<u64, u64>>(&mut group, &map_data, "hashbrown");
        // bench::<ImmutableChunkMapBenchMap<u64, u64>>(&mut group, &map_data, "immutable-chunkmap"); // no immutable iter
        bench::<IndexMapBenchMap<u64, u64>>(&mut group, &map_data, "indexmap");
        bench::<RustCHashBenchMap<u64, u64>>(&mut group, &map_data, "rustc-hash");
        bench::<StarshardBenchMap<u64, u64>>(&mut group, &map_data, "starshard");
        bench::<StdBenchMap<u64, u64>>(&mut group, &map_data, "std");
        bench::<TxMapBenchMap<u64, u64>>(&mut group, &map_data, "txmap");
    }
}

criterion_group!(group, baseline_iterate);
criterion_main!(group);
