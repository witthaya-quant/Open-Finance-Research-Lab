<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Numeric Profile Decision Worksheet

**Worksheet version:** 0.5.0  
**Status:** Working paper — NOT APPROVED  
**Purpose:** Capture the evidence and decisions required before selecting a production arithmetic backend or implementing forecast calculations.

## 1. Approval rule

This worksheet is deliberately incomplete. Blank values mean **TBD**, not zero, unlimited, or implicitly accepted.

Do not populate a limit from a stress-test point simply because a candidate can calculate it. The owner must define the model's intended operating domain, convert it to explicit bounds, prove formula-chain headroom, and approve the associated failure/reporting policy.

A production numeric profile is approved only when:
- Every required field below has a value or a documented not-applicable rationale.
- Input and intermediate bounds are derived from the declared V1 model domain.
- Candidate behaviour is tested at lower/upper boundaries and just outside them.
- Error budgets are expressed in model units and reporting currency.
- Finance/accounting and engineering review are recorded.
- The selected backend and arithmetic context are pinned and reproducible.

## 2. Model domain

| Field | Required decision | Value / status |
|---|---|---|
| Profile ID | Stable identifier | TBD |
| Profile version | Semantic version | TBD |
| Effective model convention version | Version reviewed with this profile | TBD |
| Reporting currency | One supported currency per run | TBD |
| Unit scale | units / thousands / millions | TBD |
| Minimum material monetary amount | In model units and reporting currency | TBD |
| Maximum monetary input | By relevant field class | TBD |
| Maximum calculated balance | Assets, liabilities, equity, cash, debt | TBD |
| Maximum source integer digits | Explicit lexical bound | TBD |
| Maximum source fractional digits | Explicit lexical bound | TBD |
| Negative-value domain | Which fields may be negative and why | TBD |
| Maximum revenue growth | Include permitted lower bound | TBD |
| Maximum COGS / revenue | No implicit 100% cap; define upper domain | TBD |
| Maximum operating expense ratio | Explicit domain | TBD |
| Maximum DSO / DIO / DPO | Days | TBD |
| Maximum CapEx / revenue | Explicit domain | TBD |
| Maximum depreciation rate | Explicit domain | TBD |
| Maximum debt drawdown / repayment | Explicit model-unit bounds | TBD |
| Maximum interest rate | Explicit domain | TBD |
| Maximum tax rate | Explicit domain | TBD |
| Maximum dividend payout ratio | Explicit domain | TBD |

For each bound, record its source: product requirements, intended users/data, financial materiality, regulatory/accounting context (if applicable), or a deliberately conservative operational limit. Record owner and evidence reference.

### 2.1 Current convention audit — 2026-11-04

A formula-level review of `MODEL_CONVENTIONS.md` found that the following currently proposed domains have no finite upper bound. This prevents a finite worst-case envelope from being derived from the current specification.

| Input class | Current convention | Decision still required |
|---|---|---|
| Revenue growth | Lower bound -100%; no upper bound stated | Approved finite maximum annual growth |
| COGS / revenue | Non-negative; no 100% cap | Approved finite maximum; preserve loss-making outcomes |
| Operating expenses / revenue | Non-negative; no upper bound | Approved finite maximum |
| DSO / DIO / DPO | Non-negative days; no upper bound | Separate finite maximum for each driver |
| CapEx / revenue | Non-negative; no upper bound | Approved finite maximum |
| Debt drawdowns | Non-negative; no upper bound | Maximum per-period drawdown and/or total debt cap |
| Interest rate | Non-negative; no upper bound | Approved finite maximum and convention |
| Opening / last historical balances | Structurally constrained by current validator, but no numeric profile maxima | Per-field maxima for cash, receivables, inventory, gross PP&E, accumulated depreciation, payables, debt, share capital and absolute retained earnings |

Tax, depreciation and dividend payout ratios already have proposed 0-to-1 intervals. Principal repayment is constrained by opening debt plus drawdown, but that does not bound debt if drawdowns are unbounded.

The complete symbolic formula-chain derivation is in [NUMERIC_PROFILE_DERIVATION.md](NUMERIC_PROFILE_DERIVATION.md). Do not populate the TBD cells from the 10^18 or 10^22 POC stress points; those are candidate capacity probes, not Model Owner-approved operating limits.

### 2.2 Versioned profile input and exact derivation

