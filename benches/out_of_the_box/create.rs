use bench_map::{
    config::*,
    constants::OUT_OF_THE_BOX_GROUP_NAME,
    maps::{
        AhashBenchMap, BTreeMapBenchMap, BenchMapNew, ConcreadBenchMap, DashMapBenchMap,
        HashbrownBenchMap, HordeBenchMap, ImmutableChunkMapBenchMap, IndexMapBenchMap,
        RustCHashBenchMap, StarshardBenchMap, StdBenchMap, TxMapBenchMap,
    },
};
use criterion::{
    BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main, measurement::WallTime,
};
use std::hint::black_box;

fn bench<Map>(group: &mut BenchmarkGroup<WallTime>, name: &str)
where
    Map: BenchMapNew<u64, u64>,
{
    group.bench_function(name, move |b| {
        b.iter(|| {
            for _ in 0..SAME_HASHER_MAP_COUNT {
                black_box(Map::new());
            }
        });
    });
}

fn structure_new(c: &mut Criterion) {
    let mut group = c.benchmark_group(format!("{OUT_OF_THE_BOX_GROUP_NAME}/create"));
    group.warm_up_time(WARM_UP_TIME);
    group.measurement_time(MEASUREMENT_TIME);
    group.throughput(Throughput::Elements(SAME_HASHER_MAP_COUNT as u64));

    bench::<AhashBenchMap<u64, u64>>(&mut group, "ahash");
    bench::<BTreeMapBenchMap<u64, u64>>(&mut group, "btreemap");
    bench::<ConcreadBenchMap<u64, u64>>(&mut group, "concread");
    bench::<DashMapBenchMap<u64, u64>>(&mut group, "dashmap");
    bench::<HashbrownBenchMap<u64, u64>>(&mut group, "hashbrown");
    bench::<HordeBenchMap<u64, u64>>(&mut group, "horde");
    bench::<ImmutableChunkMapBenchMap<u64, u64>>(&mut group, "immutable-chunkmap");
    bench::<IndexMapBenchMap<u64, u64>>(&mut group, "indexmap");
    bench::<RustCHashBenchMap<u64, u64>>(&mut group, "rustc-hash");
    bench::<StarshardBenchMap<u64, u64>>(&mut group, "starshard");
    bench::<StdBenchMap<u64, u64>>(&mut group, "std");
    bench::<TxMapBenchMap<u64, u64>>(&mut group, "txmap");
}

criterion_group!(group, structure_new);
criterion_main!(group);
