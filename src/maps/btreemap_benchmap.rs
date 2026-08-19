use crate::maps::benchmap::{
    BenchMapClone, BenchMapGetCloned, BenchMapIter, BenchMapMutClear, BenchMapMutGetOrInsert,
    BenchMapMutInsert, BenchMapMutRemove, BenchMapNew,
};
use std::collections::btree_map;

pub struct BTreeMapBenchMap<K, V> {
    map: btree_map::BTreeMap<K, V>,
}

impl<K, V> BenchMapNew<K, V> for BTreeMapBenchMap<K, V>
where
    K: Ord,
    V: Clone,
{
    fn new() -> Self {
        Self {
            map: btree_map::BTreeMap::new(),
        }
    }
}

impl<K, V> BenchMapClone<K, V> for BTreeMapBenchMap<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    fn clone_map(&self) -> Self {
        Self {
            map: self.map.clone(),
        }
    }
}

impl<K, V> BenchMapGetCloned<K, V> for BTreeMapBenchMap<K, V>
where
    K: Ord,
    V: Clone,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get(key).cloned()
    }
}

impl<K, V> BenchMapMutGetOrInsert<K, V> for BTreeMapBenchMap<K, V>
where
    K: Ord,
    V: Clone,
{
    fn get_or_insert(&mut self, key: K, default: V) -> V {
        self.map.entry(key).or_insert(default).clone()
    }
}

impl<K, V> BenchMapMutInsert<K, V> for BTreeMapBenchMap<K, V>
where
    K: Ord,
    V: Clone,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V> BenchMapIter<K, V> for BTreeMapBenchMap<K, V>
where
    K: Ord,
    V: Clone,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        for (key, value) in self.map.iter() {
            f(key, value);
        }
    }
}

impl<K, V> BenchMapMutRemove<K, V> for BTreeMapBenchMap<K, V>
where
    K: Ord,
    V: Clone,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}

impl<K, V> BenchMapMutClear<K, V> for BTreeMapBenchMap<K, V> {
    fn clear(&mut self) {
        self.map.clear();
    }
}
