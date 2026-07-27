use criterion::{Criterion, criterion_group, criterion_main};

fn get(c: &mut Criterion) {
    let mut group = c.benchmark_group("get");

    let concreadmap = concread::hashmap::HashMap::<&str, u64>::new();
    let dashmap = dashmap::DashMap::<&str, u64>::with_shard_amount(8);
    let starshardmap = starshard::ShardedHashMap::<&str, u64>::new(8);
    let txmap = txmap::prelude::TxMap::with_lock_policy::<txmap::prelude::RwLockPolicy>(
        txmap::prelude::Shards::_8,
    );

    concreadmap.write().insert("key", 42);
    dashmap.insert("key", 42);
    starshardmap.insert("key", 42);
    txmap.insert("key", 42);

    group.bench_function("concread", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key");
            let _ = concreadmap.read().get(&key);
        });
    });
    group.bench_function("dashmap", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key");
            let _ = dashmap.get(&key);
        });
    });
    group.bench_function("starshard", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key");
            let _ = starshardmap.get(&key);
        });
    });
    group.bench_function("txmap", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key");
            let _ = txmap.get_copied(&key);
        });
    });
}

criterion_group!(group, get);
criterion_main!(group);
