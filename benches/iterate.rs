use bench_map::{
    config::*,
    constants::*,
    data::u64_sparse::U64SparseDataGen,
    map_data::MapData,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BTreeMapBenchMap, BenchMapIter, BenchMapMutInsert, BenchMapNew,
        BenchMapNewWithHasher, ConcreadBenchMap, DashMapBenchMap, HashbrownBenchMap,
        ImmutableChunkMapBenchMap, IndexMapBenchMap, RustCHashBenchMap, StarshardBenchMap,
        StdBenchMap, TxMapBenchMap, horde_benchmap::HordeBenchMap,
    },
    number_formatter::format_n,
};
use criterion::{
    BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main, measurement::WallTime,
};
use std::hint::black_box;

type CommonHasher = ahash::RandomState;

fn bench_out_of_the_box<Map>(
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<u64, u64>,
    name: &str,
) where
    Map: BenchMapNew<u64, u64> + BenchMapMutInsert<u64, u64> + BenchMapIter<u64, u64>,
{
    group.bench_function(name, move |b| {
        let map = map_data.create_map::<Map>();
        b.iter(|| {
            let mut sum = 0u64;
            map.for_each(|_key, value| {
                sum = sum.wrapping_add(*value);
            });
            black_box(sum);
        });
    });
}

fn bench_same_hasher<Map>(
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<u64, u64>,
    name: &str,
    hasher: CommonHasher,
) where
    Map: BenchMapNewWithHasher<u64, u64, CommonHasher>
        + BenchMapMutInsert<u64, u64>
        + BenchMapIter<u64, u64>,
{
    group.bench_function(name, move |b| {
        let map = map_data.create_map_with_hasher::<Map, CommonHasher>(hasher.clone());
        b.iter(|| {
            let mut sum = 0u64;
            map.for_each(|_key, value| {
                sum = sum.wrapping_add(*value);
            });
            black_box(sum);
        });
    });
}

fn iterate(c: &mut Criterion) {
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

        // Each map uses its default hasher
        {
            let mut group = c.benchmark_group(format!(
                "iterate/{OUT_OF_THE_BOX_GROUP_NAME}/map-size-{}",
                format_n(*entry_count)
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(*entry_count as u64));

            bench_out_of_the_box::<AhashBenchMap<u64, u64>>(&mut group, &map_data, "ahash");
            bench_out_of_the_box::<BTreeMapBenchMap<u64, u64>>(&mut group, &map_data, "btreemap");
            bench_out_of_the_box::<ConcreadBenchMap<u64, u64>>(&mut group, &map_data, "concread");
            bench_out_of_the_box::<DashMapBenchMap<u64, u64>>(&mut group, &map_data, "dashmap");
            bench_out_of_the_box::<HashbrownBenchMap<u64, u64>>(&mut group, &map_data, "hashbrown");
            bench_out_of_the_box::<HordeBenchMap<u64, u64>>(&mut group, &map_data, "horde");
            bench_out_of_the_box::<ImmutableChunkMapBenchMap<u64, u64>>(
                &mut group,
                &map_data,
                "immutable-chunkmap",
            );
            bench_out_of_the_box::<IndexMapBenchMap<u64, u64>>(&mut group, &map_data, "indexmap");
            bench_out_of_the_box::<RustCHashBenchMap<u64, u64>>(
                &mut group,
                &map_data,
                "rustc-hash",
            );
            bench_out_of_the_box::<StarshardBenchMap<u64, u64>>(&mut group, &map_data, "starshard");
            bench_out_of_the_box::<StdBenchMap<u64, u64>>(&mut group, &map_data, "std");
            bench_out_of_the_box::<TxMapBenchMap<u64, u64>>(&mut group, &map_data, "txmap");
        }

        // Every map that supports a custom hasher uses the same CommonHasher
        {
            let hasher = CommonHasher::new();
            let mut group = c.benchmark_group(format!(
                "iterate/{SAME_HASHER_GROUP_NAME}/map-size-{}",
                format_n(*entry_count)
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(*entry_count as u64));

            bench_same_hasher::<AhashBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                "ahash",
                hasher.clone(),
            );
            // bench_same_hasher::<BTreeMapBenchMap<u64, u64, CommonHasher>>(&mut group, &map_data, "btreemap"); // doesn't allow setting hasher
            // bench_same_hasher::<ConcreadBenchMap<u64, u64, CommonHasher>>(&mut group, &map_data, "concread"); // doesn't allow setting hasher
            bench_same_hasher::<DashMapBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                "dashmap",
                hasher.clone(),
            );
            bench_same_hasher::<HashbrownBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                "hashbrown",
                hasher.clone(),
            );
            bench_same_hasher::<HordeBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                "horde",
                hasher.clone(),
            );
            // bench_same_hasher::<ImmutableChunkMapBenchMap<u64, u64, CommonHasher>>(&mut group, &map_data, "immutable-chunkmap"); // doesn't allow setting hasher
            bench_same_hasher::<IndexMapBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                "indexmap",
                hasher.clone(),
            );
            // bench_same_hasher::<RustCHashBenchMap<u64, u64, CommonHasher>>(&mut group, &map_data, "rustc-hash"); // doesn't allow setting hasher
            bench_same_hasher::<StarshardBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                "starshard",
                hasher.clone(),
            );
            bench_same_hasher::<StdBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                "std",
                hasher.clone(),
            );
            bench_same_hasher::<TxMapBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                "txmap",
                hasher.clone(),
            );
        }
    }
}

criterion_group!(group, iterate);
criterion_main!(group);
