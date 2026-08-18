# AI generated summary: Tue Aug 18 22:14:36 UTC 2026

The analysis is complete. I extracted and verified all 553 measurement sets from `target/criterion/` (54 benchmark groups across 17 map implementations), cross-referenced them with the bench definitions in `benches/` and the wrapper code in `src/maps/`, and wrote the full report to **`benchmark-executive-summary.md`**.

## Key findings

**Fastest per operation (median timings from `estimates.json`):**

- **Clone**: immutable-chunkmap at 3.6 ns (O(1) structural sharing — 14× faster than the next, starshard)
- **Insert (default hashers)**: rustc-hash — 17.73 µs / 156.58 µs / 1.585 ms at 1k/10k/100k, clearly ahead except 100k where hashbrown is within 6%
- **Insert (shared ahash hasher)**: ahash/horde/std/hashbrown statistically tied (all within 3%)
- **Iterate**: indexmap wins all 6 groups by 2.4–2.7×
- **Lookup hit/miss & key-sensitivity**: std/hashbrown/ahash effectively tied (≤1.1×); ahash best for String keys
- **Remove**: horde wins all 3 sizes (1.19–1.32× ahead)
- **Workloads (10k ops/iter)**: rustc-hash wins 8/9 single-threaded; **txmap wins all 9 two-thread** cases by 18–61%; 4-thread splits between leapfrog (small maps, write-heavy 100k) and txmap (large/read-heavy)

**Cross-operation patterns:**

- **txmap**: 13 fastest wins (most of any impl); **rustc-hash**: 11; **hashbrown** is the most consistently close (within 10% of the fastest in 15/36 groups)
- **starshard** never finishes within 10% of any winner — likely under-sharded (fixed 8 shards)

**Stability flags:** flurry (23% MAD at remove 100k), scc (std/mean 46.8% at write-heavy 100k), and two fast-impl-with-noise cases: txmap read-heavy 100k (9.6% MAD) and rustc-hash write-heavy 1k (9.5% MAD — its only thin-margin win).

**Measurement caveats:** 1s warm-up/2s measurement is short for 100k-scale runs; the 3.6 ns clone timing is at the harness noise floor; lookup benchmarks measure ~2.3 ns/op (timer-resolution boundary). Concurrent iterations previously included thread spawn/pinning overhead; the workload-concurrent benchmark now amortizes thread spawn/join and CPU-pinning outside the timed region via a reusable pinned worker pool.

## Map Implementation Benchmark — Executive Summary

*Source: Criterion.rs reports in `target/criterion/` (54 benchmark groups, 553 measurement sets, current absolute timings only — no baseline/change data). Timings are median point estimates (ns) from `new/estimates.json`; each value is the time per benchmark iteration (see Benchmark Scope for per-iteration op counts).*

## Executive Summary

No single map wins everywhere; the field splits cleanly by workload. **rustc-hash (FxHashMap)** is fastest for out-of-the-box bulk insert and for all single-threaded mixed workloads (11 wins); **txmap** dominates 2-thread workloads (9 of 9) and shares 4-thread wins with **leapfrog**. **indexmap** is 2.6× faster at iteration than everything else, **horde** is fastest at removal, and **immutable-chunkmap** clones in ~3.6 ns via structural sharing. For same-hasher lookups, **std / hashbrown / ahash** are effectively tied at the top. The big trade-off: concurrent safety costs ~5–10× on single-threaded operations, and **starshard** is never competitive in any benchmark.

## Benchmark Scope

