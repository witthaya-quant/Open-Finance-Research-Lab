<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# [PROJECT-ID] — Numeric Envelope and Error Budget

**Policy version:** 0.1.0  
**Owner:**  
**Last updated:** YYYY-MM-DD  
**Numeric backend / library:**  
**Status:** Proposed | Tested | Accepted

## 1. Purpose

Define the numeric representation and error controls required for the model's calculations. A finite-precision Decimal type is not arbitrary precision; its representation, operation semantics, and rounding behavior must be understood.

## 2. Numeric requirements

| Requirement | Decision / bound | Evidence |
|---|---|---|
| Numeric domain | Decimal / Integer / Floating point / Mixed | |
| Maximum absolute input | | |
| Maximum intermediate magnitude | | |
| Minimum meaningful increment | | |
| Maximum scale / fractional digits | | |
| Required output precision | | |
| Rounding mode | | |
| Overflow behavior | Explicit error / other | |
| Division-by-zero behavior | Explicit error / other | |
| Non-finite input behavior | Reject / not applicable | |

## 3. Range and scale analysis

For every material input, intermediate, and output, identify its domain, maximum magnitude, scale, and operation path. Include derived upper bounds where practical.

## 4. Error budget

Define the maximum acceptable absolute and relative error for material outputs. Explain how rounding, division, aggregation order, conversions, and repeated operations contribute to the budget.

| Calculation / output | Error source | Absolute tolerance | Relative tolerance | Acceptance test |
|---|---|---|---|---|
| | | | | |

## 5. Arithmetic semantics

Document addition, subtraction, multiplication, division, quantisation, comparison, conversion, and overflow behavior. Identify operations that can return errors and ensure callers cannot silently discard them.

## 6. Accumulation and rounding policy

State where rounding is allowed, whether intermediate values remain unrounded, when currency/reporting quantisation occurs, and how independent outputs reconcile.

## 7. Adversarial and boundary tests

Include minimum/maximum values, extreme scale, near-zero denominators, negative values, cancellation, overflow, repeated accumulation, and known difficult cases appropriate to the model.

## 8. Benchmark and independent verification

Identify authoritative or independently computed reference cases. State tolerances before comparing results and explain why the reference is suitable.

## 9. Acceptance decision

Record unresolved assumptions, known limitations, test results, decision owner, and the evidence required before the numeric policy is accepted for production calculation.
