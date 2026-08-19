use crate::maps::benchmap::{
    BenchMapClone, BenchMapGetCloned, BenchMapGetOrInsert, BenchMapInsert, BenchMapIter,
    BenchMapMutGetOrInsert, BenchMapMutInsert, BenchMapMutRemove, BenchMapNew, BenchMapRemove,
};
use concurrent_map::{ConcurrentMap, Minimum};

pub struct ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    map: ConcurrentMap<K, V>,
}

impl<K, V> BenchMapNew<K, V> for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn new() -> Self {
        Self {
            map: ConcurrentMap::new(),
        }
    }
}

impl<K, V> BenchMapClone<K, V> for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn clone_map(&self) -> Self {
        Self {
            map: self.map.clone(),
        }
    }
}

impl<K, V> BenchMapGetCloned<K, V> for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get(key)
    }
}

impl<K, V> BenchMapGetOrInsert<K, V> for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn get_or_insert(&self, key: K, default: V) -> V {
        // concurrent-map has no entry API, so emulate get-or-insert as a get
        // followed by an insert.
        if let Some(value) = self.map.get(&key) {
            value
        } else {
            self.map.insert(key, default.clone());
            default
        }
    }
}

impl<K, V> BenchMapMutGetOrInsert<K, V> for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn get_or_insert(&mut self, key: K, default: V) -> V {
        if let Some(value) = self.map.get(&key) {
            value
        } else {
            self.map.insert(key, default.clone());
            default
        }
    }
}

impl<K, V> BenchMapInsert<K, V> for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn insert(&self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V> BenchMapMutInsert<K, V> for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V> BenchMapIter<K, V> for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        for (key, value) in self.map.iter() {
            f(&key, &value);
        }
    }
}

impl<K, V> BenchMapRemove<K, V> for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn remove(&self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}

impl<K, V> BenchMapMutRemove<K, V> for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}
