use crate::maps::benchmap::{
    BenchMapClone, BenchMapGetCloned, BenchMapGetOrInsert, BenchMapInsert, BenchMapIter,
    BenchMapMutClear, BenchMapMutGetOrInsert, BenchMapMutInsert, BenchMapMutRemove, BenchMapName,
    BenchMapNew, BenchMapNewWithHasher, BenchMapRemove,
};
use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash},
};

pub struct SccBenchMap<K, V, H = RandomState>
where
    H: BuildHasher,
{
    map: scc::HashMap<K, V, H>,
}

impl<K, V, H> BenchMapName for SccBenchMap<K, V, H>
where
    H: BuildHasher,
{
    const NAME: &'static str = "scc";
}

impl<K, V, H> BenchMapNew<K, V> for SccBenchMap<K, V, H>
where
    H: BuildHasher + Default,
{
    fn new() -> Self {
        Self {
            map: scc::HashMap::with_hasher(H::default()),
        }
    }
}

impl<K, V, H> BenchMapNewWithHasher<K, V, H> for SccBenchMap<K, V, H>
where
    H: BuildHasher,
{
    fn new_with_hasher(hasher: H) -> Self {
        Self {
            map: scc::HashMap::with_hasher(hasher),
        }
    }
}

impl<K, V, H> BenchMapClone<K, V> for SccBenchMap<K, V, H>
where
    K: Clone + Hash + Eq,
    V: Clone,
    H: BuildHasher + Clone,
{
    fn clone_map(&self) -> Self {
        Self {
            map: self.map.clone(),
        }
    }
}

impl<K, V, H> BenchMapGetCloned<K, V> for SccBenchMap<K, V, H>
where
    K: Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.read_sync(key, |_key, value| value.clone())
    }
}

impl<K, V, H> BenchMapGetOrInsert<K, V> for SccBenchMap<K, V, H>
where
    K: Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    fn get_or_insert(&self, key: K, default: V) -> V {
        self.map.entry_sync(key).or_insert(default).get().clone()
    }
}

impl<K, V, H> BenchMapMutGetOrInsert<K, V> for SccBenchMap<K, V, H>
where
    K: Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    fn get_or_insert(&mut self, key: K, default: V) -> V {
        self.map.entry_sync(key).or_insert(default).get().clone()
    }
}

impl<K, V, H> BenchMapInsert<K, V> for SccBenchMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher,
{
    fn insert(&self, key: K, value: V) {
        self.map.upsert_sync(key, value);
    }
}

impl<K, V, H> BenchMapMutInsert<K, V> for SccBenchMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.upsert_sync(key, value);
    }
}

impl<K, V, H> BenchMapIter<K, V> for SccBenchMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        self.map.iter_sync(|key, value| {
            f(key, value);
            true
        });
    }
}

impl<K, V, H> BenchMapRemove<K, V> for SccBenchMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher,
{
    fn remove(&self, key: &K) -> Option<V> {
        self.map.remove_sync(key).map(|(_key, value)| value)
    }
}

impl<K, V, H> BenchMapMutRemove<K, V> for SccBenchMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove_sync(key).map(|(_key, value)| value)
    }
}

impl<K, V, H> BenchMapMutClear<K, V> for SccBenchMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher,
{
    fn clear(&mut self) {
        self.map.clear_sync();
    }
}
