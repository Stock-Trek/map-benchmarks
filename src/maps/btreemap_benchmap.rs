use crate::maps::BenchMap;
use std::collections::btree_map;

pub struct BTreeMapBenchMap<K, V> {
    map: btree_map::BTreeMap<K, V>,
}

impl<K, V> BenchMap<K, V> for BTreeMapBenchMap<K, V>
where
    K: Ord,
    V: Clone,
{
    fn new() -> Self {
        Self {
            map: btree_map::BTreeMap::new(),
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
