use crate::maps::benchmap::{
    BenchMapClone, BenchMapGetCloned, BenchMapIter, BenchMapMutClear, BenchMapMutGetOrInsert,
    BenchMapMutInsert, BenchMapMutRemove, BenchMapName, BenchMapNew, BenchMapNewWithHasher,
};
use imbl::{hashmap::GenericHashMap, shared_ptr::DefaultSharedPtr};
use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash},
};

pub struct ImblBenchMap<K, V, H = RandomState> {
    map: GenericHashMap<K, V, H, DefaultSharedPtr>,
}

impl<K, V, H> BenchMapName for ImblBenchMap<K, V, H> {
    const NAME: &'static str = "imbl";
}

impl<K, V, H> BenchMapNew<K, V> for ImblBenchMap<K, V, H>
where
    H: Default,
{
    fn new() -> Self {
        Self {
            map: GenericHashMap::with_hasher(H::default()),
        }
    }
}

impl<K, V, H> BenchMapNewWithHasher<K, V, H> for ImblBenchMap<K, V, H> {
    fn new_with_hasher(hasher: H) -> Self {
        Self {
            map: GenericHashMap::with_hasher(hasher),
        }
    }
}

impl<K, V, H> BenchMapClone<K, V> for ImblBenchMap<K, V, H>
where
    K: Clone,
    V: Clone,
    H: Clone,
{
    fn clone_map(&self) -> Self {
        Self {
            map: self.map.clone(),
        }
    }
}

impl<K, V, H> BenchMapGetCloned<K, V> for ImblBenchMap<K, V, H>
where
    K: Hash + Eq,
    V: Clone,
    H: BuildHasher + Clone,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get(key).cloned()
    }
}

impl<K, V, H> BenchMapMutGetOrInsert<K, V> for ImblBenchMap<K, V, H>
where
    K: Hash + Eq + Clone,
    V: Clone,
    H: BuildHasher + Clone,
{
    fn get_or_insert(&mut self, key: K, default: V) -> V {
        self.map.entry(key).or_insert(default).clone()
    }
}

impl<K, V, H> BenchMapMutInsert<K, V> for ImblBenchMap<K, V, H>
where
    K: Hash + Eq + Clone,
    V: Clone,
    H: BuildHasher + Clone,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V, H> BenchMapIter<K, V> for ImblBenchMap<K, V, H> {
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        for (key, value) in self.map.iter() {
            f(key, value);
        }
    }
}

impl<K, V, H> BenchMapMutRemove<K, V> for ImblBenchMap<K, V, H>
where
    K: Hash + Eq + Clone,
    V: Clone,
    H: BuildHasher + Clone,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}

impl<K, V, H> BenchMapMutClear<K, V> for ImblBenchMap<K, V, H> {
    fn clear(&mut self) {
        self.map.clear();
    }
}
