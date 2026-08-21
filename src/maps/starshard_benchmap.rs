use crate::maps::*;
use std::hash::{BuildHasher, Hash};

pub struct StarshardBenchMap<K, V, H = rustc_hash::FxBuildHasher>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
    H: BuildHasher + Clone + Send + Sync,
{
    map: starshard::ShardedHashMap<K, V, H>,
}

impl<K, V, H> BenchMapName for StarshardBenchMap<K, V, H>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
    H: BuildHasher + Clone + Send + Sync,
{
    const NAME: &'static str = "starshard";
}

impl<K, V, H> BenchMapNew<K, V> for StarshardBenchMap<K, V, H>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
    H: BuildHasher + Clone + Default + Send + Sync,
{
    fn new() -> Self {
        Self {
            map: starshard::ShardedHashMap::with_shards_and_hasher(8, H::default()),
        }
    }
}

impl<K, V, H> BenchMapNewWithHasher<K, V, H> for StarshardBenchMap<K, V, H>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
    H: BuildHasher + Clone + Send + Sync,
{
    fn new_with_hasher(hasher: H) -> Self {
        Self {
            map: starshard::ShardedHashMap::with_shards_and_hasher(8, hasher),
        }
    }
}

impl<K, V, H> BenchMapClone<K, V> for StarshardBenchMap<K, V, H>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
    H: BuildHasher + Clone + Send + Sync,
{
    fn clone_map(&self) -> Self {
        Self {
            map: self.map.clone(),
        }
    }
}

impl<K, V, H> BenchMapGetCloned<K, V> for StarshardBenchMap<K, V, H>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
    H: BuildHasher + Clone + Send + Sync,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get(key)
    }
}

impl<K, V, H> BenchMapGetOrInsert<K, V> for StarshardBenchMap<K, V, H>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
    H: BuildHasher + Clone + Send + Sync,
{
    fn get_or_insert(&self, key: K, default: V) -> V {
        self.map.compute_if_absent(key, || default)
    }
}

impl<K, V, H> BenchMapMutGetOrInsert<K, V> for StarshardBenchMap<K, V, H>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
    H: BuildHasher + Clone + Send + Sync,
{
    fn get_or_insert(&mut self, key: K, default: V) -> V {
        self.map.compute_if_absent(key, || default)
    }
}

impl<K, V, H> BenchMapInsert<K, V> for StarshardBenchMap<K, V, H>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
    H: BuildHasher + Clone + Send + Sync,
{
    fn insert(&self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V, H> BenchMapMutInsert<K, V> for StarshardBenchMap<K, V, H>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
    H: BuildHasher + Clone + Send + Sync,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V, H> BenchMapIter<K, V> for StarshardBenchMap<K, V, H>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
    H: BuildHasher + Clone + Send + Sync,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        for (key, value) in self.map.iter() {
            f(&key, &value);
        }
    }
}

impl<K, V, H> BenchMapRemove<K, V> for StarshardBenchMap<K, V, H>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
    H: BuildHasher + Clone + Send + Sync,
{
    fn remove(&self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}

impl<K, V, H> BenchMapMutRemove<K, V> for StarshardBenchMap<K, V, H>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
    H: BuildHasher + Clone + Send + Sync,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}

impl<K, V, H> BenchMapMutClear<K, V> for StarshardBenchMap<K, V, H>
where
    K: Clone + Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
    H: BuildHasher + Clone + Send + Sync,
{
    fn clear(&mut self) {
        self.map.clear();
    }
}
