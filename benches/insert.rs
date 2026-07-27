use criterion::{Criterion, criterion_group, criterion_main};
use dashmap::DashMap;
use starshard::ShardedHashMap;
use txmap::prelude::*;

fn insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert");

    let dashmap = DashMap::with_shard_amount(8);
    let starshardmap = ShardedHashMap::new(8);
    let txmap = TxMap::new(Shards::_8);

    group.bench_function("dashmap", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key");
            dashmap.insert(key, 42);
        });
    });
    group.bench_function("starshard", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key");
            starshardmap.insert(key, 42);
        });
    });
    group.bench_function("txmap", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key");
            txmap.insert(key, 42);
        });
    });
}

criterion_group!(group, insert);
criterion_main!(group);
