use bench_map::{config::*, constants::*, maps::*};
use criterion::{
    BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main, measurement::WallTime,
};
use std::hint::black_box;

fn bench<Map>(name: &str, group: &mut BenchmarkGroup<WallTime>)
where
    Map: BenchMapNew<u64, u64>,
{
    group.bench_function(name, move |b| {
        b.iter(|| {
            for _ in 0..DEFAULT_OP_COUNT {
                black_box(Map::new());
            }
        });
    });
}

fn create(c: &mut Criterion) {
    let mut group = c.benchmark_group(format!("create/{OUT_OF_THE_BOX_GROUP_NAME}"));
    group.warm_up_time(WARM_UP_TIME);
    group.measurement_time(MEASUREMENT_TIME);
    group.throughput(Throughput::Elements(DEFAULT_OP_COUNT as u64));

    bench::<AhashBenchMap<u64, u64>>("ahash", &mut group);
    bench::<BTreeMapBenchMap<u64, u64>>("btreemap", &mut group);
    // bench::<ConcreadBenchMap<u64, u64>>("concread", &mut group); // too slow
    bench::<ConcurrentMapBenchMap<u64, u64>>("concurrent-map", &mut group);
    bench::<CrossbeamSkiplistBenchMap<u64, u64>>("crossbeam-skiplist", &mut group);
    bench::<DashMapBenchMap<u64, u64>>("dashmap", &mut group);
    // bench::<FlurryBenchMap<u64, u64>>("flurry", &mut group); // too slow (creates a seize::Collector per map)
    bench::<HashbrownBenchMap<u64, u64>>("hashbrown", &mut group);
    bench::<HashlinkBenchMap<u64, u64>>("hashlink", &mut group);
    bench::<HordeBenchMap<u64, u64>>("horde", &mut group);
    bench::<ImmutableChunkMapBenchMap<u64, u64>>("immutable-chunkmap", &mut group);
    bench::<ImblBenchMap<u64, u64>>("imbl", &mut group);
    bench::<IndexMapBenchMap<u64, u64>>("indexmap", &mut group);
    bench::<LeapfrogBenchMap<u64, u64>>("leapfrog", &mut group);
    bench::<PapayaBenchMap<u64, u64>>("papaya", &mut group);
    bench::<RpdsHashTrieMapBenchMap<u64, u64>>("rpds-hash-trie-map", &mut group);
    bench::<RustCHashBenchMap<u64, u64>>("rustc-hash", &mut group);
    bench::<SccBenchMap<u64, u64>>("scc", &mut group);
    bench::<StarshardBenchMap<u64, u64>>("starshard", &mut group);
    bench::<StdBenchMap<u64, u64>>("std", &mut group);
    bench::<TxMapBenchMap<u64, u64>>("txmap", &mut group);
}

criterion_group!(group, create);
criterion_main!(group);
