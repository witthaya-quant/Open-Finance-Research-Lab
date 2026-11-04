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
| N-001 | Evaluate rust_decimal 1.43.0 as the initial fixed-point candidate | Candidate for POC | Test value range, scale, checked operations, division behaviour, and edge cases against OFRL's intended model inputs |
| N-002 | Use field-level Serde string encoding for Decimal values (serde-with-str) | Candidate for POC | JSON and CSV contract tests must prove decimal strings round-trip and numeric JSON tokens are rejected |
| N-003 | Use checked Decimal arithmetic and typed domain errors | Required design policy | Overflow, underflow, and division-by-zero must produce explicit errors; no silent zero or saturation |
| N-004 | Preserve canonical Decimal strings separately from XLSX numeric cells | Required design policy | Workbook output must disclose display precision and reconcile to canonical results within a stated tolerance |
| N-005 | Treat Decimal as fixed precision, not arbitrary precision or exact rational arithmetic | Required design policy | Repeating-division behaviour and effective scale must be documented; no claim of exact representation for non-terminating decimals |

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

These are implementation gates, not unresolved choices about the overall direction:

1. Complete the Decimal and serialization proof-of-concept.
2. Record the exact dependency versions, enabled features, MSRV, licences, and transitive dependency review.
3. Confirm the input/output schema and validation error contract.
4. Implement the calculation core and synthetic reference case.
5. Verify workbook export precision and canonical-output reconciliation.
6. Establish CI checks for formatting, Clippy, tests, documentation, and dependency review.

## 5. Change control

Material changes must record the previous decision, the new decision, the reason, the affected model assumptions, and the validation tests that must be rerun. Acceptance criteria must not be weakened after a failed test simply to obtain a pass.

A V1 approval does not waive a failed implementation or validation gate.