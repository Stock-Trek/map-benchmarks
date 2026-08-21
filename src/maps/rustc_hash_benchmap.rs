use crate::maps::*;
use std::hash::Hash;

pub struct RustCHashBenchMap<K, V> {
    map: rustc_hash::FxHashMap<K, V>,
}

impl<K, V> BenchMapName for RustCHashBenchMap<K, V> {
    const NAME: &'static str = "rustc-hash";
}

impl<K, V> BenchMapNew<K, V> for RustCHashBenchMap<K, V> {
    fn new() -> Self {
        Self {
            map: rustc_hash::FxHashMap::default(),
        }
    }
}

impl<K, V> BenchMapClone<K, V> for RustCHashBenchMap<K, V>
where
    K: Clone,
    V: Clone,
{
    fn clone_map(&self) -> Self {
        Self {
            map: self.map.clone(),
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

impl<K, V> BenchMapMutGetOrInsert<K, V> for RustCHashBenchMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn get_or_insert(&mut self, key: K, default: V) -> V {
        self.map.entry(key).or_insert(default).clone()
    }
}

impl<K, V> BenchMapMutInsert<K, V> for RustCHashBenchMap<K, V>
where
    K: Hash + Eq,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V> BenchMapIter<K, V> for RustCHashBenchMap<K, V> {
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        for (key, value) in self.map.iter() {
            f(key, value);
        }
    }
}

impl<K, V> BenchMapMutRemove<K, V> for RustCHashBenchMap<K, V>
where
    K: Hash + Eq,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}

impl<K, V> BenchMapMutClear<K, V> for RustCHashBenchMap<K, V> {
    fn clear(&mut self) {
        self.map.clear();
    }
}
