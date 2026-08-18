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

**Measurement caveats:** 1s warm-up/2s measurement is short for 100k-scale runs; the 3.6 ns clone timing is at the harness noise floor; lookup benchmarks measure ~2.3 ns/op (timer-resolution boundary); concurrent iterations include thread spawn/pinning overhead.
