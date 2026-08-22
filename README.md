# Map benchmarks

Benchmarks 20 of 22* map implementations

[AI generated executive summary can be found here](./EXECUTIVE_SUMMARY.md)

[AI generated benchmark report can be found here](./BENCHMARK_REPORT.md)

[Criterion report can be found here](https://stock-trek.github.io/map-benchmarks)

## Methodology

All benchmarks use blackbox to avoid any overly aggressive compiler optimisations.

20 of 22* map implementations are benchmarked:

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
- [intmap::IntMap](https://crates.io/crates/intmap)
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
- **Create**: Creates 10K new empty maps. Measures the fixed cost of construction, eager vs lazy allocation and per-map setup overhead.
- **Throughput (serial)**: Uses a map with 10K entries. Use cases: [write-heavy, balanced, read-heavy]. 1 thread.

### Same hasher

- **Key sensitivity**: Uses a map containing 10K entries. Keys tested: [u64, Strings of length 16 and 128]

### Both

- **Clear and reuse**: Uses maps containing 1K/10K/100K entries. Clears the map but keeps it alive, then re-inserts the same number of entries. Measures capacity-retention semantics (map pooling)
- **Contention (concurrent)**: Uses a map with 10K entries. Performs 80% reads / 20% writes on 3 threads using query key distributions of: [uniform, zipfian (exponent 1), zipfian (exponent 2)]. Threads are pinned to reduce any effects from OS scheduling.
- **Get or insert**: Uses a map containing 10K entries. Performs 10K get-or-insert operations on extant keys (hit path) and 10K on missing keys (insert path). Also concurrent: the "get-or-create cache entry" pattern with 90% of operations hitting existing keys and 10% inserting missing keys on threads [2, 3]. Threads are pinned to reduce any effects from OS scheduling.
- **Growth**: Inserts 1K/10K/100K/1M entries into an empty map using u64 sparse keys on a single thread. Measures the cost of growing a map to the target size
- **Insert**: Uses a map containing 10K entries. Inserts 10K new entries
- **Iterate**: Uses maps containing 1K/10K/100K entries. Iterates through each entry
- **Lookup hit**: Uses a map containing 10K entries. Finds 100 extant values, found values are cloned to ensure all maps are treated consistently
- **Lookup miss**: Uses a map containing 10K entries. Finds 100 non-existent values
- **Remove**: Uses a map containing 10K entries. Removes 100 entries
- **Synchronization (concurrent)**: Uses a map with a single entry. Performs workloads on 3 threads targeting the same key. Workloads: [read-only, read-mostly (80/20), read-majority (60/40), write-majority (40/60), write-mostly (20/80), write-only]. Threads are pinned to reduce any effects from OS scheduling.
- **Throughput (concurrent)**: Uses a map with 10K entries. Use cases: [write-heavy, balanced, read-heavy]. Thread counts: [2, 3]. Threads are pinned to reduce any effects from OS scheduling.
