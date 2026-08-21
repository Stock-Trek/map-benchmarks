use crate::maps::benchmap::{
    BenchMapClone, BenchMapGetCloned, BenchMapGetOrInsert, BenchMapInsert, BenchMapIter,
    BenchMapMutClear, BenchMapMutGetOrInsert, BenchMapMutInsert, BenchMapMutRemove, BenchMapName,
    BenchMapNew, BenchMapNewWithHasher, BenchMapRemove,
};
use std::hash::{BuildHasher, Hash};

pub struct FlurryBenchMap<K, V, H = flurry::DefaultHashBuilder> {
    map: flurry::HashMap<K, V, H>,
}

impl<K, V, H> BenchMapName for FlurryBenchMap<K, V, H> {
    const NAME: &'static str = "flurry";
}

impl<K, V, H> BenchMapNew<K, V> for FlurryBenchMap<K, V, H>
where
    H: BuildHasher + Default,
{
    fn new() -> Self {
        Self {
            map: flurry::HashMap::with_hasher(H::default()),
        }
    }
}

impl<K, V, H> BenchMapNewWithHasher<K, V, H> for FlurryBenchMap<K, V, H>
where
    H: BuildHasher,
{
    fn new_with_hasher(hasher: H) -> Self {
        Self {
            map: flurry::HashMap::with_hasher(hasher),
        }
    }
}

impl<K, V, H> BenchMapClone<K, V> for FlurryBenchMap<K, V, H>
where
    K: Sync + Send + Clone + Hash + Ord,
    V: Sync + Send + Clone,
    H: BuildHasher + Clone,
{
    fn clone_map(&self) -> Self {
        Self {
            map: self.map.clone(),
        }
    }
}

impl<K, V, H> BenchMapGetCloned<K, V> for FlurryBenchMap<K, V, H>
where
    K: Hash + Ord,
    V: Clone,
    H: BuildHasher,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.map.pin().get(key).cloned()
    }
}

impl<K, V, H> BenchMapGetOrInsert<K, V> for FlurryBenchMap<K, V, H>
where
    K: Sync + Send + Clone + Hash + Ord,
    V: Sync + Send + Clone,
    H: BuildHasher,
{
    fn get_or_insert(&self, key: K, default: V) -> V {
        if let Some(value) = self.map.pin().get(&key).cloned() {
            value
        } else {
            self.map.pin().insert(key, default.clone());
            default
        }
    }
}

impl<K, V, H> BenchMapInsert<K, V> for FlurryBenchMap<K, V, H>
where
    K: Sync + Send + Clone + Hash + Ord,
    V: Sync + Send,
    H: BuildHasher,
{
    fn insert(&self, key: K, value: V) {
        self.map.pin().insert(key, value);
    }
}

impl<K, V, H> BenchMapMutInsert<K, V> for FlurryBenchMap<K, V, H>
where
    K: Sync + Send + Clone + Hash + Ord,
    V: Sync + Send,
    H: BuildHasher,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.pin().insert(key, value);
    }
}

impl<K, V, H> BenchMapMutGetOrInsert<K, V> for FlurryBenchMap<K, V, H>
where
    K: Sync + Send + Clone + Hash + Ord,
    V: Sync + Send + Clone,
    H: BuildHasher,
{
    fn get_or_insert(&mut self, key: K, default: V) -> V {
        if let Some(value) = self.map.pin().get(&key).cloned() {
            value
        } else {
            self.map.pin().insert(key, default.clone());
            default
        }
    }
}

impl<K, V, H> BenchMapIter<K, V> for FlurryBenchMap<K, V, H>
where
    H: BuildHasher,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        let pin = self.map.pin();
        for (key, value) in pin.iter() {
            f(key, value);
        }
    }
}

impl<K, V, H> BenchMapRemove<K, V> for FlurryBenchMap<K, V, H>
where
    K: Sync + Send + Clone + Hash + Ord,
    V: Sync + Send + Clone,
    H: BuildHasher,
{
    fn remove(&self, key: &K) -> Option<V> {
        self.map.pin().remove(key).cloned()
    }
}

impl<K, V, H> BenchMapMutRemove<K, V> for FlurryBenchMap<K, V, H>
where
    K: Sync + Send + Clone + Hash + Ord,
    V: Sync + Send + Clone,
    H: BuildHasher,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.pin().remove(key).cloned()
    }
}

impl<K, V, H> BenchMapMutClear<K, V> for FlurryBenchMap<K, V, H>
where
    K: Clone + Ord,
    H: BuildHasher,
{
    fn clear(&mut self) {
        self.map.pin().clear();
    }
}
