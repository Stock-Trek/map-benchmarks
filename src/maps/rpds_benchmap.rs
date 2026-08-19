use crate::maps::benchmap::{
    BenchMapClone, BenchMapGetCloned, BenchMapIter, BenchMapMutClear, BenchMapMutInsert,
    BenchMapMutRemove, BenchMapNew, BenchMapNewWithHasher,
};
use rpds::HashTrieMap;
use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash},
};

/// rpds::HashTrieMap is persistent: `insert`/`remove` return a new map that
/// shares structure with the old one, so mutations are done in place via
/// `insert_mut`/`remove_mut` (copy-on-write if the map is shared). Clone is
/// O(1) structural sharing. The default pointer kind is `archery::RcK` (not
/// `Send`/`Sync`), and mutation through a shared reference is not possible,
/// so it is excluded from the concurrent workload.
pub struct RpdsHashTrieMapBenchMap<K, V, H = RandomState>
where
    K: Eq + Hash,
    H: BuildHasher + Clone,
{
    map: HashTrieMap<K, V, archery::RcK, H>,
    hasher: H,
}

impl<K, V, H> BenchMapNew<K, V> for RpdsHashTrieMapBenchMap<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Clone + Default,
{
    fn new() -> Self {
        let hasher = H::default();
        Self {
            map: HashTrieMap::new_with_hasher_and_ptr_kind(hasher.clone()),
            hasher,
        }
    }
}

impl<K, V, H> BenchMapNewWithHasher<K, V, H> for RpdsHashTrieMapBenchMap<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Clone,
{
    fn new_with_hasher(hasher: H) -> Self {
        Self {
            map: HashTrieMap::new_with_hasher_and_ptr_kind(hasher.clone()),
            hasher,
        }
    }
}

impl<K, V, H> BenchMapClone<K, V> for RpdsHashTrieMapBenchMap<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Clone,
{
    fn clone_map(&self) -> Self {
        Self {
            map: self.map.clone(),
            hasher: self.hasher.clone(),
        }
    }
}

impl<K, V, H> BenchMapGetCloned<K, V> for RpdsHashTrieMapBenchMap<K, V, H>
where
    K: Eq + Hash,
    V: Clone,
    H: BuildHasher + Clone,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get(key).cloned()
    }
}

impl<K, V, H> BenchMapMutInsert<K, V> for RpdsHashTrieMapBenchMap<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Clone,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.insert_mut(key, value);
    }
}

impl<K, V, H> BenchMapIter<K, V> for RpdsHashTrieMapBenchMap<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Clone,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        for (key, value) in self.map.iter() {
            f(key, value);
        }
    }
}

impl<K, V, H> BenchMapMutRemove<K, V> for RpdsHashTrieMapBenchMap<K, V, H>
where
    K: Eq + Hash,
    V: Clone,
    H: BuildHasher + Clone,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        let value = self.map.get(key).cloned();
        self.map.remove_mut(key);
        value
    }
}

impl<K, V, H> BenchMapMutClear<K, V> for RpdsHashTrieMapBenchMap<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Clone,
{
    fn clear(&mut self) {
        self.map = HashTrieMap::new_with_hasher_and_ptr_kind(self.hasher.clone());
    }
}
