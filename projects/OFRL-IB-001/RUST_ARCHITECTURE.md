<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: Apache-2.0 -->

# OFRL-IB-001 — Rust-First Technical Architecture

**Architecture version:** 0.1.0  
**Status:** Proposed  
**Implementation language:** Rust  
**Primary objective:** Keep financial calculation logic, model validation, and reproducibility inside a Rust-native toolchain.

## 1. Architectural decision

OFRL-IB-001 is a **Rust-first financial modelling project**.

Rust is not an optional optimization layer around a spreadsheet or Python model. The Rust implementation is the canonical calculation engine and the primary engineering deliverable.

The spreadsheet is an analyst-facing interoperability and reporting artifact. It must not become a second, undocumented source of truth. Python is not required by the first release.

### System-of-record rule

- The Rust calculation engine defines the model's financial mechanics.
- Input files are versioned and validated before calculation.
- The Rust engine produces canonical outputs and validation results.
- Excel workbooks are generated from the canonical model outputs for analyst review and presentation.
- Any later interactive spreadsheet calculations must be reconciled against the Rust engine before they can be treated as equivalent.

## 2. Logical architecture

Input schema → Validation → Domain model → Schedules → Financial statements → Integrity checks → Output adapters

### Layer responsibilities

| Layer | Responsibility | Design constraint |
|---|---|---|
| Domain | Money, rates, days, periods, model assumptions, financial balances | Strong types; no spreadsheet or file-format dependencies |
| Validation | Input domain checks and cross-field constraints | Fail explicitly; no silent coercion |
| Forecast | Revenue and operating forecast | Deterministic functions |
| Schedules | Working capital, PP&E, debt, tax, dividends | Explicit opening/closing roll-forwards |
| Statements | Income Statement, Balance Sheet, Cash Flow Statement | Derived from the same domain model |
| Integrity | Accounting identities, schedule invariants, funding feasibility | Separate hard gates and warnings |
| Input adapters | Versioned JSON and CSV input schemas | Decimal values represented as strings in canonical interchange |
| Output adapters | Canonical JSON/CSV and XLSX report | Preserve exact canonical results outside Excel's numeric representation |
| CLI | Run, validate, export, and report commands | Thin orchestration layer; no financial logic hidden in CLI |

The domain and calculation layers must remain independent from Excel, operating-system paths, network clients, and presentation concerns.

## 3. Proposed Rust workspace

Begin with a small Cargo workspace rather than many prematurely separated crates.

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

This is an initial logical layout. Modules should remain within the two crates until stable boundaries justify extraction. Avoid creating a large multi-crate architecture before the first working model exists.

## 4. Numeric policy for financial calculations

The core model should use fixed-point decimal arithmetic for monetary values and model rates where appropriate. Binary floating-point must not be used for authoritative currency calculations in v1.

rust_decimal is a candidate, not an unconditional dependency decision. Its current documentation describes a 96-bit mantissa and scale from 0 to 28, with roughly 28–29 significant decimal digits. This is suitable for many financial-model calculations, but the project must test range, scale, overflow, division, rounding, and serialization behaviour against its intended input limits before adoption.

Required policy:

- Use checked arithmetic for operations that can overflow or divide by zero.
- Return typed errors; do not hide arithmetic failure behind zero values.
- Do not round intermediate calculations.
- Define explicit rounding modes and display scales at output boundaries.
- Serialize canonical Decimal values as decimal strings, not JSON floating-point numbers.
- Preserve units, currency, period, and scale metadata.
- Excel numeric cells are presentation values and may have less precision than the Rust canonical result. Exact results must remain available in canonical text-based output.

No claim of unlimited or arbitrary precision is made.

## 5. Candidate Rust ecosystem components

These are candidates for a controlled dependency review and proof-of-concept. They are not yet locked into Cargo.toml.

| Component | Candidate | Intended role | Adoption condition |
|---|---|---|---|
| Fixed-point decimal | rust_decimal | Financial amounts and fixed-point model parameters | Pass range, precision, checked-operation, and serialization tests |
| Serialization | serde | Typed input/output schemas | Explicit decimal-string representation and stable schema tests |
| XLSX export | rust_xlsxwriter | Generate analyst-facing .xlsx reports | Confirm formatting, formula policy, output precision, and compatibility |
| Property testing | proptest | Generate input cases and test invariants | Deterministic, bounded strategies; failures recorded as reproducible cases |
| Error types | Standard Result plus a reviewed error crate if useful | Explicit failure surfaces | Keep domain errors stable and actionable |

calamine is a possible future input adapter for reading Excel and OpenDocument files. XLSX input is not required in the first release because spreadsheet numeric cells can introduce a precision boundary before values reach the Decimal calculation core. The canonical first-release input path should be JSON/CSV with Decimal strings.

