//! Regression tests for every bench map implementation.
//!
//! These tests guard against benchmarks silently eliding work:
//! - `insert`/`remove` must actually mutate the map, both through `&mut self`
//!   and (where supported, via interior mutability) through `&self`.
//! - `MapData::create_map` must fully populate the map so that lookup
//!   benchmarks measure lookups against a genuinely populated map.

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

/// `insert`/`remove` through `&mut self` must actually mutate the map rather
/// than silently discarding the operation.
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

    // Removing a missing key must not panic and must not disturb the map.
    assert_eq!(map.remove(&999), None);
    assert_eq!(map.get_cloned(&2), Some(20));
}

/// `insert`/`remove` through `&self` must also actually mutate the map. This
/// is the exact path the concurrency benchmark exercises, and it is the class
/// of bug where a mutation is silently discarded (as happened with
/// immutable-chunkmap's persistent `insert`/`remove`).
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

/// `MapData::create_map` must populate the map with every entry so lookup
/// benchmarks measure real lookups against a populated map. This exercises the
/// exact code path the lookup-hit (and related) benchmarks use.
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

macro_rules! map_tests {
    ($($map_name:ident => $map_type:ty),* $(,)?) => {
        $(
            #[test]
            fn $map_name() {
                assert_mut_insert_remove::<$map_type>();
                assert_create_map_populates_existing_keys::<$map_type>();
            }
        )*
    };
}

map_tests! {
    ahash => AhashBenchMap<u64, u64>,
    btree_map => BTreeMapBenchMap<u64, u64>,
    concread => ConcreadBenchMap<u64, u64>,
    dashmap => DashMapBenchMap<u64, u64>,
    hashbrown => HashbrownBenchMap<u64, u64>,
    immutable_chunkmap => ImmutableChunkMapBenchMap<u64, u64>,
    indexmap => IndexMapBenchMap<u64, u64>,
    rustc_hash => RustCHashBenchMap<u64, u64>,
    starshard => StarshardBenchMap<u64, u64>,
    std => StdBenchMap<u64, u64>,
    txmap => TxMapBenchMap<u64, u64>,
}

/// Maps that support mutation through a shared reference (interior mutability)
/// must also mutate correctly on that path.
#[test]
fn shared_reference_insert_remove() {
    assert_shared_insert_remove::<ConcreadBenchMap<u64, u64>>();
    assert_shared_insert_remove::<DashMapBenchMap<u64, u64>>();
    assert_shared_insert_remove::<StarshardBenchMap<u64, u64>>();
    assert_shared_insert_remove::<TxMapBenchMap<u64, u64>>();
}
