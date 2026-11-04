<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Numeric Backend Performance Observations

**Document version:** 0.1.0  
**Status:** CI measurement completed; observational evidence only  
**Purpose:** Add runtime-cost evidence to the numeric comparison without making a production backend decision.

## 1. Measurement scope

The benchmark executable compares the three isolated POC candidates over the same five-year forecast routine:

- `rust_decimal 1.43.0` — fixed-width Decimal candidate.
- `bigdecimal 0.4.11` — arbitrary-precision Decimal candidate.
- Exact Rational — correctness reference only; not a production backend recommendation.

Two deterministic inputs are measured:

1. Canonical synthetic Case A.
2. The deliberately aggressive (10^{18}) monetary stress case already used by the numeric correctness tests.

The (10^{22}) capacity probe is excluded from timing because `rust_decimal` is expected to fail its checked arithmetic path there. Its capacity result remains covered by the correctness tests and the Numeric Envelope report.

## 2. Measurement method

The executable uses only Rust standard-library timing primitives. It adds no benchmark framework or runtime dependency.

For each input/candidate pair it performs:

- 5 warm-up forecast executions.
- 9 measured samples.
- 40 complete five-year forecast executions per sample.
- Sorting of per-forecast sample durations and reporting min, median, and max nanoseconds.

The benchmark runs in Cargo's `release` profile on the same Ubuntu runner family and Rust toolchain matrix used by the POC. The BigDecimal context is inherited from the workflow's explicit 80-significant-digit / HalfEven environment.

The workflow uploads a separate artifact for each Rust toolchain. Each artifact includes the CSV observations and runner/toolchain metadata.

## 3. Interpretation and limitations

These observations are diagnostic evidence, not a performance acceptance test:

- Shared CI runners are noisy and can be affected by contention, CPU scheduling, virtualization and thermal behaviour.
- Min/median/max are descriptive; they are not a statistically controlled laboratory benchmark.
- The executable measures end-to-end forecast routine wall time, including input-field parsing from the already-parsed JSON value. It does not measure JSON file I/O, Excel export, cold process startup or production workload concurrency.
- Peak memory, allocation counts, tail latency under load and long-running process behaviour are not measured.
- Exact Rational is included as a reference-cost comparator only.
- No threshold is used to pass or fail CI based on elapsed time.

A production backend decision still requires an approved Numeric Profile, deterministic arithmetic context, maximum intermediate bounds, error budget, memory/throughput measurements under a controlled environment, serialization review, and dependency/licence/security approval.

## 4. Reproduction

From the POC directory:

```bash
RUST_BIGDECIMAL_DEFAULT_PRECISION=80 \
RUST_BIGDECIMAL_DEFAULT_ROUNDING_MODE=HalfEven \
cargo run --locked --release --bin backend_profile
```

The executable prints CSV to standard output. To retain a local observation:

```bash
cargo run --locked --release --bin backend_profile > backend-profile.csv
```

Record the Rust compiler version, CPU/runner identity and environment alongside local results. Do not compare timings from materially different machines as if they were controlled samples.

## 5. Decision boundary

A faster candidate is not automatically the correct financial arithmetic backend. Numeric correctness, approved business bounds, deterministic precision policy and auditable failure semantics are release gates; performance is evaluated only after those conditions are satisfied.


## 6. First CI measurement — 2026-11-04

**Source commit:** `334a1f835405d9520b24ebe6371f82b399e4be3d`  
**Workflow run:** [Numeric Envelope POC run 41784590895](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41784590895)

Both toolchain jobs ran on the same observed GitHub-hosted runner hostname and reported an AMD EPYC 7763 virtual CPU environment with 4 vCPUs. The arithmetic context was BigDecimal precision 80 / HalfEven. Each measurement used 5 warm-ups and 9 samples of 40 complete forecast executions; the table reports the median per forecast.

| Toolchain | Scenario | rust_decimal (µs) | BigDecimal (µs) | Exact Rational (µs) | BigDecimal / fixed | Rational / fixed |
|---|---|---:|---:|---:|---:|---:|
| Rust 1.85.0 | Case A | 42.117 | 87.877 | 316.808 | 2.09× | 7.52× |
| Rust 1.85.0 | Stress (10^{18}) | 43.032 | 64.566 | 367.050 | 1.50× | 8.53× |
| Rust 1.99.1 | Case A | 37.235 | 82.277 | 324.448 | 2.21× | 8.71× |
| Rust 1.99.1 | Stress (10^{18}) | 38.158 | 63.559 | 392.920 | 1.67× | 10.30× |

The full CSV and runner metadata are retained as two separate workflow artifacts:

- [Rust 1.85.0 timing artifact](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41784590895/artifacts/13300518040) — SHA-256 `988b161dcd40c6ece3ab215fefa6d0e04a339bae9e611f14e890182af722c59a`.
- [Rust 1.99.1 timing artifact](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41784590895/artifacts/13301030454) — SHA-256 `bf6b4309549fa9a6e66623d513f2a1830caf9b29546775c615f9f3cc6a42986b`.

### Interpretation

In these observations, BigDecimal's median forecast time was approximately 1.50×–2.21× the fixed-Decimal candidate. Exact Rational was approximately 7.52×–10.30×. The selected stress case does not reverse that ordering.

This is useful evidence that arbitrary-precision Decimal has a measurable runtime cost in the tested implementation. It does not establish production latency, throughput, memory cost or total application cost. This is one CI execution environment with small synthetic inputs, not a statistically controlled benchmark campaign. The `forecast` routine parses values from an already-parsed JSON object; file I/O, JSON decoding, validation, serialization, workbook generation, allocations, peak memory and concurrent workloads are not measured.

The numbers must not be generalized to other hardware, model sizes, company scales or concurrent production loads. They do not justify choosing `rust_decimal` while the (10^{22}) POC path has a checked overflow; nor do they justify choosing BigDecimal without an approved finite domain and bounded resource policy.

**Decision remains OPEN.** Numeric Profile, deterministic per-run arithmetic context, production migration design, memory/resource limits and source/licence/security acceptance still control backend selection.
