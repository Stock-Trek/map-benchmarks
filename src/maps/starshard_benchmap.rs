use crate::maps::benchmap::{
    BenchMapGetCloned, BenchMapInsert, BenchMapIter, BenchMapMutInsert, BenchMapMutRemove,
    BenchMapNew, BenchMapRemove,
};
use std::hash::Hash;

pub struct StarshardBenchMap<K, V>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
{
    map: starshard::ShardedHashMap<K, V>,
}

impl<K, V> BenchMapNew<K, V> for StarshardBenchMap<K, V>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
{
    fn new() -> Self {
        Self {
            map: starshard::ShardedHashMap::new(8),
        }
    }
}

impl<K, V> BenchMapGetCloned<K, V> for StarshardBenchMap<K, V>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get(key)
    }
}

impl<K, V> BenchMapInsert<K, V> for StarshardBenchMap<K, V>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
{
    fn insert(&self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V> BenchMapMutInsert<K, V> for StarshardBenchMap<K, V>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V> BenchMapIter<K, V> for StarshardBenchMap<K, V>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        for (key, value) in self.map.iter() {
            f(&key, &value);
        }
    }
}

impl<K, V> BenchMapRemove<K, V> for StarshardBenchMap<K, V>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
{
    fn remove(&self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}

impl<K, V> BenchMapMutRemove<K, V> for StarshardBenchMap<K, V>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}
