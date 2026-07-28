use crate::maps::sync_benchmap::SyncBenchMap;
use std::hash::Hash;

pub struct SyncStarshardBenchMap<K, V>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
{
    map: starshard::ShardedHashMap<K, V>,
}

impl<K, V> SyncBenchMap<K, V> for SyncStarshardBenchMap<K, V>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
{
    fn new() -> Self {
        Self {
            map: starshard::ShardedHashMap::new(8),
        }
    }
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get(key)
    }
    fn insert(&self, key: K, value: V) {
        self.map.insert(key, value);
    }
    fn remove(&self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}
