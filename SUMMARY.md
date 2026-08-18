# AI generated summary: Tue Aug 18 15:24:07 UTC 2026


All data gathered. Here is the analysis.

---

## Executive Summary

Only bulk-clear data exists in `target/criterion` (out-of-the-box hashers, 100 and 10k entries); the other nine benchmark files produced no results. At 100 entries, clearing is a statistical tie among std, ahash, rustc-hash, and indexmap (~35 ns, within 0.2% of each other); horde (42 ns) and hashbrown (48 ns) are 1.2–1.4× slower. At 10k entries, horde wins outright (137 ns vs 226 ns for ahash, 1.65×) because its clear swaps in a fresh table, and it beats concurrent maps by 8–7,000× (flurry: 968 µs, papaya: 722 µs, concread: 338 µs). No implementation is within 10% of horde at 10k. The trade-off: horde's win relies on deferred reclamation and its measurement is noisy (CV ≈ 16%). The plain hash maps form a tight, stable cluster at both sizes and are the safe default; concurrent maps are structurally slow at clear.

## Benchmark Scope

- **14 implementations** benchmarked (default hashers): ahash, btreemap, concread, dashmap, flurry, hashbrown, horde, indexmap, papaya, rustc-hash, scc, starshard, std, txmap.
- **One operation, two workloads**: `bulk-clear` — clear a pre-populated `u64→u64` map of **100** and **10,000** entries. Setup (map creation) runs outside the timed section; only `clear()` is timed.
- **Only the "out-of-the-box" group** has results; the "same-hasher" group (isolates hashing cost) has no data.
- **Only `benches/bulk_clear.rs` produced results**; the other 10 bench files (create, insert, lookup_hit, lookup_miss, iterate, remove, clone, mixed_read_write, concurrency, key_sensitivity) have no Criterion data.
- **Excluded implementations**: leapfrog is commented out in the bench ("no clear method"); immutable_chunkmap has no `BenchMapMutClear` impl. Neither appears in results.
- Timings below are `median.point_estimate` from `estimates.json` (nanoseconds), converted to µs/ms where large. Warm-up 1 s, measurement 2 s per benchmark.

## Fastest per Operation

| Operation | Fastest | Fastest time | Close contenders (within 10%) |
|---|---|---|---|
| bulk-clear, 100 entries | **std** | 34.96 ns | ahash 34.98 ns (1.00×) · rustc-hash 35.02 ns (1.00×) · indexmap 35.02 ns (1.00×) — statistically tied |
| bulk-clear, 10,000 entries | **horde** | 137.46 ns | **none** — next is ahash 226.41 ns (1.65×). Fastest is clearly ahead. |

Full ranking, 100 entries (ns): std 34.96 · ahash 34.98 · rustc-hash 35.02 · indexmap 35.02 · horde 42.37 (1.21×) · hashbrown 48.37 (1.38×) · starshard 123.78 · btreemap 275.13 · scc 347.35 · dashmap 1,133.01 (1.13 µs) · concread 2,720.64 (2.72 µs) · txmap 2,949.66 (2.95 µs) · papaya 5,143.10 (5.14 µs) · flurry 5,281.00 (5.28 µs).

Full ranking, 10,000 entries: horde 137.46 ns · ahash 226.41 · rustc-hash 227.90 · hashbrown 228.06 · indexmap 235.57 · std 237.72 · starshard 419.06 · txmap 4,142.61 (4.14 µs) · scc 29,143.34 (29.14 µs) · btreemap 61,696.67 (61.70 µs) · dashmap 107,201.10 (107.20 µs) · concread 338,292.17 (338.29 µs) · papaya 722,392.14 (722.39 µs) · flurry 967,541.72 (967.54 µs).

## Implementation Ranking Summary

| Implementation | # fastest | # within 10% | Notes |
|---|---|---|---|
| std | 1 | 1 | Fastest at 100; tied with ahash/rustc-hash/indexmap |
| horde | 1 | 0 | Fastest at 10k via table-swap clear; high variance |
| ahash | 0 | 1 | Within 0.1% at 100; 2nd at 10k |
| rustc-hash | 0 | 1 | Within 0.2% at 100 |
| indexmap | 0 | 1 | Within 0.2% at 100 |
| hashbrown | 0 | 0 | Always in the fast cluster, but 1.38×/1.66× (outside 10%) |
| starshard | 0 | 0 | 3.5× / 3.0× |
| btreemap | 0 | 0 | 7.9× / 449× |
| scc | 0 | 0 | 9.9× / 212× |
| dashmap | 0 | 0 | 32× / 780× |
| concread | 0 | 0 | 78× / 2,461× |
| txmap | 0 | 0 | 84× / 30× |
| papaya | 0 | 0 | 147× / 5,255× |
| flurry | 0 | 0 | 151× / 7,039× |

**Patterns:** std/ahash/rustc-hash/indexmap are statistically tied at 100 entries (spread 0.07 ns); at 10k the same plain-hash cluster spans only 226–238 ns (5%) but sits 1.65× behind horde. Trade-off: horde's clear is a table replacement (O(1)-ish), so it wins at 10k but its per-call overhead (42 ns) loses to the ~35 ns hash maps at 100 entries. Concurrent maps are consistently 8–7,000× slower at clear; this is structural (COW transactions for concread, shard draining for dashmap/scc/starshard, pin-guarded drains for flurry/papaya). No implementation other than horde ever breaks the 10% barrier at 10k.

## High-Variance Benchmarks

