use crate::maps::BenchMap;
use std::hash::Hash;

pub struct IndexMapBenchMap<K, V> {
    map: indexmap::IndexMap<K, V>,
}

impl<K, V> BenchMap<K, V> for IndexMapBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn new() -> Self {
        Self {
            map: indexmap::IndexMap::new(),
        }
    }
    fn get_cloned(&mut self, key: &K) -> Option<V> {
        self.map.get(key).cloned()
    }
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.swap_remove(key)
    }
}
