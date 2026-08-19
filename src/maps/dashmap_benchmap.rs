use crate::maps::benchmap::{
    BenchMapClone, BenchMapGetCloned, BenchMapGetOrInsert, BenchMapInsert, BenchMapIter,
    BenchMapMutClear, BenchMapMutGetOrInsert, BenchMapMutInsert, BenchMapMutRemove, BenchMapNew,
    BenchMapNewWithHasher, BenchMapRemove,
};
use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash},
};

pub struct DashMapBenchMap<K, V, H = RandomState> {
    map: dashmap::DashMap<K, V, H>,
}

impl<K, V, H> BenchMapNew<K, V> for DashMapBenchMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher + Clone + Default,
{
    fn new() -> Self {
        Self {
            map: dashmap::DashMap::with_hasher(H::default()),
        }
    }
}

impl<K, V, H> BenchMapNewWithHasher<K, V, H> for DashMapBenchMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher + Clone,
{
    fn new_with_hasher(hasher: H) -> Self {
        Self {
            map: dashmap::DashMap::with_hasher(hasher),
        }
    }
}

impl<K, V, H> BenchMapClone<K, V> for DashMapBenchMap<K, V, H>
where
    K: Hash + Eq + Clone,
    V: Clone,
    H: BuildHasher + Clone,
{
    fn clone_map(&self) -> Self {
        Self {
            map: self.map.clone(),
        }
    }
}

impl<K, V, H> BenchMapGetCloned<K, V> for DashMapBenchMap<K, V, H>
where
    K: Hash + Eq,
    V: Clone,
    H: BuildHasher + Clone,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get(key).map(|entry| entry.value().clone())
    }
}

impl<K, V, H> BenchMapGetOrInsert<K, V> for DashMapBenchMap<K, V, H>
where
    K: Hash + Eq,
    V: Clone,
    H: BuildHasher + Clone,
{
    fn get_or_insert(&self, key: K, default: V) -> V {
        self.map.entry(key).or_insert(default).clone()
    }
}

impl<K, V, H> BenchMapInsert<K, V> for DashMapBenchMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher + Clone,
{
    fn insert(&self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V, H> BenchMapMutInsert<K, V> for DashMapBenchMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher + Clone,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V, H> BenchMapMutGetOrInsert<K, V> for DashMapBenchMap<K, V, H>
where
    K: Hash + Eq,
    V: Clone,
    H: BuildHasher + Clone,
{
    fn get_or_insert(&mut self, key: K, default: V) -> V {
        self.map.entry(key).or_insert(default).clone()
    }
}

impl<K, V, H> BenchMapIter<K, V> for DashMapBenchMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher + Clone,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        for entry in self.map.iter() {
            f(entry.key(), entry.value());
        }
    }
}

impl<K, V, H> BenchMapRemove<K, V> for DashMapBenchMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher + Clone,
{
    fn remove(&self, key: &K) -> Option<V> {
        self.map.remove(key).map(|entry| entry.1)
    }
}

impl<K, V, H> BenchMapMutRemove<K, V> for DashMapBenchMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher + Clone,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key).map(|entry| entry.1)
    }
}

impl<K, V, H> BenchMapMutClear<K, V> for DashMapBenchMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher + Clone,
{
    fn clear(&mut self) {
        self.map.clear();
    }
}
