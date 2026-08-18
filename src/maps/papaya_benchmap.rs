use crate::maps::benchmap::{
    BenchMapClone, BenchMapGetCloned, BenchMapInsert, BenchMapIter, BenchMapMutClear,
    BenchMapMutInsert, BenchMapMutRemove, BenchMapNew, BenchMapNewWithHasher, BenchMapRemove,
};
use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash},
};

pub struct PapayaBenchMap<K, V, H = RandomState> {
    map: papaya::HashMap<K, V, H>,
}

impl<K, V, H> BenchMapNew<K, V> for PapayaBenchMap<K, V, H>
where
    H: BuildHasher + Default,
{
    fn new() -> Self {
        Self {
            map: papaya::HashMap::with_hasher(H::default()),
        }
    }
}

impl<K, V, H> BenchMapNewWithHasher<K, V, H> for PapayaBenchMap<K, V, H>
where
    H: BuildHasher,
{
    fn new_with_hasher(hasher: H) -> Self {
        Self {
            map: papaya::HashMap::with_hasher(hasher),
        }
    }
}

impl<K, V, H> BenchMapClone<K, V> for PapayaBenchMap<K, V, H>
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

impl<K, V, H> BenchMapGetCloned<K, V> for PapayaBenchMap<K, V, H>
where
    K: Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.pin().get(key).cloned()
    }
}

impl<K, V, H> BenchMapInsert<K, V> for PapayaBenchMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher,
{
    fn insert(&self, key: K, value: V) {
        self.map.pin().insert(key, value);
    }
}

impl<K, V, H> BenchMapMutInsert<K, V> for PapayaBenchMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.pin().insert(key, value);
    }
}

impl<K, V, H> BenchMapIter<K, V> for PapayaBenchMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        let pin = self.map.pin();
        for (key, value) in pin.iter() {
            f(key, value);
        }
    }
}

impl<K, V, H> BenchMapRemove<K, V> for PapayaBenchMap<K, V, H>
where
    K: Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    fn remove(&self, key: &K) -> Option<V> {
        self.map.pin().remove(key).cloned()
    }
}

impl<K, V, H> BenchMapMutRemove<K, V> for PapayaBenchMap<K, V, H>
where
    K: Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.pin().remove(key).cloned()
    }
}

impl<K, V, H> BenchMapMutClear<K, V> for PapayaBenchMap<K, V, H>
where
    K: Hash + Eq,
    H: BuildHasher,
{
    fn clear(&mut self) {
        self.map.pin().clear();
    }
}
