# AI generated summary: Tue Aug 18 15:32:29 UTC 2026


All data gathered. I have the complete Criterion measurements (only `bulk-clear` has results) and have reviewed all wrapper implementations. Here is the analysis.

## Executive Summary

Only the `bulk-clear` benchmark has Criterion measurements (map sizes 100 and 10,000, `u64→u64`, Copy keys/values). At size 100 the hash-table family dominates: ahash is nominally fastest (22.2 ns per clear) with rustc-hash, hashbrown, std, and indexmap all within 4% — statistically tied given ~5–6% standard deviation at that scale. At size 10,000, horde (114 ns) edges out the same hash maps, which stay within 10%. horde implements clear as table replacement, winning on large maps but paying fixed overhead at small sizes (2.0× slower at 100). All concurrent maps are 9×–6800× slower for bulk clear, as expected for sharded/lock-based designs; starshard is the least bad. No data exists for insert, lookup, iterate, remove, or concurrency workloads — conclusions cover clear only.

## Benchmark Scope

- **Operation with data**: `bulk-clear` (group `bulk-clear/out-of-the-box`) — time to clear a fully populated map, one clear per iteration; map drop occurs inside the timed region.
- **Map sizes**: 100 and 10,000 entries (from `BULK_CLEAR_ENTRY_COUNT`).
- **Implementations benchmarked (14)**: ahash, btreemap, concread, dashmap, flurry, hashbrown, horde, indexmap, papaya, rustc-hash, scc, starshard, std, txmap.
- **Excluded from this benchmark**: leapfrog (no `clear` method — commented out in `benches/bulk_clear.rs`) and immutable_chunkmap (no `BenchMapMutClear` impl).
- **Bench files with NO Criterion data**: clone, concurrency, create, insert, iterate, key_sensitivity, lookup_hit, lookup_miss, mixed_read_write, remove. Only `bulk-clear` was executed.
- **Criterion config**: 1 s warm-up, 2 s measurement, `BatchSize::PerIteration`, `Throughput::Elements`.

## Fastest per Operation

All timings are `median.point_estimate` per clear operation (ns → µs/ms where noted).

| Operation | Fastest | Fastest time | Close contenders (≤10% of fastest) |
|---|---|---|---|
| bulk-clear, size 100 | **ahash** | 22.2 ns | rustc-hash 22.3 ns (1.002×), hashbrown 22.6 ns (1.016×), std 23.0 ns (1.033×), indexmap 23.0 ns (1.036×) |
| bulk-clear, size 10,000 | **horde** | 114 ns | ahash 120 ns (1.053×), rustc-hash 121 ns (1.057×), hashbrown 121 ns (1.058×), indexmap 122 ns (1.071×), std 124 ns (1.084×) |

- Size 100: next after the tied group is horde at 44.7 ns (2.0×) — the top 5 are **practically equivalent**; nobody else is close.
- Size 10,000: the same five hash maps are within 10% of horde. Next is starshard at 459 ns (4.0×).
- Remaining field at size 10,000: txmap 3.93 µs, scc 50.0 µs, btreemap 56.7 µs, dashmap 79.3 µs, concread 243 µs, papaya 513 µs, flurry 782 µs.

## Implementation Ranking Summary

| Implementation | # fastest | # within 10% | Notes |
|---|---|---|---|
| ahash | 1 (size 100) | 1 | Fastest @100; within 10% @10k |
| horde | 1 (size 10k) | 0 | Fastest @10k but 2.0× slower @100 (table-replacement clear) |
| hashbrown | 0 | 2 | In top tier at both sizes |
| rustc-hash | 0 | 2 | In top tier at both sizes; noisy @10k |
| std | 0 | 2 | In top tier at both sizes; noisiest @10k |
| indexmap | 0 | 2 | In top tier at both sizes |
| starshard | 0 | 0 | Best concurrent map: 8.9× / 4.0× of fastest |
| btreemap | 0 | 0 | 12.8× / 496× |
| scc | 0 | 0 | 22.6× / 438× |
| dashmap | 0 | 0 | 42× / 694× |
| txmap | 0 | 0 | 125× / 34× |
| concread | 0 | 0 | 107× / 2128× |
| papaya | 0 | 0 | 158× / 4488× |
| flurry | 0 | 0 | 210× / 6847× |

**Patterns**: the single-threaded hash maps (ahash/hashbrown/rustc-hash/std/indexmap) are the consistent fast group across both sizes. horde is the only clear-specific trade-off: replacement-based clear scales better at large sizes. Every concurrent map is dramatically slower at clear, with starshard least affected. No implementation other than the hash group is ever close to fastest.

## High-Variance Benchmarks

Relative variability (`std_dev` and `median_abs_dev` as % of median):

