// How does it handle realistic serial workloads? Tests mixed read/write/remove workloads in a single thread.
use bench_map::{
    config::*,
    constants::*,
    data::u64_sparse::U64SparseDataGen,
    expand_bench_with_map_data_and_workload,
    map_data::MapData,
    map_gen::MapGen,
    maps::*,
    number_formatter::format_n,
    workload::{
        design::WorkloadDesign,
        op::WorkloadOp,
        thread_workload::{KeyDistribution, ThreadWorkload},
    },
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
    name: &str,
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<u64, u64>,
    workload: &ThreadWorkload,
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

fn workload_serial(c: &mut Criterion) {
    let missing_key_count = DEFAULT_OP_COUNT;
    let sort_keys = false;

    for &entry_count in DEFAULT_ENTRY_COUNTS {
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
            ("write-heavy", WorkloadDesign::write_heavy(DEFAULT_OP_COUNT)),
            ("balanced", WorkloadDesign::balanced(DEFAULT_OP_COUNT)),
            ("read-heavy", WorkloadDesign::read_heavy(DEFAULT_OP_COUNT)),
        ];

        for &(name, design) in designs {
            let workload = KeyDistribution::Uniform.thread_workload(
                &design,
                map_data.existing_keys(),
                map_data.missing_keys(),
                &mut rng,
            );

            let mut group = c.benchmark_group(format!(
                "workload/{OUT_OF_THE_BOX_GROUP_NAME}/{}/map-size-{}/threads-1",
                name,
                format_n(entry_count),
            ));
            group.warm_up_time(WARM_UP_TIME);
            group.measurement_time(MEASUREMENT_TIME);
            group.throughput(Throughput::Elements(DEFAULT_OP_COUNT as u64));

            expand_bench_with_map_data_and_workload!(bench, &mut group, &map_data, &workload,
                AhashBenchMap<u64, u64>,
                BTreeMapBenchMap<u64, u64>,
                // ConcreadBenchMap<u64, u64>, // too slow
                ConcurrentMapBenchMap<u64, u64>,
                CrossbeamSkiplistBenchMap<u64, u64>,
                DashMapBenchMap<u64, u64>,
                // FlurryBenchMap<u64, u64>, // too slow
                HashbrownBenchMap<u64, u64>,
                HashlinkBenchMap<u64, u64>,
                HordeBenchMap<u64, u64>,
                ImmutableChunkMapBenchMap<u64, u64>,
                ImblBenchMap<u64, u64>,
                IndexMapBenchMap<u64, u64>,
                LeapfrogBenchMap<u64, u64>,
                PapayaBenchMap<u64, u64>,
                RpdsHashTrieMapBenchMap<u64, u64>,
                RustCHashBenchMap<u64, u64>,
                SccBenchMap<u64, u64>,
                StarshardBenchMap<u64, u64>,
                StdBenchMap<u64, u64>,
                TxMapBenchMap<u64, u64>,
            );
        }
    }
}

criterion_group!(group, workload_serial);
criterion_main!(group);