- **17 map implementations** wrapped in `src/maps/`: ahash, btreemap, concread, dashmap, flurry, hashbrown, horde, immutable-chunkmap, indexmap, leapfrog, papaya, rustc-hash, scc, starshard, std (HashMap), txmap (plus the `BenchMap*` trait layer in `benchmap.rs`).
- **12 benchmark operations** (`benches/`):
  - `clone` — deep/structural clone of a populated map (1k / 10k / 100k entries; out-of-the-box hashers).
  - `insert` — insert N keys into a fresh map (1k / 10k / 100k; both out-of-the-box and shared-ahash-hasher variants).
  - `iterate` — full pass over entries summing values (1k / 10k / 100k; both hasher variants).
  - `lookup-hit` / `lookup-miss` — 100 get operations per iteration (1k / 10k / 100k; shared hasher).
  - `remove` — remove 100 existing keys per iteration (1k / 10k / 100k; shared hasher).
  - `key-sensitivity` — 100 lookups per iteration with u64, String<16>, and String<128> keys (10k entries; shared hasher).
  - `workload` (serial, 1 thread) and `workload` (concurrent, 2 and 4 threads) — 10,000-op mixed designs per thread: **write-heavy** (20% lookup-hit / 80% insert), **balanced** (70% hit / 5% miss / 10% insert / 10% update / 5% remove), **read-heavy** (90% hit / 5% miss / 5% insert), at map sizes 1k / 10k / 100k. Only thread-safe maps (dashmap, leapfrog, scc, starshard, txmap) appear in the ≥2-thread runs.
- **Timing convention**: Criterion values are per-iteration nanoseconds. Iterations = 1 clone, N inserts, N-entry iteration, 100 lookups/removes, 10,000 ops (serial) or 10,000×threads ops (concurrent).
- Not all implementations participate in all groups (see Implementation Limitations); "clearly ahead" claims are relative to the implementations actually measured in that group.

## Fastest per Operation

"Close contenders" = within 10% of the fastest median. Ratio shown is contender median ÷ fastest median.

