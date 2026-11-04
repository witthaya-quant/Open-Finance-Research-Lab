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