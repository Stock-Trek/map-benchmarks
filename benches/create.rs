use bench_map::{
    config::*,
    constants::*,
    maps::{
        AhashBenchMap, BTreeMapBenchMap, BenchMapNew, DashMapBenchMap, HashbrownBenchMap,
        HordeBenchMap, ImblBenchMap, ImmutableChunkMapBenchMap, IndexMapBenchMap, LeapfrogBenchMap,
        PapayaBenchMap, RustCHashBenchMap, SccBenchMap, StarshardBenchMap, StdBenchMap,
        TxMapBenchMap,
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

fn create(c: &mut Criterion) {
    let mut group = c.benchmark_group(format!("create/{OUT_OF_THE_BOX_GROUP_NAME}"));
    group.warm_up_time(WARM_UP_TIME);
    group.measurement_time(MEASUREMENT_TIME);
    group.throughput(Throughput::Elements(SAME_HASHER_MAP_COUNT as u64));

    bench::<AhashBenchMap<u64, u64>>(&mut group, "ahash");
    bench::<BTreeMapBenchMap<u64, u64>>(&mut group, "btreemap");
    // bench::<ConcreadBenchMap<u64, u64>>(&mut group, "concread"); // too slow
    bench::<DashMapBenchMap<u64, u64>>(&mut group, "dashmap");
    // bench::<FlurryBenchMap<u64, u64>>(&mut group, "flurry"); // too slow (creates a seize::Collector per map)
    bench::<HashbrownBenchMap<u64, u64>>(&mut group, "hashbrown");
    bench::<HordeBenchMap<u64, u64>>(&mut group, "horde");
    bench::<ImmutableChunkMapBenchMap<u64, u64>>(&mut group, "immutable-chunkmap");
    bench::<ImblBenchMap<u64, u64>>(&mut group, "imbl");
    bench::<IndexMapBenchMap<u64, u64>>(&mut group, "indexmap");
    bench::<LeapfrogBenchMap<u64, u64>>(&mut group, "leapfrog");
    bench::<PapayaBenchMap<u64, u64>>(&mut group, "papaya");
    bench::<RustCHashBenchMap<u64, u64>>(&mut group, "rustc-hash");
    bench::<SccBenchMap<u64, u64>>(&mut group, "scc");
    bench::<StarshardBenchMap<u64, u64>>(&mut group, "starshard");
    bench::<StdBenchMap<u64, u64>>(&mut group, "std");
    bench::<TxMapBenchMap<u64, u64>>(&mut group, "txmap");
}

criterion_group!(group, create);
criterion_main!(group);
