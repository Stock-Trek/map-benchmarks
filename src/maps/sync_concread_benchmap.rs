use crate::maps::sync_benchmap::SyncBenchMap;
use std::{fmt::Debug, hash::Hash};

pub struct SyncConcreadBenchMap<K, V>
where
    K: Clone + Debug + Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    map: concread::hashmap::HashMap<K, V>,
}

impl<K, V> SyncBenchMap<K, V> for SyncConcreadBenchMap<K, V>
where
    K: Clone + Debug + Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn new() -> Self {
        Self {
            map: concread::hashmap::HashMap::new(),
        }
    }
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.read().get(key).cloned()
    }
    fn insert(&self, key: K, value: V) {
        self.map.write().insert(key, value);
    }
    fn remove(&self, key: &K) -> Option<V> {
        self.map.write().remove(key)
    }
}