### Excel export constraint

rust_xlsxwriter writes new XLSX workbooks and supports formulas and formatting, but it does not modify existing workbooks. Its optional rust_decimal integration writes a Decimal only when it can be represented by f64. Therefore, OFRL must not assume that this export path preserves arbitrary Decimal precision. Canonical results remain in Rust and decimal-string JSON/CSV; XLSX is a presentation artifact with a documented display precision.

## 6. API and error design

The calculation engine should expose deterministic functions with explicit inputs and outputs.

Proposed principles:

- Prefer immutable input structures and pure calculation functions.
- Use newtypes for semantically different quantities such as Money, Rate, Days, and Period.
- Use enums for statement periods, validation severity, and model status where appropriate.
- Return typed Result values for invalid inputs, arithmetic errors, and infeasible calculations.
- Distinguish invalid inputs from valid but unfavourable economic outcomes.
- Keep calculations independent from command-line parsing and workbook generation.
- Avoid global mutable state and hidden configuration.

A negative forecast cash balance is a valid calculated outcome that fails the funding-feasibility gate; it is not an arithmetic exception.

## 7. Validation strategy in Rust

The validation suite should have multiple layers:

1. **Unit tests:** individual financial formulas and schedule roll-forwards.
2. **Integration tests:** complete statements and cross-statement reconciliation.
3. **Property-based tests:** invariants across generated valid inputs, with shrinking to minimal failing examples.
4. **Golden tests:** deterministic synthetic scenarios with reviewed expected outputs.
5. **Boundary tests:** zero denominators, extreme-but-valid inputs, negative earnings, and invalid domains.
6. **Serialization tests:** Decimal strings round-trip without precision loss.
7. **Export tests:** generated workbook structure, required sheets, labels, and declared display precision.

proptest is a candidate for the property-based layer. Test generators must respect input domains for valid-property tests; invalid-domain tests should deliberately generate and verify rejected inputs.

## 8. Quality gates and developer workflow

Proposed baseline commands:

~~~bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo doc --workspace --no-deps
~~~

The exact feature matrix and CI commands should be fixed once the Cargo workspace exists. Cargo.lock should be committed for reproducible application builds. Dependency additions require a recorded purpose, licence review, and maintenance/security check.

CI should not claim financial correctness merely because compilation, formatting, Clippy, or unit tests pass. Accounting and cross-statement invariants remain separate mandatory acceptance gates.

## 9. Dependency governance

Before adopting a crate, record:

- Project purpose and exact feature set.
- Current version and MSRV.
- Licence and compatibility with the repository's eventual licensing policy.
- Maintenance status, release history, open security advisories, and transitive dependencies.
- Whether a smaller standard-library implementation is preferable.
- Known precision, performance, portability, and interoperability limitations.

Pin resolved versions in Cargo.lock. Do not blindly select a dependency because it is popular or newly released.

## 10. Phased implementation

| Phase | Deliverable | Exit condition |
|---|---|---|
| A — Specification | Financial rules, schema, Rust architecture, validation matrix | Founder approves scope and conventions |
| B — Numeric and schema spike | Decimal proof-of-concept, JSON round-trip, input validation | Precision/range behaviour is documented and tests pass |
| C — Rust calculation core | Forecasts, schedules, statements, validation | Unit and integration tests pass |
| D — Deterministic CLI | Run and validate a synthetic scenario | Same version and inputs reproduce the same canonical outputs |
| E — Excel export | Analyst-facing workbook report | Workbook reconciles to canonical Rust output within declared display tolerance |
| F — Hardening | Property, boundary, golden, and export tests; documentation | All release gates pass or failures are explicitly disclosed |
| G — Ecosystem contribution | Extract reusable abstractions only where generality is demonstrated | Stable API, documentation, licence, and independent review |

Do not publish a general-purpose crate simply to create a crates.io presence. Extract and publish reusable components only after they have a stable API, meaningful documentation, tests, and a clear use case beyond this one model.

## 11. References checked for this architecture draft

The following upstream documentation was checked on 2026-11-03. Versions can change; actual dependency versions must be rechecked when the implementation spike begins.

- [Rust documentation and learning resources](https://rust-lang.org/documentation/)
- [Cargo workspaces — The Rust Programming Language](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
- [rust_decimal documentation](https://docs.rs/rust_decimal/latest/rust_decimal/)
- [serde documentation](https://docs.rs/serde/latest/serde/)
- [proptest documentation](https://docs.rs/proptest/latest/proptest/)
- [rust_xlsxwriter documentation](https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/)
- [calamine documentation](https://docs.rs/calamine/latest/calamine/)

This architecture is a proposal. No dependency has been adopted and no Rust implementation is claimed to exist yet.