use criterion::{Criterion, criterion_group, criterion_main};
use dashmap::DashMap;
use starshard::ShardedHashMap;
use std::{sync::Arc, thread};
use txmap::prelude::*;

fn concurrent_insert(c: &mut Criterion) {
    let num_threads = 8;
    let ops_per_thread = 10_000;
    let dashmap = Arc::new(DashMap::with_shard_amount(8));
    let starshardmap = Arc::new(ShardedHashMap::new(8));
    let txmap = Arc::new(TxMap::new(Shards::_8));

    c.bench_function("dashmap_concurrent_insert", |b| {
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
                            let _ = map.insert(key, 42);
                        }
                    })
                })
                .collect();

            for h in handles {
                h.join().unwrap();
            }
        })
    });
    c.bench_function("starshardmap_concurrent_insert", |b| {
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
                            let _ = map.insert(key, 42);
                        }
                    })
                })
                .collect();

            for h in handles {
                h.join().unwrap();
            }
        })
    });
    c.bench_function("txmap_concurrent_insert", |b| {
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
                            let _ = map.insert(key, 42);
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

criterion_group!(benches, concurrent_insert);
criterion_main!(benches);
