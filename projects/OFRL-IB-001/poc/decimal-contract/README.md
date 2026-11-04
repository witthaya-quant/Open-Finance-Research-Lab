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
- 0.1234567890123456789012345678 (28 fractional digits)

It rejects:

- +1
- 01
- .5
- 1.
- 1e3
- 1,000
- -0 and -0.000 (negative zero is not canonical)
- values with more than 28 fractional digits
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

The [JSON Schema](../../../../schemas/ofrl-ib-001-amount-record-0.1.0.schema.json) documents the record shape and canonical amount-string grammar. The Rust POC tests the Decimal field adapter; it does not yet implement the full production metadata validator or a JSON Schema validation engine.

The record is a proof-of-concept contract, not yet the complete production financial-model schema. Full schema evolution, metadata validation, CSV row semantics, and error taxonomy will be designed in the model workspace.

## Run locally

The package declares Rust 1.73 as its minimum supported version. The GitHub Actions matrix checks both Rust 1.73.0 (MSRV) and Rust 1.99.1 (the current toolchain used for the initial POC run). Use a matching toolchain with rustfmt and Clippy installed.

~~~bash
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets -- --nocapture
cargo doc --no-deps
~~~

The repository workflow runs the same checks on GitHub Actions. A green POC workflow establishes only that these numeric and interchange tests pass on the recorded runner/toolchain. It does not establish financial-model correctness.

## Observed candidate behaviour

The first GitHub Actions execution on 2026-11-04 exposed an input-boundary hazard in the pinned rust_decimal 1.43.0 candidate: its raw FromStr parser accepted a value with 29 fractional digits and returned a value rounded to the supported 28-digit scale (the tested value became zero).

The POC therefore does not expose raw Decimal parsing as the application input boundary. The Serde adapter now:

1. Rejects fractional text longer than Decimal::MAX_SCALE before parsing.
2. Parses the value using the pinned Decimal implementation.
3. Requires Decimal-to-string output to match the original input string exactly.
4. Rejects the value if scale, sign, or any decimal digit changes.

This is a compensating input guard that must remain covered by tests. It does not turn Decimal into arbitrary-precision arithmetic, and every calculation still requires explicit range and rounding policy.

## Adoption gate

The Decimal dependency remains a candidate until:

1. The test suite passes on CI.
2. The CI matrix passes on both the declared MSRV and the current Rust toolchain.
3. Actual 1/365 and repeating-division outputs are recorded.
4. Maximum usable scale/range and failure behaviour are documented.
5. JSON/CSV string round-trip and invalid-token rejection pass.
6. Direct/transitive dependencies, versions, licences, and MSRV are reviewed.

If the candidate fails an acceptance requirement, evaluate an alternative Decimal representation before beginning the full model engine. Do not weaken the required input/output contract solely to make a candidate pass.