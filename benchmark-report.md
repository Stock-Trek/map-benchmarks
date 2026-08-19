# Map Benchmark Executive Summary

*Source: Criterion.rs reports in `target/criterion/` (55 benchmark groups, `new/estimates.json`, median point estimates in ns). No baseline/change data available — absolute measurements only. All timings are per iteration; iteration contents vary by benchmark (see Scope).*

## Executive Summary

For single-threaded use with default hashers, **rustc-hash** is the clear winner — fastest on create, insert, and all 9 serial mixed workloads (11–42% ahead of the next best), though its FxHash is not DoS-hardened and it cannot use custom hashers. For custom-hasher comparisons, **hashbrown / ahash / std** are practically tied on u64 lookups (within 3%), with **ahash** slightly ahead for inserts and String keys. Specialists dominate elsewhere: **indexmap** for iteration (3–4× faster), **horde** for removal, **immutable-chunkmap** for cloning (O(1), ~4 ns at any size). Concurrently, **txmap** wins all 2-thread workloads and **leapfrog** most 4-thread workloads, but 4-thread measurements are extremely noisy (relative std-dev up to 93%), so concurrent conclusions are provisional.

## Benchmark Scope

- **16 map implementations**: ahash, btreemap (std BTreeMap), concread, dashmap, flurry, hashbrown, horde, immutable-chunkmap, indexmap, leapfrog, papaya, rustc-hash (FxHashMap), scc, starshard, std (HashMap), txmap — wrappers in `src/maps/`.
- **55 benchmark groups** across 9 families:
  - `create` (10,000 empty-map creations per iteration) — 1 group
  - `clone` (1 clone of a full map) — 3 sizes (1k / 10k / 100k)
  - `insert` (N key inserts per iteration) — 2 hasher modes × 3 sizes = 6
  - `iterate` (full-map scan) — 2 hasher modes × 3 sizes = 6
  - `lookup-hit`, `lookup-miss`, `remove` (100 key ops/iteration) — 3 sizes each = 9
  - `key-sensitivity` (100 lookups) — u64, String<16>, String<128> = 3
  - `workload` serial (threads-1, 10,000 ops) and concurrent (threads-2/4, 10,000 ops/thread) — read-heavy (90% read), balanced (70% read/20% write), write-heavy (80% insert) × 3 sizes = 27
- **Hasher modes**: *out-of-the-box* = each map's default hasher; *same-hasher* = all participating maps forced to use the same `ahash::RandomState` (btreemap, concread, immutable-chunkmap, rustc-hash cannot set a hasher and are excluded).
- Methodology: 1 s warm-up, 2 s measurement per benchmark; median point estimate used throughout.

## Fastest per Operation

Iteration contents: create = 10k maps; insert = map-size inserts; iterate = full scan; lookup/remove/key-sens = 100 ops; workload = 10k ops (per thread). "Clearly ahead" = no implementation within 10% of the winner's median.

