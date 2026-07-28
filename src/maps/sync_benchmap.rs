/// Trait for map implementations that support concurrent access.
/// Unlike `BenchMap` which uses `&mut self`, this trait uses `&self`
/// since concurrent maps use interior mutability.
pub trait SyncBenchMap<K, V>: Send + Sync + Sized {
    fn new() -> Self;
    fn insert(&self, key: K, value: V);
    fn get_cloned(&self, key: &K) -> Option<V>;
    fn remove(&self, key: &K) -> Option<V>;
}
