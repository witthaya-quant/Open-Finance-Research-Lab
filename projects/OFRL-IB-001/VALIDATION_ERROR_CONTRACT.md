<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Validation and Error Contract

**Contract version:** 0.2.0  
**Status:** Rust input-validation diagnostics implemented; forecast-calculation diagnostics remain part of the target contract

## 1. Purpose

Define stable failure semantics for input parsing, schema validation, semantic validation, arithmetic, and model findings.

The caller must be able to distinguish invalid data from a valid model result that is economically unfavourable. Error codes are stable identifiers; message text is explanatory and may evolve.

## 2. Result categories

| Category | Meaning | Blocks calculation? |
|---|---|---:|
| Input Syntax | JSON or decimal text cannot be parsed safely | Yes |
| Schema | Payload violates the declared schema or schema version | Yes |
| Semantic Input | Fields are individually valid but inconsistent with model rules | Yes |
| Calculation | Arithmetic or internal calculation cannot be completed safely | Yes |
| Structural Integrity Finding | Calculated or historical statements fail accounting/reconciliation invariants | Model calculation may complete; Structural Integrity FAIL |
| Scenario Feasibility Finding | A valid scenario produces an unfunded outcome such as negative cash | No; Scenario Feasibility FAIL |
| Warning | Output is valid but a ratio or analysis is undefined or limited | No |

## 3. Standard diagnostic shape

A diagnostic should expose:

| Field | Required | Meaning |
|---|---:|---|
| code | Yes | Stable machine-readable code |
| category | Yes | One of the categories above |
| severity | Yes | error, warning, or info |
| message | Yes | Human-readable explanation |
| json_pointer | No | JSON Pointer to the input field, where relevant |
| period_id | No | Related historical or forecast period |
| field | No | Stable field identifier |
| expected | No | Expected condition or value |
| actual | No | Observed condition or value |

Diagnostics must not include secrets, access tokens, personal financial data beyond the input itself, or environment-specific file paths.

## 4. Input syntax and Decimal boundary codes

| Code | Meaning |
|---|---|
| JSON_PARSE_FAILED | Input is not valid JSON; syntax errors without an unambiguous field path have no JSON Pointer |
| INPUT_DESERIALIZATION_FAILED | Defensive fail-closed code if typed Serde decoding disagrees with the preceding contract preflight |
| JSON_TYPE_MISMATCH | A field has the wrong JSON type |
| DECIMAL_STRING_REQUIRED | Decimal field was supplied as a JSON number or non-string token |
| DECIMAL_TEXT_NON_CANONICAL | Decimal text violates the canonical lexical grammar |
| DECIMAL_SCALE_EXCEEDED | Fractional scale exceeds the supported maximum |
| DECIMAL_OUT_OF_RANGE | Decimal text cannot be represented by the selected numeric type |
| SCHEMA_VERSION_UNSUPPORTED | Input schema version is not supported |

All Decimal fields must pass the same canonical text boundary before domain-specific validation.

## 5. Schema and semantic input codes

| Code | Meaning |
|---|---|
| REQUIRED_FIELD_MISSING | A required input field is absent |
| UNKNOWN_FIELD | An undeclared field is present |
| INVALID_ENUM_VALUE | A value is outside the declared enumeration |
| INVALID_DATE | A date is malformed or invalid |
| INVALID_CURRENCY_FORMAT | Reporting currency fails the declared syntax |\n| INVALID_IDENTIFIER_FORMAT | A model or period identifier fails its declared lexical format |\n| STRING_CONSTRAINT_VIOLATION | A string field violates its declared length or content constraint |
| PERIOD_COUNT_INVALID | Historical or forecast array length is incorrect |
| PERIOD_ID_DUPLICATE | Period identifier is duplicated |
| PERIOD_ORDER_INVALID | Periods are not in chronological order |
| PERIOD_OVERLAP | Period date ranges overlap |
| PERIOD_GAP | Adjacent periods are not contiguous |
| PERIOD_HANDOFF_INVALID | Opening, historical, or forecast hand-off dates do not align |\n| PERIOD_DURATION_INVALID | A reporting period is not an allowed annual period length under V1 |\n| PERIOD_ID_YEAR_MISMATCH | Period identifier does not match the calendar year in which the period ends |
| NUMERIC_DOMAIN_INVALID | A valid decimal is outside the field's allowed domain |
| UNIT_METADATA_INVALID | Currency or unit metadata is missing or inconsistent |
| DEBT_REPAYMENT_EXCEEDS_AVAILABLE | Principal repayment exceeds opening debt plus drawdowns |
| HISTORICAL_NET_INCOME_MISMATCH | Reported normalized net income does not match the defined V1 calculation |
| HISTORICAL_BALANCE_SHEET_UNBALANCED | Historical Balance Sheet fails the accounting identity |
| HISTORICAL_CASH_FLOW_RECONCILIATION_FAILED | Historical cash flow components do not reconcile |
| HISTORICAL_CASH_TIEOUT_FAILED | Cash Flow Statement closing cash differs from Balance Sheet cash |
| HISTORICAL_NWC_ROLLFORWARD_FAILED | Historical change in operating NWC does not match consecutive Balance Sheets |
| HISTORICAL_PPE_ROLLFORWARD_FAILED | Historical PP&E schedule does not reconcile under V1 assumptions |
| HISTORICAL_DEBT_ROLLFORWARD_FAILED | Historical debt schedule does not reconcile |
| HISTORICAL_RETAINED_EARNINGS_FAILED | Historical retained-earnings roll-forward does not reconcile |
| SHARE_CAPITAL_MOVEMENT_UNSUPPORTED | Share capital changes in a V1 period without a supported equity transaction |