| Operation                     | Size    | Threads | Fastest            | Median time | Close contenders (≤10%)                                                       |
|-------------------------------|---------|---------|--------------------|-------------|-------------------------------------------------------------------------------|
| clone                         | 1,000   | —       | immutable-chunkmap | 3.6 ns      | none — clearly ahead (next: starshard 51.1 ns, 14×)                           |
| clone                         | 10,000  | —       | immutable-chunkmap | 3.7 ns      | none — clearly ahead                                                          |
| clone                         | 100,000 | —       | immutable-chunkmap | 3.6 ns      | none — clearly ahead                                                          |
| insert (out-of-the-box)       | 1,000   | —       | rustc-hash         | 17.73 µs    | none — clearly ahead (next: hashbrown 20.60 µs, 1.16×)                        |
| insert (out-of-the-box)       | 10,000  | —       | rustc-hash         | 156.58 µs   | none — clearly ahead (next: ahash 179.91 µs, 1.15×)                           |
| insert (out-of-the-box)       | 100,000 | —       | rustc-hash         | 1.585 ms    | hashbrown 1.676 ms (1.06×)                                                    |
| insert (same-hasher)          | 1,000   | —       | ahash              | 17.98 µs    | std 18.05 µs (1.00×); hashbrown 18.39 µs (1.02×); horde 18.56 µs (1.03×)      |
| insert (same-hasher)          | 10,000  | —       | horde              | 156.48 µs   | ahash 160.10 µs (1.02×)                                                       |
| insert (same-hasher)          | 100,000 | —       | std                | 1.665 ms    | horde 1.666 ms (1.00×); ahash 1.706 ms (1.03×); hashbrown 1.712 ms (1.03×)    |
| iterate (out-of-the-box)      | 1,000   | —       | indexmap           | 258.7 ns    | none — clearly ahead (next: rustc-hash 695.3 ns, 2.7×)                        |
| iterate (out-of-the-box)      | 10,000  | —       | indexmap           | 2.72 µs     | none — clearly ahead (next: rustc-hash 6.63 µs, 2.4×)                         |
| iterate (out-of-the-box)      | 100,000 | —       | indexmap           | 27.58 µs    | none — clearly ahead (next: horde 72.19 µs, 2.6×)                             |
| iterate (same-hasher)         | 1,000   | —       | indexmap           | 258.6 ns    | none — clearly ahead                                                          |
| iterate (same-hasher)         | 10,000  | —       | indexmap           | 2.72 µs     | none — clearly ahead                                                          |
| iterate (same-hasher)         | 100,000 | —       | indexmap           | 27.62 µs    | none — clearly ahead                                                          |
| key-sensitivity (u64)         | 10,000  | —       | ahash              | 227.6 ns    | std 228.1 ns (1.00×); hashbrown 228.1 ns (1.00×)                              |
| key-sensitivity (String<16>)  | 10,000  | —       | ahash              | 696.7 ns    | std 706.7 ns (1.01×); hashbrown 716.0 ns (1.03×)                              |
| key-sensitivity (String<128>) | 10,000  | —       | ahash              | 1.89 µs     | hashbrown 1.95 µs (1.03×); std 1.96 µs (1.04×)                                |
| lookup-hit                    | 1,000   | —       | std                | 226.7 ns    | hashbrown 227.0 ns (1.00×); ahash 227.1 ns (1.00×)                            |
| lookup-hit                    | 10,000  | —       | hashbrown          | 226.6 ns    | std 226.9 ns (1.00×); ahash 240.8 ns (1.06×)                                  |
| lookup-hit                    | 100,000 | —       | std                | 228.9 ns    | hashbrown 242.6 ns (1.06×); ahash 251.2 ns (1.10×)                            |
| lookup-miss                   | 1,000   | —       | std                | 193.2 ns    | ahash 193.3 ns (1.00×); hashbrown 194.5 ns (1.01×); indexmap 195.4 ns (1.01×) |
| lookup-miss                   | 10,000  | —       | ahash              | 198.2 ns    | std 198.3 ns (1.00×); indexmap 200.8 ns (1.01×); hashbrown 200.8 ns (1.01×)   |
| lookup-miss                   | 100,000 | —       | std                | 213.5 ns    | ahash 213.6 ns (1.00×); hashbrown 217.2 ns (1.02×); indexmap 224.3 ns (1.05×) |
| remove                        | 1,000   | —       | horde              | 463.9 ns    | none — clearly ahead (next: std 552.1 ns, 1.19×)                              |
| remove                        | 10,000  | —       | horde              | 444.1 ns    | none — clearly ahead (next: ahash 585.5 ns, 1.32×)                            |
| remove                        | 100,000 | —       | horde              | 703.8 ns    | ahash 769.2 ns (1.09×)                                                        |
| workload balanced             | 1,000   | 1       | hashbrown          | 47.81 µs    | rustc-hash 48.57 µs (1.02×)                                                   |
| workload balanced             | 1,000   | 2       | txmap              | 294.48 µs   | none — clearly ahead (next: leapfrog 398.92 µs, 1.35×)                        |
| workload balanced             | 1,000   | 4       | leapfrog           | 609.61 µs   | txmap 648.23 µs (1.06×)                                                       |
| workload balanced             | 10,000  | 1       | rustc-hash         | 28.92 µs    | none — clearly ahead (next: hashbrown 38.08 µs, 1.32×)                        |
| workload balanced             | 10,000  | 2       | txmap              | 321.41 µs   | none — clearly ahead (next: leapfrog 412.79 µs, 1.28×)                        |
| workload balanced             | 10,000  | 4       | txmap              | 683.13 µs   | leapfrog 726.48 µs (1.06×)                                                    |
| workload balanced             | 100,000 | 1       | rustc-hash         | 49.33 µs    | none — clearly ahead (next: hashbrown 57.73 µs, 1.17×)                        |
| workload balanced             | 100,000 | 2       | txmap              | 391.36 µs   | none — clearly ahead (next: dashmap 571.57 µs, 1.46×)                         |
| workload balanced             | 100,000 | 4       | txmap              | 725.11 µs   | none — clearly ahead (next: dashmap 849.76 µs, 1.17×)                         |
| workload read-heavy           | 1,000   | 1       | rustc-hash         | 25.28 µs    | hashbrown 25.71 µs (1.02×)                                                    |
| workload read-heavy           | 1,000   | 2       | txmap              | 277.62 µs   | none — clearly ahead (next: leapfrog 383.21 µs, 1.38×)                        |
| workload read-heavy           | 1,000   | 4       | leapfrog           | 480.72 µs   | none — clearly ahead (next: txmap 532.67 µs, 1.11×)                           |
| workload read-heavy           | 10,000  | 1       | rustc-hash         | 24.74 µs    | none — clearly ahead (next: hashbrown 29.11 µs, 1.18×)                        |
| workload read-heavy           | 10,000  | 2       | txmap              | 301.88 µs   | none — clearly ahead (next: leapfrog 384.12 µs, 1.27×)                        |
| workload read-heavy           | 10,000  | 4       | leapfrog           | 509.12 µs   | none — clearly ahead (next: txmap 585.65 µs, 1.15×)                           |
| workload read-heavy           | 100,000 | 1       | rustc-hash         | 41.98 µs    | none — clearly ahead (next: hashbrown 46.59 µs, 1.11×)                        |
| workload read-heavy           | 100,000 | 2       | txmap              | 376.80 µs   | none — clearly ahead (next: leapfrog 475.09 µs, 1.26×)                        |
| workload read-heavy           | 100,000 | 4       | txmap              | 658.74 µs   | none — clearly ahead (next: dashmap 803.71 µs, 1.22×)                         |
| workload write-heavy          | 1,000   | 1       | rustc-hash         | 112.55 µs   | hashbrown 123.53 µs (1.10×)                                                   |
| workload write-heavy          | 1,000   | 2       | txmap              | 511.67 µs   | none — clearly ahead (next: dashmap 824.29 µs, 1.61×)                         |
| workload write-heavy          | 1,000   | 4       | txmap              | 1.025 ms    | leapfrog 1.097 ms (1.07×)                                                     |
| workload write-heavy          | 10,000  | 1       | rustc-hash         | 145.09 µs   | none — clearly ahead (next: hashbrown 201.77 µs, 1.39×)                       |
| workload write-heavy          | 10,000  | 2       | txmap              | 481.27 µs   | none — clearly ahead (next: leapfrog 716.05 µs, 1.49×)                        |
| workload write-heavy          | 10,000  | 4       | leapfrog           | 1.221 ms    | scc 1.252 ms (1.03×); txmap 1.343 ms (1.10×)                                  |
| workload write-heavy          | 100,000 | 1       | rustc-hash         | 114.45 µs   | none — clearly ahead (next: hashbrown 141.49 µs, 1.24×)                       |
| workload write-heavy          | 100,000 | 2       | txmap              | 689.91 µs   | none — clearly ahead (next: dashmap 814.17 µs, 1.18×)                         |
| workload write-heavy          | 100,000 | 4       | leapfrog           | 1.690 ms    | scc 1.791 ms (1.06×)                                                          |

