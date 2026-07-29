use crate::maps::benchmap::{
    BenchMapGetCloned, BenchMapInsert, BenchMapMutInsert, BenchMapMutRemove, BenchMapNew,
    BenchMapRemove,
};
use std::{fmt::Debug, hash::Hash};

pub struct ConcreadBenchMap<K, V>
where
    K: Clone + Debug + Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    map: concread::hashmap::HashMap<K, V>,
}

impl<K, V> BenchMapNew<K, V> for ConcreadBenchMap<K, V>
where
    K: Clone + Debug + Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn new() -> Self {
        Self {
            map: concread::hashmap::HashMap::new(),
        }
    }
}

impl<K, V> BenchMapGetCloned<K, V> for ConcreadBenchMap<K, V>
where
    K: Clone + Debug + Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.read().get(key).cloned()
    }
}

impl<K, V> BenchMapInsert<K, V> for ConcreadBenchMap<K, V>
where
    K: Clone + Debug + Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn insert(&self, key: K, value: V) {
        self.map.write().insert(key, value);
    }
}

impl<K, V> BenchMapMutInsert<K, V> for ConcreadBenchMap<K, V>
where
    K: Clone + Debug + Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.write().insert(key, value);
    }
}

impl<K, V> BenchMapRemove<K, V> for ConcreadBenchMap<K, V>
where
    K: Clone + Debug + Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn remove(&self, key: &K) -> Option<V> {
        self.map.write().remove(key)
    }
}

impl<K, V> BenchMapMutRemove<K, V> for ConcreadBenchMap<K, V>
where
    K: Clone + Debug + Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.write().remove(key)
    }
}
