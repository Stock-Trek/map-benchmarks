# Map benchmarks

Benchmarks 12 map implementations

[Criterion report can be found here](https://stock-trek.github.io/map-benchmarks)

## Methodology

All benchmarks use blackbox to avoid any overly aggressive compiler optimisations.

12 map implementations are benchmarked:

- [ahash::AHashMap](https://crates.io/crates/ahash)
- [std::collections::btreemap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
- [concread::hashmap::HashMap](https://crates.io/crates/concread)
- [dashmap::DashMap](https://crates.io/crates/dashmap)
- [hashbrown::HashMap](https://crates.io/crates/hashbrown)
- [horde::SyncTable](https://crates.io/crates/horde)
- [immutable_chunkmap::map::MapM](https://crates.io/crates/immutable-chunkmap)
- [indexmap::IndexMap](https://crates.io/crates/indexmap)
- [rustc_hash::FxHashMap](https://crates.io/crates/rustc-hash)
- [starshard::ShardedHashMap](https://crates.io/crates/starshard)
- [std::collections::HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [txmap::TxMap](https://crates.io/crates/txmap)

There are 2 types of test, `out-of-the-box` which uses each map's default implementation for more realistic "real-world" performance, and `same-hasher` which uses the same hasher for more accurate, "fairer" performance.

### Out of the box

- **Insert**: Inserts 10K/100K/1M entries into an empty map
- **Key sensitivity**: Uses a map containing 100K entries. Keys tested: [u64, 32 byte array, UUID, Strings of length 16, 64 and 256]
- **Lookup hit**: Uses maps containing 10k/100K/1M entries. Finds 100 extant values, found values are cloned to ensure all maps are treated consistently
- **Lookup miss**: Uses maps containing 10k/100K/1M entries. Finds 100 non-existent values
- **Mixed read/write**: Uses a map with 10K/100K/1M entries. Use cases: [write-heavy, high-churn, balanced, read-heavy]
- **Remove**: Uses maps containing 10k/100K/1M entries. Removes 100 entries

### Same hasher

- **Create**: Creates 1M new, empty maps
- **Iterate**: Uses maps containing 10k/100K/1M entries. Iterates through each entry
- **Concurrency**: Uses a map with 1M entries and tests the balanced workload. Thread counts: [1, 2, 4]. Threads are pinned to reduce any effects from OS scheduling.

If the map implementation offers additional configuration that affects performance, eg. shard count, lock type, these options will be given it's own benchmark (within reason, this cannot reasonably test a combinatorial explosion of options).
