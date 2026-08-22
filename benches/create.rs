// What is the fixed cost of constructing a map? Tests the construction design, eager vs lazy allocation and per-map setup overhead such as sharding or reclamation infrastructure.
use bench_map::{config::*, expand_bench, maps::*};
use criterion::{
    BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main, measurement::WallTime,
};
use std::hint::black_box;

fn bench<Map, K>(name: &str, group: &mut BenchmarkGroup<WallTime>)
where
    Map: BenchMapNew<K, u64>,
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
    let mut group = c.benchmark_group(format!("create"));
    group.warm_up_time(WARM_UP_TIME);
    group.measurement_time(MEASUREMENT_TIME);
    group.throughput(Throughput::Elements(DEFAULT_OP_COUNT as u64));

    expand_bench!(bench, u64, &mut group,
        AhashBenchMap<u64, u64>,
        BTreeMapBenchMap<u64, u64>,
        // ConcreadBenchMap<u64, u64>, // too slow
        ConcurrentMapBenchMap<u64, u64>,
        CrossbeamSkiplistBenchMap<u64, u64>,
        DashMapBenchMap<u64, u64>,
        // FlurryBenchMap<u64, u64>, // too slow (creates a seize::Collector per map)
        HashbrownBenchMap<u64, u64>,
        HashlinkBenchMap<u64, u64>,
        HordeBenchMap<u64, u64>,
        ImmutableChunkMapBenchMap<u64, u64>,
        ImblBenchMap<u64, u64>,
        IndexMapBenchMap<u64, u64>,
        IntMapBenchMap<u64, u64>,
        LeapfrogBenchMap<u64, u64>,
        PapayaBenchMap<u64, u64>,
        RpdsHashTrieMapBenchMap<u64, u64>,
        RustCHashBenchMap<u64, u64>,
        SccBenchMap<u64, u64>,
        StarshardBenchMap<u64, u64>,
        StdBenchMap<u64, u64>,
        TxMapBenchMap<u64, u64>,
    );
}

criterion_group!(group, create);
criterion_main!(group);
