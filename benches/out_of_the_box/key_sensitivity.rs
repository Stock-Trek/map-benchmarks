use bench_map::{
    config::*,
    contents::OUT_OF_THE_BOX_GROUP_NAME,
    data::{
        byte_array::ByteArrayDataGen, string::StringDataGen, u64_dense::U64DenseDataGen,
        u64_sparse::U64SparseDataGen, u64_zipfian::U64ZipfianDataGen, uuid_v4::UuidV4DataGen,
    },
    map_data::MapData,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BTreeMapBenchMap, BenchMapGetCloned, BenchMapMutInsert, BenchMapNew,
        ConcreadBenchMap, DashMapBenchMap, HashbrownBenchMap, ImmutableChunkMapBenchMap,
        IndexMapBenchMap, RustCHashBenchMap, StarshardBenchMap, StdBenchMap, TxMapBenchMap,
        horde_benchmap::HordeBenchMap,
    },
};
use criterion::{
    BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main, measurement::WallTime,
};
use std::{hash::Hash, hint::black_box, rc::Rc};
use uuid::Uuid;

fn bench<Map, K, V>(group: &mut BenchmarkGroup<WallTime>, map_data: &MapData<K, V>, name: &str)
where
    Map: BenchMapNew<K, V> + BenchMapMutInsert<K, V> + BenchMapGetCloned<K, V>,
    K: Clone + Hash + Eq,
    V: Clone,
{
    group.bench_function(name, move |b| {
        let map = map_data.create_map::<Map>();
        let keys = map_data.existing_keys();
        b.iter(|| {
            for key in keys {
                let key = black_box(key);
                black_box(map.get_cloned(key));
            }
        });
    });
}

