use crate::maps::*;
use std::hash::{BuildHasher, Hash};

pub struct HordeBenchMap<K, V, H = horde::sync_table::DefaultHashBuilder> {
    map: horde::SyncTable<K, V, H>,
}

impl<K, V, H> BenchMapName for HordeBenchMap<K, V, H> {
    const NAME: &'static str = "horde";
}

impl<K, V, H> BenchMapNew<K, V> for HordeBenchMap<K, V, H>
where
    H: BuildHasher + Default,
{
    fn new() -> Self {
        Self {
            map: horde::SyncTable::new_with(H::default(), 0),
        }
    }
}

impl<K, V, H> BenchMapNewWithHasher<K, V, H> for HordeBenchMap<K, V, H>
where
    H: BuildHasher,
{
    fn new_with_hasher(hasher: H) -> Self {
        Self {
            map: horde::SyncTable::new_with(hasher, 0),
        }
    }
}

impl<K, V, H> BenchMapClone<K, V> for HordeBenchMap<K, V, H>
where
    K: Clone + Hash,
    V: Clone,
    H: BuildHasher + Clone,
{
    fn clone_map(&self) -> Self {
        Self {
            map: self.map.clone(),
        }
    }
}

impl<K, V, H> BenchMapGetCloned<K, V> for HordeBenchMap<K, V, H>
where
    K: Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        horde::collect::pin(|pin| {
            self.map
                .read(pin)
                .get(key, None)
                .map(|tuple| tuple.1.clone())
        })
    }
}

impl<K, V, H> BenchMapMutGetOrInsert<K, V> for HordeBenchMap<K, V, H>
where
    K: Clone + Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    fn get_or_insert(&mut self, key: K, default: V) -> V {
        let existing = horde::collect::pin(|pin| {
            self.map
                .read(pin)
                .get(&key, None)
                .map(|(_key, value)| value.clone())
        });
        match existing {
            Some(value) => value,
            None => {
                self.map.write().insert(key, default.clone(), None);
                default
            }
        }
    }
}

impl<K, V, H> BenchMapMutInsert<K, V> for HordeBenchMap<K, V, H>
where
    K: Clone + Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    fn insert(&mut self, key: K, value: V) {
        self.map.write().insert(key, value, None);
    }
}

impl<K, V, H> BenchMapIter<K, V> for HordeBenchMap<K, V, H>
where
    H: BuildHasher,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        horde::collect::pin(|pin| {
            let guard = self.map.read(pin);
            for (key, value) in guard.iter() {
                f(key, value);
            }
        });
    }
}

impl<K, V, H> BenchMapMutRemove<K, V> for HordeBenchMap<K, V, H>
where
    K: Hash + Eq,
    V: Clone,
    H: BuildHasher,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map
            .write()
            .remove(key, None)
            .map(|tuple| tuple.1.clone())
    }
}

impl<K, V, H> BenchMapMutClear<K, V> for HordeBenchMap<K, V, H>
where
    K: Hash,
    H: BuildHasher,
{
    fn clear(&mut self) {
        self.map.write().replace(std::iter::empty(), 0);
    }
}
