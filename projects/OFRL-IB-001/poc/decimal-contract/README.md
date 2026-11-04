# OFRL Decimal Contract POC

**Project:** OFRL-IB-001  
**Status:** Proof of concept — dependency candidate evaluation  
**Scope:** Decimal representation and JSON/CSV interchange only

This isolated Rust package evaluates whether rust_decimal 1.43.0 can support the numeric and serialization requirements of OFRL-IB-001.

It is deliberately not the three-statement calculation engine. It contains no revenue forecast, working-capital schedule, PP&E schedule, debt schedule, financial statements, or workbook export.

## What this POC tests

- Decimal values serialize as JSON strings, not JSON number tokens.
- Decimal strings round-trip with scale and trailing zeros preserved.
- JSON number tokens are rejected for Decimal fields.
- Non-canonical text is rejected: exponent notation, plus signs, whitespace, thousands separators, leading zeros, and malformed decimal points.
- CSV records round-trip through the same string-based amount field.
- Checked addition, subtraction, multiplication, and division report range or zero-division failure rather than silently substituting values.
- Decimal range and scale behaviour are observable.
- Repeating division, including 1/365 and 1/3, is recorded rather than described as exact rational arithmetic.

## Canonical decimal text grammar

The POC accepts:

- 0
- -12
- 123.4500
- -0.005

It rejects:

- +1
- 01
- .5
- 1.
- 1e3
- 1,000
- leading or trailing whitespace
- NaN and Infinity strings

The Rust field adapter uses rust_decimal's string serializer and a strict string deserializer. This intentionally requires a string token at the Serde boundary before Decimal parsing.

## Minimal record contract

The record contains:

- schema_version
- model_id
- period
- line_item
- currency
- unit_scale
- amount (canonical decimal string)

The JSON Schema is published at the repository path schemas/ofrl-ib-001-amount-record-0.1.0.schema.json. The Rust POC tests the Decimal field adapter; it does not yet implement the full production metadata validator or a JSON Schema validation engine.

The record is a proof-of-concept contract, not yet the complete production financial-model schema. Full schema evolution, metadata validation, CSV row semantics, and error taxonomy will be designed in the model workspace.

## Run locally

Use a stable Rust toolchain with rustfmt and Clippy installed.

~~~bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets -- --nocapture
cargo doc --no-deps
~~~

The repository workflow runs the same checks on GitHub Actions. A green POC workflow establishes only that these numeric and interchange tests pass on the recorded runner/toolchain. It does not establish financial-model correctness.

## Adoption gate

The Decimal dependency remains a candidate until:

1. The test suite passes on CI.
2. Actual 1/365 and repeating-division outputs are recorded.
3. Maximum usable scale/range and failure behaviour are documented.
4. JSON/CSV string round-trip and invalid-token rejection pass.
5. Direct/transitive dependencies, versions, licences, and MSRV are reviewed.

If the candidate fails an acceptance requirement, evaluate an alternative Decimal representation before beginning the full model engine. Do not weaken the required input/output contract solely to make a candidate pass.