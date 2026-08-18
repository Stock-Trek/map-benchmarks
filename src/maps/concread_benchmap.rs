use crate::maps::benchmap::{
    BenchMapGetCloned, BenchMapInsert, BenchMapIter, BenchMapMutClear, BenchMapMutInsert,
    BenchMapMutRemove, BenchMapNew, BenchMapRemove,
};
use std::{fmt::Debug, hash::Hash};

pub struct ConcreadBenchMap<K, V>
where
    K: Clone + Debug + Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    map: concread::hashmap::HashMap<K, V>,
}

impl<K, V> BenchMapNew<K, V> for ConcreadBenchMap<K, V>
where
    K: Clone + Debug + Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn new() -> Self {
        Self {
            map: concread::hashmap::HashMap::new(),
        }
    }
}

impl<K, V> BenchMapGetCloned<K, V> for ConcreadBenchMap<K, V>
where
    K: Clone + Debug + Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.read().get(key).cloned()
    }
}

impl<K, V> BenchMapInsert<K, V> for ConcreadBenchMap<K, V>
where
    K: Clone + Debug + Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn insert(&self, key: K, value: V) {
        let mut write = self.map.write();
        write.insert(key, value);
        write.commit();
    }
}

impl<K, V> BenchMapMutInsert<K, V> for ConcreadBenchMap<K, V>
where
    K: Clone + Debug + Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn insert(&mut self, key: K, value: V) {
        let mut write = self.map.write();
        write.insert(key, value);
        write.commit();
    }
}

impl<K, V> BenchMapIter<K, V> for ConcreadBenchMap<K, V>
where
    K: Clone + Debug + Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        let guard = self.map.read();
        for (key, value) in guard.iter() {
            f(key, value);
        }
    }
}

impl<K, V> BenchMapRemove<K, V> for ConcreadBenchMap<K, V>
where
    K: Clone + Debug + Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn remove(&self, key: &K) -> Option<V> {
        let mut write = self.map.write();
        let removed = write.remove(key);
        write.commit();
        removed
    }
}

impl<K, V> BenchMapMutRemove<K, V> for ConcreadBenchMap<K, V>
where
    K: Clone + Debug + Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        let mut write = self.map.write();
        let removed = write.remove(key);
        write.commit();
        removed
    }
}

impl<K, V> BenchMapMutClear<K, V> for ConcreadBenchMap<K, V>
where
    K: Clone + Debug + Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn clear(&mut self) {
        let mut write = self.map.write();
        write.clear();
        write.commit();
    }
}
