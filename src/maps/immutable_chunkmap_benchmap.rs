use crate::maps::benchmap::{
    BenchMapClone, BenchMapGetCloned, BenchMapIter, BenchMapMutGetOrInsert, BenchMapMutInsert,
    BenchMapMutRemove, BenchMapNew,
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

impl<K, V> BenchMapClone<K, V> for ImmutableChunkMapBenchMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    fn clone_map(&self) -> Self {
        Self {
            map: self.map.clone(),
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

impl<K, V> BenchMapMutGetOrInsert<K, V> for ImmutableChunkMapBenchMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    fn get_or_insert(&mut self, key: K, default: V) -> V {
        self.map.get_or_insert_cow(key, || default).clone()
    }
}

impl<K, V> BenchMapMutInsert<K, V> for ImmutableChunkMapBenchMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.insert_cow(key, value);
    }
}

impl<K, V> BenchMapIter<K, V> for ImmutableChunkMapBenchMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        for (key, value) in &self.map {
            f(key, value);
        }
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