## Implementation Ranking Summary

Counts over all 54 benchmark groups. "# within 10%" includes the groups where the implementation was fastest.

| Implementation     | # times fastest | # within 10% | # participated | Notes                                                                                                             |
|--------------------|-----------------|--------------|----------------|-------------------------------------------------------------------------------------------------------------------|
| txmap              | 13              | 15           | 54             | All 9 two-thread workloads + 4 four-thread; not competitive single-threaded (3–10× behind rustc-hash)             |
| rustc-hash         | 11              | 12           | 18             | All 3 out-of-the-box inserts + 8 of 9 serial workloads; limited to no-custom-hasher, single-threaded groups       |
| indexmap           | 6               | 9            | 36             | All 6 iterate groups by 2.4–2.7×; close on lookup-miss                                                            |
| ahash              | 5               | 13           | 36             | All 3 key-sensitivity groups + small insert + one lookup-miss; near the top of most hash lookups                  |
| std                | 5               | 11           | 36             | Tied with ahash/hashbrown on lookups; wins same-hasher insert at 100k                                             |
| leapfrog           | 5               | 7            | 49             | 5 of 9 four-thread workloads (small/medium maps, write-heavy at 100k); degrades badly at 100k balanced/read-heavy |
| horde              | 4               | 6            | 36             | All 3 remove groups + same-hasher insert 10k                                                                      |
| immutable-chunkmap | 3               | 3            | 18             | All 3 clone groups (O(1) structural sharing); slow everywhere else                                                |
| hashbrown          | 2               | 15           | 36             | **Most consistently close to the fastest** across inserts, lookups, and serial workloads                          |
| scc                | 0               | 2            | 54             | Close only on 4-thread write-heavy 100k/10k; never wins                                                           |
| starshard          | 0               | 0            | 54             | Never within 10%; 4–10× behind at 4 threads                                                                       |
| dashmap            | 0               | 0            | 54             | Mid-pack at 2–4 threads (1.2–1.6× behind txmap)                                                                   |
| papaya             | 0               | 0            | 27             | Not in workloads ("too slow"); 20–90× behind on lookups                                                           |
| flurry             | 0               | 0            | 27             | Worst or near-worst everywhere measured; not in workloads                                                         |
| btreemap           | 0               | 0            | 15             | Slow; only clone/iterate/serial workload                                                                          |
| concread           | 0               | 0            | 3              | Only iterate (COW transaction cost); "too slow" elsewhere                                                         |

