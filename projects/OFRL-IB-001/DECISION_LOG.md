<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Decision Log

This log records the baseline decisions and changes that govern the first release. A decision marked **Approved for V1** is a scope decision for this project version; it is not a claim of statutory accounting compliance or proof that an implementation has passed its tests.

**Baseline authorization:** 2026-11-04  
**Basis:** Founder-directed implementation mandate to proceed with the project using disciplined engineering and research controls. Technical candidates remain subject to the proof-of-concept and validation gates below.

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
| N-007 | Do not select a production numeric backend until a finite, approved Numeric Profile and candidate-specific runtime/resource evidence exist | OPEN — no backend selected | Static capacity checks are pre-screening evidence only; no candidate has passed the complete correctness, arithmetic-context, serialization, resource and Finance/Accounting gates |

## 2.1 Numeric POC results and remaining gates

The Decimal/string-interchange POC passed on 2026-11-04 at PR #6 head commit 88e34261b6299e7bc1aadc9c567348264e037872.

- Rust 1.85.0: formatting, Clippy, 9 tests, documentation build — PASS.
- Rust 1.99.1: formatting, Clippy, 9 tests, documentation build — PASS.
- Division observations: 1/365 = 0.0027397260273972602739726027; 1/3 = 0.3333333333333333333333333333; (1/3)×3 = 0.9999999999999999999999999999.
- A 29-fractional-digit raw parser case was observed to lose precision; the POC input adapter now rejects the over-scale value and checks exact parse-to-string round-trip.
- Direct package metadata and the effective MSRV are documented in [DEPENDENCY_REVIEW.md](poc/decimal-contract/DEPENDENCY_REVIEW.md).

### 2.1.1 Numeric Profile derivation and candidate screen — 2026-11-05

PR #13 adds a closed Numeric Profile input schema, exact BigInt-rational derivation for opening/historical/forecast arithmetic, separate division-error propagation, and a static candidate-capacity evaluator. The derivation graph now contains arithmetic nodes only; comparison/tie-out checks do not create fictitious sum nodes. Each node reports an exact outward-rounded coefficient-integer upper bound at its declared scale.

The validated source head `17b77e385f9980ff00c6ab1466361a54a5d34d23` passed all six required Ruleset contexts, Numeric Envelope derivation (30/30 tests) and Release Candidate Preview before merge to `main` at `593d04d125522e5ec42bea06da6b94c3c2abf604`. The test-only fixture produced 287 arithmetic nodes; the Numeric Envelope artifact ZIP checksum and all five internal entries were verified: `c1ecdb255e2b93bd057860a457f7d439456b64b18b7ccac8052c28171684ed36`.
- `rust_decimal 1.43.0`: `STATIC_BOUND_EXCEEDS_LIMIT` on 37 nodes; 37 conservative coefficient upper bounds exceed the exact 96-bit maximum and 19 exceed scale 28 (overlapping reasons); 50 additional nodes remain conditional because the division policy is unresolved. This is not an observed runtime failure.
- `bigdecimal 0.4.11`: static CONDITIONAL on 50 division-dependent nodes; no profile static-ceiling excess was reported, but this is not a runtime or operational pass.

The fixture is illustrative only. It is not the intended V1 operating domain. BigDecimal's POC uses an 80-significant-digit compile-time default context, while the current Numeric Profile schema specifies a fixed fractional division scale; a production adapter or a deliberate versioned policy change is required to prove equivalence. The static evaluator does not execute candidate arithmetic, measure CPU/memory/allocations/concurrency, or authorize production use.

**Disposition:** N-007 remains OPEN. The Model Owner must supply finite input-domain and source-precision decisions in Issue #28. Finance/Accounting and Engineering must then approve separate error budgets and arithmetic semantics before a production backend can be selected. The Forecast Engine remains NOT AUTHORIZED.

The Decimal POC does not close the complete dependency/source/licence review. PR #11 remains open and contains a separate numeric POC lockfile, candidate CycloneDX inventories and RustSec workflow. The isolated Decimal Contract POC lockfile and locked CI commands are now on main through PR #20; the preview-only CLI SBOM/checksum workflow is on main through PR #22. The preview covers the existing Linux CLI input validator only. Point-in-time advisory scans are not a permanent guarantee; licence texts, notices, source provenance and distribution obligations remain open. Neither POC is the production calculation engine.

## 2.2 Input contract design decisions