| Operation | Fastest | Time | Close contenders (≤10%) |
|---|---|---|---|
| create (out-of-the-box) | rustc-hash | 6.23 µs | immutable-chunkmap 6.23 µs (x1.00) |
| clone 1k | immutable-chunkmap | 3.98 ns | — clearly ahead |
| clone 10k | immutable-chunkmap | 3.94 ns | — clearly ahead |
| clone 100k | immutable-chunkmap | 3.93 ns | — clearly ahead |
| insert (out-of-the-box) 1k | rustc-hash | 22.1 µs | — clearly ahead |
| insert (out-of-the-box) 10k | rustc-hash | 189 µs | — clearly ahead |
| insert (out-of-the-box) 100k | rustc-hash | 1.93 ms | — clearly ahead |
| insert (same-hasher) 1k | ahash | 23.0 µs | hashbrown 23.0 µs (x1.00), std 23.1 µs (x1.01), horde 24.1 µs (x1.05) |
| insert (same-hasher) 10k | ahash | 203 µs | hashbrown 203 µs (x1.00), std 212 µs (x1.05), horde 216 µs (x1.06) |
| insert (same-hasher) 100k | ahash | 2.07 ms | hashbrown 2.10 ms (x1.01), std 2.11 ms (x1.02) |
| iterate (out-of-the-box) 1k | indexmap | 301 ns | — clearly ahead |
| iterate (out-of-the-box) 10k | indexmap | 3.11 µs | — clearly ahead |
| iterate (out-of-the-box) 100k | indexmap | 40.4 µs | — clearly ahead |
| iterate (same-hasher) 1k | indexmap | 301 ns | — clearly ahead |
| iterate (same-hasher) 10k | indexmap | 3.11 µs | — clearly ahead |
| iterate (same-hasher) 100k | indexmap | 33.1 µs | — clearly ahead |
| key-sensitivity u64 | hashbrown | 282 ns | std 287 ns (x1.02), ahash 287 ns (x1.02) |
| key-sensitivity String<16> | ahash | 922 ns | hashbrown 1.01 µs (x1.09) |
| key-sensitivity String<128> | ahash | 2.36 µs | hashbrown 2.43 µs (x1.03), std 2.43 µs (x1.03) |
| lookup-hit 1k | hashbrown | 286 ns | std 293 ns (x1.02), ahash 294 ns (x1.03) |
| lookup-hit 10k | hashbrown | 282 ns | ahash 287 ns (x1.02), std 288 ns (x1.02) |
| lookup-hit 100k | hashbrown | 322 ns | ahash 323 ns (x1.00), std 324 ns (x1.01) |
| lookup-miss 1k | std | 230 ns | hashbrown 231 ns (x1.00), ahash 231 ns (x1.00) |
| lookup-miss 10k | std | 231 ns | hashbrown 231 ns (x1.00), ahash 231 ns (x1.00) |
| lookup-miss 100k | hashbrown | 255 ns | std 255 ns (x1.00), ahash 255 ns (x1.00) |
| remove 1k | horde | 527 ns | — clearly ahead |
| remove 10k | horde | 651 ns | — clearly ahead |
| remove 100k | horde | 929 ns | — clearly ahead |
| workload serial balanced 1k/10k/100k | rustc-hash | 50.0 / 44.8 / 66.9 µs | — clearly ahead |
| workload serial read-heavy 1k/10k/100k | rustc-hash | 26.6 / 34.5 / 58.1 µs | — clearly ahead |
| workload serial write-heavy 1k/10k/100k | rustc-hash | 111 / 205 / 143 µs | — clearly ahead |
| workload concurrent (2 thr.) balanced 1k/10k/100k | txmap | 284 / 301 / 372 µs | — clearly ahead |
| workload concurrent (2 thr.) read-heavy 1k/10k/100k | txmap | 253 / 278 / 354 µs | — clearly ahead |
| workload concurrent (2 thr.) write-heavy 1k/10k/100k | txmap | 533 / 500 / 777 µs | dashmap 837 µs (x1.08) @100k only |
| workload concurrent (4 thr.) balanced 1k | leapfrog | 559 µs | — clearly ahead |
| workload concurrent (4 thr.) balanced 10k | leapfrog | 1.03 ms | txmap 1.05 ms (x1.01), scc 1.05 ms (x1.02) |
| workload concurrent (4 thr.) balanced 100k | dashmap | 939 µs | — clearly ahead (⚠ very noisy, see below) |
| workload concurrent (4 thr.) read-heavy 1k | leapfrog | 469 µs | — clearly ahead |
| workload concurrent (4 thr.) read-heavy 10k | txmap | 817 µs | — clearly ahead |
| workload concurrent (4 thr.) read-heavy 100k | leapfrog | 572 µs | — clearly ahead (⚠ very noisy) |
| workload concurrent (4 thr.) write-heavy 1k/10k/100k | leapfrog | 1.07 / 1.27 / 1.78 ms | — clearly ahead |

## Implementation Ranking Summary

Within-10% counts are *second-place* finishes only (winner excluded).

| Implementation | # fastest (of 55) | # within 10% (2nd) | # participated | Notes |
|---|---|---|---|---|
| rustc-hash | 13 | 0 | 19 | Dominates every serial out-of-the-box group (create, insert ×3, workload ×9) |
| txmap | 10 | 1 | 55 | Wins all 9 two-thread workloads + read-heavy 10k @ 4 thr |
| leapfrog | 7 | 0 | 50 | Wins most 4-thread workloads; fast removal too (2nd at 1k) |
| indexmap | 6 | 0 | 37 | Iteration champion by 3–4× |
| ahash | 5 | 7 | 37 | Best same-hasher insert and String-key lookups; within 10% on all lookup benchmarks |
| hashbrown | 5 | 7 | 37 | Best u64 lookups; within 10% of fastest on 12/37 serial ops |
| horde | 3 | 2 | 37 | Removal champion; also within 10% on same-hasher inserts |
| immutable-chunkmap | 3 | 1 | 19 | Clone champion (O(1)); ties rustc-hash on create |
| std | 2 | 9 | 37 | Within 10% of fastest on 11/37 serial ops — consistently close, rarely first |
| dashmap | 1 | 1 | 55 | Only 4-thread balanced 100k win (very noisy) |
| btreemap | 0 | 0 | 19 | Never close |
| concread | 0 | 0 | 34 | Never close; slowest serial ops by far |
| flurry | 0 | 0 | 54 | Never close; slowest in several families |
| papaya | 0 | 0 | 55 | Mid-pack; anomalous lookup-miss @1k (see Recommendations) |
| scc | 0 | 1 | 55 | Mid-pack concurrent |
| starshard | 0 | 0 | 55 | Slow per-op; only notable for cheap clone (~60 ns) |

