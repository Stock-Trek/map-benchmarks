# Map Implementation Benchmark — Executive Summary

Generated from `target/criterion/` Criterion reports (965 measurements, 73 benchmarks, 19 implementations).
All timings are **median point estimates** from `new/estimates.json` (nanoseconds converted to human units). No baseline/change data exists; these are absolute measurements only.

## Executive Summary

On current absolute measurements, **rustc-hash is the fastest single-threaded map**: it wins every out-of-the-box insert and lookup-hit benchmark, most removals, and all 1-thread workloads (18 wins total), with hashbrown 5–16% behind and within 10% for removals. Under a common hasher, hashbrown/ahash/std are effectively tied. **indexmap dominates iteration (≈4× over hashbrown)**; hashlink edges negative lookups; horde leads same-hasher removals. For concurrency, **txmap wins every 2-thread workload; leapfrog wins most 3–4-thread workloads** (write-heavy at 3 threads favors txmap). Persistent structures (rpds / imbl / immutable-chunkmap) offer O(1) clones but much slower writes. **leapfrog's 4-thread results are extremely noisy (CV up to 91%) and should be re-measured before drawing conclusions.**

## Benchmark Scope

- **Implementations (19)**: ahash, btreemap, concurrent-map, crossbeam-skiplist, dashmap, hashbrown, hashlink, horde, imbl, immutable-chunkmap, indexmap, leapfrog, papaya, rpds-hash-trie-map, rustc-hash, scc, starshard, std, txmap.
  - Wrappers exist for **concread** and **flurry** in `src/maps/` but they are commented out of every bench ("too slow", "doesn't implement Clone") — **no data exists for them**.
