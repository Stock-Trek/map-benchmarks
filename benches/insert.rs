use criterion::{Criterion, criterion_group, criterion_main};

fn insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert");

    let concreadmap = concread::hashmap::HashMap::<&str, u64>::new();
    let dashmap = dashmap::DashMap::<&str, u64>::with_shard_amount(8);
    let starshardmap = starshard::ShardedHashMap::<&str, u64>::new(8);
    let txmap = txmap::prelude::TxMap::<&str, u64>::new(txmap::prelude::Shards::_8);

    group.bench_function("concread", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key");
            concreadmap.write().insert(key, 42);
        });
    });
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
