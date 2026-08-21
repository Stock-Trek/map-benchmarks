use bench_map::{
    config::*, constants::*, data::u64_sparse::U64SparseDataGen, map_data::MapData,
    map_gen::MapGen, maps::*, number_formatter::format_n,
};
use criterion::{
    BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main, measurement::WallTime,
};
use std::hint::black_box;

fn bench_clone<Map>(group: &mut BenchmarkGroup<WallTime>, map_data: &MapData<u64, u64>, name: &str)
where
    Map: BenchMapNew<u64, u64> + BenchMapMutInsert<u64, u64> + BenchMapClone<u64, u64>,
{
    group.bench_function(name, move |b| {
        let map = map_data.create_map::<Map>();
        b.iter(|| {
            black_box(map.clone_map());
        });
    });
}

fn bench_clone_then_write<Map>(
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<u64, u64>,
    name: &str,
) where
    Map: BenchMapNew<u64, u64> + BenchMapMutInsert<u64, u64> + BenchMapClone<u64, u64>,
{
    group.bench_function(name, move |b| {
        let map = map_data.create_map::<Map>();
        let keys = map_data.missing_keys().clone();
        b.iter(|| {
            let mut cloned = map.clone_map();
            for key in &keys {
                let key = black_box(*key);
                cloned.insert(key, 42);
            }
            black_box(cloned);
        });
    });
}

fn clone(c: &mut Criterion) {
    let existing_key_count = 0;
    let missing_key_count = 0;
    let sort_keys = false;
    for entry_count in ENTRY_COUNTS {
        let map_data = MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            *entry_count,
            existing_key_count,
            missing_key_count,
            sort_keys,
        );
        let mut group = c.benchmark_group(format!(
            "clone/{OUT_OF_THE_BOX_GROUP_NAME}/map-size-{}",
            format_n(*entry_count)
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(*entry_count as u64));

        bench_clone::<AhashBenchMap<u64, u64>>(&mut group, &map_data, "ahash");
        bench_clone::<BTreeMapBenchMap<u64, u64>>(&mut group, &map_data, "btreemap");
        // bench_clone::<ConcreadBenchMap<u64, u64>>(&mut group, &map_data, "concread"); // doesn't implement Clone
        bench_clone::<ConcurrentMapBenchMap<u64, u64>>(&mut group, &map_data, "concurrent-map");
        // bench_clone::<CrossbeamSkiplistBenchMap<u64, u64>>(&mut group, &map_data, "crossbeam-skiplist"); // doesn't implement Clone
        bench_clone::<DashMapBenchMap<u64, u64>>(&mut group, &map_data, "dashmap");
        // bench_clone::<FlurryBenchMap<u64, u64>>(&mut group, &map_data, "flurry"); // too slow
        bench_clone::<HashbrownBenchMap<u64, u64>>(&mut group, &map_data, "hashbrown");
        bench_clone::<HashlinkBenchMap<u64, u64>>(&mut group, &map_data, "hashlink");
        bench_clone::<HordeBenchMap<u64, u64>>(&mut group, &map_data, "horde");
        bench_clone::<ImmutableChunkMapBenchMap<u64, u64>>(
            &mut group,
            &map_data,
            "immutable-chunkmap",
        );
        bench_clone::<ImblBenchMap<u64, u64>>(&mut group, &map_data, "imbl");
        bench_clone::<IndexMapBenchMap<u64, u64>>(&mut group, &map_data, "indexmap");
        // bench_clone::<LeapfrogBenchMap<u64, u64>>(&mut group, &map_data, "leapfrog"); // doesn't implement Clone
        bench_clone::<PapayaBenchMap<u64, u64>>(&mut group, &map_data, "papaya");
        bench_clone::<RpdsHashTrieMapBenchMap<u64, u64>>(
            &mut group,
            &map_data,
            "rpds-hash-trie-map",
        );
        bench_clone::<RustCHashBenchMap<u64, u64>>(&mut group, &map_data, "rustc-hash");
        bench_clone::<SccBenchMap<u64, u64>>(&mut group, &map_data, "scc");
        bench_clone::<StarshardBenchMap<u64, u64>>(&mut group, &map_data, "starshard");
        bench_clone::<StdBenchMap<u64, u64>>(&mut group, &map_data, "std");
        bench_clone::<TxMapBenchMap<u64, u64>>(&mut group, &map_data, "txmap");
    }
}

