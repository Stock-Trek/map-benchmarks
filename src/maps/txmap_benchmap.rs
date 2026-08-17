use crate::maps::benchmap::{
    BenchMapGetCloned, BenchMapInsert, BenchMapIter, BenchMapMutInsert, BenchMapMutRemove,
    BenchMapNew, BenchMapRemove,
};
use std::hash::Hash;

pub struct TxMapBenchMap<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    map: txmap::prelude::TxMap<K, V>,
}

impl<K, V> BenchMapNew<K, V> for TxMapBenchMap<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    fn new() -> Self {
        Self {
            map: txmap::prelude::TxMap::new(),
        }
    }
}

impl<K, V> BenchMapGetCloned<K, V> for TxMapBenchMap<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get_cloned(key)
    }
}

impl<K, V> BenchMapInsert<K, V> for TxMapBenchMap<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    fn insert(&self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V> BenchMapMutInsert<K, V> for TxMapBenchMap<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V> BenchMapIter<K, V> for TxMapBenchMap<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        for (key, value) in self.map.iter() {
            f(key, value);
        }
    }
}

impl<K, V> BenchMapRemove<K, V> for TxMapBenchMap<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    fn remove(&self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}

impl<K, V> BenchMapMutRemove<K, V> for TxMapBenchMap<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}
