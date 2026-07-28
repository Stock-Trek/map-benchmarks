# Map benchmarks

Benchmarks map implementations with a focus on atomicity

Uses [Criterion](https://crates.io/crates/criterion)

[Report can be found here](https://stock-trek.github.io/map-benchmarks)

group.throughput(Throughput::Elements(size as u64))
always use blackbox for keys and returned values
alway use returned values in an accumulator
always pre-allocate for insert

mixed:
  [100K, 1M]
  [u64, String(medium)]
  { 20% lookup, 80% insert }
  { 50% lookup, 20% insert, 20% update, 10% remove }
  { 80% lookup, 5% insert, 10% update, 5% remove }
  { 90% lookup, 5% insert, 10% update, 5% remove }

key sensitivity:
  1M
  [u64, UUID, Byte(32), String(short), String(long)]
  lookup(existing)

concurrency:
  1M
  u64
  { 80% lookup, 20% insert }
  Threads: [1, 2, 4]
  [RwLock, Mutex]
  NB. Thread-pinning

edge-cases:
  [10K, 100K, 1M]
  u64
  [iterate, remove(existing)]