| ID | Decision / constraint | Status | Rationale / consequence |
|---|---|---|---|
| C-001 | Use a versioned JSON Schema Draft 2020-12 input contract with closed objects | Schema and Rust runtime semantic validation implemented and CI-verified | Ajv validates the published structural/lexical contract; PR #10 delivered the Rust typed historical semantic validator. Neither gate calculates forecasts |
| C-002 | V1 schema 0.1.0 accepts synthetic historical data only | Implemented for schema/runtime version 0.1.0 | Real filings and licensed feeds need expanded provenance, accounting-basis, restatement, and normalization metadata before they can be admitted in a future contract version |
| C-003 | Require an opening Balance Sheet immediately before the first historical period | Implemented and runtime-validated | Enables three historical years of working-capital, PP&E, debt, cash, and retained-earnings roll-forward checks |
| C-004 | Require exactly three historical annual periods and five forecast annual periods | Approved V1 scope; runtime validation implemented | Keeps the external model horizon deterministic; the Rust validator checks period counts and hand-offs |
| C-005 | Separate JSON Schema validation from accounting and cross-period semantic validation | Implemented design policy | Structural validity does not establish accounting consistency or model correctness; separate schema and Rust semantic gates are present |
| C-006 | Use a typed external diagnostic contract rather than expose dependency-specific error strings | Implemented and merged to `main` through PR #13 (`593d04d`) | Typed structural preflight maps contract-owned error kinds to stable codes and RFC 6901 pointers; malformed JSON has no fabricated pointer. Nine versioned mutation fixtures are consumed by regression tests. Six required contexts, Numeric Envelope derivation and Release Candidate Preview passed on source head `17b77e3`; Issue #23 closed on merge. The Founder Stage 0 disposition is recorded as AI-assisted execution, not independent human approval |

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

The schema and historical semantic-validation gates are implemented on main. The following remain open; open-PR evidence is not treated as merged capability.

1. Approve a versioned Numeric Profile: supported monetary/rate/day bounds, formula-chain intermediate headroom, deterministic division context, five-year error budget, separate accounting/schedule/display tolerances and output rounding.
2. Select the production numeric backend only after the Numeric Profile, controlled performance/resource evidence, serialization semantics and migration impact on the rust_decimal-based validator are reviewed.
3. Complete dependency governance: the Decimal Contract POC lockfile and locked CI are on main through PR #20; continue reviewing package source, duplicate versions, original licence texts, notices, legal alternatives and distribution obligations. PR #11's SBOM/RustSec workflow and PR #22's preview-only CLI SBOM are inventory and point-in-time advisory evidence, not final legal/security approval.
4. Obtain independent finance/accounting review of V1 conventions and statement formulas. PR #11's BM-002/BM-003 provide 800 exact expected outputs for the declared synthetic mechanics; they do not constitute accounting sign-off.
5. Typed deserialization diagnostics and nine versioned mutation fixtures are implemented and merged to `main` through PR #13 (`593d04d125522e5ec42bea06da6b94c3c2abf604`); all six required contexts, Numeric Envelope derivation and Release Candidate Preview passed on validated source head `17b77e385f9980ff00c6ab1466361a54a5d34d23`. Issue #23 closed automatically. Future changes to the contract must extend the diagnostic and shared-corpus regression suites.
6. Maintain and extend the versioned positive/negative contract corpus consumed by both JSON Schema and Rust runtime validation.
7. Implement typed production forecast schedules/statements, calculation errors, Structural Integrity, Scenario Feasibility and explicit reportability status.
8. Implement canonical ModelRun, versioned serialization, SHA-256 provenance, deterministic fingerprint and metric-level formula lineage.
9. Generate XLSX only from canonical Rust outputs; reconcile every material displayed value and preserve Decimal-string canonical outputs.
10. Add property, numeric-boundary, negative, determinism, output and clean-checkout reproduction evidence.

The Production Forecast Engine is not authorized until gates 1–4 are explicitly accepted and recorded.
## 5. Change control

Material changes must record the previous decision, the new decision, the reason, the affected model assumptions, and the validation tests that must be rerun. Acceptance criteria must not be weakened after a failed test simply to obtain a pass.

A V1 approval does not waive a failed implementation or validation gate.

## 6. Research evidence update — 2026-11-04

PR #11 (open Draft; not merged) now contains:
- BM-002: 160 independently expected exact-rational outputs for FY2026–FY2030.
- BM-003: 640 independently expected exact-rational outputs across four five-year edge scenarios.
- A standard-library Python reference that checks the canonical fixture identity and regenerates both goldens, reporting 800 exact expected outputs.
- CycloneDX SBOM and RustSec advisory-scan automation.
- A non-gating runtime-cost observation harness comparing the candidate arithmetic implementations; the first timing run passed in workflow 41784590895 and artifacts are checksum-identified. Initial medians showed BigDecimal at 1.50×–2.21× and Exact Rational at 7.52×–10.30× the fixed Decimal candidate on one 4-vCPU runner.

These are research results, not production approval. The following decisions remain explicitly unmade: Numeric Profile, production numeric backend, arithmetic/error budgets, accounting tolerance, output rounding, licence/source acceptance and Production Forecast Engine adoption.