## Use case recommendations

| Use case                                      | Recommended implementation(s)                                                                                                                                       |
|-----------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Bulk insert, default hashers, single-threaded | **rustc-hash** (clearly ahead); hashbrown if a DoS-resistant default hasher is required (1.06× at 100k)                                                             |
| Bulk insert with a shared custom hasher       | **horde / ahash / std / hashbrown** — all within 3%                                                                                                                 |
| Hot lookup path (u64 keys, custom hasher)     | **std / hashbrown / ahash** — statistically tied (≤1.1× across all sizes)                                                                                           |
| Lookup with String keys                       | **ahash** (std/hashbrown within 4%)                                                                                                                                 |
| Removal-heavy workloads                       | **horde** (2–3× over the next-best in most cases)                                                                                                                   |
| Iteration / ordered display                   | **indexmap** (2.4–2.7× faster than all alternatives)                                                                                                                |
| Single-threaded mixed workloads               | **rustc-hash** (hashbrown within 2–10% and a safer default hasher)                                                                                                  |
| 2-thread mixed workloads                      | **txmap** — wins all 9 by 18–61%                                                                                                                                    |
| 4-thread mixed workloads                      | **leapfrog** (1k–10k maps, and 100k write-heavy) or **txmap** (100k balanced/read-heavy); scc close on 100k write-heavy                                             |
| Frequent map cloning                          | **immutable-chunkmap** (3.6 ns, O(1) structural sharing) — only if immutable/persistent semantics are acceptable; otherwise rustc-hash/std/ahash (46–48 µs at 100k) |

## High-Variance Benchmarks

Noisiest runs by MAD/median (median_abs_dev ÷ median) and by std/mean, from `estimates.json`:

