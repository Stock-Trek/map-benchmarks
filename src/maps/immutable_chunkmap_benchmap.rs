use crate::maps::BenchMap;

pub struct ImmutableChunkMapBenchMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    map: immutable_chunkmap::map::MapM<K, V>,
}

impl<K, V> BenchMap<K, V> for ImmutableChunkMapBenchMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    fn new() -> Self {
        Self {
            map: immutable_chunkmap::map::MapM::new(),
        }
    }
    fn get_cloned(&mut self, key: &K) -> Option<V> {
        self.map.get(key).cloned()
    }
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key).1
    }
}
