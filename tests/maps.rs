use bench_map::{
    data::u64_sparse::U64SparseDataGen,
    map_gen::MapGen,
    maps::{
        AhashBenchMap, BTreeMapBenchMap, BenchMapGetCloned, BenchMapInsert, BenchMapIter,
        BenchMapMutClear, BenchMapMutInsert, BenchMapMutRemove, BenchMapNew, BenchMapNewWithHasher,
        BenchMapRemove, ConcreadBenchMap, DashMapBenchMap, HashbrownBenchMap, HordeBenchMap,
        ImmutableChunkMapBenchMap, IndexMapBenchMap, RustCHashBenchMap, StarshardBenchMap,
        StdBenchMap, TxMapBenchMap,
    },
};
use std::{collections::hash_map::RandomState, hash::BuildHasher, rc::Rc};

#[test]
fn ahash() {
    assert_create_map_populates_existing_keys::<AhashBenchMap<u64, u64>>();
    assert_iterate::<AhashBenchMap<u64, u64>>();
    assert_mut_insert_remove::<AhashBenchMap<u64, u64>>();
    assert_clear::<AhashBenchMap<u64, u64>>();
    // assert_shared_insert_remove::<AhashBenchMap<u64, u64>>();
}

#[test]
fn btree_map() {
    assert_create_map_populates_existing_keys::<BTreeMapBenchMap<u64, u64>>();
    assert_iterate::<BTreeMapBenchMap<u64, u64>>();
    assert_mut_insert_remove::<BTreeMapBenchMap<u64, u64>>();
    assert_clear::<BTreeMapBenchMap<u64, u64>>();
    // assert_shared_insert_remove::<BTreeMapBenchMap<u64, u64>>();
}

#[test]
fn concread() {
    assert_create_map_populates_existing_keys::<ConcreadBenchMap<u64, u64>>();
    assert_iterate::<ConcreadBenchMap<u64, u64>>();
    assert_mut_insert_remove::<ConcreadBenchMap<u64, u64>>();
    assert_clear::<ConcreadBenchMap<u64, u64>>();
    assert_shared_insert_remove::<ConcreadBenchMap<u64, u64>>();
}

#[test]
fn dashmap() {
    assert_create_map_populates_existing_keys::<DashMapBenchMap<u64, u64>>();
    assert_iterate::<DashMapBenchMap<u64, u64>>();
    assert_mut_insert_remove::<DashMapBenchMap<u64, u64>>();
    assert_clear::<DashMapBenchMap<u64, u64>>();
    assert_shared_insert_remove::<DashMapBenchMap<u64, u64>>();
}

#[test]
fn hashbrown() {
    assert_create_map_populates_existing_keys::<HashbrownBenchMap<u64, u64>>();
    assert_iterate::<HashbrownBenchMap<u64, u64>>();
    assert_mut_insert_remove::<HashbrownBenchMap<u64, u64>>();
    assert_clear::<HashbrownBenchMap<u64, u64>>();
    // assert_shared_insert_remove::<HashbrownBenchMap<u64, u64>>();
}

#[test]
fn horde() {
    assert_create_map_populates_existing_keys::<HordeBenchMap<u64, u64>>();
    assert_iterate::<HordeBenchMap<u64, u64>>();
    assert_mut_insert_remove::<HordeBenchMap<u64, u64>>();
    assert_clear::<HordeBenchMap<u64, u64>>();
}

#[test]
fn immutable_chunkmap() {
    assert_create_map_populates_existing_keys::<ImmutableChunkMapBenchMap<u64, u64>>();
    assert_iterate::<ImmutableChunkMapBenchMap<u64, u64>>();
    assert_mut_insert_remove::<ImmutableChunkMapBenchMap<u64, u64>>();
    assert_clear::<ImmutableChunkMapBenchMap<u64, u64>>();
    // assert_shared_insert_remove::<ImmutableChunkMapBenchMap<u64, u64>>();
}

#[test]
fn indexmap() {
    assert_create_map_populates_existing_keys::<IndexMapBenchMap<u64, u64>>();
    assert_iterate::<IndexMapBenchMap<u64, u64>>();
    assert_mut_insert_remove::<IndexMapBenchMap<u64, u64>>();
    assert_clear::<IndexMapBenchMap<u64, u64>>();
    // assert_shared_insert_remove::<IndexMapBenchMap<u64, u64>>();
}

#[test]
fn rustc_hash() {
    assert_create_map_populates_existing_keys::<RustCHashBenchMap<u64, u64>>();
    assert_iterate::<RustCHashBenchMap<u64, u64>>();
    assert_mut_insert_remove::<RustCHashBenchMap<u64, u64>>();
    assert_clear::<RustCHashBenchMap<u64, u64>>();
    // assert_shared_insert_remove::<RustCHashBenchMap<u64, u64>>();
}

#[test]
fn starshard() {
    assert_create_map_populates_existing_keys::<StarshardBenchMap<u64, u64>>();
    assert_iterate::<StarshardBenchMap<u64, u64>>();
    assert_mut_insert_remove::<StarshardBenchMap<u64, u64>>();
    assert_clear::<StarshardBenchMap<u64, u64>>();
    assert_shared_insert_remove::<StarshardBenchMap<u64, u64>>();
}

