use criterion::{
    Criterion,
    async_executor::{AsyncExecutor, FuturesExecutor},
    criterion_group, criterion_main,
};
use std::{sync::Arc, thread};

fn concurrent_insert_get_remove(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_insert_get_remove");

    let num_threads = 8;
    let ops_per_thread = 10_000;

    let concreadmap = Arc::new(concread::hashmap::HashMap::<String, u64>::new());
    let dashmap = Arc::new(dashmap::DashMap::<String, u64>::with_shard_amount(8));
    let fluxmap = Arc::new(
        FuturesExecutor
            .block_on(fluxmap::db::Database::<String, u64>::new(
                fluxmap::DurabilityLevel::InMemory,
            ))
            .unwrap(),
    );
    let starshardmap = Arc::new(starshard::ShardedHashMap::<String, u64>::new(8));
    let txmap = Arc::new(txmap::prelude::TxMap::<String, u64>::new(
        txmap::prelude::Shards::_8,
    ));

    group.bench_function("concread", |b| {
        b.iter(|| {
            let handles: Vec<_> = (0..num_threads)
                .map(|_| {
                    let map = concreadmap.clone();
                    thread::spawn(move || {
                        for i in 0..ops_per_thread {
                            let key = std::hint::black_box(format!(
                                "key_{:?}_{}",
                                thread::current().id(),
                                i
                            ));
                            let _ = map.write().insert(key.clone(), 42);
                            let _ = map.read().get(&key);
                            let _ = map.write().remove(&key);
                        }
                    })
                })
                .collect();

            for h in handles {
                h.join().unwrap();
            }
        })
    });
    group.bench_function("dashmap", |b| {
        b.iter(|| {
            let handles: Vec<_> = (0..num_threads)
                .map(|_| {
                    let map = dashmap.clone();
                    thread::spawn(move || {
                        for i in 0..ops_per_thread {
                            let key = std::hint::black_box(format!(
                                "key_{:?}_{}",
                                thread::current().id(),
                                i
                            ));
                            let _ = map.insert(key.clone(), 42);
                            let _ = map.get(&key);
                            let _ = map.remove(&key);
                        }
                    })
                })
                .collect();

            for h in handles {
                h.join().unwrap();
            }
        })
    });
    group.bench_function("fluxmap", |b| {
        b.to_async(FuturesExecutor).iter(async || {
            let handles: Vec<_> = (0..num_threads)
                .map(|_| {
                    let map = fluxmap.clone();
                    thread::spawn(async move || {
                        for i in 0..ops_per_thread {
                            let key = std::hint::black_box(format!(
                                "key_{:?}_{}",
                                thread::current().id(),
                                i
                            ));
                            let _ = map.handle().insert(key.clone(), 42).await;
                            let _ = map.handle().get(&key);
                            let _ = map.handle().remove(&key).await;
                        }
                    })
                })
                .collect();

            for h in handles {
                h.join().unwrap().await;
            }
        })
    });
    group.bench_function("starshard", |b| {
        b.iter(|| {
            let handles: Vec<_> = (0..num_threads)
                .map(|_| {
                    let map = starshardmap.clone();
                    thread::spawn(move || {
                        for i in 0..ops_per_thread {
                            let key = std::hint::black_box(format!(
                                "key_{:?}_{}",
                                thread::current().id(),
                                i
                            ));
                            let _ = map.insert(key.clone(), 42);
                            let _ = map.get(&key);
                            let _ = map.remove(&key);
                        }
                    })
                })
                .collect();

            for h in handles {
                h.join().unwrap();
            }
        })
    });
    group.bench_function("txmap", |b| {
        b.iter(|| {
            let handles: Vec<_> = (0..num_threads)
                .map(|_| {
                    let map = txmap.clone();
                    thread::spawn(move || {
                        for i in 0..ops_per_thread {
                            let key = std::hint::black_box(format!(
                                "key_{:?}_{}",
                                thread::current().id(),
                                i
                            ));
                            let _ = map.insert(key.clone(), 42);
                            let _ = map.get_copied(&key);
                            let _ = map.remove(&key);
                        }
                    })
                })
                .collect();

            for h in handles {
                h.join().unwrap();
            }
        })
    });
}

criterion_group!(group, concurrent_insert_get_remove);
criterion_main!(group);
