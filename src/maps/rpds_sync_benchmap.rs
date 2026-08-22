use crate::maps::*;
use rpds::HashTrieMap;
use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash},
    sync::Mutex,
};

/// rpds `HashTrieMap` adapted for the multi-threaded benchmark groups.
///
/// Two things block the plain `RpdsHashTrieMapBenchMap` from running on
/// multiple threads:
///
///   1. Its default pointer kind is `archery::RcK` (an `Rc`), which is neither
///      `Send` nor `Sync`. Here we use the `Arc`-based `archery::ArcTK` pointer
///      kind (the same one behind rpds' own `HashTrieMapSync` alias), which is
///      `Send + Sync` when the key/value/hasher are.
///   2. rpds mutations (`insert_mut`/`remove_mut`) require `&mut self`, but the
///      concurrent workloads mutate through a shared reference. Wrapping the
///      map in a `Mutex` makes every operation safe through `&self`.
///
/// `&mut`-based operations use `Mutex::get_mut` so they never take the lock.
pub struct RpdsHashTrieMapSyncBenchMap<K, V, H = RandomState>
where
    H: BuildHasher,
{
    map: Mutex<HashTrieMap<K, V, archery::ArcTK, H>>,
}

impl<K, V, H> BenchMapName for RpdsHashTrieMapSyncBenchMap<K, V, H>
where
    H: BuildHasher,
{
    const NAME: &'static str = "rpds-hash-trie-map-sync";
}

impl<K, V, H> BenchMapNew<K, V> for RpdsHashTrieMapSyncBenchMap<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Clone + Default,
{
    fn new() -> Self {
        Self {
            map: Mutex::new(HashTrieMap::new_with_hasher_and_ptr_kind(H::default())),
        }
    }
}

impl<K, V, H> BenchMapNewWithHasher<K, V, H> for RpdsHashTrieMapSyncBenchMap<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Clone,
{
    fn new_with_hasher(hasher: H) -> Self {
        Self {
            map: Mutex::new(HashTrieMap::new_with_hasher_and_ptr_kind(hasher)),
        }
    }
}

impl<K, V, H> BenchMapClone<K, V> for RpdsHashTrieMapSyncBenchMap<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Clone,
{
    fn clone_map(&self) -> Self {
        Self {
            map: Mutex::new(self.map.lock().unwrap().clone()),
        }
    }
}

impl<K, V, H> BenchMapGetCloned<K, V> for RpdsHashTrieMapSyncBenchMap<K, V, H>
where
    K: Eq + Hash,
    V: Clone,
    H: BuildHasher + Clone,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.lock().unwrap().get(key).cloned()
    }
}

impl<K, V, H> BenchMapGetOrInsert<K, V> for RpdsHashTrieMapSyncBenchMap<K, V, H>
where
    K: Eq + Hash,
    V: Clone,
    H: BuildHasher + Clone,
{
    fn get_or_insert(&self, key: K, default: V) -> V {
        // rpds has no entry API, so emulate get-or-insert as a get followed
        // by an in-place insert, holding the lock for the whole operation.
        let mut map = self.map.lock().unwrap();
        if let Some(value) = map.get(&key) {
            value.clone()
        } else {
            map.insert_mut(key, default.clone());
            default
        }
    }
}

impl<K, V, H> BenchMapInsert<K, V> for RpdsHashTrieMapSyncBenchMap<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Clone,
{
    fn insert(&self, key: K, value: V) {
        self.map.lock().unwrap().insert_mut(key, value);
    }
}

impl<K, V, H> BenchMapIter<K, V> for RpdsHashTrieMapSyncBenchMap<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Clone,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        let map = self.map.lock().unwrap();
        for (key, value) in map.iter() {
            f(key, value);
        }
    }
}

impl<K, V, H> BenchMapRemove<K, V> for RpdsHashTrieMapSyncBenchMap<K, V, H>
where
    K: Eq + Hash,
    V: Clone,
    H: BuildHasher + Clone,
{
    fn remove(&self, key: &K) -> Option<V> {
        // rpds has no remove that returns the value, so read it before
        // removing, holding the lock for the whole operation.
        let mut map = self.map.lock().unwrap();
        let value = map.get(key).cloned();
        map.remove_mut(key);
        value
    }
}

impl<K, V, H> BenchMapMutGetOrInsert<K, V> for RpdsHashTrieMapSyncBenchMap<K, V, H>
where
    K: Eq + Hash,
    V: Clone,
    H: BuildHasher + Clone,
{
    fn get_or_insert(&mut self, key: K, default: V) -> V {
        let map = self.map.get_mut().unwrap();
        if let Some(value) = map.get(&key) {
            value.clone()
        } else {
            map.insert_mut(key, default.clone());
            default
        }
    }
}

impl<K, V, H> BenchMapMutInsert<K, V> for RpdsHashTrieMapSyncBenchMap<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Clone,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.get_mut().unwrap().insert_mut(key, value);
    }
}

impl<K, V, H> BenchMapMutRemove<K, V> for RpdsHashTrieMapSyncBenchMap<K, V, H>
where
    K: Eq + Hash,
    V: Clone,
    H: BuildHasher + Clone,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        let map = self.map.get_mut().unwrap();
        let value = map.get(key).cloned();
        map.remove_mut(key);
        value
    }
}
