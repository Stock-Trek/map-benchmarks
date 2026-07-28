use criterion::{Criterion, criterion_group, criterion_main};

fn insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("baseline/insert");

    for size in [10_000, 100_000, 1_000_000] {}

    let concreadmap = concread::hashmap::HashMap::<String, u64>::new();
    let dashmap = dashmap::DashMap::<String, u64>::with_shard_amount(8);
    let mut hashbrownmap = hashbrown::HashMap::<String, u64>::new();
    let immutable_chunkmap = immutable_chunkmap::map::MapL::<String, u64>::new();
    let starshardmap = starshard::ShardedHashMap::<String, u64>::new(8);
    let txmap = txmap::prelude::TxMap::<String, u64>::new(txmap::prelude::Shards::_8);

    group.bench_function("concread", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            concreadmap.write().insert(key, 42);
        });
    });
    group.bench_function("dashmap", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            dashmap.insert(key, 42);
        });
    });
    group.bench_function("hashbrown", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            hashbrownmap.insert(key, 42);
        });
    });
    group.bench_function("immutable_chunkmap", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            immutable_chunkmap.insert(key, 42);
        });
    });
    group.bench_function("starshard", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            starshardmap.insert(key, 42);
        });
    });
    group.bench_function("txmap", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            txmap.insert(key, 42);
        });
    });
}

criterion_group!(group, insert);
criterion_main!(group);
