<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Numeric Envelope Proof of Concept

**Status:** Research POC — no production numeric foundation selected  
**POC version:** 0.1.0  
**Branch:** \`research/ofrl-ib-001-numeric-envelope\`  
**Purpose:** Compare candidate arithmetic behaviour over the actual V1 three-statement formula path before approving a production calculation engine.

## 1. Why this POC exists

A Decimal type passing serialization and unit tests is not enough to approve it as the calculation foundation for an integrated financial model.

The V1 formula chain contains multiplication, division by 365, averages, repeated schedule roll-forwards, and five forecast periods. Fixed-width Decimal arithmetic can be exact for some operations and finite-precision for others. An apparently small precision loss can propagate into working capital, cash, PP&E, retained earnings, and Balance Sheet reconciliation.

This POC therefore evaluates candidate numeric representations against the same versioned Case A input and the same formula sequence. The reference calculation uses exact rational arithmetic so candidate results can be compared without treating another finite-precision library as ground truth.

## 2. Candidates under evaluation

| Candidate | Role | Known representation characteristics | Current decision |
|---|---|---|---|
| \`rust_decimal 1.43.0\` | Current fixed-point candidate | 96-bit mantissa; scale 0–28; checked operations; finite precision and bounded intermediate products | Not approved for production yet |
| \`bigdecimal 0.4.11\` | Arbitrary-precision decimal candidate | BigInt-based; multiplication preserves digits; division uses a configurable precision context | Research comparator only; not approved |
| \`num_rational 0.4.2\` with \`num_bigint 0.4.8\` | Exact reference arithmetic | Rational numerator/denominator; exact addition, subtraction, multiplication, and division | Test oracle only; not a production recommendation |

The BigDecimal POC must use a pinned precision and rounding context. It must not depend on developer-machine defaults. The initial comparison context is 80 significant digits with HalfEven rounding for operations that produce non-terminating results. This is a test setting, not an adopted production policy.

## 3. Canonical test input and reference

The POC consumes the repository's canonical synthetic fixture:

\`fixtures/synthetic/ofrl-ib-001-case-a.input.v0.1.0.json\`

The fixture is not copied into a second source-of-truth. The POC reads the same file by a relative include path and parses the existing decimal-string inputs.

The reference engine must implement the V1 equations from:

- \`RESEARCH_SPECIFICATION.md\`
- \`MODEL_CONVENTIONS.md\`
- \`MODEL_INPUT_CONTRACT.md\`

It must calculate the complete five-period forecast path, not an isolated \`1 / 365\` demonstration.

## 4. Formula path under test

For each forecast year \(t\), the POC will calculate:

- Revenue and operating costs.
- EBITDA, depreciation, EBIT, interest, EBT, tax, and net income.
- Accounts receivable, inventory, accounts payable, and change in operating NWC.
- CapEx, gross PP&E, accumulated depreciation, and net PP&E.
- Opening, average, and closing debt; interest expense.
- Dividends and retained earnings.
- CFO, CFI, CFF, net change in cash, and closing cash.
- Total assets, total liabilities and equity, and Balance Sheet difference.

The candidate engines must follow the same dependency order and the same assumptions. Any difference in formula order must be recorded rather than hidden.

## 5. Reference comparison policy

The exact rational calculation is the arithmetic reference. Candidate outputs will be compared at two levels:

1. **Exact rational difference:** quantify the unrounded difference between each candidate result and the rational reference.
2. **Reporting-boundary equivalence:** quantize candidate and reference outputs using the same explicitly selected test rounding rule and compare their displayed monetary values.

The initial display comparison uses two decimal places and HalfEven as a neutral test convention. It does not establish the final V1 financial reporting rule. That remains a founder decision after the model's intended currencies and output requirements are documented.

For each year and each material line, record:

- Candidate raw output.
- Exact rational reference.
- Absolute error before display rounding.
- Difference after reporting-boundary quantization.
- Whether the three statements reconcile.
- Whether any arithmetic operation failed or reduced scale.

The rational oracle must be independently tested for sign handling, denominator normalization, exact division, and half-even quantization.

## 6. Provisional stress case — not a production input limit

The POC will derive a deliberately broad stress case from Case A by scaling historical monetary inputs and increasing selected forecast drivers.

The initial stress settings are intended to exercise representation headroom:

- Starting financial statement monetary amounts scaled by \(10^{22}\).
- Revenue growth stress of 100% per forecast year.
- COGS and operating expense ratios up to 500%.
- CapEx ratio up to 500%.
- DSO, DIO, and DPO up to 3,650 days.
- Depreciation, tax, interest, and dividend ratios up to 100%.

These are test stress values, not a claim that such operating assumptions are economically likely or that these figures will become the production V1 limits. The resulting maximum intermediate values must be measured from the actual formula path and compared with each candidate's supported range.

The model's final technical input bounds remain unapproved until this evidence and the dependency review are complete.

## 7. Required tests

### Candidate correctness

- Canonical Case A produces five forecast periods in all three engines.
- All candidate calculations use the same assumptions and formula order.
- Exact rational Balance Sheet differences are zero in every forecast year.
- Candidate Balance Sheet differences remain within the existing monetary tolerance or are explicitly reported as failures.
- Candidate monetary outputs match the exact rational reference after common reporting-boundary quantization.
- Negative earnings remain valid results; they must not be rejected by arithmetic validation.
- A negative closing cash scenario is reported as a funding-feasibility result, not repaired with automatic debt.

### Numeric behaviour

- Checked addition, subtraction, multiplication, and division boundaries.
- Scale preservation on parse/serialize.
- 1/365 and repeating division behaviour.
- Large amounts combined with high-scale day metrics.
- Small fractional amounts combined with large balances.
- Five-year propagation of rounding differences.
- Exact-rational versus candidate output deltas.
- Overflow, underflow, division-by-zero, and unsupported-scale behaviour.

### Determinism

- Same fixture, candidate version, precision context, and toolchain must produce the same canonical output.
- Test output must include candidate versions and the BigDecimal precision/rounding context.
- No candidate may rely on unpinned dependency resolution.

## 8. What the POC does not decide

The POC does not approve:

- The production numeric library.
- The final currency-specific minor-unit policy.
- The final internal scale or rounding mode.
- Maximum company size or forecast growth assumptions.
- The maximum permitted balance sheet or intermediate value.
- The final financial model materiality tolerance.
- XLSX export precision or formulas.

Those decisions require the measured results, dependency/SBOM review, and a reviewed V1 operating envelope.

## 9. Acceptance gate

The POC can be marked **Reproducible** only when:

- All three engines implement the same V1 formula path.
- Exact rational invariants pass.
- Candidate arithmetic errors are measured and reported.
- The provisional stress case completes or produces explicit typed failure evidence.
- Regression tests cover all discovered edge conditions.
- Dependency versions and BigDecimal context are pinned.
- Both Rust 1.85.0 and 1.99.1 CI jobs pass.
- Results and remaining risks are documented in \`NUMERIC_ENVELOPE.md\`.

Passing this POC does not, on its own, authorize production-engine adoption. The final choice must be recorded in the Decision Log after the model-specific envelope and dependency audit are complete.

## 10. Interpretation boundary — arithmetic evidence is not model validation

The three candidate types use one shared generic forecast implementation with different numeric representations. This deliberately isolates arithmetic behaviour while holding the formula sequence constant.

The exact-rational implementation is therefore an **arithmetic oracle for this comparison**, not an independent implementation of the financial model's business logic. A shared formula defect would be shared by all three numeric candidates. The POC's balanced Balance Sheet result proves that the defined equations reconcile under exact arithmetic; it does not prove that every accounting policy or formula choice is independently correct.

Before the production forecast engine is accepted, OFRL still requires:
- Independent finance-model review of the equations and schedule dependencies.
- Hand-calculated benchmark cases for selected periods and edge conditions.
- Independent checks of the accounting interpretation and cash-flow classifications.
- Golden outputs reviewed against a separately derived reference calculation.
- Explicit negative-cash / funding-feasibility tests, separate from Balance Sheet integrity.

The POC includes a deterministic high-CapEx case to demonstrate that a negative cash outcome can coexist with a balanced set of equations. It must be reported as **Scenario Feasibility FAIL**, not repaired through hidden funding.

## 11. Independent FY2026 benchmark

The first forecast year also has a separately derived, exact-rational benchmark with immutable expected fractions for all 32 output metrics. See [BENCHMARKS.md](BENCHMARKS.md) and the test `case_a_fy2026_matches_independently_derived_exact_benchmark`. This creates an independent first-period check in addition to the three-backend numeric comparison. It does not replace broader formula/accounting review.

## 12. Verified POC results (2026-11-04)

The committed POC lockfile is enforced in CI with `--locked`. The numeric-envelope workflow passed on Rust 1.85.0 and 1.99.1, with formatting checks, Clippy, all 10 tests, and documentation generation:

- [Numeric Envelope POC — CI run 41775548498](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41775548498)

For the five-year synthetic Case A forecast:
- 160 metrics per candidate were compared.
- Both candidates had zero mismatches after the POC's provisional two-decimal reporting quantization.
- Maximum unrounded Balance Sheet residual: rust_decimal `1/10000000000000000000000000` in FY2029; BigDecimal `0/1`.
- BigDecimal's test build was verified to use an 80-significant-digit HalfEven default context.
- The high-CapEx scenario produced negative cash while the Exact Rational Balance Sheet remained balanced; explicit debt mechanics were unchanged.

For the provisional stress case:
- rust_decimal returned a checked multiplication overflow on the FY2026 accounts-receivable numerator.
- BigDecimal completed the tested five-year path and matched the Exact Rational reference across the compared output metrics.

These results support further numeric-envelope analysis. They do **not** select the production numeric library, establish a production input range, prove the forecast formulas independently, or set the final reconciliation/materiality tolerance.
