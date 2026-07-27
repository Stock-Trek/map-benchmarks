use criterion::{
    Criterion,
    async_executor::{AsyncExecutor, FuturesExecutor},
    criterion_group, criterion_main,
};

fn insert_get_remove(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert_get_remove");

    let concreadmap = concread::hashmap::HashMap::<String, u64>::new();
    let dashmap = dashmap::DashMap::<String, u64>::with_shard_amount(8);
    let fluxmap = FuturesExecutor
        .block_on(fluxmap::db::Database::<String, u64>::new(
            fluxmap::DurabilityLevel::InMemory,
        ))
        .unwrap();
    let starshardmap = starshard::ShardedHashMap::<String, u64>::new(8);
    let txmap = txmap::prelude::TxMap::<String, u64>::new(txmap::prelude::Shards::_8);

    group.bench_function("concread", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            concreadmap.write().insert(key.clone(), 42);
            let _ = concreadmap.read().get(&key);
            let _ = concreadmap.write().remove(&key);
        });
    });
    group.bench_function("dashmap", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            dashmap.insert(key.clone(), 42);
            let _ = dashmap.get(&key);
            let _ = dashmap.remove(&key);
        });
    });
    group.bench_function("fluxmap", |b| {
        b.to_async(FuturesExecutor).iter(|| {
            let key = std::hint::black_box("key".to_string());
            async {
                fluxmap.handle().insert(key.clone(), 42).await.unwrap();
                let _ = fluxmap.handle().get(&key).unwrap();
                let _ = fluxmap.handle().insert(key, 42).await.unwrap();
            }
        });

        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            dashmap.insert(key.clone(), 42);
            let _ = dashmap.get(&key);
            let _ = dashmap.remove(&key);
        });
    });
    group.bench_function("starshard", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            starshardmap.insert(key.clone(), 42);
            let _ = starshardmap.get(&key);
            let _ = starshardmap.remove(&key);
        });
    });
    group.bench_function("txmap", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            txmap.insert(key.clone(), 42);
            let _ = txmap.get_copied(&key);
            let _ = txmap.remove(&key);
        });
    });
}

criterion_group!(group, insert_get_remove);
criterion_main!(group);