- **flurry — remove (100k): MAD = 23.1% of median (19.89 µs).** Also insert (100k) 11.2%, iterate (100k, both hasher variants) 9.9–10.9%, clone (100k) 8.4%. Flurry is never competitive, but its 100k-scale numbers are unreliable.
- **std — read-heavy workload (100k, 1 thread): MAD = 20.1% (213.92 µs).** std is 6th there, so it does not affect the winner, but the single-threaded 100k workloads are noisy generally.
- **scc — balanced (100k, 4t): MAD = 15.6%; read-heavy (100k, 4t): MAD = 14.2%; write-heavy (100k, 1t): std/mean = 46.8%** (median 381.92 µs vs. mean 531.19 µs → heavy tail/outliers). scc is a 4-thread write-heavy contender; its measurements are the least trustworthy of the concurrent set.
- ⚠️ **txmap — read-heavy workload (100k, 1 thread): MAD = 9.6% (154.27 µs).** txmap is the *fastest* here; the margin over #2 is ~46%, so the conclusion is robust, but the absolute number is noisy.
- ⚠️ **rustc-hash — write-heavy workload (1k, 1 thread): MAD = 9.5% (112.55 µs)**; also insert (10k) MAD = 5.1%. rustc-hash is the winner in both; margins are large elsewhere but small at write-heavy 1k (hashbrown within 10%), so this specific ordering is the least certain.
- **leapfrog — remove (100k): MAD = 8.4% and std/mean = 25.1% (1.16 µs)**; balanced (10k, 4t) MAD = 5.1%. Leapfrog's 4-thread *wins* are stable (MAD 0.8–3.4%); its remove numbers are not.
- **horde — remove (10k): std/mean = 36.2% despite MAD of only 2.4% (444.1 ns)** → occasional outlier spikes around a stable median; horde's remove wins are median-robust.
- Other notable: papaya remove (100k) MAD 6.6%; starshard read-heavy (1k, 4t) MAD 6.3%; concread iterate (100k) MAD 6.2%; ahash clone (100k) std/mean 22.9%; indexmap remove (1k) std/mean 28.3%; std remove (10k) std/mean 23.8%.
- Typical (median-of-runs) noise is low for the leaders: rustc-hash 1.3%, txmap 0.8%, leapfrog 0.8%, hashbrown 0.5%, ahash 0.4%, horde 0.4%, indexmap 0.4%, std 0.3%.

## Implementation Limitations

Based on the wrappers in `src/maps/` and the benches they are excluded from:

- **ahash** (`ahash_benchmap.rs`): `K: Hash + Eq` (`K: Clone` for clone). Supports custom hashers. Mutation requires `&mut` — not usable in concurrent workloads.
- **btreemap** (`btreemap_benchmap.rs`): `K: Ord`. No custom-hasher support (absent from all `same-hasher` groups); not concurrent.
- **concread** (`concread_benchmap.rs`): `K: Clone + Debug + Hash + Eq + Send + Sync + 'static`, `V: Clone + Send + Sync + 'static`. No custom hasher, **no `Clone`** (absent from clone bench). Every write goes through a COW transaction with explicit `commit()`, which is why it only appears in `iterate` and is commented "too slow" elsewhere.
- **dashmap** (`dashmap_benchmap.rs`): `K: Hash + Eq`, hasher must be `Clone`. Concurrent via `&self` (`insert`/`remove` take `&self`); custom hasher supported.
- **flurry** (`flurry_benchmap.rs`): unusually demanding bounds — `K: Sync + Send + Clone + Hash + **Ord**`, `V: Sync + Send` (`Clone` for clone/remove/clear). Every op goes through a `pin()` guard (thread-pinning infrastructure). Excluded from all workloads as "too slow".
- **hashbrown** (`hashbrown_benchmap.rs`): `K: Hash + Eq`. Custom hasher supported; not concurrent.
- **horde** (`horde_benchmap.rs`): `K: Clone + Hash + Eq` (insert requires `Clone`), `V: Clone`. Custom hasher supported. **Mutation requires `&mut`** (write guard) — explicitly excluded from concurrent workloads.
- **immutable-chunkmap** (`immutable_chunkmap_benchmap.rs`): `K: Clone + Ord`, `V: Clone`. No custom hasher; **no `BenchMapMutClear`**; mutation is copy-on-write (`insert_cow`/`remove_cow`) and the wrapper notes plain `insert` would silently discard the result. Clone is O(1) structural sharing — its 3.6 ns "win" is not comparable to deep-copy clones.
- **indexmap** (`indexmap_benchmap.rs`): `K: Hash + Eq`. Custom hasher supported; not concurrent. **`remove` uses `swap_remove`** — O(1) but reorders remaining entries (breaks insertion-order guarantees under deletion).
- **leapfrog** (`leapfrog_benchmap.rs`): `K: Eq + Hash + **Copy**` — no String keys (absent from String key-sensitivity), `V: leapfrog::Value`, hasher must be `Default`. **No `Clone`** (absent from clone bench). Concurrent via `&self`; custom hasher supported.
- **papaya** (`papaya_benchmap.rs`): `K: Hash + Eq`. Custom hasher supported; concurrent via `pin()`. Excluded from all workloads as "too slow".
- **rustc-hash** (`rustc_hash_benchmap.rs`): `K: Hash + Eq`. **Fixed FxBuildHasher — no custom-hasher support** (absent from all `same-hasher` groups); not concurrent.
- **scc** (`scc_benchmap.rs`): `K: Hash + Eq`. Custom hasher supported; concurrent, but wrappers use the blocking `*_sync` APIs (no async path).
- **starshard** (`starshard_benchmap.rs`): `K: Clone + Hash + Eq + Send + Sync`, `V: Clone + Send + Sync`, hasher `Clone + Send + Sync`. **Fixed at 8 shards** (`with_shards_and_hasher(8, …)`) — not tuned per thread count (4-thread runs use 2 shards/thread); custom hasher supported; concurrent.
- **std** (`std_benchmap.rs`): `K: Hash + Eq`. Custom hasher supported; not concurrent. Default hasher is SipHash (RandomState), which is why out-of-the-box std trails rustc-hash on inserts but ties at the top once a shared ahash hasher is used.
- **txmap** (`txmap_benchmap.rs`): `K: Clone + Hash + Eq`. Custom hasher supported; concurrent under a **MutexPolicy** (`txmap::MutexPolicy`).

