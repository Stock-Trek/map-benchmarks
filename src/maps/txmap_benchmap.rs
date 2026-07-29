use crate::maps::benchmap::{
    BenchMapGetCloned, BenchMapInsert, BenchMapIter, BenchMapMutInsert, BenchMapMutRemove,
    BenchMapNew, BenchMapRemove,
};
use std::hash::Hash;
use txmap::shards::Shards;

pub struct TxMapBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    map: txmap::prelude::TxMap<K, V>,
}

impl<K, V> BenchMapNew<K, V> for TxMapBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn new() -> Self {
        Self {
            map: txmap::prelude::TxMap::new(Shards::_8),
        }
    }
}

impl<K, V> BenchMapGetCloned<K, V> for TxMapBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get_cloned(key)
    }
}

impl<K, V> BenchMapInsert<K, V> for TxMapBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn insert(&self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V> BenchMapMutInsert<K, V> for TxMapBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V> BenchMapIter<K, V> for TxMapBenchMap<K, V>
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

impl<K, V> BenchMapRemove<K, V> for TxMapBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn remove(&self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}

impl<K, V> BenchMapMutRemove<K, V> for TxMapBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}
