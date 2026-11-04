<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# [PROJECT-ID] — Benchmark Specification

**Benchmark version:** 0.1.0  
**Owner:**  
**Project types:** Benchmark / Quantitative Research  
**Last updated:** YYYY-MM-DD

## 1. Objective and scope

State the capability, property, performance characteristic, or research question the benchmark is intended to evaluate. State what it does not measure.

## 2. Benchmark corpus

Describe the cases, data, scenarios, coverage, generation method, selection criteria, and licence. Distinguish synthetic, curated, and observed production data.

## 3. Reference implementation or oracle

Explain how expected outputs are established. Record the source of reference answers, independent calculation method, tolerances, and known uncertainty. Do not treat the implementation under test as its own independent oracle.

## 4. Baselines and comparison conditions

Identify each candidate method or implementation and the reason it is included. Define common hardware, toolchain, configuration, data, warm-up, repetitions, and resource constraints where relevant.

## 5. Evaluation metrics

| Metric | Definition | Unit | Measurement method | Acceptance / interpretation |
|---|---|---|---|---|
| | | | | |

Explain aggregation, outlier handling, uncertainty intervals, and practical significance.

## 6. Leakage and validity controls

For empirical or ML benchmarks, define training, tuning, validation, and held-out sets. Address duplicate contamination, look-ahead, target leakage, survivorship, data snooping, and benchmark overfitting as appropriate.

## 7. Reproduction procedure

Record exact commands, environment, software versions, seeds, configuration, dependencies, and output format. A reviewer should be able to reproduce the benchmark without hidden state.

## 8. Results and reporting

Report all tested methods, unsuccessful runs, deviations, error bars, resource use, and failure cases. Do not report a single score without its evaluation conditions.

## 9. Limitations and benchmark governance

Explain what the corpus does not represent, when it should be revised, who can approve new cases, and how a benchmark version is preserved to avoid silently changing the evaluation target.
