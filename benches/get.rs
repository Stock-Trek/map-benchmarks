use criterion::{
    Criterion,
    // async_executor::{AsyncExecutor, FuturesExecutor},
    criterion_group,
    criterion_main,
};

fn get(c: &mut Criterion) {
    let mut group = c.benchmark_group("get");

    let concreadmap = concread::hashmap::HashMap::<String, u64>::new();
    let dashmap = dashmap::DashMap::<String, u64>::with_shard_amount(8);
    // let fluxmap = FuturesExecutor
    //     .block_on(fluxmap::db::Database::<String, u64>::new(
    //         fluxmap::DurabilityLevel::InMemory,
    //     ))
    //     .unwrap();
    let mut hashbrownmap = hashbrown::HashMap::<String, u64>::new();
    let immutable_chunkmap = immutable_chunkmap::map::MapL::<String, u64>::new();
    let starshardmap = starshard::ShardedHashMap::<String, u64>::new(8);
    let txmap = txmap::prelude::TxMap::with_lock_policy::<txmap::prelude::RwLockPolicy>(
        txmap::prelude::Shards::_8,
    );

    concreadmap.write().insert("key".to_string(), 42);
    dashmap.insert("key".to_string(), 42);
    // FuturesExecutor.block_on(async {
    //     fluxmap
    //         .handle()
    //         .insert("key".to_string(), 42)
    //         .await
    //         .unwrap();
    // });
    immutable_chunkmap.insert("key".to_string(), 42);
    hashbrownmap.insert("key".to_string(), 42);
    starshardmap.insert("key".to_string(), 42);
    txmap.insert("key".to_string(), 42);

    group.bench_function("concread", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            let _ = concreadmap.read().get(&key);
        });
    });
    group.bench_function("dashmap", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            let _ = dashmap.get(&key);
        });
    });
    // group.bench_function("fluxmap", |b| {
    //     b.iter(|| {
    //         let key = std::hint::black_box("key".to_string());
    //         let _ = fluxmap.handle().get(&key).expect("Cannot get from fluxmap");
    //     });
    // });
    group.bench_function("hashbrown", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            let _ = hashbrownmap.get(&key);
        });
    });
    group.bench_function("immutable_chunkmap", |b| {
        b.iter(|| {
            let key = std::hint::black_box("key".to_string());
            let _ = immutable_chunkmap.get(&key);
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