fn key_sensitivity(c: &mut Criterion) {
    let entry_count = KEY_SENSITIVITY_ENTRY_COUNT;
    let existing_key_count: u64 = 100;
    let missing_key_count: u64 = 0;
    let sort_keys = false;
    // u64 keys
    {
        let map_data = MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            entry_count,
            existing_key_count as usize,
            missing_key_count as usize,
            sort_keys,
        );
        let mut group =
            c.benchmark_group(format!("{OUT_OF_THE_BOX_GROUP_NAME}/key-sensitivity/u64"));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));

        bench::<AhashBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "ahash");
        bench::<BTreeMapBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "btreemap");
        bench::<ConcreadBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "concread");
        bench::<DashMapBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "dashmap");
        bench::<HashbrownBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "hashbrown");
        bench::<ImmutableChunkMapBenchMap<u64, u64>, u64, u64>(
            &mut group,
            &map_data,
            "immutable-chunkmap",
        );
        bench::<IndexMapBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "indexmap");
        bench::<RustCHashBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "rustc-hash");
        bench::<StarshardBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "starshard");
        bench::<StdBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "std");
        bench::<TxMapBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "txmap");
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
        let mut group =
            c.benchmark_group(format!("{OUT_OF_THE_BOX_GROUP_NAME}/key-sensitivity/UUID"));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));

        bench::<AhashBenchMap<Uuid, u64>, Uuid, u64>(&mut group, &map_data, "ahash");
        bench::<BTreeMapBenchMap<Uuid, u64>, Uuid, u64>(&mut group, &map_data, "btreemap");
        bench::<ConcreadBenchMap<Uuid, u64>, Uuid, u64>(&mut group, &map_data, "concread");
        bench::<DashMapBenchMap<Uuid, u64>, Uuid, u64>(&mut group, &map_data, "dashmap");
        bench::<HashbrownBenchMap<Uuid, u64>, Uuid, u64>(&mut group, &map_data, "hashbrown");
        bench::<ImmutableChunkMapBenchMap<Uuid, u64>, Uuid, u64>(
            &mut group,
            &map_data,
            "immutable-chunkmap",
        );
        bench::<IndexMapBenchMap<Uuid, u64>, Uuid, u64>(&mut group, &map_data, "indexmap");
        bench::<RustCHashBenchMap<Uuid, u64>, Uuid, u64>(&mut group, &map_data, "rustc-hash");
        bench::<StarshardBenchMap<Uuid, u64>, Uuid, u64>(&mut group, &map_data, "starshard");
        bench::<StdBenchMap<Uuid, u64>, Uuid, u64>(&mut group, &map_data, "std");
        bench::<TxMapBenchMap<Uuid, u64>, Uuid, u64>(&mut group, &map_data, "txmap");
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
        let mut group = c.benchmark_group(format!(
            "{OUT_OF_THE_BOX_GROUP_NAME}/key-sensitivity/Byte<32>"
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));
        bench::<AhashBenchMap<[u8; 32], u64>, [u8; 32], u64>(&mut group, &map_data, "ahash");
        bench::<BTreeMapBenchMap<[u8; 32], u64>, [u8; 32], u64>(&mut group, &map_data, "btreemap");
        bench::<ConcreadBenchMap<[u8; 32], u64>, [u8; 32], u64>(&mut group, &map_data, "concread");
        bench::<DashMapBenchMap<[u8; 32], u64>, [u8; 32], u64>(&mut group, &map_data, "dashmap");
        bench::<HashbrownBenchMap<[u8; 32], u64>, [u8; 32], u64>(
            &mut group,
            &map_data,
            "hashbrown",
        );
        bench::<ImmutableChunkMapBenchMap<[u8; 32], u64>, [u8; 32], u64>(
            &mut group,
            &map_data,
            "immutable-chunkmap",
        );
        bench::<IndexMapBenchMap<[u8; 32], u64>, [u8; 32], u64>(&mut group, &map_data, "indexmap");
        bench::<RustCHashBenchMap<[u8; 32], u64>, [u8; 32], u64>(
            &mut group,
            &map_data,
            "rustc-hash",
        );
        bench::<StarshardBenchMap<[u8; 32], u64>, [u8; 32], u64>(
            &mut group,
            &map_data,
            "starshard",
        );
        bench::<StdBenchMap<[u8; 32], u64>, [u8; 32], u64>(&mut group, &map_data, "std");
        bench::<TxMapBenchMap<[u8; 32], u64>, [u8; 32], u64>(&mut group, &map_data, "txmap");
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
        let mut group = c.benchmark_group(format!(
            "{OUT_OF_THE_BOX_GROUP_NAME}/key-sensitivity/String<16>"
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));
        bench::<AhashBenchMap<String, u64>, String, u64>(&mut group, &map_data, "ahash");
        bench::<BTreeMapBenchMap<String, u64>, String, u64>(&mut group, &map_data, "btreemap");
        bench::<ConcreadBenchMap<String, u64>, String, u64>(&mut group, &map_data, "concread");
        bench::<DashMapBenchMap<String, u64>, String, u64>(&mut group, &map_data, "dashmap");
        bench::<HashbrownBenchMap<String, u64>, String, u64>(&mut group, &map_data, "hashbrown");
        bench::<ImmutableChunkMapBenchMap<String, u64>, String, u64>(
            &mut group,
            &map_data,
            "immutable-chunkmap",
        );
        bench::<IndexMapBenchMap<String, u64>, String, u64>(&mut group, &map_data, "indexmap");
        bench::<RustCHashBenchMap<String, u64>, String, u64>(&mut group, &map_data, "rustc-hash");
        bench::<StarshardBenchMap<String, u64>, String, u64>(&mut group, &map_data, "starshard");
        bench::<StdBenchMap<String, u64>, String, u64>(&mut group, &map_data, "std");
        bench::<TxMapBenchMap<String, u64>, String, u64>(&mut group, &map_data, "txmap");
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
        let mut group = c.benchmark_group(format!(
            "{OUT_OF_THE_BOX_GROUP_NAME}/key-sensitivity/String<128>"
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));
        bench::<AhashBenchMap<String, u64>, String, u64>(&mut group, &map_data, "ahash");
        bench::<BTreeMapBenchMap<String, u64>, String, u64>(&mut group, &map_data, "btreemap");
        bench::<ConcreadBenchMap<String, u64>, String, u64>(&mut group, &map_data, "concread");
        bench::<DashMapBenchMap<String, u64>, String, u64>(&mut group, &map_data, "dashmap");
        bench::<HashbrownBenchMap<String, u64>, String, u64>(&mut group, &map_data, "hashbrown");
        bench::<HordeBenchMap<String, u64>, String, u64>(&mut group, &map_data, "horde");
        bench::<ImmutableChunkMapBenchMap<String, u64>, String, u64>(
            &mut group,
            &map_data,
            "immutable-chunkmap",
        );
        bench::<IndexMapBenchMap<String, u64>, String, u64>(&mut group, &map_data, "indexmap");
        bench::<RustCHashBenchMap<String, u64>, String, u64>(&mut group, &map_data, "rustc-hash");
        bench::<StarshardBenchMap<String, u64>, String, u64>(&mut group, &map_data, "starshard");
        bench::<StdBenchMap<String, u64>, String, u64>(&mut group, &map_data, "std");
        bench::<TxMapBenchMap<String, u64>, String, u64>(&mut group, &map_data, "txmap");
    }
    // Adversarial keys (dense)
    {
        let map_data = Rc::new(MapGen::generate(
            U64DenseDataGen,
            U64SparseDataGen,
            entry_count,
            existing_key_count as usize,
            missing_key_count as usize,
            sort_keys,
        ));
        let mut group = c.benchmark_group(format!(
            "{OUT_OF_THE_BOX_GROUP_NAME}/key-sensitivity/u64-dense"
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));
        bench::<AhashBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "ahash");
        bench::<BTreeMapBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "btreemap");
        bench::<ConcreadBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "concread");
        bench::<DashMapBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "dashmap");
        bench::<HashbrownBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "hashbrown");
        bench::<HordeBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "horde");
        bench::<ImmutableChunkMapBenchMap<u64, u64>, u64, u64>(
            &mut group,
            &map_data,
            "immutable-chunkmap",
        );
        bench::<IndexMapBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "indexmap");
        bench::<RustCHashBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "rustc-hash");
        bench::<StarshardBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "starshard");
        bench::<StdBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "std");
        bench::<TxMapBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "txmap");
    }
    // Adversarial keys (zipfian)
    {
        let map_data = Rc::new(MapGen::generate(
            U64ZipfianDataGen {
                num_items: (entry_count * 1_000) as u64,
                exponent: 1.0,
            },
            U64SparseDataGen,
            entry_count,
            existing_key_count as usize,
            missing_key_count as usize,
            sort_keys,
        ));
        let mut group = c.benchmark_group(format!(
            "{OUT_OF_THE_BOX_GROUP_NAME}/key-sensitivity/u64-zipfian"
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count));
        bench::<AhashBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "ahash");
        bench::<BTreeMapBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "btreemap");
        bench::<ConcreadBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "concread");
        bench::<DashMapBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "dashmap");
        bench::<HashbrownBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "hashbrown");
        bench::<HordeBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "horde");
        bench::<ImmutableChunkMapBenchMap<u64, u64>, u64, u64>(
            &mut group,
            &map_data,
            "immutable-chunkmap",
        );
        bench::<IndexMapBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "indexmap");
        bench::<RustCHashBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "rustc-hash");
        bench::<StarshardBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "starshard");
        bench::<StdBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "std");
        bench::<TxMapBenchMap<u64, u64>, u64, u64>(&mut group, &map_data, "txmap");
    }
}

criterion_group!(group, key_sensitivity);
criterion_main!(group);
