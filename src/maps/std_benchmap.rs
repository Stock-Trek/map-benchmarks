use crate::maps::benchmap::{
    BenchMapClone, BenchMapGetCloned, BenchMapIter, BenchMapMutClear, BenchMapMutGetOrInsert,
    BenchMapMutInsert, BenchMapMutRemove, BenchMapName, BenchMapNew, BenchMapNewWithHasher,
};
use std::{
    collections::{HashMap, hash_map::RandomState},
    hash::{BuildHasher, Hash},
};

pub struct StdBenchMap<K, V, H = RandomState> {
    map: HashMap<K, V, H>,
}

impl<K, V, H> BenchMapName for StdBenchMap<K, V, H> {
    const NAME: &'static str = "std";
}

impl<K, V, H> BenchMapNew<K, V> for StdBenchMap<K, V, H>
where
    H: BuildHasher + Default,
{
    fn new() -> Self {
        Self {
            map: HashMap::with_hasher(H::default()),
        }
    }
}

impl<K, V, H> BenchMapNewWithHasher<K, V, H> for StdBenchMap<K, V, H>
where
    H: BuildHasher,
{
    fn new_with_hasher(hasher: H) -> Self {
        Self {
            map: HashMap::with_hasher(hasher),
        }
    }
}

impl<K, V, H> BenchMapClone<K, V> for StdBenchMap<K, V, H>
where
    K: Clone,
    V: Clone,
    H: BuildHasher + Clone,
{
    fn clone_map(&self) -> Self {
        Self {
            map: self.map.clone(),
        }
    }
}

impl<K, V, H> BenchMapGetCloned<K, V> for StdBenchMap<K, V, H>
where
    K: Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get(key).cloned()
    }
}

impl<K, V, H> BenchMapMutGetOrInsert<K, V> for StdBenchMap<K, V, H>
where
    K: Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    fn get_or_insert(&mut self, key: K, default: V) -> V {
        self.map.entry(key).or_insert(default).clone()
    }
}

impl<K, V, H> BenchMapMutInsert<K, V> for StdBenchMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V, H> BenchMapIter<K, V> for StdBenchMap<K, V, H>
where
    H: BuildHasher,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        for (key, value) in self.map.iter() {
            f(key, value);
        }
    }
}

impl<K, V, H> BenchMapMutRemove<K, V> for StdBenchMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}

impl<K, V, H> BenchMapMutClear<K, V> for StdBenchMap<K, V, H> {
    fn clear(&mut self) {
        self.map.clear();
    }
}
