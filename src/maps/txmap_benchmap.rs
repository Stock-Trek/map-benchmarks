use crate::maps::BenchMap;
use std::hash::Hash;

pub struct TxMapBenchMap<K, V>
where
    K: Hash + Eq,
{
    map: txmap::prelude::TxMap<K, V>,
}

impl<K, V> BenchMap<K, V> for TxMapBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn new() -> Self {
        Self {
            map: txmap::prelude::TxMap::new(txmap::shards::Shards::_8),
        }
    }
    fn get_cloned(&mut self, key: &K) -> Option<V> {
        self.map.get_cloned(key)
    }
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}