The machine-readable profile input contract is [`ofrl-ib-001-numeric-profile-0.1.0.schema.json`](../../schemas/ofrl-ib-001-numeric-profile-0.1.0.schema.json). It requires finite field-specific magnitude bounds, source precision by monetary / ratio / day class, resource targets, separate error-budget inputs, and an explicit approval state.

The candidate-neutral derivation tool is [`tools/numeric-envelope/derive.cjs`](tools/numeric-envelope/derive.cjs). It uses exact `BigInt` rational arithmetic and enumerates the opening Balance Sheet, three historical validation periods, and all five forecast formula periods. It emits formula-node bounds, source/intermediate scale evidence, exact outward-rounded coefficient-integer upper bounds at each declared scale, coefficient digit counts, error-budget unit conversion and explicit conditional states for division policies that are not yet specified. Comparison-only validations are not counted as arithmetic operations.

The separate [candidate-capacity evaluator](tools/numeric-envelope/candidate-capacity.cjs) compares every derived node with the exact `rust_decimal 1.43.0` 96-bit coefficient ceiling and scale 0–28, and with the profile's declared BigDecimal scale/coefficient ceilings. Its output is static evidence only. A missing division scale or coefficient bound remains CONDITIONAL; it never chooses a backend or authorizes production calculations.

The committed [synthetic test fixture](tools/numeric-envelope/test/fixtures/synthetic-profile.test-only.json) is deliberately marked `test_only`; its values are used only to prove the tool and CI contract. It is not a model-domain recommendation, approved tolerance, or production profile. Do not copy its values into an approval record.

For a real profile, use the following command with a complete profile file:

```bash
node projects/OFRL-IB-001/tools/numeric-envelope/derive.cjs path/to/numeric-profile.json
```

The `--require-approved` flag rejects test-only profiles. Even a profile marked approved only produces an engineering derivation; backend adoption and Production Forecast Engine authorization remain separate decisions. Null division/reporting policy fields mean unresolved, not zero or implicitly accepted.


## 3. Formula-chain intermediate envelope

The maximum permitted input is not necessarily the maximum intermediate value. Multiplication may occur before division, so bounds must be calculated for every step, not only final line items.

| Formula / schedule | Input bounds | Intermediate expression | Derived maximum absolute value | Headroom / capacity evidence | Status |
|---|---|---|---|---|---|
| Revenue | TBD | Prior revenue × (1 + growth) | TBD | TBD | OPEN |
| COGS | TBD | Revenue × COGS ratio | TBD | TBD | OPEN |
| Operating expenses | TBD | Revenue × Opex ratio | TBD | TBD | OPEN |
| AR numerator | TBD | Revenue × DSO | TBD | TBD | OPEN |
| Inventory numerator | TBD | COGS × DIO | TBD | TBD | OPEN |
| AP numerator | TBD | COGS × DPO | TBD | TBD | OPEN |
| Gross PP&E | TBD | Opening gross PP&E + CapEx | TBD | TBD | OPEN |
| Depreciation capacity | TBD | Opening net PP&E + CapEx | TBD | TBD | OPEN |
| Debt after drawdown | TBD | Opening debt + drawdowns | TBD | TBD | OPEN |
| Average debt | TBD | (Opening debt + closing debt) / 2 | TBD | TBD | OPEN |
| Interest | TBD | Average debt × interest rate | TBD | TBD | OPEN |
| NWC | TBD | AR + inventory − AP | TBD | TBD | OPEN |
| CFO | TBD | Net income + depreciation − ΔNWC | TBD | TBD | OPEN |
| Closing cash | TBD | Opening cash + CFO + CFI + CFF | TBD | TBD | OPEN |
| Retained earnings | TBD | Opening RE + NI − dividends | TBD | TBD | OPEN |
| Balance Sheet totals | TBD | Sum of reportable balance-sheet lines | TBD | TBD | OPEN |

The derivation must account for compounding across all five forecast periods, not just one year's maximum.

### 3.1 Source-scale and cumulative-scale envelope

Magnitude limits and scale limits must be approved separately. The current model input Decimal grammar permits up to 28 fractional digits per field, but that parser ceiling does not guarantee that five years of chained multiplications preserve every accepted scale.

Record source fractional-digit limits by field class, then propagate the maximum scale through each operation in the actual formula order:

