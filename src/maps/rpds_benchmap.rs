use crate::maps::benchmap::{
    BenchMapClone, BenchMapGetCloned, BenchMapIter, BenchMapMutGetOrInsert, BenchMapMutInsert,
    BenchMapMutRemove, BenchMapNew, BenchMapNewWithHasher,
};
use rpds::HashTrieMap;
use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash},
};

pub struct RpdsHashTrieMapBenchMap<K, V, H = RandomState>
where
    H: BuildHasher,
{
    map: HashTrieMap<K, V, archery::RcK, H>,
}

impl<K, V, H> BenchMapNew<K, V> for RpdsHashTrieMapBenchMap<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Clone + Default,
{
    fn new() -> Self {
        Self {
            map: HashTrieMap::new_with_hasher_and_ptr_kind(H::default()),
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
            map: HashTrieMap::new_with_hasher_and_ptr_kind(hasher),
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

impl<K, V, H> BenchMapMutGetOrInsert<K, V> for RpdsHashTrieMapBenchMap<K, V, H>
where
    K: Eq + Hash,
    V: Clone,
    H: BuildHasher + Clone,
{
    fn get_or_insert(&mut self, key: K, default: V) -> V {
        // rpds has no entry API, so emulate get-or-insert as a get followed
        // by an in-place insert.
        if let Some(value) = self.map.get(&key) {
            value.clone()
        } else {
            self.map.insert_mut(key, default.clone());
            default
        }
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
