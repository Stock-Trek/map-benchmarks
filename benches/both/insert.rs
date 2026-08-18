use bench_map::{
    config::*,
    data::u64_sparse::U64SparseDataGen,
    map_data::MapData,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BenchMapMutInsert, BenchMapNew, BenchMapNewWithHasher, DashMapBenchMap,
        HashbrownBenchMap, ImmutableChunkMapBenchMap, IndexMapBenchMap, RustCHashBenchMap,
        StarshardBenchMap, StdBenchMap, TxMapBenchMap, horde_benchmap::HordeBenchMap,
    },
    number_formatter::format_n,
};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main,
    measurement::WallTime,
};
use std::hint::black_box;

/// The hasher shared by every map implementation that supports a custom hasher,
/// so map implementations are compared against each other on a level playing
/// field rather than each using its own default hasher.
type CommonHasher = ahash::RandomState;

fn bench<Map, H>(
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<u64, u64>,
    name: &str,
    hasher: H,
) where
    Map: BenchMapNewWithHasher<u64, u64, H> + BenchMapMutInsert<u64, u64>,
    H: std::hash::BuildHasher + Clone,
{
    group.bench_function(name, move |b| {
        let map_data_ref = &map_data;
        let hasher = hasher.clone();
        b.iter_batched(
            move || {
                let map = map_data_ref.create_map_with_hasher::<Map, H>(hasher.clone());
                let keys = map_data_ref.missing_keys().clone();
                (map, keys)
            },
            |(mut map, mut keys)| {
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

fn bench_default<Map>(
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<u64, u64>,
    name: &str,
) where
    Map: BenchMapNew<u64, u64> + BenchMapMutInsert<u64, u64>,
{
    group.bench_function(name, move |b| {
        let map_data_ref = &map_data;
        b.iter_batched(
            move || {
                let map = map_data_ref.create_map::<Map>();
                let keys = map_data_ref.missing_keys().clone();
                (map, keys)
            },
            |(mut map, mut keys)| {
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

fn insert(c: &mut Criterion) {
    let entry_count = 0;
    let existing_key_count = 0;
    let sort_keys = false;
    for missing_key_count in OUT_OF_THE_BOX_ENTRY_COUNT {
        let map_data = MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            entry_count,
            existing_key_count,
            *missing_key_count,
            sort_keys,
        );

        // Default implementation: each map uses its own default hasher
        {
            let mut group = c.benchmark_group(format!(
                "both/insert/default/map-size-{}",
                format_n(*missing_key_count)
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(*missing_key_count as u64));

            bench_default::<AhashBenchMap<u64, u64>>(&mut group, &map_data, "ahash");
            // bench_default::<BTreeMapBenchMap<u64, u64>>(&mut group, &map_data, "btreemap"); // too slow
            // bench_default::<ConcreadBenchMap<u64, u64>>(&mut group, &map_data, "concread"); // too slow
            bench_default::<DashMapBenchMap<u64, u64>>(&mut group, &map_data, "dashmap");
            bench_default::<HashbrownBenchMap<u64, u64>>(&mut group, &map_data, "hashbrown");
            bench_default::<HordeBenchMap<u64, u64>>(&mut group, &map_data, "horde");
            bench_default::<ImmutableChunkMapBenchMap<u64, u64>>(
                &mut group,
                &map_data,
                "immutable-chunkmap",
            );
            bench_default::<IndexMapBenchMap<u64, u64>>(&mut group, &map_data, "indexmap");
            bench_default::<RustCHashBenchMap<u64, u64>>(&mut group, &map_data, "rustc-hash");
            bench_default::<StarshardBenchMap<u64, u64>>(&mut group, &map_data, "starshard");
            bench_default::<StdBenchMap<u64, u64>>(&mut group, &map_data, "std");
            bench_default::<TxMapBenchMap<u64, u64>>(&mut group, &map_data, "txmap");
        }

        // Same hasher: every map that supports a custom hasher uses the shared
        // CommonHasher so implementations are compared on a level playing field
        let hasher = CommonHasher::new();
        let mut group = c.benchmark_group(format!(
            "both/insert/same-hasher/map-size-{}",
            format_n(*missing_key_count)
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(*missing_key_count as u64));

        bench::<AhashBenchMap<u64, u64, CommonHasher>, CommonHasher>(
            &mut group,
            &map_data,
            "ahash",
            hasher.clone(),
        );
        // bench_default::<BTreeMapBenchMap<u64, u64>>(&mut group, &map_data, "btreemap"); // too slow, no custom hasher
        // bench_default::<ConcreadBenchMap<u64, u64>>(&mut group, &map_data, "concread"); // too slow, no custom hasher
        bench::<DashMapBenchMap<u64, u64, CommonHasher>, CommonHasher>(
            &mut group,
            &map_data,
            "dashmap",
            hasher.clone(),
        );
        bench::<HashbrownBenchMap<u64, u64, CommonHasher>, CommonHasher>(
            &mut group,
            &map_data,
            "hashbrown",
            hasher.clone(),
        );
        bench::<HordeBenchMap<u64, u64, CommonHasher>, CommonHasher>(
            &mut group,
            &map_data,
            "horde",
            hasher.clone(),
        );
        bench_default::<ImmutableChunkMapBenchMap<u64, u64>>(
            &mut group,
            &map_data,
            "immutable-chunkmap",
        ); // no custom hasher
        bench::<IndexMapBenchMap<u64, u64, CommonHasher>, CommonHasher>(
            &mut group,
            &map_data,
            "indexmap",
            hasher.clone(),
        );
        bench_default::<RustCHashBenchMap<u64, u64>>(&mut group, &map_data, "rustc-hash"); // no custom hasher
        bench::<StarshardBenchMap<u64, u64, CommonHasher>, CommonHasher>(
            &mut group,
            &map_data,
            "starshard",
            hasher.clone(),
        );
        bench::<StdBenchMap<u64, u64, CommonHasher>, CommonHasher>(
            &mut group,
            &map_data,
            "std",
            hasher.clone(),
        );
        bench::<TxMapBenchMap<u64, u64, CommonHasher>, CommonHasher>(
            &mut group,
            &map_data,
            "txmap",
            hasher.clone(),
        );
    }
}

criterion_group!(group, insert);
criterion_main!(group);
