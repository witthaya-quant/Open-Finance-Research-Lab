<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Decision Log

This log records the baseline decisions and changes that govern the first release. A decision marked **Approved for V1** is a scope decision for this project version; it is not a claim of statutory accounting compliance or proof that an implementation has passed its tests.

**Baseline authorization:** 2026-11-04  
**Basis:** Founder-directed implementation mandate to proceed with the project using professional engineering and research standards. Technical candidates remain subject to the proof-of-concept and validation gates below.

## 1. V1 baseline decisions

| ID | Decision | Status | Rationale / consequence |
|---|---|---|---|
| D-001 | Model one generic non-financial operating company first | Approved for V1 | Avoid mixing fundamentally different financial-statement structures in the first release |
| D-002 | Use three annual historical periods and five annual forecast periods | Approved for V1 | Provides historical context while keeping the initial model manageable |
| D-003 | Use a deterministic synthetic dataset for the first reference case | Approved for V1 | Avoids issuer-data licensing, redistribution, and normalization issues during initial model validation |
| D-004 | Use one reporting currency and one display scale per model instance; USD and USD thousands are illustrative defaults | Approved for V1 | Prevents silent currency/unit mixing while retaining configurable metadata |
| D-005 | Use an aggregate PP&E schedule without disposals in V1 | Approved for V1 | Keeps the first release focused on statement integration; asset-vintage accounting is deferred |
| D-006 | Use average opening/closing debt for interest calculation | Approved for V1 | A transparent approximation for intra-year debt movements; its limitation must be disclosed |
| D-007 | Assume same-period cash settlement for interest and income tax | Approved for V1 | Avoids interest payable and tax payable schedules in the baseline; not a statutory cash-flow policy |
| D-008 | Do not auto-fund negative cash with an invented revolver, cash plug, or equity issuance | Approved for V1 | Exposes funding gaps rather than concealing them |
| D-009 | Report Structural Integrity and Scenario Feasibility separately | Approved for V1 | A balanced model may still be unfunded; a cash-positive scenario may still have broken links |
| D-010 | Do not claim IFRS, US GAAP, Thai Financial Reporting Standards, or other statutory compliance | Approved for V1 | The first release is an analytical modelling framework, not a statutory reporting engine |
| D-011 | Implement financial calculations, authoritative validation, and CLI in Rust; generate Excel as an analyst-facing report | Approved for V1 | Establishes one canonical calculation engine while preserving a familiar finance-practitioner deliverable |
| D-012 | Use versioned JSON/CSV with decimal-string values as the canonical first-release interchange contract | Approved for V1 | Avoids parsing authoritative monetary inputs through spreadsheet binary floating-point |
| D-013 | Do not require Python in V1 | Approved for V1 | Keep the canonical model and authoritative test suite in one Rust toolchain |
| D-014 | Begin with one Rust model library crate and one CLI crate in a Cargo workspace | Approved for V1 | Avoid premature crate fragmentation; extract stable reusable components only when justified |
| D-015 | Select dependencies through a documented proof-of-concept and dependency review | Approved for V1 | Precision, MSRV, licence, maintenance, security, and compatibility must be checked before adoption |

## 2. Numeric proof-of-concept decisions

| ID | Candidate / policy | Status | Acceptance requirement |
|---|---|---|---|
| N-001 | Evaluate rust_decimal 1.43.0 as the initial fixed-point candidate | POC passed; candidate only | Two-toolchain CI and defined numeric/serialization tests passed; final adoption remains subject to model-specific numeric analysis and full dependency review |
| N-002 | Use field-level Serde string encoding for Decimal values (serde-with-str) | POC passed | JSON/CSV string round-trip passed; numeric JSON tokens and non-canonical strings rejected |
| N-003 | Use checked Decimal arithmetic and typed domain errors | Policy; tested boundary subset passed | Checked overflow, subtraction, multiplication, and division-by-zero cases passed; the production model error taxonomy remains to be implemented |
| N-004 | Preserve canonical Decimal strings separately from XLSX numeric cells | Design policy; workbook gate open | Canonical string boundary tested; XLSX output and reconciliation have not yet been implemented |
| N-005 | Treat Decimal as fixed precision, not arbitrary precision or exact rational arithmetic | POC passed; policy confirmed | Scale-28 division outputs recorded; repeating division does not reconstruct exact rational values |
| N-006 | Use Rust 1.85.0 as the effective minimum toolchain for the current POC dependency graph | CI passed | Rust 1.85.0 and 1.99.1 passed formatting, Clippy, 9 tests, and docs; re-evaluate if the dependency graph changes |

## 2.1 Numeric POC results and remaining gates

The Decimal/string-interchange POC passed on 2026-11-04 at PR #6 head commit 88e34261b6299e7bc1aadc9c567348264e037872.

