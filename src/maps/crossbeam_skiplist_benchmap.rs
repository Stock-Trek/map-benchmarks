use crate::maps::*;

pub struct CrossbeamSkiplistBenchMap<K, V> {
    map: crossbeam_skiplist::SkipMap<K, V>,
}

impl<K, V> BenchMapName for CrossbeamSkiplistBenchMap<K, V> {
    const NAME: &'static str = "crossbeam-skiplist";
}

impl<K, V> BenchMapNew<K, V> for CrossbeamSkiplistBenchMap<K, V> {
    fn new() -> Self {
        Self {
            map: crossbeam_skiplist::SkipMap::new(),
        }
    }
}

impl<K, V> BenchMapGetCloned<K, V> for CrossbeamSkiplistBenchMap<K, V>
where
    K: Ord,
    V: Clone,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get(key).map(|entry| entry.value().clone())
    }
}

impl<K, V> BenchMapGetOrInsert<K, V> for CrossbeamSkiplistBenchMap<K, V>
where
    K: Ord + Send + 'static,
    V: Clone + Send + 'static,
{
    fn get_or_insert(&self, key: K, default: V) -> V {
        self.map.get_or_insert(key, default).value().clone()
    }
}

impl<K, V> BenchMapMutGetOrInsert<K, V> for CrossbeamSkiplistBenchMap<K, V>
where
    K: Ord + Send + 'static,
    V: Clone + Send + 'static,
{
    fn get_or_insert(&mut self, key: K, default: V) -> V {
        self.map.get_or_insert(key, default).value().clone()
    }
}

impl<K, V> BenchMapInsert<K, V> for CrossbeamSkiplistBenchMap<K, V>
where
    K: Ord + Send + 'static,
    V: Send + 'static,
{
    fn insert(&self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V> BenchMapMutInsert<K, V> for CrossbeamSkiplistBenchMap<K, V>
where
    K: Ord + Send + 'static,
    V: Send + 'static,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V> BenchMapIter<K, V> for CrossbeamSkiplistBenchMap<K, V>
where
    K: Ord,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        for entry in self.map.iter() {
            f(entry.key(), entry.value());
        }
    }
}

impl<K, V> BenchMapRemove<K, V> for CrossbeamSkiplistBenchMap<K, V>
where
    K: Ord + Send + 'static,
    V: Clone + Send + 'static,
{
    fn remove(&self, key: &K) -> Option<V> {
        self.map.remove(key).map(|entry| entry.value().clone())
    }
}

impl<K, V> BenchMapMutRemove<K, V> for CrossbeamSkiplistBenchMap<K, V>
where
    K: Ord + Send + 'static,
    V: Clone + Send + 'static,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key).map(|entry| entry.value().clone())
    }
}

impl<K, V> BenchMapMutClear<K, V> for CrossbeamSkiplistBenchMap<K, V>
where
    K: Ord + Send + 'static,
    V: Send + 'static,
{
    fn clear(&mut self) {
        self.map.clear();
    }
}