| Field / node | Source scale or operation | Required profile value / derivation |
|---|---|---|
| Monetary source fields | Decimal text | Maximum fractional digits by field class; maximum integer digits |
| Rates / ratios | Decimal text | Maximum fractional digits by driver class |
| Days (DSO/DIO/DPO) | Decimal text | Maximum fractional digits and maximum day magnitude |
| Revenue | Prior revenue × (1 + growth) | Prior revenue scale + growth scale per year; propagate through all five years |
| COGS / Opex / CapEx | Revenue × ratio | Revenue scale + relevant ratio scale |
| AR / inventory / AP numerators | Revenue or COGS × days | Monetary scale + day scale before division by 365 |
| Average PP&E / debt | (opening + closing) / 2 | Addition scale and exact/rounded division behaviour |
| Interest / tax / dividends | Balance or earnings × rate | Operand scale + rate scale |
| Closing cash / retained earnings | Multi-line additions and subtractions | Maximum participating scale plus any upstream division policy |

For each node, document whether the candidate preserves scale exactly, normalizes trailing zeros, reduces scale with rounding, or rejects the operation. A test must cover just below, at and just above the declared scale boundary. Do not infer the boundary from Case A's observed trailing-zero pattern.

The profile must also distinguish:
- **Source scale:** what the input contract accepts.
- **Intermediate scale:** what the formula graph can produce before explicit rounding.
- **Arithmetic precision:** how repeating division and other inexact operations are evaluated.
- **Canonical output scale:** how exact model results are serialized.
- **Display scale:** what JSON/CSV presentation or XLSX can show without implying false precision.



## 4. Arithmetic representation and context

| Decision | Required specification | Status |
|---|---|---|
| Backend | Exact crate/version and representation limits | TBD |
| Input type | Canonical decimal-string contract and max scale | TBD |
| Internal precision | Significant digits / fixed scale or exact rational rules | TBD |
| Division precision | Explicit precision and rounding context | TBD |
| Rounding mode | Exact named mode and version | TBD |
| Context ownership | Explicit immutable context; no ambient mutable/global default | TBD |
| Overflow behaviour | Typed failure; no saturation/clamp/default | Required |
| Division by zero | Typed failure or domain-level N/A where specified | Required |
| Serialization | Canonical decimal-string formatting and round-trip rule | TBD |
| Determinism | Same inputs/context/toolchain → same canonical outputs | Required |
| Resource bounds | CPU, memory, maximum input/intermediate size | TBD |

If BigDecimal remains a candidate, the production design must not rely on an implicit environment variable as the sole arithmetic-policy authority. The precision, rounding mode and context identity must be owned by the model run and be visible in its provenance.

## 5. Error-budget separation

Do not collapse the following tolerances into one generic epsilon.

| Budget | Purpose | Unit | Derivation required | Status |
|---|---|---|---|---|
| Arithmetic approximation | Difference from an exact mathematical result due to finite representation/division | Model units and currency | Worst-case formula-chain propagation across five years | OPEN |
| Accounting identity | Acceptable residual in BS and cross-statement identities | Model units and currency | Accounting materiality and backend error contribution | OPEN |
| Schedule roll-forward | Acceptable residual for PP&E, debt, retained earnings and cash movements | Model units and currency | Operation depth and model convention | OPEN |
| Display/export | Difference introduced by output formatting or Excel numeric cells | Display currency/scale | Workbook display policy and 15-digit precision boundary | OPEN |

### Review the currently proposed formula

The Validation Matrix currently proposes:

`Tolerance = 10^{-6} + 10^{-8} \times \max(|A|, |B|, 1)`

This is a proposal, not an approved acceptance threshold. The relative term grows with balance size:

| Max magnitude (model units) | Relative component at (10^{-8}) |
|---:|---:|
| (10^6) | 0.01 |
| (10^9) | 10 |
| (10^{12}) | 10,000 |

Convert each value using the model's unit scale before deciding whether it is immaterial. For example, when amounts are stated in thousands, 10,000 model units represent 10 million reporting-currency units. A mathematically small percentage can still be financially material.

## 6. Candidate evaluation matrix

Use the separate [Numeric Backend Evaluation Protocol](NUMERIC_BACKEND_EVALUATION.md) for hard acceptance gates, candidate-specific checks, controlled measurement requirements and the decision-record procedure. This matrix remains evidence collection, not a weighted score or backend approval.

