use crate::maps::BenchMap;
use std::hash::Hash;

pub struct AhashBenchMap<K, V> {
    map: ahash::AHashMap<K, V>,
}

impl<K, V> BenchMap<K, V> for AhashBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn new() -> Self {
        Self {
            map: ahash::AHashMap::new(),
        }
    }
    fn get_cloned(&mut self, key: &K) -> Option<V> {
        self.map.get(key).cloned()
    }
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}
