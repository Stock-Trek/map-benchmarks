use crate::maps::benchmap::{
    BenchMapGetCloned, BenchMapMutInsert, BenchMapMutRemove, BenchMapNew,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_populates_map_and_remove_removes() {
        let mut map = ImmutableChunkMapBenchMap::<u64, u64>::new();
        map.insert(1, 10);
        map.insert(2, 20);
        assert_eq!(map.get_cloned(&1), Some(10));
        assert_eq!(map.get_cloned(&2), Some(20));
        assert_eq!(map.remove(&1), Some(10));
        assert_eq!(map.get_cloned(&1), None);
        assert_eq!(map.get_cloned(&2), Some(20));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::{data::u64_sparse::U64SparseDataGen, map_gen::MapGen};
    use std::rc::Rc;

    #[test]
    fn create_map_populates_existing_keys() {
        let map_data = Rc::new(MapGen::generate(
            U64SparseDataGen,
            U64SparseDataGen,
            100_000,
            100,
            0,
            false,
        ));
        let map = map_data.create_map::<ImmutableChunkMapBenchMap<u64, u64>>();
        let found = map_data
            .existing_keys()
            .iter()
            .filter(|k| map.get_cloned(k).is_some())
            .count();
        assert_eq!(found, 100);
    }
}
