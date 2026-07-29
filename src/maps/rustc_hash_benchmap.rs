use crate::maps::benchmap::{
    BenchMapGetCloned, BenchMapIter, BenchMapMutInsert, BenchMapMutRemove, BenchMapNew,
};
use std::hash::Hash;

pub struct RustCHashBenchMap<K, V> {
    map: rustc_hash::FxHashMap<K, V>,
}

impl<K, V> BenchMapNew<K, V> for RustCHashBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn new() -> Self {
        Self {
            map: rustc_hash::FxHashMap::default(),
        }
    }
}

impl<K, V> BenchMapGetCloned<K, V> for RustCHashBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get(key).cloned()
    }
}

impl<K, V> BenchMapMutInsert<K, V> for RustCHashBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V> BenchMapIter<K, V> for RustCHashBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    type Item<'a>
        = (&'a K, &'a V)
    where
        Self: 'a,
        K: 'a,
        V: 'a;
    fn iter<'a>(&'a self) -> impl Iterator<Item = Self::Item<'a>>
    where
        K: 'a,
        V: 'a,
    {
        self.map.iter()
    }
    fn item_value_ref<'a>(&'a self, item: &'a Self::Item<'a>) -> &'a V {
        item.1
    }
}

impl<K, V> BenchMapMutRemove<K, V> for RustCHashBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}
