use bench_map::{
    config::*,
    contents::SAME_HASHER_GROUP_NAME,
    data::u64_sparse::U64SparseDataGen,
    map_data::MapData,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BenchMapMutInsert, BenchMapMutRemove, BenchMapNewWithHasher,
        DashMapBenchMap, HashbrownBenchMap, IndexMapBenchMap, StarshardBenchMap, StdBenchMap,
        TxMapBenchMap, horde_benchmap::HordeBenchMap,
    },
    number_formatter::format_n,
};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main,
    measurement::WallTime,
};
use std::hint::black_box;

type CommonHasher = ahash::RandomState;

fn bench<Map>(
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<u64, u64>,
    name: &str,
    hasher: CommonHasher,
) where
    Map: BenchMapNewWithHasher<u64, u64, CommonHasher>
        + BenchMapMutInsert<u64, u64>
        + BenchMapMutRemove<u64, u64>,
{
    group.bench_function(name, move |b| {
        let map_data_ref = &map_data;
        let removal_keys = map_data_ref.existing_keys();
        b.iter_batched(
            || {
                let map = map_data_ref.create_map_with_hasher::<Map, CommonHasher>(hasher.clone());
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

fn data_remove(c: &mut Criterion) {
    let existing_key_count = 100;
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
            "{SAME_HASHER_GROUP_NAME}/remove/map-size-{}",
            format_n(*entry_count)
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count as u64));

        let hasher = CommonHasher::new();

        bench::<AhashBenchMap<u64, u64, CommonHasher>>(
            &mut group,
            &map_data,
            "ahash",
            hasher.clone(),
        );
        // bench::<BTreeMapBenchMap<u64, u64, CommonHasher>>(&mut group, &map_data, "btreemap"); // doesn't allow setting hasher
        // bench::<ConcreadBenchMap<u64, u64, CommonHasher>>(&mut group, &map_data, "concread"); // doesn't allow setting hasher
        bench::<DashMapBenchMap<u64, u64, CommonHasher>>(
            &mut group,
            &map_data,
            "dashmap",
            hasher.clone(),
        );
        bench::<HashbrownBenchMap<u64, u64, CommonHasher>>(
            &mut group,
            &map_data,
            "hashbrown",
            hasher.clone(),
        );
        bench::<HordeBenchMap<u64, u64, CommonHasher>>(
            &mut group,
            &map_data,
            "horde",
            hasher.clone(),
        );
        // bench::<ImmutableChunkMapBenchMap<u64, u64, CommonHasher>>(&mut group, &map_data, "immutable-chunkmap"); // doesn't allow setting hasher
        bench::<IndexMapBenchMap<u64, u64, CommonHasher>>(
            &mut group,
            &map_data,
            "indexmap",
            hasher.clone(),
        );
        // bench::<RustCHashBenchMap<u64, u64, CommonHasher>>(&mut group, &map_data, "rustc-hash"); // doesn't allow setting hasher
        bench::<StarshardBenchMap<u64, u64, CommonHasher>>(
            &mut group,
            &map_data,
            "starshard",
            hasher.clone(),
        );
        bench::<StdBenchMap<u64, u64, CommonHasher>>(&mut group, &map_data, "std", hasher.clone());
        bench::<TxMapBenchMap<u64, u64, CommonHasher>>(
            &mut group,
            &map_data,
            "txmap",
            hasher.clone(),
        );
    }
}

criterion_group!(group, data_remove);
criterion_main!(group);
