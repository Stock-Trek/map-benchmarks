use crate::maps::BenchMap;
use std::hash::Hash;

pub struct StdBenchMap<K, V> {
    map: std::collections::HashMap<K, V>,
}

impl<K, V> BenchMap<K, V> for StdBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn new() -> Self {
        Self {
            map: std::collections::HashMap::new(),
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
