use crate::maps::BenchMap;
use std::hash::Hash;

pub struct MapData<K, V> {
    entries: Vec<(K, V)>,
    existing_keys: Vec<K>,
    missing_keys: Vec<K>,
}

impl<K, V> MapData<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    pub fn new(entries: Vec<(K, V)>, existing_keys: Vec<K>, missing_keys: Vec<K>) -> Self {
        Self {
            entries,
            existing_keys,
            missing_keys,
        }
    }
    pub fn create_map<M>(&self) -> M
    where
        M: BenchMap<K, V>,
    {
        let mut map = M::new();
        for (key, value) in &self.entries {
            map.insert(key.clone(), value.clone());
        }
        map
    }
    pub fn existing_keys(&self) -> &Vec<K> {
        &self.existing_keys
    }
    pub fn missing_keys(&self) -> &Vec<K> {
        &self.missing_keys
    }
}
