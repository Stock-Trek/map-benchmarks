use criterion::{Criterion, criterion_group, criterion_main};
use dashmap::DashMap;
use starshard::ShardedHashMap;
use txmap::prelude::*;

fn insert_remove(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert_remove");

    let dashmap = DashMap::<String, u64>::with_shard_amount(8);
    let starshardmap = ShardedHashMap::<String, u64>::new(8);
    let txmap = TxMap::<String, u64>::new(Shards::_8);

    group.bench_function("dashmap", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            dashmap.insert("key".to_string(), 42);
            let _ = dashmap.get(&key);
            let _ = dashmap.remove(&key);
        });
    });
    group.bench_function("starshard", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            starshardmap.insert("key".to_string(), 42);
            let _ = starshardmap.get(&key);
            let _ = starshardmap.remove(&key);
        });
    });
    group.bench_function("txmap", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            txmap.insert("key".to_string(), 42);
            let _ = txmap.get_copied(&key);
            let _ = txmap.remove(&key);
        });
    });
}

criterion_group!(group, insert_remove);
criterion_main!(group);
