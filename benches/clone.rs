// How fast is it to clone a map? Tests cloning a fully populated map and cloning-then-writing to the copy.
use bench_map::{
    config::*, constants::*, data::u64_sparse::U64SparseDataGen, expand_bench_with_map_data,
    map_data::MapData, map_gen::MapGen, maps::*, number_formatter::format_n,
};
use criterion::{
    BenchmarkGroup, Criterion, Throughput, criterion_group, criterion_main, measurement::WallTime,
};
use std::hint::black_box;

fn bench_clone<Map>(name: &str, group: &mut BenchmarkGroup<WallTime>, map_data: &MapData<u64, u64>)
where
    Map: BenchMapNew<u64, u64> + BenchMapMutInsert<u64, u64> + BenchMapClone<u64, u64>,
{
    group.bench_function(name, move |b| {
        let map = map_data.create_map::<Map>();
        b.iter(|| {
            black_box(map.clone_map());
        });
    });
}

fn bench_clone_then_write<Map>(
    name: &str,
    group: &mut BenchmarkGroup<WallTime>,
    map_data: &MapData<u64, u64>,
) where
    Map: BenchMapNew<u64, u64> + BenchMapMutInsert<u64, u64> + BenchMapClone<u64, u64>,
{
    group.bench_function(name, move |b| {
        let map = map_data.create_map::<Map>();
        let keys = map_data.missing_keys().clone();
        b.iter(|| {
            let mut cloned = map.clone_map();
            for key in &keys {
                let key = black_box(*key);
                cloned.insert(key, 42);
            }
            black_box(cloned);
        });
    });
}

fn clone(c: &mut Criterion) {
    let existing_key_count = 0;
    let missing_key_count = 0;
    let sort_keys = false;
    for entry_count in DEFAULT_ENTRY_COUNTS {
        let map_data = MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            *entry_count,
            existing_key_count,
            missing_key_count,
            sort_keys,
        );
        let mut group = c.benchmark_group(format!(
            "clone/{OUT_OF_THE_BOX_GROUP_NAME}/map-size-{}",
            format_n(*entry_count)
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(*entry_count as u64));

        expand_bench_with_map_data!(bench_clone, &mut group, &map_data,
            AhashBenchMap<u64, u64>,
            BTreeMapBenchMap<u64, u64>,
            // ConcreadBenchMap<u64, u64>, // doesn't implement Clone
            ConcurrentMapBenchMap<u64, u64>,
            // CrossbeamSkiplistBenchMap<u64, u64>, // doesn't implement Clone
            DashMapBenchMap<u64, u64>,
            // FlurryBenchMap<u64, u64>, // too slow
            HashbrownBenchMap<u64, u64>,
            HashlinkBenchMap<u64, u64>,
            HordeBenchMap<u64, u64>,
            ImmutableChunkMapBenchMap<u64, u64>,
            ImblBenchMap<u64, u64>,
            IndexMapBenchMap<u64, u64>,
            // LeapfrogBenchMap<u64, u64>, // doesn't implement Clone
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

fn clone_then_write(c: &mut Criterion) {
    let existing_key_count = 0;
    let sort_keys = false;
    for entry_count in DEFAULT_ENTRY_COUNTS {
        let missing_key_count = *entry_count / 10;
        let map_data = MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            *entry_count,
            existing_key_count,
            missing_key_count,
            sort_keys,
        );
        let mut group = c.benchmark_group(format!(
            "clone-then-write/{OUT_OF_THE_BOX_GROUP_NAME}/map-size-{}",
            format_n(*entry_count)
        ));
        group.warm_up_time(WARM_UP_TIME);
        group.measurement_time(MEASUREMENT_TIME);
        group.throughput(Throughput::Elements(*entry_count as u64));

        expand_bench_with_map_data!(bench_clone_then_write, &mut group, &map_data,
            AhashBenchMap<u64, u64>,
            BTreeMapBenchMap<u64, u64>,
            // ConcreadBenchMap<u64, u64>, // doesn't implement Clone
            ConcurrentMapBenchMap<u64, u64>,
            // CrossbeamSkiplistBenchMap<u64, u64>, // doesn't implement Clone
            DashMapBenchMap<u64, u64>,
            // FlurryBenchMap<u64, u64>, // too slow
            HashbrownBenchMap<u64, u64>,
            HashlinkBenchMap<u64, u64>,
            HordeBenchMap<u64, u64>,
            ImmutableChunkMapBenchMap<u64, u64>,
            ImblBenchMap<u64, u64>,
            IndexMapBenchMap<u64, u64>,
            // LeapfrogBenchMap<u64, u64>, // doesn't implement Clone
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

criterion_group!(group, clone, clone_then_write);
criterion_main!(group);
