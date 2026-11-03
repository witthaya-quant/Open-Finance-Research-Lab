<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Engineering Architecture

**Architecture version:** 0.1.0  
**Status:** Proposed  
**Implementation language:** Rust

## 1. Implementation direction

Rust will be used for the financial calculation engine, core validation logic, and command-line tooling.

This is a deliberate engineering choice. The project provides a practical way to develop stronger Rust skills while building a useful financial model. It also gives the work a path toward reusable software components, provided those components prove useful beyond this project.

The Rust implementation will be the reference for financial calculations. The Excel workbook will be generated for analyst review and reporting; it will not define a separate set of calculation rules.

Other tools may be introduced for a specific task such as data inspection, independent comparison, or file interoperability. They should not create a second source of truth for the model.

## 2. Logical architecture

Input schema → Validation → Domain model → Forecasts and schedules → Financial statements → Integrity checks → Output adapters

| Layer | Responsibility | Design constraint |
|---|---|---|
| Domain | Money, rates, days, periods, assumptions, balances | Strong types; no spreadsheet or file-format dependencies |
| Validation | Input domains and cross-field constraints | Explicit errors; no silent coercion |
| Forecast | Revenue and operating forecast | Deterministic calculation functions |
| Schedules | Working capital, PP&E, debt, tax, dividends | Explicit opening and closing roll-forwards |
| Statements | Income Statement, Balance Sheet, Cash Flow Statement | Derived from the same domain model |
| Integrity | Accounting identities, schedule invariants, funding feasibility | Separate hard gates and warnings |
| Input adapters | Versioned JSON and CSV schemas | Decimal values represented as strings in canonical interchange |
| Output adapters | Canonical JSON/CSV and XLSX report | Preserve canonical values outside Excel numeric representation |
| CLI | Run, validate, export, report | Thin orchestration layer; no financial logic hidden in CLI |

The domain and calculation layers should remain independent of Excel, operating-system paths, and network clients.

## 3. Proposed Cargo workspace

Start with one workspace containing a model library and a command-line application. Keep modules inside those crates until stable boundaries justify extraction.

~~~text
ofrl-ib-001/
├── Cargo.toml
├── Cargo.lock
├── crates/
│   ├── ofrl_model/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── domain/
│   │       ├── assumptions/
│   │       ├── forecast/
│   │       ├── schedules/
│   │       ├── statements/
│   │       ├── validation/
│   │       └── serialization/
│   └── ofrl_cli/
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
├── schemas/
├── fixtures/
│   └── synthetic/
├── tests/
│   ├── integration/
│   ├── properties/
│   └── golden/
├── workbooks/
│   └── generated/
└── docs/
~~~

This is an initial layout, not a mandate to create every folder immediately. The first working model should determine which module boundaries are useful.

## 4. Numeric policy

The core model should use fixed-point decimal arithmetic for monetary values and model rates where appropriate. Binary floating-point must not be used for authoritative currency calculations in the initial release.

rust_decimal is a candidate, not an unconditional dependency decision. Its documented representation has a 96-bit mantissa and a scale from 0 to 28. This may suit the intended financial-model use, but the project must test range, scale, overflow, division, rounding, and serialization behaviour before adopting it.

Required behaviour:

- Use checked arithmetic where operations may overflow or divide by zero.
- Return typed errors; do not hide arithmetic failures behind zero values.
- Do not round intermediate calculations.
- Define rounding modes and display scales at output boundaries.
- Serialize canonical Decimal values as strings, not JSON floating-point numbers.
- Preserve currency, unit, scale, and period metadata.
- Keep exact canonical values in JSON/CSV output; Excel numeric cells are presentation values with a documented precision boundary.

No claim of unlimited or arbitrary precision is made.

## 5. Candidate Rust ecosystem components

These are candidates for a controlled dependency review and proof of concept. They are not yet adopted or locked into Cargo.toml.

| Component | Candidate | Intended role | Adoption condition |
|---|---|---|---|
| Fixed-point decimal | rust_decimal | Financial amounts and fixed-point model parameters | Pass range, precision, checked-operation, and serialization tests |
| Serialization | serde | Typed input/output schemas | Decimal-string representation and stable schema tests |
| XLSX export | rust_xlsxwriter | Generate analyst-facing .xlsx reports | Confirm formatting, formula policy, output precision, and compatibility |
| Property testing | proptest | Generate inputs and test invariants | Bounded strategies and reproducible failure cases |
| Error handling | Standard Result, with a reviewed error crate if useful | Explicit failure surfaces | Keep domain errors stable and actionable |

