use crate::maps::sync_benchmap::SyncBenchMap;
use std::hash::Hash;

pub struct SyncTxMapBenchMap<K, V>
where
    K: Hash + Eq,
{
    map: txmap::prelude::TxMap<K, V>,
}

impl<K, V> SyncBenchMap<K, V> for SyncTxMapBenchMap<K, V>
where
    K: Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
{
    fn new() -> Self {
        Self {
            map: txmap::prelude::TxMap::new(txmap::shards::Shards::_8),
        }
    }
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get_cloned(key)
    }
    fn insert(&self, key: K, value: V) {
        self.map.insert(key, value);
    }
    fn remove(&self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}
