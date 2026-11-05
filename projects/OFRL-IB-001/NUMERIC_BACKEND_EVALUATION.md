<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Numeric Backend Evaluation Protocol

**Document version:** 0.1.0  
**Status:** Decision protocol — NO BACKEND SELECTED  
**Scope:** OFRL-IB-001 V1 only  
**Precondition:** A Model Owner-approved Numeric Profile and Finance/Accounting review  
**Decision authority:** Model Owner, Finance/Accounting reviewer, Engineering reviewer

## 1. Purpose and decision boundary

This document defines how numeric candidates will be evaluated after the V1 operating domain and numerical policy have been approved. It is not a backend recommendation and it does not approve the current POC defaults.

The production decision must be made against one versioned Numeric Profile. The profile must specify finite input and intermediate bounds, source precision, division and reporting policies, separate error budgets, and operational resource ceilings. A candidate that is fast but fails any hard correctness, representability, determinism, resource, or dependency gate is ineligible.

The decision is deliberately split into:

1. **Mathematical domain acceptance:** the intended V1 inputs and formula graph have finite, reviewed bounds.
2. **Candidate representation acceptance:** a candidate can represent and calculate every admitted path without silent loss, untyped failure, or resource-limit breach.
3. **Operational acceptance:** deterministic performance and resource use fit the approved workload.
4. **Production adoption:** the selected candidate is integrated through a typed production arithmetic boundary and independently reviewed.

Passing one stage does not imply passing the next.

## 2. Evidence currently available

The isolated POC at `poc/model-numeric-envelope/` compares:

| Candidate | POC version | Evidence role | Current disposition |
|---|---|---|---|
| `rust_decimal` | 1.43.0 | Existing production-type candidate; fixed 96-bit coefficient and scale 0–28 | Not approved |
| `bigdecimal` | 0.4.11 | Arbitrary-precision decimal candidate | Research comparator; not approved |
| `num_rational` + `num_bigint` | 0.4.2 + 0.4.8 | Exact rational reference | Correctness oracle; not a production recommendation |

The POC compares the complete V1 five-year formula path against exact-rational outputs. It includes an 800-value independent golden suite (BM-002/BM-003) and a standard-library Python reference. These establish evidence for the documented synthetic cases, not full-domain correctness or accounting approval.

The measured shared-runner timing observations used 5 warm-ups and 9 samples of 40 five-year forecasts for two synthetic inputs. They showed BigDecimal at approximately 1.50×–2.21× the fixed-Decimal median and Exact Rational at approximately 7.52×–10.30× in that single observed environment. The measurements do not include peak memory, allocations, tail latency under load, concurrency, file I/O, JSON decoding, validation, serialization or XLSX costs. They are observational, non-gating evidence.

The POC's BigDecimal division path uses the test build's configured 80-significant-digit / HalfEven behaviour. Its current generic arithmetic adapter invokes the BigDecimal division operator directly; it does not demonstrate an immutable, per-ModelRun arithmetic context or prove that ambient/default context cannot affect production calculations. This gap must be closed before BigDecimal can be accepted.

The 10^18 and 10^22 monetary scaling probes are capacity probes only. They do not define a company-size limit or an admitted production input domain.

## 3. Hard acceptance gates

A candidate is eligible only when every gate below is PASS against the same approved profile version.

| Gate | Required evidence | Failure disposition |
|---|---|---|
| Domain coverage | Every input and formula intermediate is within the approved profile | Reject candidate for this profile |
| Arithmetic semantics | Addition, subtraction, multiplication, division, scale handling and rounding match the approved policy | Reject until semantics are corrected and retested |
| Representability | No silent overflow, truncation, saturation, or unapproved scale reduction at any node | Reject candidate or narrow/version the model domain through an approved change |
| Error budgets | Exact-rational propagation and candidate outputs fit each separate approved budget | Reject candidate or revise policy only through documented review before outcome inspection |
| Serialization | Parse/format round-trip is deterministic and preserves the approved canonical Decimal contract | Reject until corrected |
| Determinism | Same canonical input, profile, context, model and engine identity produce identical canonical outputs | Reject until deterministic |
| Resource bounds | CPU, peak memory, allocations, coefficient growth and concurrency fit the approved ceilings | Reject candidate for the approved workload |
| Failure semantics | Overflow, division-by-zero, invalid context and resource exhaustion produce typed, stable failures; no silent fallback | Reject until corrected |
| Dependencies | Exact lockfile, source, licence/notice and advisory disposition are accepted | Block production adoption |
| Migration | Production validator/domain types and public contracts can migrate without ambiguous mixed numeric authority | Block adoption until a migration plan is reviewed |

These are veto gates, not weighted preferences. A low runtime cost cannot compensate for a failed arithmetic or domain gate.

## 4. Candidate-specific verification

### 4.1 Fixed-width Decimal

The candidate-specific proof must use the actual fixed representation constraints, not only a decimal digit-count approximation.

- Verify the supported scale and coefficient range for each monetary and ratio operation.
- Compare each exact-rational intermediate against the exact maximum coefficient value, not merely the number of decimal digits.
- Exercise checked addition, subtraction, multiplication and division at every approved node's lower/upper endpoint and one representable step inside/outside.
- Record where multiplication scale is reduced, if at all; prove that any reduction follows the approved arithmetic policy and contributes to the correct error budget.
- Verify division-by-zero and overflow are typed failures and cannot be confused with a valid zero result.
- Verify canonical input parsing and output formatting round-trip without losing information.
- Include migration impact because production input/history domain types currently use `rust_decimal::Decimal`.

