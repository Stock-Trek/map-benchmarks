use crate::maps::*;
use concurrent_map::{ConcurrentMap, Minimum};
use std::{
    any::Any,
    cell::RefCell,
    sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    },
};

/// concurrent-map's `ConcurrentMap` is `Send` but not `Sync`: its `ebr`
/// dependency keeps the epoch-based reclamation state in a `RefCell`, so a
/// single `&ConcurrentMap` cannot be shared across threads (and forcing it
/// would panic on `RefCell` borrow conflicts). The crate's intended usage
/// (see its own concurrent tests) is to *clone* the map per thread: clones
/// share the underlying tree through an internal `Arc` but give each thread
/// its own EBR instance.
///
/// This wrapper is `Send + Sync` and follows that pattern: the map populated
/// during setup is kept as a "master", and each thread lazily clones it into
/// a thread-local (once per benchmark iteration, detected via a per-instance
/// token) so it always operates on its own private `ConcurrentMap`. Cloning
/// the master is serialized by the mutex, so the master's non-`Sync` EBR
/// state is only ever touched by one thread at a time.
pub struct ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    /// The master map, populated during setup. Worker threads clone it into
    /// a thread-local once per iteration; the clones share the tree via the
    /// crate's internal `Arc`.
    master: Mutex<ConcurrentMap<K, V>>,
    /// Identity of this wrapper instance. A thread re-clones the master when
    /// the token differs, i.e. the harness published a fresh map for the next
    /// iteration.
    token: u64,
}

thread_local! {
    /// This thread's private clone of the current master map, plus the token
    /// of the wrapper it was cloned from. Type-erased so the wrapper can be
    /// generic over `K`/`V`; the stored type always matches the wrapper whose
    /// token is stored alongside it.
    static THREAD_MAP: RefCell<Option<(u64, Box<dyn Any + Send>)>> = const { RefCell::new(None) };
}

static NEXT_TOKEN: AtomicU64 = AtomicU64::new(0);

fn next_token() -> u64 {
    NEXT_TOKEN.fetch_add(1, Ordering::Relaxed)
}

impl<K, V> ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    /// Runs `f` against this thread's private clone of the master map,
    /// re-cloning the master (once per benchmark iteration) when the cached
    /// clone belongs to a different wrapper instance.
    fn with_thread_map<R>(&self, f: impl FnOnce(&ConcurrentMap<K, V>) -> R) -> R {
        THREAD_MAP.with(|slot| {
            let mut slot = slot.borrow_mut();
            let is_current = matches!(
                slot.as_ref(),
                Some((token, map))
                    if *token == self.token && map.is::<ConcurrentMap<K, V>>()
            );
            if !is_current {
                let map: Box<dyn Any + Send> = Box::new(self.master.lock().unwrap().clone());
                *slot = Some((self.token, map));
            }
            let (_, map) = slot.as_mut().unwrap();
            f(map.downcast_mut::<ConcurrentMap<K, V>>().unwrap())
        })
    }
}

impl<K, V> BenchMapName for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    const NAME: &'static str = "concurrent-map";
}

impl<K, V> BenchMapNew<K, V> for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn new() -> Self {
        Self {
            master: Mutex::new(ConcurrentMap::new()),
            token: next_token(),
        }
    }
}

impl<K, V> BenchMapClone<K, V> for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn clone_map(&self) -> Self {
        Self {
            master: Mutex::new(self.master.lock().unwrap().clone()),
            token: next_token(),
        }
    }
}

impl<K, V> BenchMapGetCloned<K, V> for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn get_cloned(&self, key: &K) -> Option<V> {
        self.with_thread_map(|map| map.get(key))
    }
}

impl<K, V> BenchMapGetOrInsert<K, V> for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn get_or_insert(&self, key: K, default: V) -> V {
        // concurrent-map has no entry API, so emulate get-or-insert as a get
        // followed by an insert.
        self.with_thread_map(|map| {
            if let Some(value) = map.get(&key) {
                value
            } else {
                map.insert(key, default.clone());
                default
            }
        })
    }
}

impl<K, V> BenchMapMutGetOrInsert<K, V> for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn get_or_insert(&mut self, key: K, default: V) -> V {
        let map = self.master.get_mut().unwrap();
        if let Some(value) = map.get(&key) {
            value
        } else {
            map.insert(key, default.clone());
            default
        }
    }
}

impl<K, V> BenchMapInsert<K, V> for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn insert(&self, key: K, value: V) {
        self.with_thread_map(|map| {
            map.insert(key, value);
        });
    }
}

impl<K, V> BenchMapMutInsert<K, V> for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn insert(&mut self, key: K, value: V) {
        self.master.get_mut().unwrap().insert(key, value);
    }
}

impl<K, V> BenchMapIter<K, V> for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn for_each(&self, mut f: impl FnMut(&K, &V)) {
        self.with_thread_map(|map| {
            for (key, value) in map.iter() {
                f(&key, &value);
            }
        });
    }
}

impl<K, V> BenchMapRemove<K, V> for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn remove(&self, key: &K) -> Option<V> {
        self.with_thread_map(|map| map.remove(key))
    }
}

impl<K, V> BenchMapMutRemove<K, V> for ConcurrentMapBenchMap<K, V>
where
    K: 'static + Clone + Minimum + Send + Sync,
    V: 'static + Clone + Send + Sync,
{
    fn remove(&mut self, key: &K) -> Option<V> {
        self.master.get_mut().unwrap().remove(key)
    }
}
