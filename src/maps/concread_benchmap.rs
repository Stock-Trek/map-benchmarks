use crate::maps::BenchMap;
use std::{fmt::Debug, hash::Hash};

pub struct ConcreadBenchMap<K, V>
where
    K: Clone + Debug + Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    map: concread::hashmap::HashMap<K, V>,
}

impl<K, V> BenchMap<K, V> for ConcreadBenchMap<K, V>
where
    K: Clone + Debug + Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn new() -> Self {
        Self {
            map: concread::hashmap::HashMap::new(),
        }
    }
    fn get_cloned(&mut self, key: &K) -> Option<V> {
        self.map.read().get(key).cloned()
    }
    fn insert(&mut self, key: K, value: V) {
        self.map.write().insert(key, value);
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.write().remove(key)
    }
}
