use bench_map::{
    config::*,
    constants::*,
    data::u64_sparse::U64SparseDataGen,
    map_data::MapData,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BTreeMapBenchMap, BenchMapGetCloned, BenchMapMutInsert, BenchMapMutRemove,
        BenchMapNew, DashMapBenchMap, HashbrownBenchMap, ImmutableChunkMapBenchMap,
        IndexMapBenchMap, LeapfrogBenchMap, RustCHashBenchMap, SccBenchMap, StarshardBenchMap,
        StdBenchMap, TxMapBenchMap, horde_benchmap::HordeBenchMap,
    },
    number_formatter::format_n,
    workload::{design::WorkloadDesign, op::WorkloadOp, thread_workload::ThreadWorkload},
};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main,
    measurement::WallTime,
};
use std::hint::black_box;

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
) where
    Map: BenchMapNew<u64, u64>
        + BenchMapMutInsert<u64, u64>
        + BenchMapMutRemove<u64, u64>
        + BenchMapGetCloned<u64, u64>,
{
    group.bench_function(name, move |b| {
        b.iter_batched(
            || map_data.create_map::<Map>(),
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
                "mixed-read-write/{OUT_OF_THE_BOX_GROUP_NAME}/{}/map-size-{}",
                name,
                format_n(entry_count),
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(MIXED_OPS_PER_DESIGN as u64));

            bench::<AhashBenchMap<u64, u64>>(&mut group, &map_data, &workload, "ahash");
            bench::<BTreeMapBenchMap<u64, u64>>(&mut group, &map_data, &workload, "btreemap");
            // bench::<ConcreadBenchMap<u64, u64>>(&mut group, &map_data, &workload, "concread"); // too slow
            bench::<DashMapBenchMap<u64, u64>>(&mut group, &map_data, &workload, "dashmap");
            // bench::<FlurryBenchMap<u64, u64>>(&mut group, &map_data, &workload, "flurry"); // too slow
            bench::<HashbrownBenchMap<u64, u64>>(&mut group, &map_data, &workload, "hashbrown");
            bench::<HordeBenchMap<u64, u64>>(&mut group, &map_data, &workload, "horde");
            bench::<ImmutableChunkMapBenchMap<u64, u64>>(
                &mut group,
                &map_data,
                &workload,
                "immutable-chunkmap",
            );
            bench::<IndexMapBenchMap<u64, u64>>(&mut group, &map_data, &workload, "indexmap");
            bench::<LeapfrogBenchMap<u64, u64>>(&mut group, &map_data, &workload, "leapfrog");
            // bench::<PapayaBenchMap<u64, u64>>(&mut group, &map_data, &workload, "papaya"); // too slow
            bench::<RustCHashBenchMap<u64, u64>>(&mut group, &map_data, &workload, "rustc-hash");
            bench::<SccBenchMap<u64, u64>>(&mut group, &map_data, &workload, "scc");
            bench::<StarshardBenchMap<u64, u64>>(&mut group, &map_data, &workload, "starshard");
            bench::<StdBenchMap<u64, u64>>(&mut group, &map_data, &workload, "std");
            bench::<TxMapBenchMap<u64, u64>>(&mut group, &map_data, &workload, "txmap");
        }
    }
}

criterion_group!(group, mixed_read_write);
criterion_main!(group);