- Rust 1.85.0: formatting, Clippy, 9 tests, documentation build — PASS.
- Rust 1.99.1: formatting, Clippy, 9 tests, documentation build — PASS.
- Division observations: 1/365 = 0.0027397260273972602739726027; 1/3 = 0.3333333333333333333333333333; (1/3)×3 = 0.9999999999999999999999999999.
- A 29-fractional-digit raw parser case was observed to lose precision; the POC input adapter now rejects the over-scale value and checks exact parse-to-string round-trip.
- Direct package metadata and the effective MSRV are documented in [DEPENDENCY_REVIEW.md](poc/decimal-contract/DEPENDENCY_REVIEW.md).

The Decimal POC does not close the complete dependency/security review. The production application Cargo.lock is committed in PR #10; the separate full-formula numeric POC lockfile is present on the still-open PR #11 branch and is not yet part of main. No complete Rust SBOM/licence/security review is recorded. The POC also does not validate the full production financial model or XLSX export.

## 2.2 Input contract design decisions

| ID | Decision / constraint | Status | Rationale / consequence |
|---|---|---|---|
| C-001 | Use a versioned JSON Schema Draft 2020-12 input contract with closed objects | Schema and Rust runtime semantic validation implemented and CI-verified | Ajv validates the published structural/lexical contract; PR #10 delivered the Rust typed historical semantic validator. Neither gate calculates forecasts |
| C-002 | V1 schema 0.1.0 accepts synthetic historical data only | Proposed | Real filings and licensed feeds need expanded provenance, accounting-basis, restatement, and normalization metadata |
| C-003 | Require an opening Balance Sheet immediately before the first historical period | Proposed | Enables three historical years of working-capital, PP&E, debt, cash, and retained-earnings roll-forward checks |
| C-004 | Require exactly three historical annual periods and five forecast annual periods | Approved V1 scope; enforced by the runtime validator | Keeps the external model horizon deterministic and prevents silent horizon changes |
| C-005 | Separate JSON Schema validation from accounting and cross-period semantic validation | Required design policy | Structural validity does not establish accounting consistency or model correctness |
| C-006 | Use a typed external diagnostic contract rather than expose dependency-specific error strings | Contract and diagnostic types implemented; deserialization path/cause mapping still needs hardening | Stable public codes exist, but the current mapper inspects Serde message text and does not expose a JSON Pointer for every deserialization error |

The schema and synthetic fixture passed Draft 2020-12 compilation and positive/negative fixture validation in PR #8. PR #10 subsequently delivered the Rust typed input and historical semantic validator, including accounting/cross-period checks, and passed the Rust 1.85.0/1.99.1 CI matrix. This validates input acceptance and historical statements only; forecast calculation, Scenario Feasibility and the production calculation-error surface remain unimplemented.

## 3. Deferred scope

The following are deliberately outside the V1 baseline:

- Banking, insurance, REIT, funds, and other financial-institution statement structures.
- Group consolidation, minority interests, complex segment reporting, and intercompany eliminations.
- Multi-currency translation and FX remeasurement within one model instance.
- Leases, pensions, share-based compensation, goodwill impairment, and complex intangible-asset schedules.
- Deferred tax, tax-loss carryforwards, tax payable, interest payable, and jurisdiction-specific tax calculations.
- Asset disposals, impairments, revaluations, asset-vintage depreciation, and purchase-price allocation.
- Revolvers, cash sweeps, minimum-cash debt loops, and circular interest calculations.
- Equity issuance, share buybacks, and complex dividend policy.
- Quarterly/monthly periods and DCF/DDM/LBO/M&A valuation.

A deferred item must not be approximated silently. Adding it requires a versioned scope change and new validation criteria.

## 4. Open engineering gates

The schema and historical semantic-validation gates are complete. The following implementation gates remain open:

1. Finalize the model-specific Numeric Profile: supported monetary ranges, rate/day bounds, intermediate headroom, division behaviour, accumulated error budget, reconciliation policy and reporting rounding.
2. Complete transitive dependency inventories, machine-readable SBOMs, licence/notice audits and security-advisory review for the production Rust workspace and isolated POCs.
3. Complete an independently derived five-year Case A formula benchmark and an independent accounting/formula review; the Exact Rational POC shares its generic forecast formulas with the candidate backends.
4. Record the production numeric-backend decision only after the Numeric Profile, dependency review and operational cost/complexity comparison are accepted.
5. Harden the public diagnostic mapping, including stable typed errors and field-path capture.
6. Implement the Rust forecast calculation core and deterministic ModelRun result with unit, integration, property, golden and boundary tests.
7. Validate forecast statement identities, schedule roll-forwards, funding feasibility and reportability without plugs.
8. Generate the analyst-facing XLSX from canonical Rust outputs and reconcile display values to canonical outputs.
9. Establish release CI, deterministic run provenance, reproduction instructions and independent review.

## 5. Change control

Material changes must record the previous decision, the new decision, the reason, the affected model assumptions, and the validation tests that must be rerun. Acceptance criteria must not be weakened after a failed test simply to obtain a pass.

A V1 approval does not waive a failed implementation or validation gate.