| Dimension | rust_decimal | BigDecimal | Other candidate | Evidence / decision |
|---|---|---|---|---|
| Supported range | 96-bit mantissa; scale 0–28; magnitude depends on retained scale | Arbitrary-size coefficient and scale; resource growth must be bounded | TBD | OPEN |
| Source-scale propagation | Product scale may grow as operand scales add; five-year recurrence must be bounded | Multiplication preserves all coefficient digits; scale/coefficient growth must be explicitly capped | TBD | OPEN |
| Exact input round-trip | Tested strict parser for current Decimal grammar | Production adapter and canonical format must be tested | TBD | OPEN |
| Division semantics | Finite Decimal result; scale and rounding behaviour must be characterized at every formula node | POC uses an 80-digit compile-time default context; explicit run-scoped division policy not yet demonstrated | TBD | OPEN |
| Five-year exact error | POC evidence on selected fixtures; no approved max-error budget | POC evidence on selected fixtures; no approved max-error budget | TBD | OPEN |
| Boundary/overflow behaviour | Checked failure at the 10^22 monetary-input multiplier stress case; exact profile boundary not known | Completes the same selected stress case; coefficient and resource limits remain open | TBD | OPEN |
| Runtime latency | 42.117/43.032 µs on Rust 1.85; 37.235/38.158 µs on Rust 1.99 | 87.877/64.566 µs on Rust 1.85; 82.277/63.559 µs on Rust 1.99 | 316.808/367.050 µs on Rust 1.85; 324.448/392.920 µs on Rust 1.99 | Observational only; one 4-vCPU CI runner; no memory/resource measurement |
| Peak memory / allocations / tail latency | Not measured | Not measured; multiplication can preserve all digits | Not measured | OPEN |
| Serialization stability | Production input parser exists | Production adapter not implemented; exponent/trailing-zero/canonical policy must be tested | TBD | OPEN |
| Error typing | Checked arithmetic available; production typed error surface required | Must map context, scale, coefficient and resource failures | TBD | OPEN |
| MSRV / target support | Reviewed in POC | Reviewed in POC | TBD | OPEN |
| Dependency source / licence | Review required | Review required | TBD | OPEN |
| Migration cost from current validator | Existing type | Requires boundary migration from current rust_decimal-based validator | TBD | OPEN |

Performance observations are from one CI runner and are not a controlled workload benchmark. They compare only the measured synthetic five-year forecast routine; they do not measure memory, allocations, tail latency or concurrency. BigDecimal's 80-digit precision in this POC is a compile-time default for rounded operations, not a cap on exact multiplication coefficient growth. See `BENCHMARKS_PERFORMANCE.md` on PR #11 for artifact hashes and exact sample protocol.

No candidate wins by having the largest representable number alone. The selected representation must fit the approved operating domain, have deterministic arithmetic semantics, expose failure clearly and satisfy resource constraints.

## 7. Required test plan

For each approved numeric input/intermediate boundary, include:
- Exact lower and upper values.
- One representable value immediately inside each bound.
- One value outside each bound, expected to fail with a stable typed diagnostic.
- Maximum multiplication before division, including AR / inventory / AP numerators.
- Repeating divisions and five-year propagation.
- Source-scale and intermediate-scale boundaries, including scale growth through the revenue recurrence and all rate/day multiplications.
- Candidate coefficient-digit growth and resource limits, especially for arbitrary-precision multiplication.
- Negative, zero and very small positive amounts where domain-valid.
- Large balances combined with maximum rates/days.
- Checked add/subtract/multiply/divide overflow.
- Division-by-zero handling.
- Serialization round-trip without information loss.
- Deterministic repeat-run comparison.
- Structural, schedule and display reconciliation under the approved budgets.

Property-based tests must run inside the approved profile, use reproducible seeds and preserve minimized failures as regression cases.

## 8. Required sign-off

| Reviewer | Responsibility | Name / role | Date | Decision / evidence |
|---|---|---|---|---|
| Model owner | Intended model domain, user requirements and scope | TBD | TBD | OPEN |
| Finance/accounting reviewer | Formula interpretation, materiality and statement treatment | TBD | TBD | OPEN |
| Engineering reviewer | Representation, resource bounds, determinism and failure handling | TBD | TBD | OPEN |
| Dependency/licence reviewer | Source, licences, notices, security disposition | TBD | TBD | OPEN |

## 9. Decision record output

After the worksheet is complete, create a separate dated decision record containing:
- Approved Numeric Profile ID/version and complete bounds.
- Chosen backend/version and arithmetic context.
- Rejected alternatives with evidence.
- Input, intermediate and resource limits.
- Error budgets and reporting rounding.
- Test commit and CI run references.
- Reviewer scope and sign-off.
- Explicit limitations and conditions requiring a profile version change.

Until all required approvals are recorded, the production numeric backend and Production Forecast Engine remain **NOT APPROVED**.
