use crate::{data::data_gen::DataGen, maps::BenchMap};
use hashbrown::HashSet;
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
    pub fn new<KeyGen, ValueGen>(
        key_gen: KeyGen,
        value_gen: ValueGen,
        entry_count: usize,
        unique_keys_proportion: f32,
        sorted_keys: bool,
    ) -> Self
    where
        KeyGen: DataGen<Output = K>,
        ValueGen: DataGen<Output = V>,
    {
        let mut entries = Vec::with_capacity(entry_count);
        let entry_keys = key_gen.generate(entry_count, unique_keys_proportion, sorted_keys);
        let existing_keys = entry_keys.clone();
        let missing_keys = key_gen.generate_unique(
            existing_keys.len(),
            &existing_keys
                .iter()
                .map(|k| k.clone())
                .collect::<HashSet<K>>(),
        );
        let values = value_gen.generate(entry_count, 1.0, false);
        for (key, value) in entry_keys.into_iter().zip(values) {
            entries.push((key, value));
        }
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