## Use case recommendations

| Use case | Recommended implementation(s) |
|---|---|
| Single-threaded, default hasher, create/insert/mixed workloads | **rustc-hash** (11–42% ahead; caveat: FxHash not DoS-hardened) |
| Single-threaded, custom/DoS-resistant hasher, generic use | **hashbrown** or **ahash** (effectively tied); **std** within ~3% |
| u64 lookup-heavy (any map size, custom hasher) | **hashbrown** (ahash/std within 1–3%) |
| String-key lookups | **ahash** (hashbrown/std within 3–9%) |
| Insert-heavy serial (custom hasher) | **ahash** (hashbrown/std within 1–5%) |
| Full-map iteration / scans | **indexmap** (3–4× faster, contiguous storage) |
| Cheap copies of large maps | **immutable-chunkmap** (~4 ns, structural sharing) or **starshard** (~60 ns) |
| Removal-heavy | **horde** (~1.2–1.4× faster than std/ahash at all sizes) |
| Concurrent, 2 threads | **txmap** (all three workload shapes) |
| Concurrent, 4 threads | **leapfrog** (most workloads; provisional — noisy) |
| Concurrent, balanced, large map, 4 threads | **dashmap** (single win, very noisy — needs re-measurement) |

## High-Variance Benchmarks

Relative median-absolute-deviation (MAD) and relative std-dev computed against the median. Thresholds: rel-MAD > 5% or rel-std > 10%. **All 4-thread concurrent groups are noisy**; worst offenders:

- **4-thread concurrent (rel-std 20–93%)**: papaya read-heavy 100k (rel-std 93%), dashmap read-heavy 100k (83%), leapfrog read-heavy 100k (75%), flurry balanced 100k (74%), scc write-heavy 100k t-1 (69%), dashmap balanced 100k (66% — this is the group dashmap "wins"), leapfrog balanced 1k (54%), flurry read-heavy 100k (53%).
- **High rel-MAD (skewed distributions)**: flurry read-heavy 10k @4thr (43%), scc balanced/read-heavy 100k @4thr (37%/34%), dashmap write-heavy 100k @4thr (34%), txmap read-heavy 100k @4thr (30%), flurry balanced 100k @2thr (19%).
- **Serial outliers**: flurry remove 100k (rel-std 31%, rel-MAD 13%), starshard key-sensitivity String<16> (rel-std 31%, rel-MAD 8.5%), ahash clone 100k (rel-std 22%), ahash iterate 10k out-of-the-box (rel-MAD 7.1%).
- **Fast winners with noisy measurements**: leapfrog and txmap at 4 threads (rel-std up to 75% and 46% respectively), and dashmap's single 4-thread win (median 939 µs vs mean 1,329 µs). Their 4-thread rankings should be treated as provisional.

## Implementation Limitations

Based on wrapper code in `src/maps/`:

