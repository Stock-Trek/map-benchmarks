use crate::maps::benchmap::{
    BenchMapClone, BenchMapGetCloned, BenchMapInsert, BenchMapIter, BenchMapMutClear,
    BenchMapMutInsert, BenchMapMutRemove, BenchMapNew, BenchMapNewWithHasher, BenchMapRemove,
};
use std::hash::{BuildHasher, Hash};

pub struct TxMapBenchMap<K, V, H = txmap::DefaultBuildHasher>
where
    K: Clone + Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    map: txmap::TxMap<K, V, txmap::MutexPolicy, H>,
}

impl<K, V, H> BenchMapNew<K, V> for TxMapBenchMap<K, V, H>
where
    K: Clone + Hash + Eq,
    V: Clone,
    H: BuildHasher + Default,
{
    fn new() -> Self {
        Self {
            map: txmap::TxMapBuilder::default()
                .with_hasher(H::default())
                .build(),
        }
    }
}

impl<K, V, H> BenchMapNewWithHasher<K, V, H> for TxMapBenchMap<K, V, H>
where
    K: Clone + Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    fn new_with_hasher(hasher: H) -> Self {
        Self {
            map: txmap::TxMapBuilder::default().with_hasher(hasher).build(),
        }
    }
}

impl<K, V, H> BenchMapClone<K, V> for TxMapBenchMap<K, V, H>
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

impl<K, V, H> BenchMapGetCloned<K, V> for TxMapBenchMap<K, V, H>
where
    K: Clone + Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.get_cloned(key)
    }
}

impl<K, V, H> BenchMapInsert<K, V> for TxMapBenchMap<K, V, H>
where
    K: Clone + Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    fn insert(&self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V, H> BenchMapMutInsert<K, V> for TxMapBenchMap<K, V, H>
where
    K: Clone + Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

impl<K, V, H> BenchMapIter<K, V> for TxMapBenchMap<K, V, H>
where
    K: Clone + Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        for (key, value) in self.map.iter() {
            f(key, value);
        }
    }
}

impl<K, V, H> BenchMapRemove<K, V> for TxMapBenchMap<K, V, H>
where
    K: Clone + Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    fn remove(&self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}

impl<K, V, H> BenchMapMutRemove<K, V> for TxMapBenchMap<K, V, H>
where
    K: Clone + Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }
}

impl<K, V, H> BenchMapMutClear<K, V> for TxMapBenchMap<K, V, H>
where
    K: Clone + Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    fn clear(&mut self) {
        self.map.clear();
    }
}
