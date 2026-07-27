use criterion::{
    Criterion,
    // async_executor::{AsyncExecutor, FuturesExecutor},
    criterion_group,
    criterion_main,
};
use std::{sync::Arc, thread};

fn concurrent_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_get");

    let num_threads = 8;
    let ops_per_thread = 10_000;

    let concreadmap = Arc::new(concread::hashmap::HashMap::<String, u64>::new());
    let dashmap = Arc::new(dashmap::DashMap::<String, u64>::with_shard_amount(8));
    // let fluxmap = Arc::new(
    //     FuturesExecutor
    //         .block_on(fluxmap::db::Database::<String, u64>::new(
    //             fluxmap::DurabilityLevel::InMemory,
    //         ))
    //         .unwrap(),
    // );
    let immutable_chunkmap = Arc::new(immutable_chunkmap::map::MapL::<String, u64>::new());
    let starshardmap = Arc::new(starshard::ShardedHashMap::<String, u64>::new(8));
    let txmap = Arc::new(txmap::prelude::TxMap::with_lock_policy::<
        txmap::prelude::RwLockPolicy,
    >(txmap::prelude::Shards::_8));

    for i in 0..num_threads {
        let key = std::hint::black_box(format!("key_{}", i));
        dashmap.insert(key.clone(), 42);
        // FuturesExecutor.block_on(async {
        //     fluxmap.handle().insert(key.clone(), 42).await.unwrap();
        // });
        starshardmap.insert(key.clone(), 42);
        txmap.insert(key.clone(), 42);
    }

    group.bench_function("concread", |b| {
        b.iter(|| {
            let handles: Vec<_> = (0..num_threads)
                .map(|_| {
                    let map = concreadmap.clone();
                    thread::spawn(move || {
                        for i in 0..ops_per_thread {
                            let key = std::hint::black_box(format!("key_{}", i));
                            let _ = map.read().get(&key);
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
                            let key = std::hint::black_box(format!("key_{}", i));
                            let _ = map.get(&key);
                        }
                    })
                })
                .collect();

            for h in handles {
                h.join().unwrap();
            }
        })
    });
    // group.bench_function("fluxmap", |b| {
    //     b.iter(|| {
    //         let handles: Vec<_> = (0..num_threads)
    //             .map(|_| {
    //                 let map = fluxmap.clone();
    //                 thread::spawn(move || {
    //                     for i in 0..ops_per_thread {
    //                         let key = std::hint::black_box(format!("key_{}", i));
    //                         let _ = map.handle().get(&key);
    //                     }
    //                 })
    //             })
    //             .collect();

    //         for h in handles {
    //             h.join().unwrap();
    //         }
    //     })
    // });
    group.bench_function("immutable_chunkmap", |b| {
        b.iter(|| {
            let handles: Vec<_> = (0..num_threads)
                .map(|_| {
                    let map = immutable_chunkmap.clone();
                    thread::spawn(move || {
                        for i in 0..ops_per_thread {
                            let key = std::hint::black_box(format!("key_{}", i));
                            let _ = map.get(&key);
                        }
                    })
                })
                .collect();

            for h in handles {
                h.join().unwrap();
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
                            let key = std::hint::black_box(format!("key_{}", i));
                            let _ = map.get(&key);
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
                            let key = std::hint::black_box(format!("key_{}", i));
                            let _ = map.get_copied(&key);
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

criterion_group!(group, concurrent_get);
criterion_main!(group);
