use bench_map::{
    data::u64_sparse::U64SparseDataGen,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BTreeMapBenchMap, BenchMapGetCloned, BenchMapInsert, BenchMapMutInsert,
        BenchMapMutRemove, BenchMapNew, BenchMapRemove, ConcreadBenchMap, DashMapBenchMap,
        HashbrownBenchMap, ImmutableChunkMapBenchMap, IndexMapBenchMap, RustCHashBenchMap,
        StarshardBenchMap, StdBenchMap, TxMapBenchMap,
    },
};
use std::rc::Rc;

fn assert_mut_insert_remove<M>()
where
    M: BenchMapNew<u64, u64>
        + BenchMapMutInsert<u64, u64>
        + BenchMapMutRemove<u64, u64>
        + BenchMapGetCloned<u64, u64>,
{
    let mut map = M::new();
    map.insert(1, 10);
    map.insert(2, 20);
    assert_eq!(map.get_cloned(&1), Some(10));
    assert_eq!(map.get_cloned(&2), Some(20));

    assert_eq!(map.remove(&1), Some(10));
    assert_eq!(map.get_cloned(&1), None);
    assert_eq!(map.get_cloned(&2), Some(20));

    assert_eq!(map.remove(&999), None);
    assert_eq!(map.get_cloned(&2), Some(20));
}

fn assert_shared_insert_remove<M>()
where
    M: BenchMapNew<u64, u64>
        + BenchMapInsert<u64, u64>
        + BenchMapRemove<u64, u64>
        + BenchMapGetCloned<u64, u64>,
{
    let map = M::new();
    map.insert(1, 10);
    map.insert(2, 20);
    assert_eq!(map.get_cloned(&1), Some(10));
    assert_eq!(map.get_cloned(&2), Some(20));

    assert_eq!(map.remove(&1), Some(10));
    assert_eq!(map.get_cloned(&1), None);
    assert_eq!(map.get_cloned(&2), Some(20));
}

fn assert_create_map_populates_existing_keys<M>()
where
    M: BenchMapNew<u64, u64> + BenchMapMutInsert<u64, u64> + BenchMapGetCloned<u64, u64>,
{
    let map_data = Rc::new(MapGen::generate(
        U64SparseDataGen,
        U64SparseDataGen,
        100_000,
        100,
        0,
        false,
    ));
    let map = map_data.create_map::<M>();
    let found = map_data
        .existing_keys()
        .iter()
        .filter(|k| map.get_cloned(k).is_some())
        .count();
    assert_eq!(found, 100);
}

fn run_map_tests<M>()
where
    M: BenchMapNew<u64, u64>
        + BenchMapMutInsert<u64, u64>
        + BenchMapMutRemove<u64, u64>
        + BenchMapGetCloned<u64, u64>,
{
    assert_mut_insert_remove::<M>();
    assert_create_map_populates_existing_keys::<M>();
}

#[test]
fn ahash() {
    run_map_tests::<AhashBenchMap<u64, u64>>();
}

#[test]
fn btree_map() {
    run_map_tests::<BTreeMapBenchMap<u64, u64>>();
}

#[test]
fn concread() {
    run_map_tests::<ConcreadBenchMap<u64, u64>>();
}

#[test]
fn dashmap() {
    run_map_tests::<DashMapBenchMap<u64, u64>>();
}

#[test]
fn hashbrown() {
    run_map_tests::<HashbrownBenchMap<u64, u64>>();
}

#[test]
fn immutable_chunkmap() {
    run_map_tests::<ImmutableChunkMapBenchMap<u64, u64>>();
}

#[test]
fn indexmap() {
    run_map_tests::<IndexMapBenchMap<u64, u64>>();
}

#[test]
fn rustc_hash() {
    run_map_tests::<RustCHashBenchMap<u64, u64>>();
}

#[test]
fn starshard() {
    run_map_tests::<StarshardBenchMap<u64, u64>>();
}

#[test]
fn std() {
    run_map_tests::<StdBenchMap<u64, u64>>();
}

#[test]
fn txmap() {
    run_map_tests::<TxMapBenchMap<u64, u64>>();
}

#[test]
fn shared_reference_insert_remove() {
    assert_shared_insert_remove::<ConcreadBenchMap<u64, u64>>();
    assert_shared_insert_remove::<DashMapBenchMap<u64, u64>>();
    assert_shared_insert_remove::<StarshardBenchMap<u64, u64>>();
    assert_shared_insert_remove::<TxMapBenchMap<u64, u64>>();
}
