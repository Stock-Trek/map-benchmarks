use bench_map::{
    config::*,
    data::{
        byte_array::ByteArrayDataGen, string::StringDataGen, u64_sparse::U64SparseDataGen,
        uuid_v4::UuidV4DataGen,
    },
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BTreeMapBenchMap, BenchMapGetCloned, ConcreadBenchMap, DashMapBenchMap,
        HashbrownBenchMap, ImmutableChunkMapBenchMap, IndexMapBenchMap, RustCHashBenchMap,
        StarshardBenchMap, StdBenchMap, TxMapBenchMap,
    },
};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::rc::Rc;

macro_rules! bench_lookup_hit {
    ($group:ident, $map_data:expr, $map_type:path, $name:expr) => {
        let map_data = $map_data;
        $group.bench_function($name, move |b| {
            let map = map_data.create_map::<$map_type>();
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
    let entry_count = KEY_SENSITIVITY_ENTRY_COUNT;
    let existing_key_count: u64 = 100;
    let missing_key_count: u64 = 0;
    let sort_keys = false;
    // u64 keys
    {
        let map_data = Rc::new(MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            entry_count,
            existing_key_count as usize,
            missing_key_count as usize,
            sort_keys,
        ));
        let mut group = c.benchmark_group("key-sensitivity/u64");
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));
        bench_lookup_hit!(group, map_data.clone(), AhashBenchMap<_, _>, "ahash");
        bench_lookup_hit!(group, map_data.clone(), BTreeMapBenchMap<_, _>, "btreemap");
        bench_lookup_hit!(group, map_data.clone(), ConcreadBenchMap<_, _>, "concread");
        bench_lookup_hit!(group, map_data.clone(), DashMapBenchMap<_, _>, "dashmap");
        bench_lookup_hit!(group, map_data.clone(), HashbrownBenchMap<_, _>, "hashbrown");
        bench_lookup_hit!(group, map_data.clone(), ImmutableChunkMapBenchMap<_, _>, "immutable-chunkmap");
        bench_lookup_hit!(group, map_data.clone(), IndexMapBenchMap<_, _>, "indexmap");
        bench_lookup_hit!(group, map_data.clone(), RustCHashBenchMap<_, _>, "rustc-hash");
        bench_lookup_hit!(group, map_data.clone(), StarshardBenchMap<_, _>, "starshard");
        bench_lookup_hit!(group, map_data.clone(), StdBenchMap<_, _>, "std");
        bench_lookup_hit!(group, map_data.clone(), TxMapBenchMap<_, _>, "txmap");
    }
    // UUID v4 keys
    {
        let map_data = Rc::new(MapGen::generate(
            UuidV4DataGen,
            U64SparseDataGen,
            entry_count,
            existing_key_count as usize,
            missing_key_count as usize,
            sort_keys,
        ));
        let mut group = c.benchmark_group("key-sensitivity/UUID");
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));
        bench_lookup_hit!(group, map_data.clone(), AhashBenchMap<_, _>, "ahash");
        bench_lookup_hit!(group, map_data.clone(), BTreeMapBenchMap<_, _>, "btreemap");
        bench_lookup_hit!(group, map_data.clone(), ConcreadBenchMap<_, _>, "concread");
        bench_lookup_hit!(group, map_data.clone(), DashMapBenchMap<_, _>, "dashmap");
        bench_lookup_hit!(group, map_data.clone(), HashbrownBenchMap<_, _>, "hashbrown");
        bench_lookup_hit!(group, map_data.clone(), ImmutableChunkMapBenchMap<_, _>, "immutable-chunkmap");
        bench_lookup_hit!(group, map_data.clone(), IndexMapBenchMap<_, _>, "indexmap");
        bench_lookup_hit!(group, map_data.clone(), RustCHashBenchMap<_, _>, "rustc-hash");
        bench_lookup_hit!(group, map_data.clone(), StarshardBenchMap<_, _>, "starshard");
        bench_lookup_hit!(group, map_data.clone(), StdBenchMap<_, _>, "std");
        bench_lookup_hit!(group, map_data.clone(), TxMapBenchMap<_, _>, "txmap");
    }
    // Byte(32) keys
    {
        let map_data = Rc::new(MapGen::generate(
            ByteArrayDataGen::<32>,
            U64SparseDataGen,
            entry_count,
            existing_key_count as usize,
            missing_key_count as usize,
            sort_keys,
        ));
        let mut group = c.benchmark_group("key-sensitivity/Byte<32>");
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));
        bench_lookup_hit!(group, map_data.clone(), AhashBenchMap<_, _>, "ahash");
        bench_lookup_hit!(group, map_data.clone(), BTreeMapBenchMap<_, _>, "btreemap");
        bench_lookup_hit!(group, map_data.clone(), ConcreadBenchMap<_, _>, "concread");
        bench_lookup_hit!(group, map_data.clone(), DashMapBenchMap<_, _>, "dashmap");
        bench_lookup_hit!(group, map_data.clone(), HashbrownBenchMap<_, _>, "hashbrown");
        bench_lookup_hit!(group, map_data.clone(), ImmutableChunkMapBenchMap<_, _>, "immutable-chunkmap");
        bench_lookup_hit!(group, map_data.clone(), IndexMapBenchMap<_, _>, "indexmap");
        bench_lookup_hit!(group, map_data.clone(), RustCHashBenchMap<_, _>, "rustc-hash");
        bench_lookup_hit!(group, map_data.clone(), StarshardBenchMap<_, _>, "starshard");
        bench_lookup_hit!(group, map_data.clone(), StdBenchMap<_, _>, "std");
        bench_lookup_hit!(group, map_data.clone(), TxMapBenchMap<_, _>, "txmap");
    }
    // String<16> keys
    {
        let map_data = Rc::new(MapGen::generate(
            StringDataGen::<16>,
            U64SparseDataGen,
            entry_count,
            existing_key_count as usize,
            missing_key_count as usize,
            sort_keys,
        ));
        let mut group = c.benchmark_group("key-sensitivity/String<16>");
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));
        bench_lookup_hit!(group, map_data.clone(), AhashBenchMap<_, _>, "ahash");
        bench_lookup_hit!(group, map_data.clone(), BTreeMapBenchMap<_, _>, "btreemap");
        bench_lookup_hit!(group, map_data.clone(), ConcreadBenchMap<_, _>, "concread");
        bench_lookup_hit!(group, map_data.clone(), DashMapBenchMap<_, _>, "dashmap");
        bench_lookup_hit!(group, map_data.clone(), HashbrownBenchMap<_, _>, "hashbrown");
        bench_lookup_hit!(group, map_data.clone(), ImmutableChunkMapBenchMap<_, _>, "immutable-chunkmap");
        bench_lookup_hit!(group, map_data.clone(), IndexMapBenchMap<_, _>, "indexmap");
        bench_lookup_hit!(group, map_data.clone(), RustCHashBenchMap<_, _>, "rustc-hash");
        bench_lookup_hit!(group, map_data.clone(), StarshardBenchMap<_, _>, "starshard");
        bench_lookup_hit!(group, map_data.clone(), StdBenchMap<_, _>, "std");
        bench_lookup_hit!(group, map_data.clone(), TxMapBenchMap<_, _>, "txmap");
    }
    // String<128> keys
    {
        let map_data = Rc::new(MapGen::generate(
            StringDataGen::<128>,
            U64SparseDataGen,
            entry_count,
            existing_key_count as usize,
            missing_key_count as usize,
            sort_keys,
        ));
        let mut group = c.benchmark_group("key-sensitivity/String<128>");
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));
        bench_lookup_hit!(group, map_data.clone(), AhashBenchMap<_, _>, "ahash");
        bench_lookup_hit!(group, map_data.clone(), BTreeMapBenchMap<_, _>, "btreemap");
        bench_lookup_hit!(group, map_data.clone(), ConcreadBenchMap<_, _>, "concread");
        bench_lookup_hit!(group, map_data.clone(), DashMapBenchMap<_, _>, "dashmap");
        bench_lookup_hit!(group, map_data.clone(), HashbrownBenchMap<_, _>, "hashbrown");
        bench_lookup_hit!(group, map_data.clone(), ImmutableChunkMapBenchMap<_, _>, "immutable-chunkmap");
        bench_lookup_hit!(group, map_data.clone(), IndexMapBenchMap<_, _>, "indexmap");
        bench_lookup_hit!(group, map_data.clone(), RustCHashBenchMap<_, _>, "rustc-hash");
        bench_lookup_hit!(group, map_data.clone(), StarshardBenchMap<_, _>, "starshard");
        bench_lookup_hit!(group, map_data.clone(), StdBenchMap<_, _>, "std");
        bench_lookup_hit!(group, map_data.clone(), TxMapBenchMap<_, _>, "txmap");
    }
}

criterion_group!(group, baseline_key_sensitivity);
criterion_main!(group);
