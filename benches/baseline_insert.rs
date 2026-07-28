use bench_map::{
    config::*,
    data::{string::StringDataGen, u64_sparse::U64SparseDataGen},
    map_gen::MapGen,
    maps::BenchMap,
    thousands_format::format_with_underscores,
};
use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
use std::rc::Rc;

macro_rules! bench_insert_missing {
    ($group:ident, $map_data:expr, $map_type:path, $name:expr) => {
        let map_data = $map_data;
        $group.bench_function($name, move |b| {
            let map_data_ref = &map_data;
            b.iter_batched(
                move || {
                    let map = map_data_ref.create_map::<$map_type>();
                    let keys = map_data_ref.missing_keys().clone();
                    (map, keys)
                },
                |(mut map, mut keys)| {
                    for key in keys.drain(..) {
                        let key = std::hint::black_box(key);
                        std::hint::black_box(map.insert(key, 42));
                    }
                    std::hint::black_box(map)
                },
                BatchSize::PerIteration,
            );
        });
    };
}

fn insert(c: &mut Criterion) {
    let entry_count = 0;
    let existing_key_count = 0;
    let sort_keys = false;
    for missing_key_count in ENTRY_COUNT {
        {
            // u64 keys
            let map_data = Rc::new(MapGen::generate(
                U64SparseDataGen,
                U64SparseDataGen,
                entry_count,
                existing_key_count,
                *missing_key_count,
                sort_keys,
            ));
            let mut group = c.benchmark_group(format!(
                "baseline/insert-u64-keys/{}",
                format_with_underscores(*missing_key_count as u64)
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(*missing_key_count as u64));
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::ahash_benchmap::AhashBenchMap<_, _>, "ahash");
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::btreemap_benchmap::BTreeMapBenchMap<_, _>, "btreemap");
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::concread_benchmap::ConcreadBenchMap<_, _>, "concread");
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::dashmap_benchmap::DashMapBenchMap<_, _>, "dashmap");
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::hashbrown_benchmap::HashbrownBenchMap<_, _>, "hashbrown");
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::immutable_chunkmap_benchmap::ImmutableChunkMapBenchMap<_, _>, "immutable-chunkmap");
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::indexmap_benchmap::IndexMapBenchMap<_, _>, "indexmap");
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::rustc_hash_benchmap::RustCHashBenchMap<_, _>, "rustc-hash");
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::starshard_benchmap::StarshardBenchMap<_, _>, "starshard");
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::std_benchmap::StdBenchMap<_, _>, "std");
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::txmap_benchmap::TxMapBenchMap<_, _>, "txmap");
        }
        {
            // String keys
            let map_data = Rc::new(MapGen::generate(
                StringDataGen::<32>,
                U64SparseDataGen,
                entry_count,
                existing_key_count,
                *missing_key_count,
                sort_keys,
            ));
            let mut group = c.benchmark_group(format!(
                "baseline/insert-String<32>-keys/{}",
                format_with_underscores(*missing_key_count as u64)
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(*missing_key_count as u64));
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::ahash_benchmap::AhashBenchMap<_, _>, "ahash");
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::btreemap_benchmap::BTreeMapBenchMap<_, _>, "btreemap");
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::concread_benchmap::ConcreadBenchMap<_, _>, "concread");
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::dashmap_benchmap::DashMapBenchMap<_, _>, "dashmap");
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::hashbrown_benchmap::HashbrownBenchMap<_, _>, "hashbrown");
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::immutable_chunkmap_benchmap::ImmutableChunkMapBenchMap<_, _>, "immutable-chunkmap");
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::indexmap_benchmap::IndexMapBenchMap<_, _>, "indexmap");
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::rustc_hash_benchmap::RustCHashBenchMap<_, _>, "rustc-hash");
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::starshard_benchmap::StarshardBenchMap<_, _>, "starshard");
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::std_benchmap::StdBenchMap<_, _>, "std");
            bench_insert_missing!(group, map_data.clone(), bench_map::maps::txmap_benchmap::TxMapBenchMap<_, _>, "txmap");
        }
    }
}

criterion_group!(group, insert);
criterion_main!(group);
