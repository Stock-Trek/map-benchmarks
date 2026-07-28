use crate::maps::BenchMap;
use std::hash::Hash;

pub struct RustCHashBenchMap<K, V> {
    map: rustc_hash::FxHashMap<K, V>,
}

impl<K, V> BenchMap<K, V> for RustCHashBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn new() -> Self {
        Self {
            map: rustc_hash::FxHashMap::default(),
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