- **ahash** — Requires `K: Hash + Eq`; `V: Clone` for `get_cloned`. Supports custom hasher. Serial only (no `&self` insert/remove impls), so absent from concurrent workloads.
- **btreemap** — Requires `K: Ord` (sorted map, no hashing). No custom hasher support → excluded from same-hasher groups. Serial only.
- **concread** — Copy-on-write concurrent map. Heavy bounds: `K: Clone + Debug + Hash + Eq + Send + Sync + 'static`, `V: Clone + Send + Sync + 'static`. No custom hasher; **no `Clone` impl** → excluded from clone benchmark. Every write commits a full COW transaction — dramatically slowest in serial workloads (6.9–73 ms).
- **dashmap** — `K: Hash + Eq`; `H: BuildHasher + Clone`; `V: Clone` needed for lookups. Fully featured: custom hasher, `&self` mutation, `Clone`.
- **flurry** — Requires **`K: Ord`** (unusual for a hash map) plus `K: Sync + Send + Clone + Hash`, `V: Sync + Send`. Requires `pin()` per operation. Excluded from `create` (constructs a `seize::Collector` per map — "too slow"). Slow across nearly all ops.
- **hashbrown** — `K: Hash + Eq`; custom hasher supported; serial only.
- **horde** — `K: Clone + Hash + Eq`, `V: Clone` for mutation. Custom hasher supported. **Mutation requires `&mut self`** (write guard) → no `&self` insert/remove, excluded from concurrent workloads. `Clone` supported. Fastest remover.
- **immutable-chunkmap** — Persistent map: `K: Clone + Ord`, `V: Clone`. No custom hasher; serial only (`insert_cow`/`remove_cow` mutate in place via copy-on-write). Clone is **O(1) structural sharing** (~4 ns at any size) — but any write after cloning triggers COW cost not captured by the clone benchmark.
- **indexmap** — `K: Hash + Eq`; custom hasher; serial only. Insertion-ordered iteration is the fastest (contiguous entries). **`remove` uses `swap_remove`** — does not preserve order of remaining entries (semantic difference vs. other maps).
- **leapfrog** — Requires **`K: Eq + Hash + Copy`** (keys must be `Copy`; excluded from String key-sensitivity) and `V: leapfrog::Value` (u64 qualifies). Hasher must be `Default`. Supports `&self` mutation (concurrent). **No `Clone` impl** → excluded from clone benchmark.
- **papaya** — `K: Hash + Eq`; custom hasher; `pin()` per operation; concurrent + `Clone` supported. Lookup-miss at map-size 1k measures 4.85 µs vs 0.96 µs at 10k — a 5× anomaly vs. its own scaling and vs. every other implementation (see Recommendations).
- **rustc-hash** — `K: Hash + Eq`. **No custom hasher** (fixed FxBuildHasher) → excluded from all same-hasher groups; therefore no lookup/remove/key-sensitivity coverage. Serial only. FxHash is fast but weak against adversarial key distributions.
- **scc** — `K: Hash + Eq`; custom hasher; concurrent via `*_sync` APIs; `Clone` supported.
- **starshard** — Heavy bounds: `K: Clone + Hash + Eq + Send + Sync`, **`V: Clone + Send + Sync` (Clone required even for insert)**, `H: BuildHasher + Clone + Send + Sync`. Fixed 8 shards. Default FxBuildHasher. Concurrent + `Clone` (~60 ns, cheap since it clones shard structs, not contents... note the clone benchmark clones the whole map). Slow per-operation in this harness.
- **std** — `K: Hash + Eq`; custom hasher; serial only. Default `RandomState` (SipHash) makes it slow in out-of-the-box mixed workloads (e.g., 3–4× slower than ahash/hashbrown), but with the common hasher it is within ~3% of the fastest on every lookup benchmark.
- **txmap** — `K: Clone + Hash + Eq`; custom hasher; concurrent via default `MutexPolicy`; `get_cloned`/`Clone` supported. Fastest at 2-thread concurrency.

## Recommendations

1. **Investigate the papaya lookup-miss anomaly**: 4.85 µs at map-size 1k vs 0.96 µs at 10k (5×, with low MAD — consistently slow, not noise). Likely a harness/pinning artifact or a pathological small-map path; re-run before trusting papaya's lookup-miss numbers.
2. **Re-measure 4-thread concurrent workloads**: relative std-dev up to 93% and large median-vs-mean gaps (e.g., dashmap balanced 100k @4thr: median 939 µs, mean 1,329 µs) indicate bimodal/scheduling interference. Longer measurement time, dedicated/isolated cores, or more iterations are needed before leapfrog/txmap/dashmap ordering at 4 threads can be considered conclusive.
3. **Add out-of-the-box lookup/remove benchmarks**: rustc-hash dominates every serial out-of-the-box group it appears in but cannot join same-hasher groups (no custom hasher), so there is no data on how it compares for lookups/removals against ahash/hashbrown/std.
4. **Watch the measurement floor**: several serial ops run at ~230–290 ns per 100 lookups (~2.3–2.9 ns/op), and immutable-chunkmap/starshard clones are ~4 ns / ~60 ns — near the harness resolution. Treat sub-10 ns differences as noise.
5. **Validate immutable-chunkmap clone semantics**: the ~4 ns clone is real (structural sharing), but the benchmark measures only the clone; writes-after-clone incur COW cost. If the use case is "clone then mutate independently," a clone+write benchmark would change the picture.
6. **Document indexmap's `swap_remove`**: its removal benchmark doesn't preserve insertion order; if order preservation matters, this is a functional (not just performance) difference.
7. **Consider dropping or re-architecting flurry and concread**: flurry is excluded from create (collector overhead) and is never competitive; concread's COW-per-write design makes it 10–100× slower than alternatives in serial use — neither contributes actionable signal as currently benchmarked.
