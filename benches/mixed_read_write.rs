use bench_map::{
    config::*,
    data::u64_sparse::U64SparseDataGen,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BenchMapGetCloned, BenchMapMutInsert, BenchMapMutRemove, ConcreadBenchMap,
        DashMapBenchMap, HashbrownBenchMap, ImmutableChunkMapBenchMap, IndexMapBenchMap,
        RustCHashBenchMap, StarshardBenchMap, StdBenchMap, TxMapBenchMap,
    },
    thousands_format::format_with_underscores,
    workload::{design::WorkloadDesign, op::WorkloadOp, thread_workload::ThreadWorkload},
};
use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
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

macro_rules! bench_mixed {
    ($group:ident, $map_data:expr, $workload:expr, $map_type:path, $name:expr) => {
        let map_data = $map_data.clone();
        let workload = $workload.clone();
        $group.bench_function($name, move |b| {
            let map_data = map_data.clone();
            let workload = workload.clone();
            b.iter_batched(
                move || {
                    let map = map_data.create_map::<$map_type>();
                    (map, workload.clone())
                },
                |(mut map, workload)| {
                    run_workload(&workload, &mut map);
                },
                BatchSize::PerIteration,
            );
        });
    };
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

            bench_mixed!(group, map_data.clone(),workload.clone(), AhashBenchMap<_, _>, "ahash");
            // bench_mixed!(group, map_data.clone(), workload.clone(), BTreeMapBenchMap<_, _>, "btreemap"); too slow
            bench_mixed!(group, map_data.clone(), workload.clone(), ConcreadBenchMap<_, _>, "concread");
            bench_mixed!(group, map_data.clone(), workload.clone(), DashMapBenchMap<_, _>, "dashmap");
            bench_mixed!(group, map_data.clone(), workload.clone(), HashbrownBenchMap<_, _>, "hashbrown");
            bench_mixed!(group, map_data.clone(), workload.clone(), ImmutableChunkMapBenchMap<_, _>, "immutable-chunkmap");
            bench_mixed!(group, map_data.clone(), workload.clone(), IndexMapBenchMap<_, _>, "indexmap");
            bench_mixed!(group, map_data.clone(), workload.clone(), RustCHashBenchMap<_, _>, "rustc-hash");
            bench_mixed!(group, map_data.clone(), workload.clone(), StarshardBenchMap<_, _>, "starshard");
            bench_mixed!(group, map_data.clone(), workload.clone(), StdBenchMap<_, _>, "std");
            bench_mixed!(group, map_data.clone(), workload.clone(), TxMapBenchMap<_, _>, "txmap");
        }
    }
}

criterion_group!(group, mixed_read_write);
criterion_main!(group);
