use crate::maps::BenchMap;
use std::hash::Hash;

pub struct StarshardBenchMap<K, V>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
{
    map: starshard::ShardedHashMap<K, V>,
}

impl<K, V> BenchMap<K, V> for StarshardBenchMap<K, V>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
{
    fn new() -> Self {
        Self {
            map: starshard::ShardedHashMap::new(8),
        }
    }
    fn get_cloned(&mut self, key: &K) -> Option<V> {
        self.map.get(key)
    }
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}
