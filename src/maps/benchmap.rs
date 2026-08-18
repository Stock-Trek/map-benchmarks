pub trait BenchMapNew<K, V>: Sized {
    fn new() -> Self;
}
pub trait BenchMapNewWithHasher<K, V, H>: Sized {
    fn new_with_hasher(hasher: H) -> Self;
}
pub trait BenchMapGetCloned<K, V> {
    fn get_cloned(&self, key: &K) -> Option<V>;
}
pub trait BenchMapInsert<K, V> {
    fn insert(&self, key: K, value: V);
}
pub trait BenchMapIter<K, V> {
    fn for_each(&self, f: impl FnMut(&K, &V));
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
