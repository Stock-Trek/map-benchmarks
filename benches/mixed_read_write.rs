use bench_map::{
    config::*,
    data::u64_sparse::U64SparseDataGen,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BTreeMapBenchMap, BenchMapGetCloned, BenchMapMutInsert, BenchMapMutRemove,
        ConcreadBenchMap, DashMapBenchMap, HashbrownBenchMap, ImmutableChunkMapBenchMap,
        IndexMapBenchMap, RustCHashBenchMap, StarshardBenchMap, StdBenchMap, TxMapBenchMap,
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
                let _ = map.get_cloned(&item.key);
            }
            WorkloadOp::Insert => {
                map.insert(item.key, 42u64);
            }
            WorkloadOp::Remove => {
                map.remove(&item.key);
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
    let sort_keys = false;

    for &map_size in BASELINE_ENTRY_COUNT {
        let map_data = Rc::new(MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            0,
            0,
            MIXED_MISSING_KEY_COUNT,
            sort_keys,
        ));

        let map_data_with_entries = Rc::new(MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            map_size,
            map_size,
            MIXED_MISSING_KEY_COUNT,
            sort_keys,
        ));

        let mut rng = rand::rng();
        let designs: &[(WorkloadDesign, &str)] = &[
            (
                WorkloadDesign::write_heavy(MIXED_OPS_PER_DESIGN),
                "write_heavy",
            ),
            (
                WorkloadDesign::high_churn(MIXED_OPS_PER_DESIGN),
                "high_churn",
            ),
            (WorkloadDesign::balanced(MIXED_OPS_PER_DESIGN), "balanced"),
            (
                WorkloadDesign::read_heavy(MIXED_OPS_PER_DESIGN),
                "read_heavy",
            ),
        ];

        for &(design, name) in designs {
            let workload = ThreadWorkload::new(
                &design,
                map_data_with_entries.existing_keys(),
                map_data.missing_keys(),
                &mut rng,
            );

            let mut group = c.benchmark_group(format!(
                "mixed-read-write/{}/{}/{}",
                name,
                format_with_underscores(map_size),
                MIXED_OPS_PER_DESIGN
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(MIXED_OPS_PER_DESIGN as u64));

            bench_mixed!(
                group,
                map_data_with_entries.clone(),
                workload.clone(),
                AhashBenchMap<_, _>,
                "ahash"
            );
            bench_mixed!(
                group,
                map_data_with_entries.clone(),
                workload.clone(),
                BTreeMapBenchMap<_, _>,
                "btreemap"
            );
            bench_mixed!(
                group,
                map_data_with_entries.clone(),
                workload.clone(),
                ConcreadBenchMap<_, _>,
                "concread"
            );
            bench_mixed!(
                group,
                map_data_with_entries.clone(),
                workload.clone(),
                DashMapBenchMap<_, _>,
                "dashmap"
            );
            bench_mixed!(
                group,
                map_data_with_entries.clone(),
                workload.clone(),
                HashbrownBenchMap<_, _>,
                "hashbrown"
            );
            bench_mixed!(
                group,
                map_data_with_entries.clone(),
                workload.clone(),
                ImmutableChunkMapBenchMap<_, _>,
                "immutable-chunkmap"
            );
            bench_mixed!(
                group,
                map_data_with_entries.clone(),
                workload.clone(),
                IndexMapBenchMap<_, _>,
                "indexmap"
            );
            bench_mixed!(
                group,
                map_data_with_entries.clone(),
                workload.clone(),
                RustCHashBenchMap<_, _>,
                "rustc-hash"
            );
            bench_mixed!(
                group,
                map_data_with_entries.clone(),
                workload.clone(),
                StarshardBenchMap<_, _>,
                "starshard"
            );
            bench_mixed!(
                group,
                map_data_with_entries.clone(),
                workload.clone(),
                StdBenchMap<_, _>,
                "std"
            );
            bench_mixed!(
                group,
                map_data_with_entries.clone(),
                workload.clone(),
                TxMapBenchMap<_, _>,
                "txmap"
            );
        }
    }
}

criterion_group!(group, mixed_read_write);
criterion_main!(group);
