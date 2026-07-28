pub trait BenchMap<K, V>: Sized {
    fn new() -> Self;
    fn insert(&mut self, key: K, value: V);
    fn get_cloned(&mut self, key: &K) -> Option<V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}