- **Operations (9 groups)**: clone, create, insert, iterate, lookup-hit, lookup-miss, remove, key-sensitivity (u64 / String<16> / String<128> keys), workload (write-heavy / balanced / read-heavy designs).
- **Sizes**: 1_000, 10_000, 100_000 entries.
- **Hasher modes**: `out-of-the-box` (each map's default hasher) and `same-hasher` (all maps share one `ahash::RandomState`). rustc-hash, btreemap, concurrent-map, immutable-chunkmap and crossbeam-skiplist cannot take a custom hasher and are absent from same-hasher benches.
- **Concurrency**: workload at 1 thread runs all 19 maps; at 2/3/4 threads only the 7 concurrent maps (crossbeam-skiplist, dashmap, leapfrog, papaya, scc, starshard, txmap). Worker threads are pinned to CPUs via `sched_setaffinity`; 1 s warm-up / 2 s measurement.
- Per-iteration workload = 10,000 mixed ops (lookup/insert/remove) against a pre-populated map.

## Fastest per Operation

| Operation | Fastest | Median time (1k / 10k / 100k) | Close contenders (≤10% of fastest) |
|---|---|---|---|
| clone | **rpds-hash-trie-map** | 2.2 ns / 2.2 ns / 2.2 ns (O(1) persistent clone) | none — immutable-chunkmap 3.9 ns (1.8×), imbl 4.6 ns (2.1×) |
| create (empty map) | **immutable-chunkmap** | 3.1 µs | none — rustc-hash 6.2 µs (2.0×), btreemap 31.1 µs (10×) |
| insert, default hashers | **rustc-hash** | 22.2 µs / 193 µs / 1.96 ms | none — hashbrown 25.6 µs / 2.24 ms (1.14–1.16×) |
| insert, same hasher | **std** (1k), **hashbrown** (10k, 100k) | 23.5 µs / 206.5 µs / 2.10 ms | ahash (1.00–1.01×), std (1.00–1.03×), horde (1.04–1.08×) — effectively tied |
| iterate | **indexmap** | 0.30 µs / 3.13 µs / 32.4 µs | none — hashbrown 0.84 µs / 10.0 µs / 137 µs (2.8–4.2×) |
| key-sensitivity u64 | **hashbrown** | 286 ns | std (1.00×), ahash (1.00×), hashlink (1.08×) |
| key-sensitivity String<16> | **hashbrown** | 1.02 µs | ahash (1.00×), std (1.02×), hashlink (1.04×) |
| key-sensitivity String<128> | **std** | 2.27 µs | ahash (1.04×), hashbrown (1.08×) |
| lookup-hit, default hashers | **rustc-hash** | 205 ns / 204 ns / 243 ns | none — hashbrown 231/272 ns (1.12–1.13×) |
| lookup-hit, same hasher | **hashbrown** (1k, 10k), **std** (100k) | 281 ns / 281 ns / 317 ns | ahash (1.02×), std (1.02×), hashlink (1.07×) |
| lookup-miss, default hashers | **hashlink** (1k, 100k), **rustc-hash** (10k) | 200 ns / 193 ns / 222 ns | hashbrown (1.01–1.05×), rustc-hash (1.00–1.05×) |
| lookup-miss, same hasher | **hashlink** | 246 ns / 242 ns / 268 ns | std (1.04–1.07×), ahash (1.05×), hashbrown (1.05–1.09×) |
| remove, default hashers | **rustc-hash** (1k, 10k), **hashbrown** (100k) | 674 ns / 756 ns / 1.08 µs | the other within 1.05–1.07× — practically tied |
| remove, same hasher | **horde** | 587 ns / 639 ns / 1.01 µs | leapfrog (1.06×) at 100k only; hashbrown 1.13–1.30× |
| workload, 1 thread (9 configs) | **rustc-hash** (all 9) | 29–212 µs | hashbrown (1.08–1.10×) in read-heavy only |
| workload, 2 threads (9 configs) | **txmap** (8 of 9), leapfrog (write-heavy 100k) | 250–774 µs | none — next best dashmap/leapfrog 1.2–1.9× |
| workload, 3 threads (9 configs) | **leapfrog** (7 of 9), txmap (write-heavy 1k, 10k) | 418–1354 µs | txmap within 1.00–1.05× in 4 of leapfrog's wins |
| workload, 4 threads (9 configs) | **leapfrog** (7 of 9), txmap (read-heavy 10k), scc (balanced 10k) | 472–1290 µs | scc's balanced-10k win: txmap 1.02×, leapfrog 1.02× |

## Implementation Ranking Summary

| Implementation | # times fastest | # times within 10% | Notes |
|---|---|---|---|
| rustc-hash | 18 | 21 | Single-threaded default-hasher king; absent from same-hasher & concurrent benches |
| leapfrog | 15 | 18 | Wins most 3–4-thread workloads; very noisy at 4 threads; `K: Copy` only |
| txmap | 11 | 16 | Dominates 2-thread workloads; strong #2 at 3 threads |
| hashbrown | 7 | 20 | Never far behind anyone; the "safe default" |
| indexmap | 6 | 6 | All iteration wins (contiguous storage) |
| hashlink | 5 | 9 | All lookup-miss wins |
| horde | 3 | 6 | All same-hasher remove wins |
| rpds-hash-trie-map | 3 | 3 | All clone wins (O(1) persistent clone) |
| std | 3 | 12 | Wins only with custom (ahash) hasher or String<128> keys |
| immutable-chunkmap | 1 | 1 | create; also 2nd-fastest clone |
| scc | 1 | 1 | balanced 10k @ 4 threads (tied with txmap/leapfrog within 2%) |
| ahash | 0 | 11 | Always within 10% of hashbrown/std (same hashing family) |
| btreemap | 0 | 0 | Never competitive (3–10× behind on all ops) |
| concurrent-map | 0 | 0 | Not concurrent-safe for shared `&` access; absent from multi-thread benches |
| crossbeam-skiplist | 0 | 0 | Slowest in most concurrent workloads (ordered structure) |
| dashmap | 0 | 0 | Mid-pack in concurrent workloads |
| imbl | 0 | 0 | Persistent; slow writes, O(1) clone |
| papaya | 0 | 0 | Slowest concurrent map in most workloads |
| starshard | 0 | 0 | Consistently mid/late pack |

## Use case recommendations

| Use case | Recommended implementation(s) |
|---|---|
| Single-threaded, integer/u64 keys, any mix of ops | **rustc-hash**; use **hashbrown** if custom hasher or DoS-resistance needed (≈5–15% slower) |
| Single-threaded, custom hasher (e.g. shared ahash) | **hashbrown / ahash / std** — interchangeable (within 1–8%) |
| Read-heavy / lookup hits | **rustc-hash** (default) or **hashbrown** |
| Negative lookups (miss-heavy) | **hashlink** (marginally) |
| Iteration / full scans | **indexmap** (4×+ faster than hashbrown) |
| Insertion-order preservation | **hashlink** or **indexmap** |
| Bulk removals | **rustc-hash / hashbrown** (default hasher); **horde** with shared hasher |
| Concurrent, 2 threads | **txmap** |
| Concurrent, 3–4 threads | **leapfrog** (`Copy` keys); **txmap** for write-heavy at 3 threads |
| Concurrent, ordered access | **crossbeam-skiplist** (accept slow: 3–8× behind) |
| Persistent snapshots / O(1) clone | **rpds / imbl / immutable-chunkmap** (accept 10–100× slower writes) |
| Ordered single-threaded | **btreemap** (accept 3–10× slower) |

## High-Variance Benchmarks

Relative MAD (MAD/median) and CV (std_dev/mean) from the reports. Thresholds: rel_MAD > 10% or CV > 25%.

- **rpds-hash-trie-map — workload 100k @ 1 thread (all 3 designs)**: rel_MAD 44–51%, CV 37–42% (MAD ≈ 4.3–7.2 ms). Median/mean diverge strongly (e.g. balanced: median 11.1 ms vs mean 9.3 ms) — looks bimodal; possible allocator/GC stalls.
- **leapfrog — 4-thread workloads @ 100k**: balanced rel_MAD 47.7% / CV 76.4%; write-heavy rel_MAD 46.7% / CV 70.4%; read-heavy rel_MAD 23.1% / CV 91.4%. Also anomalous: write-heavy 3-thread is 1.35 ms at 10k but only 0.67 ms at 100k — non-monotonic, interference suspected.
- **scc — read-heavy 100k @ 4 threads**: rel_MAD 72.0%, CV 44.3%. Its one "fastest" win (balanced 10k @ 4 threads) has rel_MAD 10.7% / CV 12.9%.
- **txmap — 4-thread 100k**: read-heavy rel_MAD 30.5%, CV 34.3%; balanced rel_MAD 22.8%, CV 37.1%; read-heavy 1k @ 4 threads CV 40.7%.
- **dashmap — 4-thread workloads**: rel_MAD 10.6–19.6%; CV up to 52.2% (read-heavy 100k) and 49.1% (balanced 100k).
- **papaya — 4-thread**: CV up to 63.2%; also lookup-miss 1k (rel_MAD 18.8%, CV 35.4%) and remove 100k (rel_MAD 21.2%).
- **imbl — insert 100k** (rel_MAD 16.8%), remove 100k (rel_MAD 16.4%), lookup-hit 1k (rel_MAD 12.1%), workload 100k @ 1 thread (rel_MAD 7.0%).
- **starshard — 4-thread**: rel_MAD 6–10%, CV 13–14% (large absolute MAD up to 430–665 µs).
- **crossbeam-skiplist — 4-thread**: rel_MAD 5–9.5%, CV up to 17.6%.

**Fast implementations with unusually noisy measurements**: **leapfrog** (the 3–4-thread winner — its 4-thread/100k numbers are unreliable), **txmap** (2-thread winner — noisy at 4 threads), **scc** (noisy at 4 threads).
**Very stable implementations**: rustc-hash (rel_MAD < 1% in nearly all benches), hashbrown/std/ahash in serial benches (rel_MAD 1–3%), indexmap, hashlink.

## Implementation Limitations

Based on `src/maps/*_benchmap.rs` wrapper code (trait bounds, unavailable functionality, ergonomics):

- **ahash** (`ahash_benchmap.rs`) — `K: Hash + Eq`, `V: Clone` (for get/remove); custom hasher supported; single-threaded only.
- **std** (`std_benchmap.rs`) — `K: Hash + Eq`; custom hasher supported; default `RandomState` (SipHash) is the reason std is slow out-of-the-box for u64 keys (48 µs vs 22 µs for rustc-hash at 1k inserts) but matches ahash/hashbrown when given an ahash hasher.
- **hashbrown** (`hashbrown_benchmap.rs`) — `K: Hash + Eq`; custom hasher supported; the robust all-rounder.
- **rustc-hash** (`rustc_hash_benchmap.rs`) — `K: Hash + Eq`; **no custom hasher** (`BenchMapNewWithHasher` not implemented) → excluded from same-hasher and key-sensitivity benches; FxHash is fast for small keys but weak against hash-flooding / adversarial keys; single-threaded.
- **btreemap** (`btreemap_benchmap.rs`) — `K: Ord`; no custom hasher; sorted iteration.
- **hashlink** (`hashlink_benchmap.rs`) — `K: Hash + Eq`; custom hasher supported; insertion-ordered (LinkedHashMap) — extra bookkeeping for clone/iterate.
- **indexmap** (`indexmap_benchmap.rs`) — `K: Hash + Eq`; custom hasher supported; insertion-ordered; `remove` uses **`swap_remove`** (changes iteration order); contiguous storage explains its iteration dominance.
- **horde** (`horde_benchmap.rs`) — `horde::SyncTable`; concurrent AND persistent (copy-on-write, shared pointers); **mutation requires `&mut`** (excluded from 2–4-thread shared-`&` workloads); reads go through a pin guard; custom hasher supported.
- **imbl** (`imbl_benchmap.rs`) — persistent `GenericHashMap`; mutation requires `&mut` (excluded from multi-thread); clone is O(1); custom hasher supported.
- **immutable-chunkmap** (`immutable_chunkmap_benchmap.rs`) — `K: Clone + Ord`; **no custom hasher**; no `BenchMapMutClear`; uses `insert_cow`/`remove_cow` (in-place COW) rather than the returning `insert` API.
- **rpds-hash-trie-map** (`rpds_benchmap.rs`) — persistent `HashTrieMap` with **`RcK` (non-Send/Sync) pointers** → cannot be shared across threads (comment in workload_concurrent.rs); clone is O(1); custom hasher supported.
- **concurrent-map** (`concurrent_map_benchmap.rs`) — `K: 'static + Clone + Minimum + Send + Sync` (crate-specific `Minimum` bound); **`Send` but not `Sync`** → excluded from shared-`&` multi-thread workloads; no custom hasher; no `clear`.
- **crossbeam-skiplist** (`crossbeam_skiplist_benchmap.rs`) — `K: Ord`; insert/remove require `K/V: Send + 'static`; **no `Clone`** (excluded from clone bench); no custom hasher; ordered; `'static` bound is ergonomically restrictive.
- **dashmap** (`dashmap_benchmap.rs`) — `K: Hash + Eq`; custom hasher supported; deep `Clone` is expensive (clone bench: 1.9 µs @ 1k scaling to 195 µs @ 100k).
- **leapfrog** (`leapfrog_benchmap.rs`) — **`K: Eq + Hash + Copy`** and **`V: leapfrog::Value`** → excluded from clone (no `Clone` impl) and String key-sensitivity benches; the `Copy`-key restriction is significant.
- **papaya** (`papaya_benchmap.rs`) — `K: Hash + Eq`; custom hasher supported; every op requires a `pin()` guard (ergonomics); no clone limitation but slow.
- **scc** (`scc_benchmap.rs`) — `K: Hash + Eq`; custom hasher supported; wrapper uses the `*_sync` API.
- **starshard** (`starshard_benchmap.rs`) — `K: Clone + Hash + Eq + Send + Sync`, `V: Clone + Send + Sync`, hasher `Send + Sync`; fixed 8 shards in wrapper; default hasher FxBuildHasher.
- **txmap** (`txmap_benchmap.rs`) — `K: Clone + Hash + Eq`; custom hasher supported; mutex-based transactions (`MutexPolicy`).
- **concread** (`concread_benchmap.rs`) — **not benchmarked** ("too slow"); `K: Clone + Debug + Hash + Eq + Send + Sync + 'static`; MVCC copy-on-write.
- **flurry** (`flurry_benchmap.rs`) — **not benchmarked** ("too slow"; creates a `seize::Collector` per map); note `K: Hash + Ord` bound (needs Ord for its internal structure).

## Recommendations

1. **Re-measure leapfrog at 4 threads (100k)** before trusting the concurrent conclusions — rel_MAD up to 48%, CV up to 91%, plus the non-monotonic write-heavy 3-thread results (10k slower than 100k) indicate external interference (CPU contention, turbo, or scheduler noise), not map behavior.
2. **Re-measure rpds workload @ 100k**: extreme MAD (4.3–7.2 ms) and median-vs-mean divergence suggest bimodal behavior worth investigating (allocator stalls?).
3. **Measurement quality**: 2 s measurement windows are short for the noisy multi-thread benches. Increase measurement time, pin the *main* benchmark thread too (only workers are pinned today), run on a quiet/dedicated host, and consider disabling turbo/HT for future runs. The 4-thread numbers across **all** concurrent maps show elevated variance and should be treated as approximate.
4. **Single-threaded default recommendation is robust**: rustc-hash's wins are consistent and low-variance; hashbrown is the safe alternative if hasher control or hash-flood resistance matters.
5. **Close the coverage gaps**: concread and flurry wrappers exist but were never run; leapfrog cannot be assessed on non-`Copy` keys or clones; rustc-hash cannot be assessed under a common hasher (its FxHash advantage is partly hasher choice). These absences limit how far the "fastest" conclusions generalize to string keys and mixed workloads.
6. **indexmap's `swap_remove`**: if stable (order-preserving) removal is required, indexmap's remove numbers don't transfer — use `shift_remove` and re-benchmark.
7. **Document the persistent-map caveat**: rpds/imbl/immutable-chunkmap clone at 2–5 ns because clones share structure — this is a feature, not a measurement error, but the same property means writes are 10–100× slower than hash maps.
