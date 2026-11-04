<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Rust Runtime Semantic Validator

**Status:** Implemented; CI matrix passed on Rust 1.85.0 and 1.99.1  
**Scope:** V1 input parsing, typed validation, and historical accounting checks  
**Not included:** Forecast calculation engine, Scenario Feasibility evaluation, XLSX output

## 1. Purpose

The runtime validator is the gate between an external model-input document and any later financial calculation.

A JSON document can be syntactically valid and still be unsuitable for a financial model. It may have missing or unknown fields, malformed Decimal strings, invalid period hand-offs, inconsistent historical statements, or schedule movements that cannot be reconciled.

The validator rejects these cases before a future calculation engine is allowed to run. It does not attempt to repair the input.

## 2. Position in the architecture

The V1 processing sequence is:

1. Read the input document.
2. Parse JSON syntax.
3. Deserialize into closed, typed Rust structures.
4. Enforce the Decimal string boundary.
5. Validate model metadata and period semantics.
6. Validate opening and historical accounting relationships.
7. Validate forecast assumption domains and debt repayment bounds.
8. Return a typed validation report.

The forecast calculation engine is a separate later stage. This crate does not generate forecast statements or balances.

## 3. Strict input boundary

The Rust model types use closed Serde objects with unknown fields rejected. All Decimal values use a custom deserializer that:

- Requires a JSON string token; JSON numeric tokens are rejected.
- Accepts canonical base-10 notation only.
- Rejects leading zeros, plus signs, exponent notation, separators, whitespace, malformed decimal points, and negative zero.
- Rejects more than 28 fractional digits before Decimal parsing.
- Parses with the pinned rust_decimal candidate.
- Requires the parsed Decimal's string representation to match the original text exactly.

This is a fixed-precision Decimal boundary, not arbitrary-precision or rational arithmetic. The current validator uses checked addition and subtraction for accounting checks. It does not yet establish the model-level numeric envelope or accumulated forecast error budget.

## 4. Metadata and period rules

The runtime checks:

- Schema version 0.1.0.
- Model ID syntax and length, model name length, and three-letter uppercase currency syntax.
- The supported unit-scale enum and V1 day-count basis of 365.
- Exactly three historical periods and five forecast periods.
- Unique period_id values in the form FYyyyy.
- Valid Gregorian start, end, and opening Balance Sheet dates.
- Annual period lengths of 364, 365, 366, or 371 inclusive calendar days.
- Chronological ordering and exact date contiguity.
- Opening Balance Sheet date to first historical period hand-off.
- Last historical period to first forecast period hand-off.
- Period ID year equal to the calendar year in which the period ends.

The accepted period lengths cover common calendar-year/leap-year and 52/53-week annual periods. The validator does not infer a company's fiscal calendar. Period start/end dates are the source of truth.

## 5. Accounting validation

### Opening Balance Sheet

The opening position is checked for non-negative asset, liability, and share-capital balances, accumulated depreciation not exceeding gross PP&E, and the Balance Sheet identity:

\[
\text{Cash}+\text{AR}+\text{Inventory}+\text{Net PP\&E}
=
\text{AP}+\text{Debt}+\text{Share Capital}+\text{Retained Earnings}
\]

Retained earnings may be negative. Net PP&E is calculated as gross PP&E less accumulated depreciation.

### Historical Income Statement

For each historical period:

\[
\text{Net Income}
=
\text{Revenue}
-\text{COGS}
-\text{Operating Expenses}
-\text{Depreciation}
-\text{Interest Expense}
-\text{Income Tax Expense}
\]

Expense inputs must be non-negative. Negative net income remains a valid historical outcome.

### Historical Cash Flow Statement

The validator checks:

- Cash Flow Statement net income against Income Statement net income.
- Depreciation addback against Income Statement depreciation.
- Change in operating NWC against consecutive Balance Sheets.
- CFO = net income + depreciation addback − change in operating NWC.
- CFI = −CapEx.
- CFF = debt drawdowns − principal repayments − dividends.
- Net change in cash = CFO + CFI + CFF.
- Closing cash = opening cash + net change in cash.
- Opening and closing cash ties to the prior and current Balance Sheets.

### Historical roll-forwards

Under the explicit V1 simplifications, the validator checks:

