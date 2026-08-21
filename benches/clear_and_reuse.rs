use bench_map::{
    config::*, constants::*, data::u64_sparse::U64SparseDataGen, map_data::MapData,
    map_gen::MapGen, maps::*, number_formatter::format_n,
};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main,
    measurement::WallTime,
};
use std::hint::black_box;

type CommonHasher = ahash::RandomState;

fn bench_out_of_the_box<Map>(
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<u64, u64>,
    name: &str,
) where
    Map: BenchMapNew<u64, u64> + BenchMapMutInsert<u64, u64> + BenchMapMutClear<u64, u64>,
{
    group.bench_function(name, move |b| {
        let map_data_ref = &map_data;
        b.iter_batched(
            move || {
                // "Pool" setup: a fully populated map.
                let map = map_data_ref.create_map::<Map>();
                let keys = map_data_ref.missing_keys().clone();
                (map, keys)
            },
            // Measured cycle: empty the map but keep it alive, then refill it.
            |(mut map, mut keys)| {
                map.clear();
                for key in keys.drain(..) {
                    let key = black_box(key);
                    map.insert(key, 42);
                }
                black_box(map)
            },
            BatchSize::PerIteration,
        );
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
        + BenchMapMutClear<u64, u64>,
{
    group.bench_function(name, move |b| {
        let map_data_ref = &map_data;
        let hasher = hasher.clone();
        b.iter_batched(
            move || {
                // "Pool" setup: a fully populated map.
                let map = map_data_ref.create_map_with_hasher::<Map, CommonHasher>(hasher.clone());
                let keys = map_data_ref.missing_keys().clone();
                (map, keys)
            },
            // Measured cycle: empty the map but keep it alive, then refill it.
            |(mut map, mut keys)| {
                map.clear();
                for key in keys.drain(..) {
                    let key = black_box(key);
                    map.insert(key, 42);
                }
                black_box(map)
            },
            BatchSize::PerIteration,
        );
    });
}

fn clear_and_reuse(c: &mut Criterion) {
    let sort_keys = false;
    for entry_count in DEFAULT_ENTRY_COUNTS {
        let existing_key_count = *entry_count;
        let missing_key_count = *entry_count;
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
                "clear-and-reuse/{OUT_OF_THE_BOX_GROUP_NAME}/map-size-{}",
                format_n(*entry_count)
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(*entry_count as u64));

            bench_out_of_the_box::<AhashBenchMap<u64, u64>>(&mut group, &map_data, "ahash");
            bench_out_of_the_box::<BTreeMapBenchMap<u64, u64>>(&mut group, &map_data, "btreemap");
            // bench_out_of_the_box::<ConcreadBenchMap<u64, u64>>(&mut group, &map_data, "concread"); // too slow
            // bench_out_of_the_box::<ConcurrentMapBenchMap<u64, u64>>(&mut group, &map_data, "concurrent-map"); // no clear
            bench_out_of_the_box::<CrossbeamSkiplistBenchMap<u64, u64>>(
                &mut group,
                &map_data,
                "crossbeam-skiplist",
            );
            bench_out_of_the_box::<DashMapBenchMap<u64, u64>>(&mut group, &map_data, "dashmap");
            // bench_out_of_the_box::<FlurryBenchMap<u64, u64>>(&mut group, &map_data, "flurry"); // too slow
            bench_out_of_the_box::<HashbrownBenchMap<u64, u64>>(&mut group, &map_data, "hashbrown");
            bench_out_of_the_box::<HashlinkBenchMap<u64, u64>>(&mut group, &map_data, "hashlink");
            bench_out_of_the_box::<HordeBenchMap<u64, u64>>(&mut group, &map_data, "horde");
            // bench_out_of_the_box::<ImmutableChunkMapBenchMap<u64, u64>>(&mut group, &map_data, "immutable-chunkmap"); // no clear
            bench_out_of_the_box::<ImblBenchMap<u64, u64>>(&mut group, &map_data, "imbl");
            bench_out_of_the_box::<IndexMapBenchMap<u64, u64>>(&mut group, &map_data, "indexmap");
            // bench_out_of_the_box::<LeapfrogBenchMap<u64, u64>>(&mut group, &map_data, "leapfrog"); // no clear
            bench_out_of_the_box::<PapayaBenchMap<u64, u64>>(&mut group, &map_data, "papaya");
            // bench_out_of_the_box::<RpdsHashTrieMapBenchMap<u64, u64>>(&mut group, &map_data, "rpds-hash-trie-map"); // no clear
            bench_out_of_the_box::<RustCHashBenchMap<u64, u64>>(
                &mut group,
                &map_data,
                "rustc-hash",
            );
            bench_out_of_the_box::<SccBenchMap<u64, u64>>(&mut group, &map_data, "scc");
            bench_out_of_the_box::<StarshardBenchMap<u64, u64>>(&mut group, &map_data, "starshard");
            bench_out_of_the_box::<StdBenchMap<u64, u64>>(&mut group, &map_data, "std");
            bench_out_of_the_box::<TxMapBenchMap<u64, u64>>(&mut group, &map_data, "txmap");
        }

        // Every map that supports a custom hasher uses the same CommonHasher
        {
            let hasher = CommonHasher::new();
            let mut group = c.benchmark_group(format!(
                "clear-and-reuse/{SAME_HASHER_GROUP_NAME}/map-size-{}",
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
            // bench_same_hasher::<ConcurrentMapBenchMap<u64, u64, CommonHasher>>(&mut group, &map_data, "concurrent-map"); // doesn't allow setting hasher
            bench_same_hasher::<DashMapBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                "dashmap",
                hasher.clone(),
            );
            // bench_same_hasher::<FlurryBenchMap<u64, u64, CommonHasher>>(&mut group, &map_data, "flurry", hasher.clone()); // too slow
            bench_same_hasher::<HashbrownBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                "hashbrown",
                hasher.clone(),
            );
            bench_same_hasher::<HashlinkBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                "hashlink",
                hasher.clone(),
            );
            bench_same_hasher::<HordeBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                "horde",
                hasher.clone(),
            );
            // bench_same_hasher::<ImmutableChunkMapBenchMap<u64, u64, CommonHasher>>(&mut group, &map_data, "immutable-chunkmap"); // doesn't allow setting hasher
            bench_same_hasher::<ImblBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                "imbl",
                hasher.clone(),
            );
            bench_same_hasher::<IndexMapBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                "indexmap",
                hasher.clone(),
            );
            // bench_same_hasher::<LeapfrogBenchMap<u64, u64, CommonHasher>>(&mut group, &map_data, "leapfrog"); // doesn't allow setting hasher
            bench_same_hasher::<PapayaBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                "papaya",
                hasher.clone(),
            );
            // bench_same_hasher::<RpdsHashTrieMapBenchMap<u64, u64, CommonHasher>>(&mut group, &map_data, "rpds-hash-trie-map"); // doesn't allow setting hasher
            // bench_same_hasher::<RustCHashBenchMap<u64, u64, CommonHasher>>(&mut group, &map_data, "rustc-hash"); // doesn't allow setting hasher
            bench_same_hasher::<SccBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                "scc",
                hasher.clone(),
            );
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

criterion_group!(group, clear_and_reuse);
criterion_main!(group);
