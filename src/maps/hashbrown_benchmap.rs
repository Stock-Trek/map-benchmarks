use crate::maps::benchmap::{
    BenchMapGetCloned, BenchMapIter, BenchMapMutInsert, BenchMapMutRemove, BenchMapNew,
};
use std::hash::Hash;

pub struct HashbrownBenchMap<K, V> {
    map: hashbrown::HashMap<K, V>,
}

impl<K, V> BenchMapNew<K, V> for HashbrownBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn new() -> Self {
        Self {
            map: hashbrown::HashMap::new(),
        }
    }
}

impl<K, V> BenchMapGetCloned<K, V> for HashbrownBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get(key).cloned()
    }
}

impl<K, V> BenchMapMutInsert<K, V> for HashbrownBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V> BenchMapIter<K, V> for HashbrownBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        for (key, value) in self.map.iter() {
            f(key, value);
        }
    }
}

impl<K, V> BenchMapMutRemove<K, V> for HashbrownBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}
