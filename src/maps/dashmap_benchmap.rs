use crate::maps::BenchMap;
use std::hash::Hash;

pub struct DashMapBenchMap<K, V> {
    map: dashmap::DashMap<K, V>,
}

impl<K, V> BenchMap<K, V> for DashMapBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn new() -> Self {
        Self {
            map: dashmap::DashMap::new(),
        }
    }
    fn get_cloned(&mut self, key: &K) -> Option<V> {
        self.map.get(key).map(|r| r.value().clone())
    }
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key).map(|e| e.1)
    }
}