### 4.2 Arbitrary-precision Decimal

Arbitrary coefficient precision removes a fixed-width mantissa ceiling; it does not remove the need for finite scale, coefficient-size, CPU or memory limits.

- Replace reliance on ambient/default division settings with an explicit immutable arithmetic context owned by each ModelRun.
- The context identity must include division fractional/significant precision as selected by the approved policy, rounding mode, and implementation/version.
- Prove all context-aware operations use that context; a single environment variable or developer-machine default is not sufficient authority.
- Verify multiplication coefficient growth separately from division precision. Multiplication can continue to grow coefficient digits even when division precision is finite.
- Bound maximum coefficient digits, scale, numerator/denominator intermediates (where applicable), memory and runtime for repeated five-year paths and concurrent runs.
- Verify serialization is canonical and does not depend on incidental BigDecimal internal scale or formatting.
- Map library/context errors into typed model calculation errors.

### 4.3 Exact Rational

Exact Rational is retained as an independent mathematical oracle for the supported V1 formulas and boundary cases.

It is not automatically the preferred production backend. Any proposal to use it in production must separately establish finite numerator/denominator growth, CPU, memory, serialization, resource controls and operational latency against the approved profile. The fact that it is exact does not make its resource use bounded.

## 5. Required candidate test matrix

Every candidate must run the same profile-generated corpus.

| Test family | Minimum scope | Evidence |
|---|---|---|
| Source parsing | Minimum/maximum magnitude, integer digits, fractional digits, canonical grammar, negative/zero rules | Typed accept/reject result and round-trip |
| Formula intermediates | Each node in opening/historical validation and all five forecast periods | Exact bound and candidate value/error |
| Scale | Just below, at and just above each approved scale limit | No silent normalization or loss |
| Coefficient | Just below, at and just above the exact candidate coefficient capacity | Checked result or typed failure |
| Division | Repeating and terminating divisions, approved rounding modes, zero denominator | Exact-rational difference and stable error |
| Error propagation | Arithmetic, accounting, schedule and display/export budgets independently | Per-period and worst-case bound |
| Five-year state | Working capital, PP&E, debt, interest, tax, dividends, cash, retained earnings and statements | All periods, no hidden plug |
| Determinism | Repeat identical run; change only the declared profile/context/version | Stable output and changed identity when policy changes |
| Negative cases | Unsupported inputs, unrepresentable intermediates, resource exhaustion | Fail-closed typed diagnostics |
| Migration | Existing input validator, serialization and public diagnostic contract | Reviewed compatibility report |

All expected values must originate from the independent exact-rational reference or independently reviewed values. Candidate output must never generate its own expected values.

## 6. Controlled operational measurement protocol

Performance measurement begins only after a candidate passes the correctness and representability gates on the approved profile.

The measurement record must identify:

- Exact repository commit, profile ID/version and profile content hash.
- Candidate crate/version, lockfile hash, Rust toolchain, compiler flags and build profile.
- CPU model, physical/virtual CPU allocation, memory, operating system, runner/host identity and relevant environment.
- Scenario/profile input IDs and hashes.
- Warm-up count, measured sample count, executions per sample and concurrency level.
- Wall-clock p50/p95/p99 and maximum latency; throughput under declared concurrency.
- Peak resident memory, allocation count/bytes where measurable, and coefficient/scale maxima observed.
- Any retries, dropped runs, throttling or environmental anomalies.
- Raw machine-readable samples, summary output and artifact SHA-256.

Use the same machine class and workload for candidate comparisons. Repeat runs sufficiently to characterize variability; one shared CI timing run is not a controlled acceptance benchmark. Keep correctness CI gating and performance observation jobs separate until the measurement environment is stable.

The required thresholds come from the approved profile's resource limits and target workload. This protocol intentionally does not invent CPU, memory, allocation or concurrency limits.

## 7. Decision procedure

1. Freeze the approved Numeric Profile and its source/evidence references.
2. Generate exact magnitude, source-scale, intermediate-scale and coefficient envelopes for every formula node.
3. Run candidate correctness and boundary suites against the same frozen profile.
4. Reject any candidate that fails a hard gate.
5. Measure operational behaviour only for candidates that survive correctness gates.
6. Review dependency/source/licence/security status and production migration cost.
7. Produce a candidate comparison report with PASS/FAIL/OPEN per gate; do not collapse the matrix into an opaque single score.
8. Record the chosen candidate, rejected candidates, context, limitations, reviewers, CI commit and artifact hashes in a separate versioned decision record.

A profile or tolerance must not be widened after a candidate fails merely to obtain a PASS. If the admitted domain or model convention changes, issue a new profile/model-policy version, document the rationale and rerun the complete matrix.

## 8. Current decision

- Numeric Profile: **NOT APPROVED**.
- Model Owner input-domain decisions: **OPEN**.
- Finance/Accounting approval: **NOT EVIDENCED**.
- Production arithmetic backend: **NOT SELECTED**.
- BigDecimal run-scoped context: **NOT PROVEN**.
- CPU/memory/allocation/concurrency acceptance: **NOT MEASURED**.
- Production Forecast Engine: **NOT AUTHORIZED**.

The exact-rational derivation tool and test-only fixture validate the evaluation process. They do not approve any field maximum, resource ceiling, error budget, arithmetic context, backend or production calculation.
