<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Numeric Envelope and Precision Policy

**Policy version:** 0.1.0  
**Status:** Proposed for V1; candidate-specific evidence recorded  
**Candidate assessed:** rust_decimal 1.43.0  
**Effective POC toolchain floor:** Rust 1.85.0

## 1. Purpose

Define what the model's numeric representation can and cannot guarantee before implementing financial calculations.

The numeric policy must distinguish:

- Input text precision.
- Internal Decimal representation.
- Intermediate arithmetic precision.
- Financial-model materiality and acceptable error.
- Display precision in JSON/CSV and Excel outputs.

These are not interchangeable concepts. A value can be represented exactly at input and still produce a finite-precision approximation after division.

## 2. Candidate representation

The tested rust_decimal candidate represents values in the form:

m / 10^s

where m is a signed integer mantissa constrained by a 96-bit representation and s is an integer scale from 0 through 28.

The maximum positive mantissa is:

79228162514264337593543950335

Therefore the maximum absolute value at a particular scale s is bounded by:

(2^96 − 1) / 10^s

The maximum numeric magnitude depends on the scale. Scale 28 does not permit every large monetary value; higher fractional scale reduces the maximum integral magnitude.

The candidate has finite fixed precision. It is not arbitrary precision, a rational-number system, or an exact representation of all decimal fractions.

Reference: https://docs.rs/rust_decimal/1.43.0/rust_decimal/struct.Decimal.html

## 3. Input representation

All monetary amounts, percentages, ratios, and day metrics use canonical base-10 strings in JSON/CSV.

The accepted lexical form:

- Optional leading minus sign for signed fields.
- Integer part is 0 or a non-zero digit followed by digits.
- Optional fractional part with 1–28 digits.
- No leading plus sign, exponent notation, thousands separators, whitespace, malformed decimal points, or leading zeros.
- Negative zero representations such as -0 and -0.000 are rejected.
- The string must parse to Decimal and round-trip back to the exact original text.
- The value must fit the candidate Decimal range.

The lexical grammar is not a substitute for numerical domain validation. For example, a decimal string can be syntactically valid but invalid for a non-negative Balance Sheet field.

## 4. Model units

Every model instance uses one reporting currency and one monetary unit scale.

| unit_scale | Meaning of amount string |
|---|---|
| units | Amount is expressed in single currency units |
| thousands | Amount is expressed in thousands of the reporting currency |
| millions | Amount is expressed in millions of the reporting currency |

Example: reporting_currency USD and unit_scale thousands means an input amount of 125.50 represents USD 125,500.

No conversion between units or currencies occurs implicitly inside the model. All monetary values in the same model use the same model-level metadata. Unit conversion, if introduced, must be an explicit operation with a separately tested conversion policy.

## 5. Ratios, percentages, and days

Ratios and percentages use fractional representation:

- 0.20 means 20%.
- 0.075 means 7.5%.
- 1.00 means 100%.

They are not entered as percentage points. No automatic division by 100 is performed by the parser.

Days such as DSO, DIO, and DPO are Decimal values. The V1 working-capital calculations use a fixed day_count_basis of 365:

- Accounts receivable = Revenue × DSO / 365.
- Inventory = COGS × DIO / 365.
- Accounts payable = COGS × DPO / 365.

The use of 365 is a model convention. It does not assert that every fiscal year contains exactly 365 calendar days.

## 6. Arithmetic policy

The calculation core must:

1. Use checked Decimal addition, subtraction, multiplication, and division for operations that can exceed the numeric domain.
2. Convert arithmetic failures into typed calculation errors.
3. Never use saturation, silent default-to-zero, or hidden clamping to conceal overflow.
4. Reject division by zero and invalid model domains explicitly.
5. Avoid binary floating-point in the canonical financial calculation path.
6. Avoid intentional application-level rescaling/rounding inside schedules.
7. Record that division may still produce a finite-scale approximation within the Decimal representation, even when no explicit rounding function is called.
8. Keep the chosen rounding mode and presentation scale at an explicit reporting boundary.

The phrase “no intermediate rounding” means the application does not deliberately quantize values at every schedule step. It does not mean every Decimal division is mathematically exact.

## 7. Observed division behaviour

