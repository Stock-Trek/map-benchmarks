use bench_map::{
    config::*,
    data::u64_sparse::U64SparseDataGen,
    map_data::MapData,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BenchMapIter, BenchMapMutInsert, BenchMapNewWithHasher, DashMapBenchMap,
        HashbrownBenchMap, HordeBenchMap, IndexMapBenchMap, StarshardBenchMap, StdBenchMap,
        TxMapBenchMap,
    },
    number_formatter::format_n,
};
use criterion::{
    BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main, measurement::WallTime,
};
use std::hint::black_box;

type CommonHasher = ahash::RandomState;

fn bench<Map, H>(
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<u64, u64>,
    name: &str,
    hasher: H,
) where
    Map: BenchMapNewWithHasher<u64, u64, H> + BenchMapMutInsert<u64, u64> + BenchMapIter<u64, u64>,
    H: std::hash::BuildHasher + Clone,
{
    group.bench_function(name, move |b| {
        let map = map_data.create_map_with_hasher::<Map, H>(hasher.clone());
        b.iter(|| {
            let mut sum = 0u64;
            map.for_each(|_key, value| {
                sum = sum.wrapping_add(*value);
            });
            black_box(sum);
        });
    });
}

fn structure_iterate(c: &mut Criterion) {
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
            "same-hasher/iterate/map-size-{}",
            format_n(*entry_count)
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(*entry_count as u64));

        let hasher = CommonHasher::new();

        bench::<AhashBenchMap<u64, u64, CommonHasher>, CommonHasher>(
            &mut group,
            &map_data,
            "ahash",
            hasher.clone(),
        );
        // bench::<BTreeMapBenchMap<u64, u64, CommonHasher>, CommonHasher>(&mut group, &map_data, "btreemap"); // doesn't allow setting hasher
        // bench::<ConcreadBenchMap<u64, u64, CommonHasher>, CommonHasher>(&mut group, &map_data, "concread"); // doesn't allow setting hasher
        bench::<DashMapBenchMap<u64, u64, CommonHasher>, CommonHasher>(
            &mut group,
            &map_data,
            "dashmap",
            hasher.clone(),
        );
        bench::<HashbrownBenchMap<u64, u64, CommonHasher>, CommonHasher>(
            &mut group,
            &map_data,
            "hashbrown",
            hasher.clone(),
        );
        bench::<HordeBenchMap<u64, u64, CommonHasher>, CommonHasher>(
            &mut group,
            &map_data,
            "horde",
            hasher.clone(),
        );
        // bench::<ImmutableChunkMapBenchMap<u64, u64, CommonHasher>, CommonHasher>(&mut group, &map_data, "immutable-chunkmap"); // doesn't allow setting hasher
        bench::<IndexMapBenchMap<u64, u64, CommonHasher>, CommonHasher>(
            &mut group,
            &map_data,
            "indexmap",
            hasher.clone(),
        );
        // bench::<RustCHashBenchMap<u64, u64, CommonHasher>, CommonHasher>(&mut group, &map_data, "rustc-hash"); // doesn't allow setting hasher
        bench::<StarshardBenchMap<u64, u64, CommonHasher>, CommonHasher>(
            &mut group,
            &map_data,
            "starshard",
            hasher.clone(),
        );
        bench::<StdBenchMap<u64, u64, CommonHasher>, CommonHasher>(
            &mut group,
            &map_data,
            "std",
            hasher.clone(),
        );
        bench::<TxMapBenchMap<u64, u64, CommonHasher>, CommonHasher>(
            &mut group,
            &map_data,
            "txmap",
            hasher.clone(),
        );
    }
}

criterion_group!(group, structure_iterate);
criterion_main!(group);
