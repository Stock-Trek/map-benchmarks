use bench_map::{
    config::*,
    constants::SAME_HASHER_GROUP_NAME,
    data::u64_sparse::U64SparseDataGen,
    map_data::MapData,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BenchMapGetCloned, BenchMapMutInsert, BenchMapMutRemove,
        BenchMapNewWithHasher, DashMapBenchMap, HashbrownBenchMap, IndexMapBenchMap,
        StarshardBenchMap, StdBenchMap, TxMapBenchMap, horde_benchmap::HordeBenchMap,
    },
    number_formatter::format_n,
    workload::{design::WorkloadDesign, op::WorkloadOp, thread_workload::ThreadWorkload},
};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main,
    measurement::WallTime,
};
use std::hint::black_box;

type CommonHasher = ahash::RandomState;

fn run_workload<M>(workload: &ThreadWorkload, map: &mut M)
where
    M: BenchMapGetCloned<u64, u64>,
    M: BenchMapMutInsert<u64, u64>,
    M: BenchMapMutRemove<u64, u64>,
{
    for item in &workload.items {
        match item.op {
            WorkloadOp::Lookup => {
                let key = black_box(&item.key);
                black_box(map.get_cloned(key));
            }
            WorkloadOp::Insert => {
                let key = black_box(item.key);
                map.insert(key, 42u64);
            }
            WorkloadOp::Remove => {
                let key = black_box(&item.key);
                black_box(map.remove(key));
            }
        }
    }
}

fn bench<Map>(
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<u64, u64>,
    workload: &ThreadWorkload,
    name: &str,
    hasher: CommonHasher,
) where
    Map: BenchMapNewWithHasher<u64, u64, CommonHasher>
        + BenchMapMutInsert<u64, u64>
        + BenchMapMutRemove<u64, u64>
        + BenchMapGetCloned<u64, u64>,
{
    group.bench_function(name, move |b| {
        b.iter_batched(
            || map_data.create_map_with_hasher::<Map, CommonHasher>(hasher.clone()),
            |mut map| {
                run_workload(workload, &mut map);
            },
            BatchSize::PerIteration,
        );
    });
}

fn mixed_read_write(c: &mut Criterion) {
    let missing_key_count = MIXED_MISSING_KEY_COUNT;
    let sort_keys = false;

    for &entry_count in MIXED_ENTRY_COUNT {
        let existing_key_count = entry_count;

        let map_data = MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            entry_count,
            existing_key_count,
            missing_key_count,
            sort_keys,
        );

        let mut rng = rand::rng();
        let designs: &[(&str, WorkloadDesign)] = &[
            (
                "write-heavy",
                WorkloadDesign::write_heavy(MIXED_OPS_PER_DESIGN),
            ),
            (
                "high-churn",
                WorkloadDesign::high_churn(MIXED_OPS_PER_DESIGN),
            ),
            ("balanced", WorkloadDesign::balanced(MIXED_OPS_PER_DESIGN)),
            (
                "read-heavy",
                WorkloadDesign::read_heavy(MIXED_OPS_PER_DESIGN),
            ),
        ];

        for &(name, design) in designs {
            let workload = ThreadWorkload::new(
                &design,
                map_data.existing_keys(),
                map_data.missing_keys(),
                &mut rng,
            );

            let mut group = c.benchmark_group(format!(
                "{SAME_HASHER_GROUP_NAME}/mixed-read-write/{}-workload/map-size-{}",
                name,
                format_n(entry_count),
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(MIXED_OPS_PER_DESIGN as u64));

            let hasher = CommonHasher::new();

            bench::<AhashBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                &workload,
                "ahash",
                hasher.clone(),
            );
            // bench::<BTreeMapBenchMap<u64, u64, CommonHasher>>(&mut group, &map_data, &workload, "btreemap"); // doesn't allow setting hasher
            // bench::<ConcreadBenchMap<u64, u64, CommonHasher>>(&mut group, &map_data, &workload, "concread"); // doesn't allow setting hasher
            bench::<DashMapBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                &workload,
                "dashmap",
                hasher.clone(),
            );
            bench::<HashbrownBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                &workload,
                "hashbrown",
                hasher.clone(),
            );
            bench::<HordeBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                &workload,
                "horde",
                hasher.clone(),
            );
            // bench::<ImmutableChunkMapBenchMap<u64, u64, CommonHasher>>(&mut group, &map_data, &workload, "immutable-chunkmap"); // doesn't allow setting hasher
            bench::<IndexMapBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                &workload,
                "indexmap",
                hasher.clone(),
            );
            // bench::<RustCHashBenchMap<u64, u64, CommonHasher>>(&mut group, &map_data, &workload, "rustc-hash"); // doesn't allow setting hasher
            bench::<StarshardBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                &workload,
                "starshard",
                hasher.clone(),
            );
            bench::<StdBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                &workload,
                "std",
                hasher.clone(),
            );
            bench::<TxMapBenchMap<u64, u64, CommonHasher>>(
                &mut group,
                &map_data,
                &workload,
                "txmap",
                hasher.clone(),
            );
        }
    }
}

criterion_group!(group, mixed_read_write);
criterion_main!(group);