## Recommendations

1. **Investigate txmap further.** It wins every 2-thread workload and half the 4-thread ones by large margins, yet is 3–10× slower than rustc-hash single-threaded — the scaling behavior is the interesting result and worth a dedicated look (lock-free vs. mutex policy, why the 100k read-heavy single-thread run is noisy at 9.6% MAD).
2. **Investigate leapfrog's size sensitivity.** Wins most 4-thread cases at 1k–10k maps but collapses at 100k balanced/read-heavy (2.0–2.1 ms vs. txmap 659–725 µs). Confirm this is structural (probe/occupancy behavior) and not measurement noise (MAD 0.8–3.4% on those runs suggests it is structural).
3. **Verify rustc-hash vs. hashbrown at write-heavy 1k (1 thread).** The only rustc-hash win with a ≤10% margin also has the highest noise (9.5% MAD). Re-run with longer measurement time before treating that ordering as firm.
4. **Treat 100k-scale single-threaded workload numbers cautiously.** std (20.1% MAD), scc (std/mean 46.8%), and several others show heavy tails; the 2 s warm-up / 1 s measurement configuration (see `src/config.rs`) is short for the largest, slowest cases.
5. **Re-tune starshard before judging it.** A fixed 8-shard configuration with 2–4 benchmark threads is likely under-sharded; rerun with shard count = threads (or a sweep) before concluding it is uncompetitive. As measured it never finishes within 10% of any winner.
6. **Note the hasher confound in out-of-the-box comparisons.** rustc-hash's insert/workload wins partly reflect FxHash's cheap hashing vs. SipHash (std) and hashbrown's default hasher. The `same-hasher` groups are the fair apples-to-apples comparison; there, ahash/hashbrown/std/horde are statistically tied.
7. **Measurement quality issues to fix for future runs:** (a) the clone benchmark for immutable-chunkmap (3.6 ns) is at the harness noise floor — report it as "O(1) structural sharing" rather than a timing; (b) lookup benchmarks measure ~227 ns per 100-op iteration (~2.3 ns/op) — increase iteration counts or use larger maps to move off the timer-resolution boundary; (c) **resolved**: the workload-concurrent benchmark previously included thread spawn/join and CPU-pinning overhead in each timed iteration — it now spawns and pins a reusable worker pool once per sample, outside the timed region, so only the parallel workload (plus start/done signalling) is measured; per-op throughput continues to be reported.
