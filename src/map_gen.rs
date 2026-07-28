use crate::{data::data_gen::DataGen, map_data::MapData};
use hashbrown::HashSet;
use std::hash::Hash;

pub struct MapGen;

impl MapGen {
    pub fn generate<Kgen, Vgen, K, V>(
        key_gen: Kgen,
        value_gen: Vgen,
        entry_count: usize,
        existing_key_count: usize,
        missing_key_count: usize,
        sort_keys: bool,
    ) -> MapData<K, V>
    where
        Kgen: DataGen<Output = K>,
        Vgen: DataGen<Output = V>,
        K: Clone + Hash + Eq + Ord,
        V: Clone,
    {
        assert!(existing_key_count <= entry_count);

        let entry_keys = key_gen.generate(entry_count);
        let existing_keys = entry_keys
            .iter()
            .take(existing_key_count)
            .cloned()
            .collect::<HashSet<_>>();
        let missing_keys = key_gen.generate_avoiding(missing_key_count, &entry_keys);
        let mut entry_keys = entry_keys.into_iter().collect::<Vec<_>>();
        let mut existing_keys = existing_keys.into_iter().collect::<Vec<_>>();
        let mut missing_keys = missing_keys.into_iter().collect::<Vec<_>>();
        if sort_keys {
            entry_keys.sort();
            existing_keys.sort();
            missing_keys.sort();
        }

        let values = value_gen.generate(entry_count);

        let mut entries = Vec::with_capacity(entry_count);
        for (key, value) in entry_keys.into_iter().zip(values) {
            entries.push((key, value));
        }

        MapData::new(entries, existing_keys, missing_keys)
    }
}
