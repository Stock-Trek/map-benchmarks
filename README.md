# Map benchmarks

Benchmarks 11 map implementations

[Criterion report can be found here](https://stock-trek.github.io/map-benchmarks)

## Methodology

All benchmarks use blackbox to avoid any overly aggressive compiler optimisations.

11 map implementations are benchmarked:

- [ahash::AHashMap](https://crates.io/crates/ahash)
- [std::collections::btreemap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
- [concread::hashmap::HashMap](https://crates.io/crates/concread)
- [dashmap::DashMap](https://crates.io/crates/dashmap)
- [hashbrown::HashMap](https://crates.io/crates/hashbrown)
- [immutable_chunkmap::map::MapM](https://crates.io/crates/immutable-chunkmap)
- [indexmap::IndexMap](https://crates.io/crates/indexmap)
- [rustc_hash::FxHashMap](https://crates.io/crates/rustc-hash)
- [starshard::ShardedHashMap](https://crates.io/crates/starshard)
- [std::collections::HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [txmap::TxMap](https://crates.io/crates/txmap)

### Baseline

- **New**: Creates 1,000,000 new, empty maps
- **Insert**: Inserts 10K/100K/1M entries into an empty map
- **Iterate**: Uses maps containing 10k/100K/1M entries. Iterates through each entry
- **Lookup hit**: Uses maps containing 10k/100K/1M entries. Finds 100 extant values, found values are cloned to ensure all maps are treated consistently
- **Lookup miss**: Uses maps containing 10k/100K/1M entries. Finds 100 non-existent values
- **Remove**: Uses maps containing 10k/100K/1M entries. Removes 100 entries

### Key sensitivity

Uses a map containing 100K entries.
Keys tested are:

- u64
- Byte array, 32 bytes in length
- UUID, from the [uuid crate](https://crates.io/crates/uuid)
- Short String, 16 characters in length
- Medium String, 64 characters in length
- Long String, 256 characters in length

### Mixed read/write

Uses a map with 10K/100K/1M entries.
Benchmarks 4 use cases:

- **Write heavy** [20% lookup, 80% insert]
- **High churn** [50% lookup, 10% insert, 30% update, 10% remove]
- **Balanced** [80% lookup, 5% insert, 10% update, 5% remove]
- **Read heavy** [95% lookup, 5% insert]

### Concurrency

Uses a map with 1M entries.
Uses the balanced case of [80% lookup, 5% insert, 10% update, 5% remove].
Threads are pinned to reduce any effects from OS scheduling.
Benchmarks thread counts.

#### Thread count

- 1
- 2
- 4

If the map implementation offers additional configuration that affects performance, eg. shard count, lock type, these options will be given it's own benchmark (within reason, this cannot reasonably test a combinatorial explosion of options).
