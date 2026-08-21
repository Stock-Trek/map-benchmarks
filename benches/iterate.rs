use bench_map::{
    config::*, constants::*, data::u64_sparse::U64SparseDataGen, map_data::MapData,
    map_gen::MapGen, maps::*, number_formatter::format_n,
};
use criterion::{
    BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main, measurement::WallTime,
};
use std::hint::black_box;

type CommonHasher = ahash::RandomState;

fn bench_out_of_the_box<Map>(
    name: &str,
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<u64, u64>,
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
    name: &str,
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<u64, u64>,
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
    for entry_count in DEFAULT_ENTRY_COUNTS {
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

            bench_out_of_the_box::<AhashBenchMap<u64, u64>>("ahash", &mut group, &map_data);
            bench_out_of_the_box::<BTreeMapBenchMap<u64, u64>>("btreemap", &mut group, &map_data);
            // bench_out_of_the_box::<ConcreadBenchMap<u64, u64>>("concread", &mut group, &map_data); // too slow
            bench_out_of_the_box::<ConcurrentMapBenchMap<u64, u64>>(
                "concurrent-map",
                &mut group,
                &map_data,
            );
            bench_out_of_the_box::<CrossbeamSkiplistBenchMap<u64, u64>>(
                "crossbeam-skiplist",
                &mut group,
                &map_data,
            );
            bench_out_of_the_box::<DashMapBenchMap<u64, u64>>("dashmap", &mut group, &map_data);
            // bench_out_of_the_box::<FlurryBenchMap<u64, u64>>("flurry", &mut group, &map_data); // too slow
            bench_out_of_the_box::<HashbrownBenchMap<u64, u64>>("hashbrown", &mut group, &map_data);
            bench_out_of_the_box::<HashlinkBenchMap<u64, u64>>("hashlink", &mut group, &map_data);
            bench_out_of_the_box::<HordeBenchMap<u64, u64>>("horde", &mut group, &map_data);
            bench_out_of_the_box::<ImmutableChunkMapBenchMap<u64, u64>>(
                "immutable-chunkmap",
                &mut group,
                &map_data,
            );
            bench_out_of_the_box::<ImblBenchMap<u64, u64>>("imbl", &mut group, &map_data);
            bench_out_of_the_box::<IndexMapBenchMap<u64, u64>>("indexmap", &mut group, &map_data);
            bench_out_of_the_box::<LeapfrogBenchMap<u64, u64>>("leapfrog", &mut group, &map_data);
            bench_out_of_the_box::<PapayaBenchMap<u64, u64>>("papaya", &mut group, &map_data);
            bench_out_of_the_box::<RpdsHashTrieMapBenchMap<u64, u64>>(
                "rpds-hash-trie-map",
                &mut group,
                &map_data,
            );
            bench_out_of_the_box::<RustCHashBenchMap<u64, u64>>(
                "rustc-hash",
                &mut group,
                &map_data,
            );
            bench_out_of_the_box::<SccBenchMap<u64, u64>>("scc", &mut group, &map_data);
            bench_out_of_the_box::<StarshardBenchMap<u64, u64>>("starshard", &mut group, &map_data);
            bench_out_of_the_box::<StdBenchMap<u64, u64>>("std", &mut group, &map_data);
            bench_out_of_the_box::<TxMapBenchMap<u64, u64>>("txmap", &mut group, &map_data);
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
                "ahash",
                &mut group,
                &map_data,
                hasher.clone(),
            );
            // bench_same_hasher::<BTreeMapBenchMap<u64, u64, CommonHasher>>("btreemap", &mut group, &map_data); // doesn't allow setting hasher
            // bench_same_hasher::<ConcreadBenchMap<u64, u64, CommonHasher>>("concread", &mut group, &map_data); // doesn't allow setting hasher
            // bench_same_hasher::<ConcurrentMapBenchMap<u64, u64, CommonHasher>>("concurrent-map", &mut group, &map_data); // doesn't allow setting hasher
            bench_same_hasher::<DashMapBenchMap<u64, u64, CommonHasher>>(
                "dashmap",
                &mut group,
                &map_data,
                hasher.clone(),
            );
            // bench_same_hasher::<FlurryBenchMap<u64, u64, CommonHasher>>("flurry", &mut group, &map_data, hasher.clone()); // too slow
            bench_same_hasher::<HashbrownBenchMap<u64, u64, CommonHasher>>(
                "hashbrown",
                &mut group,
                &map_data,
                hasher.clone(),
            );
            bench_same_hasher::<HashlinkBenchMap<u64, u64, CommonHasher>>(
                "hashlink",
                &mut group,
                &map_data,
                hasher.clone(),
            );
            bench_same_hasher::<HordeBenchMap<u64, u64, CommonHasher>>(
                "horde",
                &mut group,
                &map_data,
                hasher.clone(),
            );
            // bench_same_hasher::<ImmutableChunkMapBenchMap<u64, u64, CommonHasher>>("immutable-chunkmap", &mut group, &map_data); // doesn't allow setting hasher
            bench_same_hasher::<ImblBenchMap<u64, u64, CommonHasher>>(
                "imbl",
                &mut group,
                &map_data,
                hasher.clone(),
            );
            bench_same_hasher::<IndexMapBenchMap<u64, u64, CommonHasher>>(
                "indexmap",
                &mut group,
                &map_data,
                hasher.clone(),
            );
            bench_same_hasher::<LeapfrogBenchMap<u64, u64, CommonHasher>>(
                "leapfrog",
                &mut group,
                &map_data,
                hasher.clone(),
            );
            bench_same_hasher::<PapayaBenchMap<u64, u64, CommonHasher>>(
                "papaya",
                &mut group,
                &map_data,
                hasher.clone(),
            );
            bench_same_hasher::<RpdsHashTrieMapBenchMap<u64, u64, CommonHasher>>(
                "rpds-hash-trie-map",
                &mut group,
                &map_data,
                hasher.clone(),
            );
            // bench_same_hasher::<RustCHashBenchMap<u64, u64, CommonHasher>>("rustc-hash", &mut group, &map_data); // doesn't allow setting hasher
            bench_same_hasher::<SccBenchMap<u64, u64, CommonHasher>>(
                "scc",
                &mut group,
                &map_data,
                hasher.clone(),
            );
            bench_same_hasher::<StarshardBenchMap<u64, u64, CommonHasher>>(
                "starshard",
                &mut group,
                &map_data,
                hasher.clone(),
            );
            bench_same_hasher::<StdBenchMap<u64, u64, CommonHasher>>(
                "std",
                &mut group,
                &map_data,
                hasher.clone(),
            );
            bench_same_hasher::<TxMapBenchMap<u64, u64, CommonHasher>>(
                "txmap",
                &mut group,
                &map_data,
                hasher.clone(),
            );
        }
    }
}

criterion_group!(group, iterate);
criterion_main!(group);