- Gross PP&E closing = opening gross PP&E + CapEx.
- Accumulated depreciation closing = opening accumulated depreciation + depreciation.
- Debt closing = opening debt + drawdowns − repayments.
- Retained earnings closing = opening retained earnings + net income − dividends.
- Share capital remains unchanged.
- Principal repayments do not exceed opening debt plus current-period drawdowns.

These are V1 normalized-model rules. They are not universal assertions about statutory financial statements.

## 6. Forecast assumption validation

The current runtime checks forecast inputs only; it does not calculate forecasts.

- Revenue growth must be at least -1.0.
- COGS and operating expense ratios are non-negative, with no 100% cap.
- DSO, DIO, and DPO are non-negative.
- CapEx ratio is non-negative.
- Depreciation rate, tax rate, and dividend payout ratio are between 0 and 1 inclusive.
- Debt drawdowns, principal repayments, and interest rate are non-negative.
- Principal repayments cannot exceed opening debt plus current-period drawdowns, rolled forward across forecast periods.

Negative cash is not evaluated here because there are no calculated forecast statements yet. A future engine must report it as Scenario Feasibility FAIL rather than an input error or an instruction to invent financing.

## 7. Diagnostics

The validate_input_json API returns a ValidationReport with:

- input_status: accepted or rejected.
- diagnostics: stable code, category, severity, human-readable message, optional JSON Pointer, period ID, field, expected value, and actual value.

Diagnostics are emitted in a deterministic validation order. Parsing and deserialization failures stop early because later semantic checks would not be reliable. Semantic validation accumulates independent findings where safe.

The external diagnostic codes are defined in [VALIDATION_ERROR_CONTRACT.md](VALIDATION_ERROR_CONTRACT.md). Human-readable message text may improve without changing the stable code.

## 8. JSON Schema relationship

The Draft 2020-12 JSON Schema remains a separate, independently tested contract artifact. Ajv CI compiles the schema and exercises valid and invalid payloads.

The Rust runtime uses typed Serde structures plus explicit semantic rules. It does not embed a general JSON Schema interpreter. Both gates serve different purposes:

- JSON Schema CI checks the published machine-readable interchange contract.
- Rust runtime validation protects the actual calculation boundary and checks accounting semantics.

Passing either gate alone is not sufficient to establish financial-model correctness.

## 9. CLI

From the OFRL-IB-001 project directory:

~~~bash
cargo run --locked -p ofrl_cli -- validate ../../fixtures/synthetic/ofrl-ib-001-case-a.input.v0.1.0.json
~~~

The command prints a JSON validation report.

Exit codes:

| Code | Meaning |
|---:|---|
| 0 | Input accepted by the runtime validator |
| 1 | File access or command invocation failure |
| 2 | Input rejected; inspect the JSON diagnostics |

## 10. Tests

The test suite covers the canonical synthetic Case A fixture and negative cases for:

- Invalid JSON and unsupported schema versions.
- JSON numeric tokens where Decimal strings are required.
- Non-canonical Decimal strings, negative zero, and excessive scale.
- Unknown fields and missing provenance.
- Invalid Gregorian dates and fiscal-period gaps.
- Period identifier/year mismatch and malformed identifiers.
- Opening Balance Sheet imbalance.
- Historical net-income and NWC reconciliation failures.
- PP&E and debt roll-forward failures.
- Debt repayments above available debt.
- Forecast assumption domain violations.
- A valid -100% revenue-growth assumption, which must not be rejected merely because it is economically unfavourable.

The CI matrix runs formatting, Clippy with warnings denied, all workspace tests, documentation builds with warnings denied, a valid Case A CLI smoke test, and a rejected-input CLI exit-code test on Rust 1.85.0 and 1.99.1. Cargo commands that resolve dependencies use the committed Cargo.lock with --locked.

## 11. Explicit limitations

The current validator does not:

- Calculate revenue, margins, interest, tax, dividends, cash, or forecast financial statements.
- Evaluate forecast Balance Sheet integrity or negative forecast cash.
- Apply a numeric reconciliation tolerance; historical arithmetic checks use exact Decimal equality.
- Validate a jurisdiction's statutory reporting framework.
- Normalize real issuer statements or verify source filings.
- Validate ISO 4217 membership; it validates the three-uppercase-letter currency syntax only.
- Execute the Draft 2020-12 schema in Rust.
- Establish the full Decimal range, division precision, or five-year accumulated-error envelope.

These limitations are intentional. They keep the input-acceptance gate separate from calculation correctness and prevent a successful parse from being misrepresented as a working financial model.