fn clone_then_write(c: &mut Criterion) {
    let existing_key_count = 0;
    let sort_keys = false;
    for entry_count in ENTRY_COUNTS {
        let missing_key_count = *entry_count / 10;
        let map_data = MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            *entry_count,
            existing_key_count,
            missing_key_count,
            sort_keys,
        );
        let mut group = c.benchmark_group(format!(
            "clone-then-write/{OUT_OF_THE_BOX_GROUP_NAME}/map-size-{}",
            format_n(*entry_count)
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(*entry_count as u64));

        bench_clone_then_write::<AhashBenchMap<u64, u64>>(&mut group, &map_data, "ahash");
        bench_clone_then_write::<BTreeMapBenchMap<u64, u64>>(&mut group, &map_data, "btreemap");
        // bench_clone_then_write::<ConcreadBenchMap<u64, u64>>(&mut group, &map_data, "concread"); // doesn't implement Clone
        bench_clone_then_write::<ConcurrentMapBenchMap<u64, u64>>(
            &mut group,
            &map_data,
            "concurrent-map",
        );
        // bench_clone_then_write::<CrossbeamSkiplistBenchMap<u64, u64>>(&mut group, &map_data, "crossbeam-skiplist"); // doesn't implement Clone
        bench_clone_then_write::<DashMapBenchMap<u64, u64>>(&mut group, &map_data, "dashmap");
        // bench_clone_then_write::<FlurryBenchMap<u64, u64>>(&mut group, &map_data, "flurry"); // too slow
        bench_clone_then_write::<HashbrownBenchMap<u64, u64>>(&mut group, &map_data, "hashbrown");
        bench_clone_then_write::<HashlinkBenchMap<u64, u64>>(&mut group, &map_data, "hashlink");
        bench_clone_then_write::<HordeBenchMap<u64, u64>>(&mut group, &map_data, "horde");
        bench_clone_then_write::<ImmutableChunkMapBenchMap<u64, u64>>(
            &mut group,
            &map_data,
            "immutable-chunkmap",
        );
        bench_clone_then_write::<ImblBenchMap<u64, u64>>(&mut group, &map_data, "imbl");
        bench_clone_then_write::<IndexMapBenchMap<u64, u64>>(&mut group, &map_data, "indexmap");
        // bench_clone_then_write::<LeapfrogBenchMap<u64, u64>>(&mut group, &map_data, "leapfrog"); // doesn't implement Clone
        bench_clone_then_write::<PapayaBenchMap<u64, u64>>(&mut group, &map_data, "papaya");
        bench_clone_then_write::<RpdsHashTrieMapBenchMap<u64, u64>>(
            &mut group,
            &map_data,
            "rpds-hash-trie-map",
        );
        bench_clone_then_write::<RustCHashBenchMap<u64, u64>>(&mut group, &map_data, "rustc-hash");
        bench_clone_then_write::<SccBenchMap<u64, u64>>(&mut group, &map_data, "scc");
        bench_clone_then_write::<StarshardBenchMap<u64, u64>>(&mut group, &map_data, "starshard");
        bench_clone_then_write::<StdBenchMap<u64, u64>>(&mut group, &map_data, "std");
        bench_clone_then_write::<TxMapBenchMap<u64, u64>>(&mut group, &map_data, "txmap");
    }
}

criterion_group!(group, clone, clone_then_write);
criterion_main!(group);