#[test]
fn std() {
    assert_create_map_populates_existing_keys::<StdBenchMap<u64, u64>>();
    assert_iterate::<StdBenchMap<u64, u64>>();
    assert_mut_insert_remove::<StdBenchMap<u64, u64>>();
    assert_clear::<StdBenchMap<u64, u64>>();
    // assert_shared_insert_remove::<StdBenchMap<u64, u64>>();
}

#[test]
fn txmap() {
    assert_create_map_populates_existing_keys::<TxMapBenchMap<u64, u64>>();
    assert_iterate::<TxMapBenchMap<u64, u64>>();
    assert_mut_insert_remove::<TxMapBenchMap<u64, u64>>();
    assert_clear::<TxMapBenchMap<u64, u64>>();
    assert_shared_insert_remove::<TxMapBenchMap<u64, u64>>();
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

fn assert_iterate<M>()
where
    M: BenchMapNew<u64, u64> + BenchMapMutInsert<u64, u64> + BenchMapIter<u64, u64>,
{
    let mut map = M::new();
    map.insert(1, 10);
    map.insert(2, 20);

    let mut count = 0;
    map.for_each(|_key, _value| {
        count += 1;
    });
    assert_eq!(count, 2);
}

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

#[test]
fn ahash_with_hasher() {
    assert_new_with_hasher::<AhashBenchMap<u64, u64, ahash::RandomState>, _>(
        ahash::RandomState::new(),
    );
    assert_new_with_hasher::<AhashBenchMap<u64, u64, RandomState>, _>(RandomState::new());
}

#[test]
fn dashmap_with_hasher() {
    assert_new_with_hasher::<DashMapBenchMap<u64, u64, ahash::RandomState>, _>(
        ahash::RandomState::new(),
    );
    assert_new_with_hasher::<DashMapBenchMap<u64, u64, RandomState>, _>(RandomState::new());
}

#[test]
fn hashbrown_with_hasher() {
    assert_new_with_hasher::<HashbrownBenchMap<u64, u64, ahash::RandomState>, _>(
        ahash::RandomState::new(),
    );
    assert_new_with_hasher::<HashbrownBenchMap<u64, u64, RandomState>, _>(RandomState::new());
}

#[test]
fn horde_with_hasher() {
    assert_new_with_hasher::<HordeBenchMap<u64, u64, ahash::RandomState>, _>(
        ahash::RandomState::new(),
    );
    assert_new_with_hasher::<HordeBenchMap<u64, u64, RandomState>, _>(RandomState::new());
}

#[test]
fn indexmap_with_hasher() {
    assert_new_with_hasher::<IndexMapBenchMap<u64, u64, ahash::RandomState>, _>(
        ahash::RandomState::new(),
    );
    assert_new_with_hasher::<IndexMapBenchMap<u64, u64, RandomState>, _>(RandomState::new());
}

#[test]
fn starshard_with_hasher() {
    assert_new_with_hasher::<StarshardBenchMap<u64, u64, ahash::RandomState>, _>(
        ahash::RandomState::new(),
    );
    assert_new_with_hasher::<StarshardBenchMap<u64, u64, RandomState>, _>(RandomState::new());
}

#[test]
fn std_with_hasher() {
    assert_new_with_hasher::<StdBenchMap<u64, u64, ahash::RandomState>, _>(
        ahash::RandomState::new(),
    );
    assert_new_with_hasher::<StdBenchMap<u64, u64, RandomState>, _>(RandomState::new());
}

#[test]
fn txmap_with_hasher() {
    assert_new_with_hasher::<TxMapBenchMap<u64, u64, ahash::RandomState>, _>(
        ahash::RandomState::new(),
    );
    assert_new_with_hasher::<TxMapBenchMap<u64, u64, RandomState>, _>(RandomState::new());
}

#[test]
fn create_map_with_hasher_populates_existing_keys() {
    let map_data = Rc::new(MapGen::generate(
        U64SparseDataGen,
        U64SparseDataGen,
        100_000,
        100,
        0,
        false,
    ));

    let map = map_data.create_map_with_hasher::<StdBenchMap<u64, u64, ahash::RandomState>, _>(
        ahash::RandomState::new(),
    );
    let found = map_data
        .existing_keys()
        .iter()
        .filter(|k| map.get_cloned(k).is_some())
        .count();
    assert_eq!(found, 100);
}

fn assert_new_with_hasher<M, H>(hasher: H)
where
    M: BenchMapNewWithHasher<u64, u64, H>
        + BenchMapMutInsert<u64, u64>
        + BenchMapMutRemove<u64, u64>
        + BenchMapGetCloned<u64, u64>,
    H: BuildHasher,
{
    let mut map = M::new_with_hasher(hasher);
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

fn assert_clear<M>()
where
    M: BenchMapNew<u64, u64>
        + BenchMapMutInsert<u64, u64>
        + BenchMapMutClear<u64, u64>
        + BenchMapGetCloned<u64, u64>,
{
    let mut map = M::new();
    map.insert(1, 10);
    map.insert(2, 20);
    assert_eq!(map.get_cloned(&1), Some(10));
    assert_eq!(map.get_cloned(&2), Some(20));

    map.clear();
    assert_eq!(map.get_cloned(&1), None);
    assert_eq!(map.get_cloned(&2), None);
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
