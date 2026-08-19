# Executive Summary — Map Implementation Benchmarks

**Date:** 2026-08-19 · **Source:** single-run Criterion medians from `target/criterion/` (94 benchmark groups; no baseline data)

## The headline

There is **no single best map**. The field cleanly splits into three regimes, each with its own winner:

1. **Single-threaded, default-hasher workloads → rustc-hash.** rustc-hash (`FxHashMap`) is fastest in 24 of 94 benchmark groups — every serial lookup/insert/get-or-insert, most removes, and all nine serial mixed workloads — typically by 10–40% over its nearest rival. The caveat: its FxHasher is fast precisely because it is not randomized, so it is **not safe for untrusted keys** (hash-flooding). When all maps are forced to share a safe hasher (`ahash::RandomState`), **ahash / hashbrown / std are effectively tied** (within 1–2% across insert, lookup, get-or-insert, clear-and-reuse); all three are the same swiss-table design and are interchangeable.

2. **Concurrent workloads → txmap, with leapfrog close behind.** Among the 7 concurrent implementations (crossbeam-skiplist, dashmap, leapfrog, papaya, scc, starshard, txmap), **txmap wins 22 of 36 concurrent groups** (16/27 mixed workloads, 6/9 get-or-insert cache patterns) and **leapfrog wins 13** — mostly write-heavy and 3–4-thread cases. scc is the only other implementation that ever comes within 10% of the winner. The pattern is systematic: txmap dominates at 2 threads and large maps; leapfrog dominates write-heavy mixes at 3–4 threads. For a "get-or-create cache entry" pattern specifically, txmap wins 6 of 9 groups (leapfrog the other 3).

3. **Specialized structures dominate their niche.** **indexmap wins every iteration benchmark by 2.8–3.4×** (contiguous storage); **horde wins every same-hasher removal benchmark** (~1.2–1.3× faster than the nearest rival); **immutable-chunkmap** is fastest at empty-map creation (essentially zero-cost); and the **persistent maps (rpds, imbl, immutable-chunkmap)** clone in ~2–5 ns regardless of map size because their clone is an O(1) reference-count bump — a structural property, not a comparable "clone speed".

## What the numbers do not tell you yet

- **Concurrent measurements at 4 threads are noisy.** The host has 4 CPUs and the worker threads are pinned to all of them, so the timing thread shares a core with a worker. Relative scatter (median absolute deviation / median) reaches 23–56% for some winners (e.g. txmap balanced-100k @ 4 threads, dashmap read-heavy-100k @ 4 threads, where dashmap's only win has ~82% relative std-dev). **Sub-10–20% margins at 4 threads should be treated as provisional** pending a rerun with more samples.
- **Two of the 21 wrappers were never measured** (concread, flurry — excluded everywhere as "too slow"), and several implementations are absent from specific benchmarks for API reasons (leapfrog requires `Copy` keys; rpds/concurrent-map are not shareable across threads; crossbeam-skiplist has no `Clone`; horde's wrapper needs `&mut` for writes even though the underlying `SyncTable` is concurrent).
- **The benchmarks use `u64` keys/values throughout** (plus string keys in key-sensitivity), and values are always `Clone`. Results do not cover zero-copy value handling, non-`Copy` keys for leapfrog, or txmap's alternative lock policies.

## Suggested direction for consumers

- **Pick rustc-hash for single-threaded hot paths only when keys are trusted**, and hashbrown/ahash/std otherwise — they cost little (≤1.5×) and give hash-flooding resistance.
- **Pick indexmap when iteration order/performance matters**, accepting its ~1.7–4.4× penalty on writes.
- **Pick txmap for concurrent get-or-create/balanced work at low thread counts; pick leapfrog for write-heavy concurrent work at 3–4 threads; scc is a safe, consistently-close second choice.** Before standardizing on any one, run a dedicated scaling study (2/4/8/16 threads) with longer measurement windows — the current 4-thread data is too noisy to justify a single "concurrent winner" claim, and the txmap-vs-leapfrog split depends on workload mix.
- **Do not use persistent maps (rpds/imbl/immutable-chunkmap) where throughput is the goal** — they are 5–60× slower on mutation/lookup — unless structural sharing, cheap cloning, or immutability is an actual requirement.
