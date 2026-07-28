use bench_map::{
    config::*,
    data::{
        byte_array::ByteArrayDataGen, string::StringDataGen, u64_sparse::U64SparseDataGen,
        uuid_v4::UuidV4DataGen,
    },
    map_gen::MapGen,
    maps::BenchMap,
    thousands_format::format_with_underscores,
};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::rc::Rc;

macro_rules! bench_lookup_hit {
    ($group:ident, $map_data:expr, $map_type:path, $name:expr) => {
        let map_data = $map_data;
        $group.bench_function($name, move |b| {
            let mut map = map_data.create_map::<$map_type>();
            let keys = map_data.existing_keys();
            b.iter(|| {
                for key in keys {
                    let key = std::hint::black_box(key);
                    std::hint::black_box(map.get_cloned(key));
                }
            });
        });
    };
}

fn baseline_key_sensitivity(c: &mut Criterion) {
    let existing_key_count: u64 = 100;
    let missing_key_count: u64 = 0;
    let sort_keys = false;
    for entry_count in KEY_SENSITIVITY_ENTRY_COUNT {
        // u64 keys
        {
            let map_data = Rc::new(MapGen::generate(
                U64SparseDataGen,
                U64SparseDataGen,
                *entry_count,
                existing_key_count as usize,
                missing_key_count as usize,
                sort_keys,
            ));
            let mut group = c.benchmark_group(format!(
                "key-sensitivity/u64/{}",
                format_with_underscores(*entry_count)
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(existing_key_count));
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::ahash_benchmap::AhashBenchMap<_, _>, "ahash");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::btreemap_benchmap::BTreeMapBenchMap<_, _>, "btreemap");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::concread_benchmap::ConcreadBenchMap<_, _>, "concread");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::dashmap_benchmap::DashMapBenchMap<_, _>, "dashmap");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::hashbrown_benchmap::HashbrownBenchMap<_, _>, "hashbrown");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::immutable_chunkmap_benchmap::ImmutableChunkMapBenchMap<_, _>, "immutable-chunkmap");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::indexmap_benchmap::IndexMapBenchMap<_, _>, "indexmap");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::rustc_hash_benchmap::RustCHashBenchMap<_, _>, "rustc-hash");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::starshard_benchmap::StarshardBenchMap<_, _>, "starshard");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::std_benchmap::StdBenchMap<_, _>, "std");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::txmap_benchmap::TxMapBenchMap<_, _>, "txmap");
        }
        // UUID v4 keys
        {
            let map_data = Rc::new(MapGen::generate(
                UuidV4DataGen,
                U64SparseDataGen,
                *entry_count,
                existing_key_count as usize,
                missing_key_count as usize,
                sort_keys,
            ));
            let mut group = c.benchmark_group(format!(
                "key-sensitivity/UUID/{}",
                format_with_underscores(*entry_count)
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(existing_key_count));
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::ahash_benchmap::AhashBenchMap<_, _>, "ahash");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::btreemap_benchmap::BTreeMapBenchMap<_, _>, "btreemap");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::concread_benchmap::ConcreadBenchMap<_, _>, "concread");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::dashmap_benchmap::DashMapBenchMap<_, _>, "dashmap");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::hashbrown_benchmap::HashbrownBenchMap<_, _>, "hashbrown");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::immutable_chunkmap_benchmap::ImmutableChunkMapBenchMap<_, _>, "immutable-chunkmap");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::indexmap_benchmap::IndexMapBenchMap<_, _>, "indexmap");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::rustc_hash_benchmap::RustCHashBenchMap<_, _>, "rustc-hash");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::starshard_benchmap::StarshardBenchMap<_, _>, "starshard");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::std_benchmap::StdBenchMap<_, _>, "std");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::txmap_benchmap::TxMapBenchMap<_, _>, "txmap");
        }
        // Byte(32) keys
        {
            let map_data = Rc::new(MapGen::generate(
                ByteArrayDataGen::<32>,
                U64SparseDataGen,
                *entry_count,
                existing_key_count as usize,
                missing_key_count as usize,
                sort_keys,
            ));
            let mut group = c.benchmark_group(format!(
                "key-sensitivity/Byte<32>/{}",
                format_with_underscores(*entry_count)
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(existing_key_count));
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::ahash_benchmap::AhashBenchMap<_, _>, "ahash");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::btreemap_benchmap::BTreeMapBenchMap<_, _>, "btreemap");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::concread_benchmap::ConcreadBenchMap<_, _>, "concread");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::dashmap_benchmap::DashMapBenchMap<_, _>, "dashmap");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::hashbrown_benchmap::HashbrownBenchMap<_, _>, "hashbrown");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::immutable_chunkmap_benchmap::ImmutableChunkMapBenchMap<_, _>, "immutable-chunkmap");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::indexmap_benchmap::IndexMapBenchMap<_, _>, "indexmap");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::rustc_hash_benchmap::RustCHashBenchMap<_, _>, "rustc-hash");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::starshard_benchmap::StarshardBenchMap<_, _>, "starshard");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::std_benchmap::StdBenchMap<_, _>, "std");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::txmap_benchmap::TxMapBenchMap<_, _>, "txmap");
        }
        // String<16> keys
        {
            let map_data = Rc::new(MapGen::generate(
                StringDataGen::<16>,
                U64SparseDataGen,
                *entry_count,
                existing_key_count as usize,
                missing_key_count as usize,
                sort_keys,
            ));
            let mut group = c.benchmark_group(format!(
                "key-sensitivity/String<16>/{}",
                format_with_underscores(*entry_count)
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(existing_key_count));
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::ahash_benchmap::AhashBenchMap<_, _>, "ahash");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::btreemap_benchmap::BTreeMapBenchMap<_, _>, "btreemap");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::concread_benchmap::ConcreadBenchMap<_, _>, "concread");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::dashmap_benchmap::DashMapBenchMap<_, _>, "dashmap");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::hashbrown_benchmap::HashbrownBenchMap<_, _>, "hashbrown");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::immutable_chunkmap_benchmap::ImmutableChunkMapBenchMap<_, _>, "immutable-chunkmap");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::indexmap_benchmap::IndexMapBenchMap<_, _>, "indexmap");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::rustc_hash_benchmap::RustCHashBenchMap<_, _>, "rustc-hash");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::starshard_benchmap::StarshardBenchMap<_, _>, "starshard");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::std_benchmap::StdBenchMap<_, _>, "std");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::txmap_benchmap::TxMapBenchMap<_, _>, "txmap");
        }
        // String<128> keys
        {
            let map_data = Rc::new(MapGen::generate(
                StringDataGen::<128>,
                U64SparseDataGen,
                *entry_count,
                existing_key_count as usize,
                missing_key_count as usize,
                sort_keys,
            ));
            let mut group = c.benchmark_group(format!(
                "key-sensitivity/String<128>/{}",
                format_with_underscores(*entry_count)
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(existing_key_count));
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::ahash_benchmap::AhashBenchMap<_, _>, "ahash");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::btreemap_benchmap::BTreeMapBenchMap<_, _>, "btreemap");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::concread_benchmap::ConcreadBenchMap<_, _>, "concread");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::dashmap_benchmap::DashMapBenchMap<_, _>, "dashmap");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::hashbrown_benchmap::HashbrownBenchMap<_, _>, "hashbrown");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::immutable_chunkmap_benchmap::ImmutableChunkMapBenchMap<_, _>, "immutable-chunkmap");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::indexmap_benchmap::IndexMapBenchMap<_, _>, "indexmap");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::rustc_hash_benchmap::RustCHashBenchMap<_, _>, "rustc-hash");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::starshard_benchmap::StarshardBenchMap<_, _>, "starshard");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::std_benchmap::StdBenchMap<_, _>, "std");
            bench_lookup_hit!(group, map_data.clone(), bench_map::maps::txmap_benchmap::TxMapBenchMap<_, _>, "txmap");
        }
    }
}

criterion_group!(group, baseline_key_sensitivity);
criterion_main!(group);
