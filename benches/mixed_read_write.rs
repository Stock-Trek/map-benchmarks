use bench_map::{
    config::*,
    data::u64_sparse::U64SparseDataGen,
    map_data::MapData,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BenchMapGetCloned, BenchMapMutInsert, BenchMapMutRemove, BenchMapNew,
        ConcreadBenchMap, DashMapBenchMap, HashbrownBenchMap, ImmutableChunkMapBenchMap,
        IndexMapBenchMap, RustCHashBenchMap, StarshardBenchMap, StdBenchMap, TxMapBenchMap,
    },
    thousands_format::format_with_underscores,
    workload::{design::WorkloadDesign, op::WorkloadOp, thread_workload::ThreadWorkload},
};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main,
    measurement::WallTime,
};
use std::rc::Rc;

fn run_workload<M>(workload: &ThreadWorkload, map: &mut M)
where
    M: BenchMapGetCloned<u64, u64>,
    M: BenchMapMutInsert<u64, u64>,
    M: BenchMapMutRemove<u64, u64>,
{
    for item in &workload.items {
        match item.op {
            WorkloadOp::Lookup => {
                let key = std::hint::black_box(&item.key);
                std::hint::black_box(map.get_cloned(key));
            }
            WorkloadOp::Insert => {
                let key = std::hint::black_box(item.key);
                map.insert(key, 42u64);
            }
            WorkloadOp::Remove => {
                let key = std::hint::black_box(&item.key);
                std::hint::black_box(map.remove(key));
            }
        }
    }
}

fn bench_mixed<Map>(
    group: &mut BenchmarkGroup<WallTime>,
    map_data: Rc<MapData<u64, u64>>,
    workload: ThreadWorkload,
    name: &str,
) where
    Map: BenchMapNew<u64, u64>
        + BenchMapMutInsert<u64, u64>
        + BenchMapMutRemove<u64, u64>
        + BenchMapGetCloned<u64, u64>,
{
    group.bench_function(name, move |b| {
        let map_data = map_data.clone();
        let workload = workload.clone();
        b.iter_batched(
            move || {
                let map = map_data.create_map::<Map>();
                (map, workload.clone())
            },
            |(mut map, workload)| {
                run_workload(&workload, &mut map);
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

        let map_data = Rc::new(MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            entry_count,
            existing_key_count,
            missing_key_count,
            sort_keys,
        ));

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
                "mixed-read-write/{}-workload/map-size-{}",
                name,
                format_with_underscores(entry_count),
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(MIXED_OPS_PER_DESIGN as u64));

            bench_mixed::<AhashBenchMap<u64, u64>>(
                &mut group,
                map_data.clone(),
                workload.clone(),
                "ahash",
            );
            // bench_mixed::<BTreeMapBenchMap<u64, u64>>(&mut group, map_data.clone(), workload.clone(), "btreemap"); too slow
            bench_mixed::<ConcreadBenchMap<u64, u64>>(
                &mut group,
                map_data.clone(),
                workload.clone(),
                "concread",
            );
            bench_mixed::<DashMapBenchMap<u64, u64>>(
                &mut group,
                map_data.clone(),
                workload.clone(),
                "dashmap",
            );
            bench_mixed::<HashbrownBenchMap<u64, u64>>(
                &mut group,
                map_data.clone(),
                workload.clone(),
                "hashbrown",
            );
            bench_mixed::<ImmutableChunkMapBenchMap<u64, u64>>(
                &mut group,
                map_data.clone(),
                workload.clone(),
                "immutable-chunkmap",
            );
            bench_mixed::<IndexMapBenchMap<u64, u64>>(
                &mut group,
                map_data.clone(),
                workload.clone(),
                "indexmap",
            );
            bench_mixed::<RustCHashBenchMap<u64, u64>>(
                &mut group,
                map_data.clone(),
                workload.clone(),
                "rustc-hash",
            );
            bench_mixed::<StarshardBenchMap<u64, u64>>(
                &mut group,
                map_data.clone(),
                workload.clone(),
                "starshard",
            );
            bench_mixed::<StdBenchMap<u64, u64>>(
                &mut group,
                map_data.clone(),
                workload.clone(),
                "std",
            );
            bench_mixed::<TxMapBenchMap<u64, u64>>(
                &mut group,
                map_data.clone(),
                workload.clone(),
                "txmap",
            );
        }
    }
}

criterion_group!(group, mixed_read_write);
criterion_main!(group);