calamine may be considered later if reading existing Excel or OpenDocument files becomes a requirement. The initial input path should be JSON/CSV with Decimal strings, avoiding an unnecessary precision conversion at the start of the calculation process.

### Excel export boundary

rust_xlsxwriter creates new XLSX workbooks and supports formulas and formatting; it does not modify existing workbooks. Its optional rust_decimal integration writes a Decimal only when it can be represented by f64. OFRL must therefore not assume this export path preserves arbitrary Decimal precision.

The Rust model and decimal-string JSON/CSV outputs remain authoritative. The Excel report must state its display precision and be reconciled to the canonical outputs using a documented tolerance.

## 6. API and error design

The calculation library should expose deterministic functions with explicit inputs and outputs.

- Prefer immutable input structures and pure calculation functions.
- Use newtypes for semantically different quantities such as Money, Rate, Days, and Period.
- Use enums for period types, validation severity, and model status where appropriate.
- Return typed Result values for invalid inputs and arithmetic failures.
- Distinguish invalid inputs from valid but unfavourable economic outcomes.
- Keep financial calculations independent of command-line parsing and workbook generation.
- Avoid global mutable state and hidden configuration.

A negative forecast cash balance is a valid calculated outcome. It fails the funding-feasibility gate, but it is not an arithmetic exception.

## 7. Testing and validation

The Rust test suite should include:

1. **Unit tests:** individual financial formulas and schedule roll-forwards.
2. **Integration tests:** complete statements and cross-statement reconciliation.
3. **Property-based tests:** invariants over generated valid inputs, with shrinking to minimal failing examples.
4. **Golden tests:** deterministic synthetic scenarios with reviewed expected outputs.
5. **Boundary tests:** zero denominators, extreme-but-valid inputs, negative earnings, and invalid domains.
6. **Serialization tests:** Decimal strings round-trip without precision loss.
7. **Export tests:** workbook structure, required sheets, labels, and declared display precision.

proptest is a candidate for property-based testing. Generators for valid-property tests must respect the model's input domains. Separate invalid-input tests should verify that rejected values produce explicit errors.

## 8. Development checks

Proposed baseline commands after the Cargo workspace exists:

~~~bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo doc --workspace --no-deps
~~~

The exact feature matrix and CI commands should be fixed with the initial implementation. Commit Cargo.lock for reproducible application builds. Dependency additions need a recorded purpose, licence review, and maintenance/security check.

Passing compilation, formatting, Clippy, or unit tests does not by itself establish financial correctness. Accounting and cross-statement invariants remain separate acceptance requirements.

## 9. Dependency review

Before adopting a crate, record:

- Purpose and exact feature set.
- Current version and minimum supported Rust version (MSRV).
- Licence and compatibility with OFRL's licensing policy.
- Maintenance status, release history, security advisories, and transitive dependencies.
- Whether a smaller standard-library implementation would be preferable.
- Known precision, performance, portability, and interoperability limitations.

Do not select a dependency solely because it is popular or newly released.

## 10. Delivery stages

| Stage | Deliverable | Exit condition |
|---|---|---|
| A — Specification | Financial rules, schema, engineering architecture, validation matrix | Founder approves scope and conventions |
| B — Numeric and schema proof of concept | Decimal checks, JSON round-trip, input validation | Precision and range behaviour are documented and tests pass |
| C — Calculation library | Forecasts, schedules, statements, validation | Unit and integration tests pass |
| D — Command-line tool | Run and validate a synthetic scenario | Identical inputs and version reproduce identical canonical outputs |
| E — Excel report | Analyst-facing workbook | Workbook reconciles to canonical outputs within declared display tolerance |
| F — Hardening | Property, boundary, golden, and export tests; documentation | Release gates pass or failures are disclosed |
| G — Reusable components | Extract abstractions that have value beyond this project | Stable API, documentation, tests, licence, and review |

Do not publish a general-purpose crate merely to create a crates.io presence. Release a component when it has a stable purpose, a useful API, meaningful tests, documentation, and a likely audience beyond this project.

## 11. References

The following upstream documentation was checked on 2026-11-03. Versions and dependency health must be checked again when implementation begins.

- Rust documentation: https://rust-lang.org/documentation/
- Cargo workspaces: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
- rust_decimal: https://docs.rs/rust_decimal/latest/rust_decimal/
- serde: https://docs.rs/serde/latest/serde/
- proptest: https://docs.rs/proptest/latest/proptest/
- rust_xlsxwriter: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/
- calamine: https://docs.rs/calamine/latest/calamine/

This document records a proposed design. No dependency has been adopted and no implementation is claimed to exist yet.