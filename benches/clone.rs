use bench_map::{
    config::*,
    constants::*,
    data::u64_sparse::U64SparseDataGen,
    map_data::MapData,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BTreeMapBenchMap, BenchMapClone, BenchMapMutInsert, BenchMapNew,
        DashMapBenchMap, HashbrownBenchMap, HordeBenchMap, ImblBenchMap, ImmutableChunkMapBenchMap,
        IndexMapBenchMap, PapayaBenchMap, RustCHashBenchMap, SccBenchMap, StarshardBenchMap,
        StdBenchMap, TxMapBenchMap,
    },
    number_formatter::format_n,
};
use criterion::{
    BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main, measurement::WallTime,
};
use std::hint::black_box;

fn bench<Map>(group: &mut BenchmarkGroup<WallTime>, map_data: &MapData<u64, u64>, name: &str)
where
    Map: BenchMapNew<u64, u64> + BenchMapMutInsert<u64, u64> + BenchMapClone<u64, u64>,
{
    group.bench_function(name, move |b| {
        let map = map_data.create_map::<Map>();
        b.iter(|| {
            black_box(map.clone_map());
        });
    });
}

fn clone(c: &mut Criterion) {
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
            "clone/{OUT_OF_THE_BOX_GROUP_NAME}/map-size-{}",
            format_n(*entry_count)
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(*entry_count as u64));

        bench::<AhashBenchMap<u64, u64>>(&mut group, &map_data, "ahash");
        bench::<BTreeMapBenchMap<u64, u64>>(&mut group, &map_data, "btreemap");
        // bench::<ConcreadBenchMap<u64, u64>>(&mut group, &map_data, "concread"); // doesn't implement Clone
        bench::<DashMapBenchMap<u64, u64>>(&mut group, &map_data, "dashmap");
        // bench::<FlurryBenchMap<u64, u64>>(&mut group, &map_data, "flurry"); // too slow
        bench::<HashbrownBenchMap<u64, u64>>(&mut group, &map_data, "hashbrown");
        bench::<HordeBenchMap<u64, u64>>(&mut group, &map_data, "horde");
        bench::<ImmutableChunkMapBenchMap<u64, u64>>(&mut group, &map_data, "immutable-chunkmap");
        bench::<ImblBenchMap<u64, u64>>(&mut group, &map_data, "imbl");
        bench::<IndexMapBenchMap<u64, u64>>(&mut group, &map_data, "indexmap");
        // bench::<LeapfrogBenchMap<u64, u64>>(&mut group, &map_data, "leapfrog"); // doesn't implement Clone
        bench::<PapayaBenchMap<u64, u64>>(&mut group, &map_data, "papaya");
        bench::<RustCHashBenchMap<u64, u64>>(&mut group, &map_data, "rustc-hash");
        bench::<SccBenchMap<u64, u64>>(&mut group, &map_data, "scc");
        bench::<StarshardBenchMap<u64, u64>>(&mut group, &map_data, "starshard");
        bench::<StdBenchMap<u64, u64>>(&mut group, &map_data, "std");
        bench::<TxMapBenchMap<u64, u64>>(&mut group, &map_data, "txmap");
    }
}

criterion_group!(group, clone);
criterion_main!(group);