The POC passed on Rust 1.85.0 and 1.99.1 and recorded the following results at scale 28:

| Expression | Result |
|---|---|
| 1 / 365 | 0.0027397260273972602739726027 |
| 1 / 3 | 0.3333333333333333333333333333 |
| (1 / 3) × 3 | 0.9999999999999999999999999999 |

The result demonstrates finite Decimal scale and non-exact reconstruction of a repeating rational value. It is not, by itself, a measure of material financial-model error. The model must quantify how such differences propagate through the specific schedules and statements.

## 8. Input parser safety

The POC observed that the raw parser could accept a 29-fractional-digit input and reduce it to the maximum supported scale. The POC's Serde input adapter therefore:

- Rejects fractional length above Decimal::MAX_SCALE before parsing.
- Parses only after the lexical and scale checks.
- Requires Decimal-to-string output to match the original input exactly.
- Rejects values whose sign, scale, or digits change during parsing.
- Rejects values outside the Decimal range.

This exact round-trip check is mandatory for the candidate's input boundary unless a later parser is demonstrated to provide equivalent or stronger guarantees.

## 9. Numeric envelope still to be finalized for the model

The Decimal type defines a representation envelope, but the financial model still needs a domain-specific envelope. Before the calculation engine is accepted, define and test:

- Maximum expected revenue, expense, debt, cash, and balance-sheet amounts in the selected model unit.
- Minimum material monetary amount for the chosen unit scale.
- Maximum number of significant digits and fractional digits expected from source data.
- Maximum supported rate and day values.
- Maximum intermediate value in every schedule and formula chain.
- Headroom required for additions, multiplications, and division scaling.
- Error tolerance for statement identities and schedule roll-forwards.
- Maximum acceptable accumulated numerical difference across the five forecast periods.
- Final reporting precision and explicit rounding mode.
- XLSX conversion/display tolerance relative to canonical Decimal outputs.

Do not invent a universal maximum company size before the model's unit conventions and intended data envelope are agreed. The implementation must reject inputs outside its declared envelope and report the violated bound.

## 10. Excel and binary floating-point boundary

Excel numeric cells use binary floating-point and have approximately 15 significant decimal digits of precision. The workbook is an analyst-facing report, not the authoritative calculation environment.

The export layer must:

- Preserve canonical decimal strings in JSON/CSV outputs.
- Declare workbook display precision and rounding.
- Reconcile displayed values against canonical engine results.
- Avoid presenting values with more precision than the workbook can faithfully retain.
- Retain the canonical source value in a clearly accessible audit/export field when useful.
- Use text cells or a clearly labelled display representation when a numeric cell would imply false precision.

No financial calculation may be delegated to spreadsheet formulas as a parallel source of truth in V1.

## 11. Acceptance gates for numeric policy

The numeric foundation cannot be declared approved for the full model until:

1. The model-specific input and intermediate numeric envelope is documented.
2. The synthetic reference case runs without arithmetic overflow or silent value changes.
3. Property and boundary tests cover the approved numeric range.
4. Division and rounding behaviour is explicitly characterized for the schedule formulas.
5. Model-level error accumulation remains within the approved tolerance.
6. The canonical output and XLSX reporting layer reconcile within documented display tolerance.
7. Direct/transitive dependency, lockfile, licence, and security reviews are completed.

The Decimal POC passed its own test scope. These model-level acceptance gates remain open.

## 12. Comparative full-formula POC evidence (2026-11-04)

The isolated comparison in [the model numeric-envelope POC](poc/model-numeric-envelope/README.md) ran the five-year Case A formula sequence using one shared generic forecast implementation with three numeric backends. Both CI jobs passed on Rust 1.85.0 and 1.99.1 in the initial measurement run [Numeric Envelope POC run 41769881131](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41769881131). The strict-formatting, committed-lockfile, and expanded 10-test rerun passed in [run 41775548498](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41775548498). The 11-test run that includes the independent FY2026 benchmark passed both toolchains in [run 41776373568](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41776373568).

**Independent formula benchmark**