- **std @ size 10,000**: SD 21.2 ns = **17.1%** of median (CI up to 33.5 ns) — worst of the fast group; flag.
- **rustc-hash @ size 10,000**: MAD 9.0 ns (7.4%), SD 14.3 ns (**11.9%**) — noisy and it is a top-tier map.
- **hashbrown @ size 10,000**: MAD 7.6 ns (6.3%), SD 11.0 ns (9.1%).
- **horde @ size 10,000**: MAD 5.3 ns (4.7%), SD 9.9 ns (8.7%) — matters because horde is the winner here.
- **txmap @ size 10,000**: MAD 284 ns (7.2%).
- **btreemap @ size 100**: SD 39.3 ns (13.8%).
- **All top-tier maps @ size 100**: SD ≈ 5.3–6.6% on 22–23 ns medians — the 0.04–0.8 ns gaps between ahash/rustc-hash/hashbrown/std/indexmap are below measurement noise; their ordering at size 100 is not statistically meaningful.

## Implementation Limitations

Based on `src/maps/` wrapper code:

- **ahash**: requires `K: Hash + Eq`; **`V: Clone` is required even for insert** (wrapper-level bound); supports custom hasher (`BenchMapNewWithHasher`); default `ahash::RandomState`.
- **hashbrown**: `K: Hash + Eq` for get/insert/remove; `V: Clone` only for `get_cloned`/`clone`; supports custom hasher. Most flexible hash map.
- **std**: same shape as hashbrown; default `RandomState` (DoS-resistant SipHash); custom hasher supported.
- **rustc-hash**: `K: Hash + Eq`, `V: Clone` for get/clone; **no custom-hasher support** (no `BenchMapNewWithHasher`; fixed `FxBuildHasher`). FxHash is fast but not collision-hardened.
- **indexmap**: `K: Hash + Eq`; custom hasher supported (default `RandomState`); **`remove` uses `swap_remove`** (does not preserve insertion order, unlike the other ops); iteration is insertion-ordered.
- **btreemap**: `K: Ord` (no hashing); **`V: Clone` required even for insert**; no hasher concept; sorted iteration.
- **dashmap**: `K: Hash + Eq`; `H: BuildHasher + Clone` required for most ops (DashMap's hasher must be `Clone`); sharded — clear must lock all shards.
- **concread**: heaviest bounds — `K: Clone + Debug + Hash + Eq + Send + Sync + 'static`, `V: Clone + Send + Sync + 'static`; every mutation is a COW write transaction + `commit()`; **no custom hasher, no clone support**.
- **flurry**: `K: Sync + Send + Clone + Hash + Ord` (Ord needed even for clear); `V: Sync + Send (+ Clone for clone/remove)`; **every op requires an explicit `.pin()` guard** (hazard-pointer ergonomics).
- **papaya**: `K: Hash + Eq`, `V: Clone` for get/remove/clone; **every op requires `.pin()` guard**; custom hasher supported (default `RandomState`).
- **scc**: `K: Hash + Eq`; uses synchronous `*_sync` variants (no guards); custom hasher supported (default `RandomState`); `clear_sync` on 10k entries ≈ 50 µs.
- **starshard**: `K: Clone + Hash + Eq + Send + Sync`, `V: Clone + Send + Sync`, `H: BuildHasher + Clone + Send + Sync`; **hardcoded 8 shards**; **defaults to `FxBuildHasher`** (rustc-hash); `get`/`remove` return owned clones.
- **txmap**: `K: Clone + Hash + Eq`; `V: Clone` for `get_cloned`; `MutexPolicy` (single-writer transactional); custom hasher supported.
- **leapfrog** (not in this benchmark): `K: Eq + Hash + Copy`, `V: leapfrog::Value` (Copy-like) — **no `clear` API**, so excluded; no clone support.
- **immutable_chunkmap** (not in this benchmark): `K: Clone + Ord`, `V: Clone`; COW semantics (`insert_cow`/`remove_cow`); **no `clear` impl**, so excluded.

## Recommendations

- **Run the remaining benchmark files** (insert, lookup_hit/miss, iterate, remove, clone, mixed_read_write, concurrency, key_sensitivity, create). Current conclusions rest on a single operation; clear is unrepresentative of normal map workloads (it is O(1)-ish for hash maps with Copy values).
- **Treat the size-100 top-5 as tied.** With SD ≈ 5–6% on 22–23 ns medians, the 1 ns differences are noise. Increase `MEASUREMENT_TIME`/`sample_size` for ns-scale ops before drawing ranking conclusions.
- **Investigate noise in std and rustc-hash at size 10,000** (SD 17% and 12% of median) — possible allocator/frequency jitter; re-measure with longer windows.
- **Verify horde's clear semantics** (`write().replace(empty, 0)`): constant-time table replacement explains the size-100 (2.0×) vs size-10k (1.0×) flip. Confirm what is freed inside the timed region.
- **Note that the map drop is inside the timed iteration** (`black_box(map)` then implicit drop in the closure). With Copy `u64` values drop cost is ~nil; results will not transfer to drop-heavy value types. Add such a workload.
- **Relax wrapper bounds** where possible: ahash and btreemap require `V: Clone` for insert, concread requires `Send + Sync + 'static`, flurry requires `K: Ord` — these will block future workloads (e.g., non-Clone values).
- **Tune starshard's hardcoded 8 shards and confirm the FxBuildHasher default** before treating its numbers as representative of the library's tuning.
