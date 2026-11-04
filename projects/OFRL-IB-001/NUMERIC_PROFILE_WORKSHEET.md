<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Numeric Profile Decision Worksheet

**Worksheet version:** 0.1.0  
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

| Dimension | rust_decimal | BigDecimal | Other candidate | Evidence / decision |
|---|---|---|---|---|
| Supported range | Finite mantissa/scale | Arbitrary-size integer coefficient with configured division precision | TBD | OPEN |
| Exact input round-trip | Tested strict parser | Must be tested | TBD | OPEN |
| Division semantics | Finite Decimal scale | Context-dependent significant precision | TBD | OPEN |
| Five-year exact error | POC evidence | POC evidence | TBD | OPEN |
| Boundary/overflow behaviour | POC evidence | Must bound resource use | TBD | OPEN |
| Runtime latency | 42.117/43.032 µs on Rust 1.85; 37.235/38.158 µs on Rust 1.99 | 87.877/64.566 µs on Rust 1.85; 82.277/63.559 µs on Rust 1.99 | 316.808/367.050 µs on Rust 1.85; 324.448/392.920 µs on Rust 1.99 | Observational only; one 4-vCPU CI runner; memory/resource review OPEN |
| Peak memory / allocations | Not measured | Not measured | TBD | OPEN |
| Serialization stability | Production input parser exists | Production adapter not implemented | TBD | OPEN |
| Error typing | Checked arithmetic available | Must map context/scale failures | TBD | OPEN |
| MSRV / target support | Reviewed in POC | Reviewed in POC | TBD | OPEN |
| Dependency source / licence | Review required | Review required | TBD | OPEN |
| Migration cost from current validator | Existing type | Requires boundary migration | TBD | OPEN |

Performance observations are from one CI runner and are not a controlled workload benchmark. They compare only the measured synthetic five-year forecast routine; they do not measure memory, allocations or concurrency. See `BENCHMARKS_PERFORMANCE.md` on PR #11 for artifact hashes and exact sample protocol.

No candidate wins by having the largest representable number alone. The selected representation must fit the approved operating domain, have deterministic arithmetic semantics, expose failure clearly and satisfy resource constraints.

## 7. Required test plan

For each approved numeric input/intermediate boundary, include:
- Exact lower and upper values.
- One representable value immediately inside each bound.
- One value outside each bound, expected to fail with a stable typed diagnostic.
- Maximum multiplication before division.
- Repeating divisions and five-year propagation.
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
