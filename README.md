# Map benchmarks

Benchmarks 20 map implementations

[Criterion report can be found here](https://stock-trek.github.io/map-benchmarks)

[AI generated summary can be found here](./SUMMARY.md)

## Methodology

All benchmarks use blackbox to avoid any overly aggressive compiler optimisations.

20 map implementations are benchmarked:

- [ahash::AHashMap](https://crates.io/crates/ahash)
- [std::collections::btreemap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
- [concread::hashmap::HashMap](https://crates.io/crates/concread)
- [concurrent_map::ConcurrentMap](https://crates.io/crates/concurrent-map)
- [crossbeam_skiplist::SkipMap](https://crates.io/crates/crossbeam-skiplist)
- [dashmap::DashMap](https://crates.io/crates/dashmap)
- [flurry::HashMap](https://crates.io/crates/flurry)
- [hashbrown::HashMap](https://crates.io/crates/hashbrown)
- [hashlink::LinkedHashMap](https://crates.io/crates/hashlink)
- [horde::SyncTable](https://crates.io/crates/horde)
- [immutable_chunkmap::map::MapM](https://crates.io/crates/immutable-chunkmap)
- [imbl::HashMap](https://crates.io/crates/imbl)
- [indexmap::IndexMap](https://crates.io/crates/indexmap)
- [leapfrog::LeapMap](https://crates.io/crates/leapfrog)
- [papaya::HashMap](https://crates.io/crates/papaya)
- [rustc_hash::FxHashMap](https://crates.io/crates/rustc-hash)
- [scc::HashMap](https://crates.io/crates/scc)
- [starshard::ShardedHashMap](https://crates.io/crates/starshard)
- [std::collections::HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [txmap::TxMap](https://crates.io/crates/txmap)

There are 3 groups of tests, `out-of-the-box` which uses each map's default implementation for more realistic "real-world" performance, `same-hasher` which uses the same hasher for more accurate, "fairer" performance, and `both` which contains benchmarks that are run for both types of comparison.

### Out of the box

- **Clone**: Clones maps containing 1K/10K/100K entries
- **Workload**: Uses a map with 1K/10K/100K entries. Use cases: [write-heavy, balanced, read-heavy]. Thread counts: [1, 2, 3, 4]. Threads for concurrent tests are pinned to reduce any effects from OS scheduling.

### Same hasher

- **Key sensitivity**: Uses a map containing 10K entries. Keys tested: [u64, Strings of length 16 and 128]

### Both

- **Insert**: Inserts 1K/10K/100K entries into an empty map
- **Iterate**: Uses maps containing 1K/10K/100K entries. Iterates through each entry
- **Lookup hit**: Uses maps containing 1K/10K/100K entries. Finds 100 extant values, found values are cloned to ensure all maps are treated consistently
- **Lookup miss**: Uses maps containing 1K/10K/100K entries. Finds 100 non-existent values
- **Remove**: Uses maps containing 1K/10K/100K entries. Removes 100 entries

