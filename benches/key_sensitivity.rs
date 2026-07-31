use bench_map::{
    config::*,
    data::{
        byte_array::ByteArrayDataGen, string::StringDataGen, u64_sparse::U64SparseDataGen,
        uuid_v4::UuidV4DataGen,
    },
    map_data::MapData,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BenchMapGetCloned, BenchMapMutInsert, BenchMapNew, ConcreadBenchMap,
        DashMapBenchMap, HashbrownBenchMap, ImmutableChunkMapBenchMap, IndexMapBenchMap,
        RustCHashBenchMap, StarshardBenchMap, StdBenchMap, TxMapBenchMap,
    },
};
use criterion::{
    BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main, measurement::WallTime,
};
use std::{hash::Hash, rc::Rc};
use uuid::Uuid;

fn bench_lookup_hit<Map, K, V>(
    group: &mut BenchmarkGroup<WallTime>,
    map_data: Rc<MapData<K, V>>,
    name: &str,
) where
    Map: BenchMapNew<K, V> + BenchMapMutInsert<K, V> + BenchMapGetCloned<K, V>,
    K: Clone + Hash + Eq,
    V: Clone,
{
    group.bench_function(name, move |b| {
        let map = map_data.create_map::<Map>();
        let keys = map_data.existing_keys();
        b.iter(|| {
            for key in keys {
                let key = std::hint::black_box(key);
                std::hint::black_box(map.get_cloned(key));
            }
        });
    });
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
        bench_lookup_hit::<AhashBenchMap<u64, u64>, u64, u64>(
            &mut group,
            map_data.clone(),
            "ahash",
        );
        // bench_lookup_hit::<BTreeMapBenchMap<u64, u64>, u64, u64>(&mut group, map_data.clone(), "btreemap"); // too slow
        bench_lookup_hit::<ConcreadBenchMap<u64, u64>, u64, u64>(
            &mut group,
            map_data.clone(),
            "concread",
        );
        bench_lookup_hit::<DashMapBenchMap<u64, u64>, u64, u64>(
            &mut group,
            map_data.clone(),
            "dashmap",
        );
        bench_lookup_hit::<HashbrownBenchMap<u64, u64>, u64, u64>(
            &mut group,
            map_data.clone(),
            "hashbrown",
        );
        bench_lookup_hit::<ImmutableChunkMapBenchMap<u64, u64>, u64, u64>(
            &mut group,
            map_data.clone(),
            "immutable-chunkmap",
        );
        bench_lookup_hit::<IndexMapBenchMap<u64, u64>, u64, u64>(
            &mut group,
            map_data.clone(),
            "indexmap",
        );
        bench_lookup_hit::<RustCHashBenchMap<u64, u64>, u64, u64>(
            &mut group,
            map_data.clone(),
            "rustc-hash",
        );
        bench_lookup_hit::<StarshardBenchMap<u64, u64>, u64, u64>(
            &mut group,
            map_data.clone(),
            "starshard",
        );
        bench_lookup_hit::<StdBenchMap<u64, u64>, u64, u64>(&mut group, map_data.clone(), "std");
        bench_lookup_hit::<TxMapBenchMap<u64, u64>, u64, u64>(
            &mut group,
            map_data.clone(),
            "txmap",
        );
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
        bench_lookup_hit::<AhashBenchMap<Uuid, u64>, Uuid, u64>(
            &mut group,
            map_data.clone(),
            "ahash",
        );
        // bench_lookup_hit::<BTreeMapBenchMap<Uuid, u64>, Uuid, u64>(&mut group, map_data.clone(), "btreemap"); // too slow
        bench_lookup_hit::<ConcreadBenchMap<Uuid, u64>, Uuid, u64>(
            &mut group,
            map_data.clone(),
            "concread",
        );
        bench_lookup_hit::<DashMapBenchMap<Uuid, u64>, Uuid, u64>(
            &mut group,
            map_data.clone(),
            "dashmap",
        );
        bench_lookup_hit::<HashbrownBenchMap<Uuid, u64>, Uuid, u64>(
            &mut group,
            map_data.clone(),
            "hashbrown",
        );
        bench_lookup_hit::<ImmutableChunkMapBenchMap<Uuid, u64>, Uuid, u64>(
            &mut group,
            map_data.clone(),
            "immutable-chunkmap",
        );
        bench_lookup_hit::<IndexMapBenchMap<Uuid, u64>, Uuid, u64>(
            &mut group,
            map_data.clone(),
            "indexmap",
        );
        bench_lookup_hit::<RustCHashBenchMap<Uuid, u64>, Uuid, u64>(
            &mut group,
            map_data.clone(),
            "rustc-hash",
        );
        bench_lookup_hit::<StarshardBenchMap<Uuid, u64>, Uuid, u64>(
            &mut group,
            map_data.clone(),
            "starshard",
        );
        bench_lookup_hit::<StdBenchMap<Uuid, u64>, Uuid, u64>(&mut group, map_data.clone(), "std");
        bench_lookup_hit::<TxMapBenchMap<Uuid, u64>, Uuid, u64>(
            &mut group,
            map_data.clone(),
            "txmap",
        );
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
        bench_lookup_hit::<AhashBenchMap<[u8; 32], u64>, [u8; 32], u64>(
            &mut group,
            map_data.clone(),
            "ahash",
        );
        // bench_lookup_hit::<BTreeMapBenchMap<[u8; 32], u64>, [u8; 32], u64>(&mut group, map_data.clone(), "btreemap"); // too slow
        bench_lookup_hit::<ConcreadBenchMap<[u8; 32], u64>, [u8; 32], u64>(
            &mut group,
            map_data.clone(),
            "concread",
        );
        bench_lookup_hit::<DashMapBenchMap<[u8; 32], u64>, [u8; 32], u64>(
            &mut group,
            map_data.clone(),
            "dashmap",
        );
        bench_lookup_hit::<HashbrownBenchMap<[u8; 32], u64>, [u8; 32], u64>(
            &mut group,
            map_data.clone(),
            "hashbrown",
        );
        bench_lookup_hit::<ImmutableChunkMapBenchMap<[u8; 32], u64>, [u8; 32], u64>(
            &mut group,
            map_data.clone(),
            "immutable-chunkmap",
        );
        bench_lookup_hit::<IndexMapBenchMap<[u8; 32], u64>, [u8; 32], u64>(
            &mut group,
            map_data.clone(),
            "indexmap",
        );
        bench_lookup_hit::<RustCHashBenchMap<[u8; 32], u64>, [u8; 32], u64>(
            &mut group,
            map_data.clone(),
            "rustc-hash",
        );
        bench_lookup_hit::<StarshardBenchMap<[u8; 32], u64>, [u8; 32], u64>(
            &mut group,
            map_data.clone(),
            "starshard",
        );
        bench_lookup_hit::<StdBenchMap<[u8; 32], u64>, [u8; 32], u64>(
            &mut group,
            map_data.clone(),
            "std",
        );
        bench_lookup_hit::<TxMapBenchMap<[u8; 32], u64>, [u8; 32], u64>(
            &mut group,
            map_data.clone(),
            "txmap",
        );
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
        bench_lookup_hit::<AhashBenchMap<String, u64>, String, u64>(
            &mut group,
            map_data.clone(),
            "ahash",
        );
        // bench_lookup_hit::<BTreeMapBenchMap<String, u64>, String, u64>(&mut group, map_data.clone(), "btreemap"); // too slow
        bench_lookup_hit::<ConcreadBenchMap<String, u64>, String, u64>(
            &mut group,
            map_data.clone(),
            "concread",
        );
        bench_lookup_hit::<DashMapBenchMap<String, u64>, String, u64>(
            &mut group,
            map_data.clone(),
            "dashmap",
        );
        bench_lookup_hit::<HashbrownBenchMap<String, u64>, String, u64>(
            &mut group,
            map_data.clone(),
            "hashbrown",
        );
        bench_lookup_hit::<ImmutableChunkMapBenchMap<String, u64>, String, u64>(
            &mut group,
            map_data.clone(),
            "immutable-chunkmap",
        );
        bench_lookup_hit::<IndexMapBenchMap<String, u64>, String, u64>(
            &mut group,
            map_data.clone(),
            "indexmap",
        );
        bench_lookup_hit::<RustCHashBenchMap<String, u64>, String, u64>(
            &mut group,
            map_data.clone(),
            "rustc-hash",
        );
        bench_lookup_hit::<StarshardBenchMap<String, u64>, String, u64>(
            &mut group,
            map_data.clone(),
            "starshard",
        );
        bench_lookup_hit::<StdBenchMap<String, u64>, String, u64>(
            &mut group,
            map_data.clone(),
            "std",
        );
        bench_lookup_hit::<TxMapBenchMap<String, u64>, String, u64>(
            &mut group,
            map_data.clone(),
            "txmap",
        );
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
        bench_lookup_hit::<AhashBenchMap<String, u64>, String, u64>(
            &mut group,
            map_data.clone(),
            "ahash",
        );
        // bench_lookup_hit::<BTreeMapBenchMap<String, u64>, String, u64>(&mut group, map_data.clone(), "btreemap"); // too slow
        bench_lookup_hit::<ConcreadBenchMap<String, u64>, String, u64>(
            &mut group,
            map_data.clone(),
            "concread",
        );
        bench_lookup_hit::<DashMapBenchMap<String, u64>, String, u64>(
            &mut group,
            map_data.clone(),
            "dashmap",
        );
        bench_lookup_hit::<HashbrownBenchMap<String, u64>, String, u64>(
            &mut group,
            map_data.clone(),
            "hashbrown",
        );
        bench_lookup_hit::<ImmutableChunkMapBenchMap<String, u64>, String, u64>(
            &mut group,
            map_data.clone(),
            "immutable-chunkmap",
        );
        bench_lookup_hit::<IndexMapBenchMap<String, u64>, String, u64>(
            &mut group,
            map_data.clone(),
            "indexmap",
        );
        bench_lookup_hit::<RustCHashBenchMap<String, u64>, String, u64>(
            &mut group,
            map_data.clone(),
            "rustc-hash",
        );
        bench_lookup_hit::<StarshardBenchMap<String, u64>, String, u64>(
            &mut group,
            map_data.clone(),
            "starshard",
        );
        bench_lookup_hit::<StdBenchMap<String, u64>, String, u64>(
            &mut group,
            map_data.clone(),
            "std",
        );
        bench_lookup_hit::<TxMapBenchMap<String, u64>, String, u64>(
            &mut group,
            map_data.clone(),
            "txmap",
        );
    }
}

criterion_group!(group, baseline_key_sensitivity);
criterion_main!(group);
