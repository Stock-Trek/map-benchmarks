# Context

- No baseline or change data is available. Only current absolute measurements exist.
- Each benchmark has a `new/estimates.json` file.
- The Criterion directory structure is typically:
  `target/criterion/<benchmark_name>/<implementation>/new/estimates.json`
  where `<benchmark_name>` identifies the operation/workload and `<implementation>` identifies the map implementation.
- Inside `estimates.json`, use `median.point_estimate` as the primary timing value if present; otherwise use `mean.point_estimate`. Timings are in nanoseconds.