- [Independent FY2026 calculation benchmark](poc/model-numeric-envelope/BENCHMARKS.md) records the opening balances, model assumptions, separate calculation sequence, and exact expected fractions for all 32 first-year forecast metrics.
- Rust test `case_a_fy2026_matches_independently_derived_exact_benchmark` asserts the exact fractions without using the generic forecast routine to generate its expected values. It passed in both toolchains as part of the 11-test run above.
- This is an independent first-period benchmark, not a full independent model review; additional edge-case and multi-period benchmarks remain required.

**Canonical Case A observations**
- 160 forecast metric values were compared per candidate across five years.
- Both `rust_decimal 1.43.0` and `bigdecimal 0.4.11` had zero mismatches after the POC's provisional two-decimal display quantization.
- The maximum exact rational difference reported for rust_decimal was `39/365000000000000000000000000`, in FY2030 closing cash.
- The maximum exact rational difference reported for BigDecimal was `1/114062500000000000000000000000000000000000000000000000000000000000000000000000`, in FY2029 cash from operations.
- These are arithmetic differences in the model's configured unit (illustratively USD thousands), not final materiality conclusions. The two-decimal display comparison is a POC diagnostic, not the approved reporting or reconciliation tolerance.
- Maximum unrounded Case A Balance Sheet residual for `rust_decimal`: `1/10000000000000000000000000` in FY2029 (10^-25 model units).
- Maximum unrounded Case A Balance Sheet residual for `bigdecimal`: `0/1` in the five-year Case A run.
- Both residual results were identical on Rust 1.85.0 and 1.99.1. They are observed test outputs, not the final model reconciliation tolerance.



**Intermediate stress point (monetary inputs scaled by (10^{18}))**
- A second full-formula stress case uses the same extreme operating assumptions as the (10^{22}) capacity probe but scales the historical monetary inputs by (10^{18}).
- Both `rust_decimal 1.43.0` and `bigdecimal 0.4.11` completed all five forecast years without arithmetic failure.
- Each candidate matched the Exact Rational reference exactly across all 160 compared output metrics: maximum absolute error `0/1`, zero provisional two-decimal display mismatches.
- Maximum unrounded Balance Sheet residual was `0/1` for both candidates.
- The largest Exact Rational forecast metric was `2,160,000,000,000,000,000,000,000.00` in FY2030 accounts payable.
- The result passed on Rust 1.85.0 and 1.99.1, including strict formatting, Clippy, 12 tests, and documentation build in [Numeric Envelope POC run 41777136666](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41777136666).

The (10^{18}) and (10^{22}) cases are two deliberately selected numerical stress points, not a precise overflow boundary. They do not define supported production company size, maximum monetary input, or the final V1 model-specific operating envelope.

**Provisional stress observations**
- The deliberately extreme stress case scaled starting monetary amounts by (10^{22}) and raised selected assumptions. Its exact-rational reference completed and remained arithmetically balanced.
- The largest exact-rational forecast metric was `21,600,000,000,000,000,000,000,000,000.00` in FY2030 accounts payable.
- rust_decimal rejected the stressed path at FY2026 accounts-receivable numerator multiplication due to checked Decimal overflow.
- BigDecimal completed the stress path with zero difference from the exact-rational reference across the 160 compared output values under the POC context.
- This stress case is a representation-capacity probe, not a realistic company-size requirement or a production V1 input limit.
- A separate high-CapEx diagnostic produced FY2026 closing cash of -5,246.116462328767 model units (exact value -1531866007/292000) while the algebraic Balance Sheet residual remained zero. Closing debt remained 140 (opening debt 180 less explicit repayment 40); no automatic financing was inserted. This is a funding-shortfall diagnostic, not a valid negative cash asset or a reportable feasible Balance Sheet.

**Interpretation and remaining gates**

The evidence demonstrates that the fixed-width candidate has a finite capacity boundary and that the arbitrary-precision candidate can represent this particular extreme path under an 80-significant-digit HalfEven division context. It does not establish that BigDecimal is the better production choice: performance, operational bounds, memory use, deterministic context control, dependency health, serialization, and the intended real-world model envelope still require review.

The exact-rational backend shares the same generic forecast formula implementation as the candidates. It is an arithmetic oracle, not an independent verification of the financial-model formulas. One independent first-period benchmark now exists; broader edge-case and multi-period benchmarks plus independent accounting review remain open. A model-specific error budget, the full dependency/SBOM/security review, and production numeric-library decision also remain open.
