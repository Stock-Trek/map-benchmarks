# Required outcomes

- A full report in `docs/REPORT.md`
- An executive summary in `docs/EXECUTIVE_SUMMARY.md`

## Report

Include:

1. **Scope**
   - Which map implementations were benchmarked.
   - Which distinct benchmark operations/workloads were used.

2. **Per-Operation Comparison**

    - For each benchmark operation, identify the fastest implementation.
    - [table: operation, fastest implementation, fastest time, close contenders (within 10%) with times]
    - Identify any other implementations that are within 10% of the fastest median time (i.e., close enough to be practically equivalent). Report their times and the ratio to the fastest.
    - If no implementation is within 10%, state that the fastest is clearly ahead.

3. **Cross-Operation Patterns**

    - Which implementation is most often the fastest?
    - [table: implementation, # times fastest, # times within 10% of fastest, notes]
    - Which implementation(s) is/are recommended for each use case?
    - [table: use case, recommended implementation(s)]
    - Are there cases where different implementations excel in different operations (trade-offs)?
    - Are there any implementations that are consistently close to the fastest across many operations?

4. **Stability**

    - Identify high-variance benchmarks using `median_abs_dev` or `std_dev`.
    - Flag any fast implementations which have unusually noisy measurements.

5. **Implementation Limitations**

    - For each implementation, examine the wrapper code in `src/maps/` to determine:
      - Required trait bounds (e.g., `K: Eq + Hash`, `K: Ord`, `V: Clone`).
      - Unavailable functionality (e.g. no API required for a benchmark, no custom hasher).
      - Ergonomically unusual requirements
      - Any other restrictions
    - Use the directory or module names in `src/maps/` to identify each implementation.
    - Report findings per implementation.
    - [bullets per implementation, based on src/maps wrapper code]

6. **Recommendations**

    - Which implementations or benchmarks need further investigation?
    - Any measurement quality issues that could affect conclusions.

## Executive Summary

A few paragraphs, summarising the key findings of the report. The audience will be senior engineers who are authors and consumers of map implementations.
