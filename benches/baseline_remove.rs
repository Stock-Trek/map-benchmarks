use bench_map::{
    config::*, data::u64_sparse::U64SparseDataGen, map_gen::MapGen, maps::BenchMap,
    thousands_format::format_with_underscores,
};
use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
use std::rc::Rc;

macro_rules! bench_remove {
    ($group:ident, $map_data:expr, $map_type:path, $name:expr) => {
        let map_data = $map_data;
        $group.bench_function($name, move |b| {
            let map_data_ref = &map_data;
            let removal_keys = map_data_ref.existing_keys();
            b.iter_batched(
                move || {
                    let map = map_data_ref.create_map::<$map_type>();
                    let keys_to_remove = removal_keys.clone();
                    (map, keys_to_remove)
                },
                |(mut map, mut keys_to_remove)| {
                    for key in keys_to_remove.drain(..) {
                        let key = std::hint::black_box(key);
                        std::hint::black_box(map.remove(&key));
                    }
                    std::hint::black_box(map)
                },
                BatchSize::PerIteration,
            );
        });
    };
}

fn baseline_remove(c: &mut Criterion) {
    let existing_key_count = 100;
    let missing_key_count = 0;
    let sort_keys = false;
    for entry_count in BASELINE_ENTRY_COUNT {
        let map_data = Rc::new(MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            *entry_count,
            existing_key_count,
            missing_key_count,
            sort_keys,
        ));
        let mut group = c.benchmark_group(format!(
            "baseline/remove/{}",
            format_with_underscores(*entry_count)
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(existing_key_count as u64));
        bench_remove!(group, map_data.clone(), bench_map::maps::ahash_benchmap::AhashBenchMap<_, _>, "ahash");
        bench_remove!(group, map_data.clone(), bench_map::maps::btreemap_benchmap::BTreeMapBenchMap<_, _>, "btreemap");
        bench_remove!(group, map_data.clone(), bench_map::maps::concread_benchmap::ConcreadBenchMap<_, _>, "concread");
        bench_remove!(group, map_data.clone(), bench_map::maps::dashmap_benchmap::DashMapBenchMap<_, _>, "dashmap");
        bench_remove!(group, map_data.clone(), bench_map::maps::hashbrown_benchmap::HashbrownBenchMap<_, _>, "hashbrown");
        bench_remove!(group, map_data.clone(), bench_map::maps::immutable_chunkmap_benchmap::ImmutableChunkMapBenchMap<_, _>, "immutable-chunkmap");
        bench_remove!(group, map_data.clone(), bench_map::maps::indexmap_benchmap::IndexMapBenchMap<_, _>, "indexmap");
        bench_remove!(group, map_data.clone(), bench_map::maps::rustc_hash_benchmap::RustCHashBenchMap<_, _>, "rustc-hash");
        bench_remove!(group, map_data.clone(), bench_map::maps::starshard_benchmap::StarshardBenchMap<_, _>, "starshard");
        bench_remove!(group, map_data.clone(), bench_map::maps::std_benchmap::StdBenchMap<_, _>, "std");
        bench_remove!(group, map_data.clone(), bench_map::maps::txmap_benchmap::TxMapBenchMap<_, _>, "txmap");
    }
}

criterion_group!(group, baseline_remove);
criterion_main!(group);