The validator should collect independent semantic diagnostics in a deterministic order where safe to do so. A malformed or unparseable payload may stop earlier because later checks would not be reliable.

## 6. Calculation error codes

| Code | Meaning |
|---|---|
| DECIMAL_OVERFLOW | Checked Decimal operation exceeds the supported representation |
| DECIMAL_UNDERFLOW_OR_SCALE_LOSS | An operation cannot preserve the required supported scale/value under the approved policy |
| DIVISION_BY_ZERO | A calculation attempts division by zero |
| INVALID_INTERMEDIATE_STATE | An internal schedule state violates a required invariant |
| UNSUPPORTED_CALCULATION | A requested calculation is outside the V1 model scope |

Arithmetic errors must never be converted to zero, NaN, infinity, or a silently clamped amount.

## 7. Model validation findings

These are findings attached to a completed calculation, not necessarily exceptions that prevent the calculation engine from returning outputs.

| Code | Category | Result effect |
|---|---|---|
| HISTORICAL_BS_INTEGRITY_FAIL | Structural Integrity | FAIL |
| FORECAST_BS_INTEGRITY_FAIL | Structural Integrity | FAIL |
| CASH_ROLLFORWARD_FAIL | Structural Integrity | FAIL |
| SCHEDULE_ROLLFORWARD_FAIL | Structural Integrity | FAIL |
| NEGATIVE_FORECAST_CASH | Scenario Feasibility | FAIL |
| UNDEFINED_RATIO | Warning | N/A with reason |
| LOW_HEADROOM_TO_DECIMAL_LIMIT | Warning or error per approved numeric envelope | Must be disclosed |
| DISPLAY_PRECISION_LIMIT | Reporting | Workbook display policy must be applied and disclosed |

Negative forecast cash is a valid calculated outcome. It must remain visible and cause Scenario Feasibility FAIL. The engine must not create automatic debt, equity, or cash plugs.

## 8. Validation order

The proposed validation order is:

1. Parse the JSON document.
2. Validate schema version and JSON Schema structure.
3. Parse canonical Decimal strings.
4. Validate metadata and period semantics.
5. Validate numeric domains and cross-field constraints.
6. Validate historical statements and roll-forwards.
7. Calculate forecast schedules and statements.
8. Run forecast structural-integrity checks.
9. Run scenario-feasibility checks.
10. Generate output and diagnostics.

This order prevents model calculations from starting with malformed or semantically inconsistent input.

## 9. Output status contract

The model result should report separate statuses:

- input_status: accepted or rejected.
- calculation_status: completed or failed.
- structural_integrity_status: pass or fail.
- scenario_feasibility_status: pass or fail.
- release_readiness_status: pass or fail (research/reproducibility gate, not a financial calculation).

A single overall PASS must not hide a failed sub-status. The CLI and workbook must show the dimensions separately.

## 10. Implementation note

These codes and categories are a proposed stable contract. The Rust implementation must map internal errors to these public codes rather than leak crate-specific error strings as the external API.

## 11. Rust runtime-validator implementation status

The V1 Rust input validator implements the following subset of this contract:

- JSON syntax and strict typed deserialization.
- Unsupported schema version, missing fields, unknown fields, wrong types, and invalid enum values.
- Decimal-string-only parsing, canonical lexical form, scale limit, negative-zero rejection, and exact Decimal parse-to-string preservation.
- Model metadata syntax and constraints.
- Annual period count, unique identifiers, identifier format/year mapping, Gregorian date validity, period duration, contiguity, and opening-to-history/forecast hand-offs.
- Opening and historical Balance Sheet identities.
- Historical Income Statement net-income reconciliation.
- Historical Cash Flow Statement arithmetic, opening/closing cash ties, and working-capital movement.
- Historical PP&E, debt, retained-earnings, and share-capital roll-forwards.
- Forecast assumption domains and debt repayment limits.

The validator returns an \`input_status\` and a deterministic list of diagnostics. It does not calculate forecast statements, produce Scenario Feasibility findings, or implement the production calculation-error surface. Those remain later milestones.

The Rust runtime uses strict typed Serde deserialization and semantic rules. The separate Draft 2020-12 JSON Schema and its Ajv CI validation remain an independent contract gate; the Rust runtime does not embed an arbitrary JSON Schema execution engine.

Before constructing `ModelInput`, the runtime performs a typed structural preflight against the closed Rust domain shape. It classifies unknown and missing properties, JSON type mismatches, enum values, Decimal string-token requirements, canonical Decimal text, scale, and representational range through contract-owned error variants. Public diagnostic codes are not selected by inspecting Serde display text.

The preflight constructs RFC 6901 JSON Pointers from path segments, escaping `~` as `~0` and `/` as `~1`; array indices are decimal path segments. The empty pointer `""` identifies a root-document type error. When the affected historical or forecast row has a valid `period_id`, it is included as context.

Malformed JSON syntax is classified before structural preflight. When the parser cannot identify an unambiguous field path, `json_pointer`, `field`, and `period_id` remain absent rather than fabricated. Typed Serde decoding runs only after preflight passes. If it nevertheless rejects the value, the validator emits `INPUT_DESERIALIZATION_FAILED` without leaking dependency error text. Semantic diagnostics continue to include field pointers where available.
