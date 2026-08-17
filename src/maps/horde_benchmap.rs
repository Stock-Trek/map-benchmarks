use crate::maps::benchmap::{BenchMapGetCloned, BenchMapMutInsert, BenchMapMutRemove, BenchMapNew};
use std::hash::Hash;

pub struct HordeBenchMap<K, V> {
    map: horde::SyncTable<K, V>,
}

impl<K, V> BenchMapNew<K, V> for HordeBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn new() -> Self {
        Self {
            map: horde::SyncTable::new(),
        }
    }
}

impl<K, V> BenchMapGetCloned<K, V> for HordeBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        let hash = self.map.hash_key(key);
        horde::collect::pin(|pin| {
            self.map
                .read(pin)
                .get(key, Some(hash))
                .and_then(|tuple| Some(tuple.1.clone()))
        })
    }
}

impl<K, V> BenchMapMutInsert<K, V> for HordeBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn insert(&mut self, key: K, value: V) {
        horde::collect::pin(|_| self.map.write().replace(vec![(key, value)], 1))
    }
}

impl<K, V> BenchMapMutRemove<K, V> for HordeBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        let hash = self.map.hash_key(key);
        horde::collect::pin(|_| {
            self.map
                .write()
                .remove(key, Some(hash))
                .and_then(|tuple| Some(tuple.1.clone()))
        })
    }
}
