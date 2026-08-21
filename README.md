# Map benchmarks

Benchmarks 19 of 21* map implementations

[AI generated executive summary can be found here](./EXECUTIVE_SUMMARY.md)

[AI generated benchmark report can be found here](./BENCHMARK_REPORT.md)

[Criterion report can be found here](https://stock-trek.github.io/map-benchmarks)

## Methodology

All benchmarks use blackbox to avoid any overly aggressive compiler optimisations.

19 of 21* map implementations are benchmarked:

- [ahash::AHashMap](https://crates.io/crates/ahash)
- [std::collections::btreemap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
- ~~[concread::hashmap::HashMap](https://crates.io/crates/concread)~~
- [concurrent_map::ConcurrentMap](https://crates.io/crates/concurrent-map)
- [crossbeam_skiplist::SkipMap](https://crates.io/crates/crossbeam-skiplist)
- [dashmap::DashMap](https://crates.io/crates/dashmap)
- ~~[flurry::HashMap](https://crates.io/crates/flurry)~~
- [hashbrown::HashMap](https://crates.io/crates/hashbrown)
- [hashlink::LinkedHashMap](https://crates.io/crates/hashlink)
- [horde::SyncTable](https://crates.io/crates/horde)
- [immutable_chunkmap::map::MapM](https://crates.io/crates/immutable-chunkmap)
- [imbl::HashMap](https://crates.io/crates/imbl)
- [indexmap::IndexMap](https://crates.io/crates/indexmap)
- [leapfrog::LeapMap](https://crates.io/crates/leapfrog)
- [papaya::HashMap](https://crates.io/crates/papaya)
- [rustc_hash::FxHashMap](https://crates.io/crates/rustc-hash)
- [rpds::HashTrieMap](https://crates.io/crates/rpds)
- [scc::HashMap](https://crates.io/crates/scc)
- [starshard::ShardedHashMap](https://crates.io/crates/starshard)
- [std::collections::HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [txmap::TxMap](https://crates.io/crates/txmap)

*`concread` and `flurry` were both found to be substantially slower than the others so were removed from the benchmarks.

There are 3 groups of tests, `out-of-the-box` which uses each map's default implementation for more realistic "real-world" performance, `same-hasher` which uses the same hasher for more accurate, "fairer" performance, and `both` which contains benchmarks that are run for both types of comparison.

### Out of the box

- **Clone**: Clones maps containing 1K/10K/100K entries
- **Clone then write**: Clones maps containing 1K/10K/100K entries, then inserts 10% new entries into the clone
- **Workload**: Uses a map with 1K/10K/100K entries. Use cases: [write-heavy, balanced, read-heavy]. Thread counts: [1, 2, 3]. Threads for concurrent tests are pinned to reduce any effects from OS scheduling.
- **Contention (concurrent)**: Uses a map with a dense 10K-key working set so the 3 threads repeatedly hit the same keys. 80% reads / 20% writes. Query key distributions drawn from the dense key set: [uniform, zipfian (exponent 1), zipfian (exponent 2)]. Threads are pinned to reduce any effects from OS scheduling.
- **Get or insert (concurrent)**: Uses a map with 1K/10K/100K entries. The "get-or-create cache entry" pattern: 90% of operations hit existing keys, 10% insert missing keys. Thread counts: [2, 3]. Threads are pinned to reduce any effects from OS scheduling.
- **Synchronization (concurrent)**: Uses a map with a single entry. All 3 threads contend on the same key. Workloads: [read-only, read-mostly (80/20), read-majority (60/40), write-majority (40/60), write-mostly (20/80), write-only]. Threads are pinned to reduce any effects from OS scheduling.

### Same hasher

- **Key sensitivity**: Uses a map containing 10K entries. Keys tested: [u64, Strings of length 16 and 128]

### Both

- **Clear and reuse**: Uses maps containing 1K/10K/100K entries. Clears the map but keeps it alive, then re-inserts the same number of entries. Measures capacity-retention semantics (map pooling)
- **Insert**: Inserts 1K/10K/100K entries into an empty map
- **Iterate**: Uses maps containing 1K/10K/100K entries. Iterates through each entry
- **Lookup hit**: Uses maps containing 1K/10K/100K entries. Finds 100 extant values, found values are cloned to ensure all maps are treated consistently
- **Lookup miss**: Uses maps containing 1K/10K/100K entries. Finds 100 non-existent values
- **Remove**: Uses maps containing 1K/10K/100K entries. Removes 100 entries
- **Get or insert**: Uses maps containing 1K/10K/100K entries. Performs 100 get-or-insert operations on extant keys (hit path) and 100 on missing keys (insert path)
