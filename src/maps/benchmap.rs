pub trait BenchMapName {
    const NAME: &'static str;
}
pub trait BenchMapNew<K, V>: Sized {
    fn new() -> Self;
}
pub trait BenchMapNewWithHasher<K, V, H>: Sized {
    fn new_with_hasher(hasher: H) -> Self;
}
pub trait BenchMapClone<K, V>: Sized {
    fn clone_map(&self) -> Self;
}
pub trait BenchMapGetCloned<K, V> {
    fn get_cloned(&self, key: &K) -> Option<V>;
}
pub trait BenchMapGetOrInsert<K, V> {
    fn get_or_insert(&self, key: K, default: V) -> V;
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
pub trait BenchMapMutGetOrInsert<K, V> {
    fn get_or_insert(&mut self, key: K, default: V) -> V;
}
pub trait BenchMapMutRemove<K, V> {
    fn remove(&mut self, key: &K) -> Option<V>;
}
pub trait BenchMapMutClear<K, V> {
    fn clear(&mut self);
}
