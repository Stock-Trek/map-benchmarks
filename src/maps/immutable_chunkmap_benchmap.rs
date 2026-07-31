use crate::maps::benchmap::{BenchMapGetCloned, BenchMapMutInsert, BenchMapMutRemove, BenchMapNew};

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

impl<K, V> BenchMapMutInsert<K, V> for ImmutableChunkMapBenchMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    fn insert(&mut self, key: K, value: V) {
        // insert_cow mutates the map in place (copy-on-write if shared),
        // unlike `insert`, which returns a new map that would otherwise
        // be silently discarded, leaving the map empty.
        self.map.insert_cow(key, value);
    }
}

impl<K, V> BenchMapMutRemove<K, V> for ImmutableChunkMapBenchMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove_cow(key)
    }
}
