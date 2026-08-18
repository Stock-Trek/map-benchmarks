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

There are 3 groups of tests, `out-of-the-box` which uses each map's default implementation for more realistic "real-world" performance, `same-hasher` which uses the same hasher for more accurate, "fairer" performance, and `both` which contains benchmarks that are run for both types of comparison.

### Out of the box

- **Key sensitivity**: Uses a map containing 100K entries. Keys tested: [u64, 32 byte array, UUID, Strings of length 16, 64 and 256]
- **Create**: Creates 1M new, empty maps

### Same hasher

- **Lookup hit**: Uses maps containing 10k/100K/1M entries. Finds 100 extant values, found values are cloned to ensure all maps are treated consistently
- **Lookup miss**: Uses maps containing 10k/100K/1M entries. Finds 100 non-existent values
- **Mixed read/write**: Uses a map with 10K/100K/1M entries. Use cases: [write-heavy, high-churn, balanced, read-heavy]
- **Remove**: Uses maps containing 10k/100K/1M entries. Removes 100 entries
- **Iterate**: Uses maps containing 10k/100K/1M entries. Iterates through each entry
- **Concurrency**: Uses a map with 1M entries and tests the balanced workload. Thread counts: [1, 2, 4]. Threads are pinned to reduce any effects from OS scheduling.

### Both

- **Insert**: Inserts 10K/100K/1M entries into an empty map
