use criterion::{Criterion, criterion_group, criterion_main};
use dashmap::DashMap;
use starshard::ShardedHashMap;
use txmap::prelude::*;

fn get(c: &mut Criterion) {
    let mut group = c.benchmark_group("get");

    let dashmap = DashMap::<String, u64>::with_shard_amount(8);
    let starshardmap = ShardedHashMap::<String, u64>::new(8);
    let txmap = TxMap::<String, u64>::new(Shards::_8);

    dashmap.insert("key".to_string(), 42);
    starshardmap.insert("key".to_string(), 42);
    txmap.insert("key".to_string(), 42);

    group.bench_function("dashmap", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            let _ = dashmap.get(&key);
        });
    });
    group.bench_function("starshard", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            let _ = starshardmap.get(&key);
        });
    });
    group.bench_function("txmap", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            let _ = txmap.get_copied(&key);
        });
    });
}

criterion_group!(group, get);
criterion_main!(group);
