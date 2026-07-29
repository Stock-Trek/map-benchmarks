use crate::maps::benchmap::{
    BenchMapGetCloned, BenchMapInsert, BenchMapMutInsert, BenchMapMutRemove, BenchMapNew,
    BenchMapRemove,
};

pub struct ImmutableChunkMapBenchMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    map: immutable_chunkmap::map::MapM<K, V>,
}

impl<K, V> BenchMapNew<K, V> for ImmutableChunkMapBenchMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    fn new() -> Self {
        Self {
            map: immutable_chunkmap::map::MapM::new(),
        }
    }
}

impl<K, V> BenchMapGetCloned<K, V> for ImmutableChunkMapBenchMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get(key).cloned()
    }
}

impl<K, V> BenchMapInsert<K, V> for ImmutableChunkMapBenchMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    fn insert(&self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V> BenchMapMutInsert<K, V> for ImmutableChunkMapBenchMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V> BenchMapRemove<K, V> for ImmutableChunkMapBenchMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    fn remove(&self, key: &K) -> Option<V> {
        self.map.remove(key).1
    }
}

impl<K, V> BenchMapMutRemove<K, V> for ImmutableChunkMapBenchMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key).1
    }
}
