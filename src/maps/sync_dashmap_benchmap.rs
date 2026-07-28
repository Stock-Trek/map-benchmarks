use crate::maps::sync_benchmap::SyncBenchMap;
use std::hash::Hash;

pub struct SyncDashMapBenchMap<K, V> {
    map: dashmap::DashMap<K, V>,
}

impl<K, V> SyncBenchMap<K, V> for SyncDashMapBenchMap<K, V>
where
    K: Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
{
    fn new() -> Self {
        Self {
            map: dashmap::DashMap::new(),
        }
    }
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get(key).map(|r| r.value().clone())
    }
    fn insert(&self, key: K, value: V) {
        self.map.insert(key, value);
    }
    fn remove(&self, key: &K) -> Option<V> {
        self.map.remove(key).map(|e| e.1)
    }
}
