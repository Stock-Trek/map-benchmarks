use bench_map::{
    config::*,
    maps::{
        AhashBenchMap, BTreeMapBenchMap, BenchMapNew, BenchMapNewWithHasher, ConcreadBenchMap,
        DashMapBenchMap, HashbrownBenchMap, HordeBenchMap, ImmutableChunkMapBenchMap,
        IndexMapBenchMap, RustCHashBenchMap, StarshardBenchMap, StdBenchMap, TxMapBenchMap,
    },
};
use criterion::{
    BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main, measurement::WallTime,
};
use std::hint::black_box;

/// The hasher shared by every map implementation that supports a custom hasher,
/// so map implementations are compared against each other on a level playing
/// field rather than each using its own default hasher.
type CommonHasher = ahash::RandomState;

fn bench<Map, H>(group: &mut BenchmarkGroup<WallTime>, name: &str, hasher: H)
where
    Map: BenchMapNewWithHasher<u64, u64, H>,
    H: std::hash::BuildHasher + Clone,
{
    group.bench_function(name, move |b| {
        b.iter(|| {
            for _ in 0..SAME_HASHER_MAP_COUNT {
                black_box(Map::new_with_hasher(hasher.clone()));
            }
        });
    });
}

fn bench_default<Map>(group: &mut BenchmarkGroup<WallTime>, name: &str)
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
    let mut group = c.benchmark_group("same-hasher/create");
    group.warm_up_time(WARM_UP_TIME);
    group.measurement_time(MEASUREMENT_TIME);
    group.throughput(Throughput::Elements(SAME_HASHER_MAP_COUNT as u64));

    let hasher = CommonHasher::new();

    bench::<AhashBenchMap<u64, u64, CommonHasher>, CommonHasher>(
        &mut group,
        "ahash",
        hasher.clone(),
    );
    bench_default::<BTreeMapBenchMap<u64, u64>>(&mut group, "btreemap"); // no custom hasher
    bench_default::<ConcreadBenchMap<u64, u64>>(&mut group, "concread"); // no custom hasher
    bench::<DashMapBenchMap<u64, u64, CommonHasher>, CommonHasher>(
        &mut group,
        "dashmap",
        hasher.clone(),
    );
    bench::<HashbrownBenchMap<u64, u64, CommonHasher>, CommonHasher>(
        &mut group,
        "hashbrown",
        hasher.clone(),
    );
    bench::<HordeBenchMap<u64, u64, CommonHasher>, CommonHasher>(
        &mut group,
        "horde",
        hasher.clone(),
    );
    bench_default::<ImmutableChunkMapBenchMap<u64, u64>>(&mut group, "immutable-chunkmap"); // no custom hasher
    bench::<IndexMapBenchMap<u64, u64, CommonHasher>, CommonHasher>(
        &mut group,
        "indexmap",
        hasher.clone(),
    );
    bench_default::<RustCHashBenchMap<u64, u64>>(&mut group, "rustc-hash"); // no custom hasher
    bench::<StarshardBenchMap<u64, u64, CommonHasher>, CommonHasher>(
        &mut group,
        "starshard",
        hasher.clone(),
    );
    bench::<StdBenchMap<u64, u64, CommonHasher>, CommonHasher>(&mut group, "std", hasher.clone());
    bench::<TxMapBenchMap<u64, u64, CommonHasher>, CommonHasher>(
        &mut group,
        "txmap",
        hasher.clone(),
    );
}

criterion_group!(group, structure_new);
criterion_main!(group);
