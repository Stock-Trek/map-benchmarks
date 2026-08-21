use crate::maps::benchmap::{
    BenchMapGetCloned, BenchMapGetOrInsert, BenchMapInsert, BenchMapIter, BenchMapMutGetOrInsert,
    BenchMapMutInsert, BenchMapMutRemove, BenchMapName, BenchMapNew, BenchMapNewWithHasher,
    BenchMapRemove,
};
use leapfrog::Value;
use std::hash::{BuildHasher, Hash};

pub struct LeapfrogBenchMap<
    K,
    V,
    H = std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>,
> where
    K: Eq + Hash + Copy,
    V: Value,
    H: BuildHasher + Default,
{
    map: leapfrog::LeapMap<K, V, H>,
}

impl<K, V, H> BenchMapName for LeapfrogBenchMap<K, V, H>
where
    K: Eq + Hash + Copy,
    V: Value,
    H: BuildHasher + Default,
{
    const NAME: &'static str = "leapfrog";
}

impl<K, V, H> BenchMapNew<K, V> for LeapfrogBenchMap<K, V, H>
where
    K: Eq + Hash + Copy,
    V: Value,
    H: BuildHasher + Default,
{
    fn new() -> Self {
        Self {
            map: leapfrog::LeapMap::with_capacity_and_hasher(0, H::default()),
        }
    }
}

impl<K, V, H> BenchMapNewWithHasher<K, V, H> for LeapfrogBenchMap<K, V, H>
where
    K: Eq + Hash + Copy,
    V: Value,
    H: BuildHasher + Default,
{
    fn new_with_hasher(hasher: H) -> Self {
        Self {
            map: leapfrog::LeapMap::with_capacity_and_hasher(0, hasher),
        }
    }
}

impl<K, V, H> BenchMapGetCloned<K, V> for LeapfrogBenchMap<K, V, H>
where
    K: Eq + Hash + Copy,
    V: Value,
    H: BuildHasher + Default,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get(key).and_then(|mut entry| entry.value())
    }
}

impl<K, V, H> BenchMapGetOrInsert<K, V> for LeapfrogBenchMap<K, V, H>
where
    K: Eq + Hash + Copy,
    V: Value,
    H: BuildHasher + Default,
{
    fn get_or_insert(&self, key: K, default: V) -> V {
        match self.map.try_insert(key, default) {
            Some(existing) => existing,
            None => default,
        }
    }
}

impl<K, V, H> BenchMapMutGetOrInsert<K, V> for LeapfrogBenchMap<K, V, H>
where
    K: Eq + Hash + Copy,
    V: Value,
    H: BuildHasher + Default,
{
    fn get_or_insert(&mut self, key: K, default: V) -> V {
        match self.map.try_insert(key, default) {
            Some(existing) => existing,
            None => default,
        }
    }
}

impl<K, V, H> BenchMapInsert<K, V> for LeapfrogBenchMap<K, V, H>
where
    K: Eq + Hash + Copy,
    V: Value,
    H: BuildHasher + Default,
{
    fn insert(&self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V, H> BenchMapMutInsert<K, V> for LeapfrogBenchMap<K, V, H>
where
    K: Eq + Hash + Copy,
    V: Value,
    H: BuildHasher + Default,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V, H> BenchMapIter<K, V> for LeapfrogBenchMap<K, V, H>
where
    K: Eq + Hash + Copy,
    V: Value,
    H: BuildHasher + Default,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        for mut entry in self.map.iter() {
            if let Some((key, value)) = entry.key_value() {
                f(&key, &value);
            }
        }
    }
}

impl<K, V, H> BenchMapRemove<K, V> for LeapfrogBenchMap<K, V, H>
where
    K: Eq + Hash + Copy,
    V: Value,
    H: BuildHasher + Default,
{
    fn remove(&self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}

impl<K, V, H> BenchMapMutRemove<K, V> for LeapfrogBenchMap<K, V, H>
where
    K: Eq + Hash + Copy,
    V: Value,
    H: BuildHasher + Default,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}
