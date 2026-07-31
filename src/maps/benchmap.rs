pub trait BenchMapNew<K, V>: Sized {
    fn new() -> Self;
}
pub trait BenchMapGetCloned<K, V> {
    fn get_cloned(&self, key: &K) -> Option<V>;
}
pub trait BenchMapInsert<K, V> {
    fn insert(&self, key: K, value: V);
}
pub trait BenchMapIter<K, V> {
    type Item<'a>
    where
        Self: 'a,
        K: 'a,
        V: 'a;
    fn iter<'a>(&'a self) -> impl Iterator<Item = Self::Item<'a>>
    where
        K: 'a,
        V: 'a;
    fn item_value_ref<'a, 'b>(&'a self, item: &'b Self::Item<'a>) -> &'b V;
}
pub trait BenchMapRemove<K, V> {
    fn remove(&self, key: &K) -> Option<V>;
}

pub trait BenchMapMutInsert<K, V> {
    fn insert(&mut self, key: K, value: V);
}
pub trait BenchMapMutRemove<K, V> {
    fn remove(&mut self, key: &K) -> Option<V>;
}
