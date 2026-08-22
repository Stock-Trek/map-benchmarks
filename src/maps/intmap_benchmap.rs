use crate::maps::*;
use intmap::IntKey;

pub struct IntMapBenchMap<K, V> {
    map: intmap::IntMap<K, V>,
}

impl<K, V> BenchMapName for IntMapBenchMap<K, V> {
    const NAME: &'static str = "intmap";
}

impl<K, V> BenchMapNew<K, V> for IntMapBenchMap<K, V>
where
    K: IntKey,
{
    fn new() -> Self {
        Self {
            map: intmap::IntMap::new(),
        }
    }
}

impl<K, V> BenchMapClone<K, V> for IntMapBenchMap<K, V>
where
    K: IntKey,
    V: Clone,
{
    fn clone_map(&self) -> Self {
        Self {
            map: self.map.clone(),
        }
    }
}

impl<K, V> BenchMapGetCloned<K, V> for IntMapBenchMap<K, V>
where
    K: IntKey,
    V: Clone,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get(*key).cloned()
    }
}

impl<K, V> BenchMapMutGetOrInsert<K, V> for IntMapBenchMap<K, V>
where
    K: IntKey,
    V: Clone,
{
    fn get_or_insert(&mut self, key: K, default: V) -> V {
        self.map.entry(key).or_insert(default).clone()
    }
}

impl<K, V> BenchMapMutInsert<K, V> for IntMapBenchMap<K, V>
where
    K: IntKey,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V> BenchMapIter<K, V> for IntMapBenchMap<K, V>
where
    K: IntKey,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        for (key, value) in self.map.iter() {
            f(&key, value);
        }
    }
}

impl<K, V> BenchMapMutRemove<K, V> for IntMapBenchMap<K, V>
where
    K: IntKey,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(*key)
    }
}

impl<K, V> BenchMapMutClear<K, V> for IntMapBenchMap<K, V>
where
    K: IntKey,
{
    fn clear(&mut self) {
        self.map.clear();
    }
}
