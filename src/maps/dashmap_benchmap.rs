use crate::maps::benchmap::{
    BenchMapGetCloned, BenchMapInsert, BenchMapIter, BenchMapMutInsert, BenchMapMutRemove,
    BenchMapNew, BenchMapRemove,
};
use std::hash::Hash;

pub struct DashMapBenchMap<K, V> {
    map: dashmap::DashMap<K, V>,
}

impl<K, V> BenchMapNew<K, V> for DashMapBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn new() -> Self {
        Self {
            map: dashmap::DashMap::new(),
        }
    }
}

impl<K, V> BenchMapGetCloned<K, V> for DashMapBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get(key).map(|entry| entry.value().clone())
    }
}

impl<K, V> BenchMapInsert<K, V> for DashMapBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn insert(&self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V> BenchMapMutInsert<K, V> for DashMapBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V> BenchMapIter<K, V> for DashMapBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        for entry in self.map.iter() {
            f(entry.key(), entry.value());
        }
    }
}

impl<K, V> BenchMapRemove<K, V> for DashMapBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn remove(&self, key: &K) -> Option<V> {
        self.map.remove(key).map(|entry| entry.1)
    }
}

impl<K, V> BenchMapMutRemove<K, V> for DashMapBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key).map(|entry| entry.1)
    }
}
