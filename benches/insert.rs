use criterion::{Criterion, criterion_group, criterion_main};
use dashmap::DashMap;
use starshard::ShardedHashMap;
use txmap::prelude::*;

fn insert(c: &mut Criterion) {
    let dashmap = DashMap::with_shard_amount(8);
    let starshardmap = ShardedHashMap::new(8);
    let txmap = TxMap::new(Shards::_8);

    c.bench_function("dashmap_insert", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key");
            dashmap.insert(key, 42);
        });
    });
    c.bench_function("starshardmap_insert", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key");
            starshardmap.insert(key, 42);
        });
    });
    c.bench_function("txmap_insert", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key");
            txmap.insert(key, 42);
        });
    });
}

criterion_group!(benches, insert);
criterion_main!(benches);