- **bulk-clear 10k / horde** — biggest flag: `std_dev` 21.5 ns = **15.6% of median** (CI upper bound 33.8 ns ≈ 25%); `median_abs_dev` is only 3.9%, so variance is outlier-driven. The fastest result in the suite is also the noisiest — treat 137 ns with caution.
- **bulk-clear 10k / fast tier** — hashbrown, ahash, std all show `std_dev` CI upper bounds of **~10% of median** (22.8, 22.4, 23.2 ns on ~228 ns). Ordering within this tier is not measurement-solid at 2 s of measurement time.
- **bulk-clear 100 / flurry** — `std_dev` CI up to 404.6 ns on 5.28 µs (7.7%).
- **bulk-clear 100 / scc** — `std_dev` CI up to 31.3 ns on 347 ns (9.0%); MAD 3.6 ns.
- **bulk-clear 100 / horde** — `std_dev` 6.2% but MAD 0.21 ns (0.5%): clean median, occasional outliers.

## Implementation Limitations

Based on `src/maps/` wrappers:

- **ahash** (`ahash_benchmap.rs`): needs `K: Hash + Eq`; supports custom hashers (`BenchMapNewWithHasher`); wrapper requires `V: Clone` for `new()` (harness artifact of the clone-based `get_cloned` API). Default `ahash::RandomState` is DoS-resistant.
- **btreemap**: needs `K: Ord` (no `Hash`); **no custom-hasher support**; sorted iteration order.
- **concread**: heaviest bounds — `K: Clone + Debug + Hash + Eq + Send + Sync + 'static`, `V: Clone + Send + Sync + 'static`; every write is a copy-on-write transaction (`write()` → `commit()`), so `clear()` rewrites the whole map (structurally why it's 78–2,461× slower); **no custom-hasher impl**; all reads return clones.
- **dashmap**: `K: Hash + Eq`, `H: BuildHasher + Clone`; custom hasher supported; clear drains all shards eagerly.
- **flurry**: adds an **extra `K: Ord` bound** (hash map that needs ordering); inserts require `K: Sync + Send + Clone`, `V: Sync + Send`; every op requires a `pin()` guard.
- **hashbrown**: minimal bounds (`K: Hash + Eq` for get/insert/remove; none for clear); custom hasher supported; cleanest wrapper.
- **horde**: clear needs only `K: Hash`, but `clear()` calls `write().replace(empty, 0)` — **replaces the table rather than draining it**, with reclamation deferred to hazard-pointer collection. Semantics differ from every other map (capacity resets to 0; memory freed asynchronously).
- **indexmap**: `K: Hash + Eq`; custom hasher supported; `remove` uses `swap_remove` (order-changing — matters for remove benchmarks, not clear); insertion-order iteration.
- **papaya**: `K: Hash + Eq`; all ops require a `pin()` guard; custom hasher supported.
- **rustc-hash**: `K: Hash + Eq`; **no custom-hasher support** (fixed FxHasher); fastest hashing but not DoS-resistant.
- **scc**: `K: Hash + Eq`; custom hasher supported; all ops use synchronous `_sync` variants; `clear_sync` is a global sweep.
- **starshard**: `K: Clone + Hash + Eq + Send + Sync`, `V: Clone + Send + Sync`, hasher must be `Clone + Send + Sync`; wrapper hard-codes **8 shards**; default hasher `FxBuildHasher`.
- **std**: `K: Hash + Eq`; custom hasher supported; default `RandomState` (SipHash) is DoS-resistant but slower to hash.
- **txmap**: `K: Clone + Hash + Eq`; uses `MutexPolicy`; custom hasher supported; transactional semantics.
- **leapfrog** (not benchmarked here): `K: Eq + Hash + Copy` — **keys must be `Copy`**; `V: leapfrog::Value`; requires `H: Default` even for `new_with_hasher`; **no `clear()`** — excluded from bulk-clear (commented out in bench source).
- **immutable_chunkmap** (not benchmarked here): `K: Clone + Ord`, `V: Clone`; **no `BenchMapMutClear` impl** — silently absent from this benchmark; in-place mutation requires the COW `insert_cow`/`remove_cow` API.

## Recommendations

- **Verify horde's 10k result.** Its clear is a table swap with deferred reclamation, not an in-place drain; if eager memory release is required, the 137 ns figure overstates practical "clear" semantics. Its measurement is also the noisiest in the suite (CV up to ~25% at the CI bound) — re-measure with longer sampling before treating 1.65× as solid.
- **Extend measurement time for the 10k fast tier.** ahash/hashbrown/std show `std_dev` CI upper bounds ≈10% of median at the current 2 s window; the ~228 ns cluster ordering isn't statistically settled.
- **Fill in the missing data before broad conclusions.** Nine of ten bench files produced no Criterion output, and the "same-hasher" group (which would isolate hashing cost — relevant to rustc-hash vs std) is absent. The concurrency benchmark in particular is where concurrent maps should be judged; bulk-clear is a pathological workload for them.
- **Flag flurry (size 100) and scc (size 100)** as noisy measurements; don't read small deltas around them.
- **Decide leapfrog/immutable_chunkmap coverage.** Both are silently missing from bulk-clear (no clear API); if clear matters, add a drop-and-recreate variant so the suite doesn't drop two implementations.
- **Engineering takeaway:** for bulk-clear throughput, plain hash maps (std/ahash/rustc-hash/indexmap/hashbrown) are equivalent and safe; horde is the specialist if deferred reclamation is acceptable; concurrent maps should not be chosen on clear performance.
