use crate::maps::benchmap::{
    BenchMapGetCloned, BenchMapIter, BenchMapMutInsert, BenchMapMutRemove, BenchMapNew,
};
use std::hash::Hash;

pub struct HordeBenchMap<K, V> {
    map: horde::SyncTable<K, V>,
}

impl<K, V> BenchMapNew<K, V> for HordeBenchMap<K, V>
where
    K: Clone + Hash + Eq,
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
    K: Clone + Hash + Eq,
    V: Clone,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        horde::collect::pin(|pin| {
            self.map
                .read(pin)
                .get(key, None)
                .and_then(|tuple| Some(tuple.1.clone()))
        })
    }
}

impl<K, V> BenchMapMutInsert<K, V> for HordeBenchMap<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.write().insert(key, value, None);
    }
}

impl<K, V> BenchMapIter<K, V> for HordeBenchMap<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        horde::collect::pin(|pin| {
            let guard = self.map.read(pin);
            for (key, value) in guard.iter() {
                f(key, value);
            }
        });
    }
}

impl<K, V> BenchMapMutRemove<K, V> for HordeBenchMap<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map
            .write()
            .remove(key, None)
            .and_then(|tuple| Some(tuple.1.clone()))
    }
}
