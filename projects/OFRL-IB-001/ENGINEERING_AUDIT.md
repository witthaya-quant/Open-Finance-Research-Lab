<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Engineering Audit

**Audit version:** 0.1.0  
**Audit date:** 2026-11-04  
**Audited baseline:** `main` at `4992ae702bf0c378c92112966f0f727262d615c4`  
**Related research branches:** PR #11 (`research/ofrl-ib-001-numeric-envelope`), PR #12 (`chore/schema-validator-lockfile`)  
**Status:** Initial technical audit; findings require closure evidence

## 1. Executive assessment

OFRL-IB-001 has a sound V1 direction and a useful first layer of engineering controls. The repository already contains a versioned input contract, a strict Decimal-string boundary, historical semantic validation, a deterministic synthetic reference input, a Rust workspace, and CI coverage on two Rust toolchains.

It is **not yet a production three-statement model**. The production workspace validates inputs and historical relationships; it does not calculate forecast statements, return a complete model-run result, evaluate forecast funding feasibility, or generate an analyst-facing workbook. The formula path in PR #11 is an isolated research POC, not a production engine.

The correct next move is to close the numeric, dependency, diagnostic, and independent-formula gates, then implement one canonical Rust calculation path. Building workbook formulas or selecting a numeric backend from the two stress points alone would create avoidable rework and control risk.

## 2. Scope and evidence

This audit is limited to the files and branches of OFRL-IB-001 in `witthaya-quant/Open-Finance-Research-Lab`.

| Evidence area | Audited reference |
|---|---|
| Production baseline | `main` at `4992ae702bf0c378c92112966f0f727262d615c4` |
| Numeric comparison POC | PR #11; research branch and its CI evidence |
| Schema-tooling POC | PR #12; lockfile, pinned Node/npm toolchain and npm audit |
| Input contract | `MODEL_INPUT_CONTRACT.md`, Draft 2020-12 schema, Case A fixture |
| Runtime implementation | `crates/ofrl_model/src/domain.rs`, `validation.rs`, `diagnostics.rs` |
| CLI | `crates/ofrl_cli/src/main.rs` |
| Model policy | `MODEL_CONVENTIONS.md`, `NUMERIC_ENVELOPE.md`, `VALIDATION_MATRIX.md` |
| Dependency evidence | Cargo.toml, Cargo.lock, POC dependency review and PR #12 |

The application Cargo.lock on the audited baseline contains 54 package entries. This is a dependency inventory observation, not a completed SBOM, licence audit, or security approval.

PR #11 and PR #12 remain separate open Draft pull requests in the audited state. Their CI evidence is useful but does not make the changes part of `main`.

## 3. What is already sound

| Area | Evidence-based assessment |
|---|---|
| V1 scope | Three historical years, five forecast years, one generic non-financial operating company and one currency/unit scale per model are explicitly bounded. |
| Canonical data | JSON Decimal fields are strings; unknown fields are rejected; schema version is explicit. |
| Decimal lexical safety | The current parser rejects non-string tokens, non-canonical notation, negative zero and scale above 28, and checks parse-to-string equality. |
| Historical validation | Opening and historical Balance Sheet, Income Statement, Cash Flow Statement, and key roll-forwards are checked. |
| Period controls | Period counts, identifiers, Gregorian dates, ordering, duration, contiguity and hand-offs are checked. |
| No silent funding | The project policy prohibits an invented revolver, cash plug or equity issuance to conceal a cash shortfall. |
| Separation of concerns | Excel is specified as an output adapter; Python is not a V1 runtime dependency. |
| Reproducibility | The production Rust workspace has a committed lockfile and CI runs on Rust 1.85.0 and 1.99.1. |
| Numeric research | PR #11 compares two Decimal candidates against an exact-rational arithmetic reference and records candidate-specific differences and capacity observations. |
| Schema tooling | PR #12 isolates and pins the JSON Schema validator toolchain and lockfile; its npm audit is a point-in-time result for that tooling package. |

These are meaningful foundations. They should be extended, not replaced.

## 4. Findings and required disposition

Severity definitions:

- **Blocker:** must close before Production Forecast Engine acceptance.
- **Major:** must close before a reproducible V1 release.
- **Moderate:** should close before external distribution or broader data intake.

### EA-01 — Production forecast engine is not implemented

**Severity:** Blocker  
**Evidence:** `crates/ofrl_model/src/lib.rs` describes an input and historical semantic validator. `RUNTIME_VALIDATOR.md` explicitly excludes forecast calculation. The forecast implementation in PR #11 is an isolated POC.

**Risk:** Treating the POC as production would bypass domain types, calculation error contracts, production result types, full independent accounting review and the model-level numeric envelope.

**Required disposition:** Implement the forecast engine only after the numeric and dependency gates below are accepted. Keep the existing validator as the input gate; do not replace it with POC code.

### EA-02 — Production numeric policy is not approved

**Severity:** Blocker  
**Evidence:** `NUMERIC_ENVELOPE.md` is Proposed. The POC tests selected (10^{18}) and (10^{22}) capacity points, not a complete operating envelope. Neither point establishes the exact overflow boundary or the V1 maximum supported company scale.

**Risk:** An apparently passing test range may omit intermediate-value overflow, rate/day extremes, scale loss, or five-year accumulated division error.

**Required disposition:** Ratify a versioned Numeric Profile that defines input bounds, intermediate headroom, rate/day bounds, scale, arithmetic behaviour, error budget, reconciliation criteria and display policy. Do not infer a universal company-size ceiling from a stress test.

### EA-03 — Input validation is currently coupled to rust_decimal

**Severity:** Blocker if BigDecimal or another backend is selected  
**Evidence:** `domain.rs` stores parsed fields as `rust_decimal::Decimal`; `validation.rs` uses Decimal arithmetic for historical checks; the production Cargo manifest pins rust_decimal 1.43.0. PR #11's BigDecimal candidate is confined to its isolated POC.

**Risk:** Selecting BigDecimal for production after implementing the engine would force a broad refactor across input types, validation, calculations and serialization.

**Required disposition:** Decide whether the production numeric backend is fixed before forecast-engine implementation. If the selected backend differs from the validator's current type, introduce a reviewed canonical decimal boundary and perform an explicit migration. Do not hide multiple numeric implementations behind casual type conversions.

### EA-04 — Rust dependency/SBOM review remains incomplete

**Severity:** Blocker before production dependency approval  
**Evidence:** The production application has Cargo.lock. The POC records candidate-specific dependencies. PR #12's npm audit covers the separate JavaScript schema-validator toolchain, not the Rust application or numeric POC dependency graphs.

**Risk:** Direct package review alone does not establish transitive licence compatibility, security posture, maintenance status or reproducible software composition.

**Required disposition:** Produce machine-readable SBOMs for the production workspace and each isolated POC workspace; record direct and transitive licence data, advisories, source and tool versions, exceptions and review date. A lockfile is necessary but not sufficient.

### EA-05 — The numeric oracle is not an independent model oracle

**Severity:** Blocker before model correctness claims  
**Evidence:** PR #11 documents that Exact Rational, rust_decimal and BigDecimal use the same generic forecast implementation. An independent FY2026 benchmark checks 32 exact output metrics, while the full five-year comparison contains 160 metrics per candidate.

**Risk:** A shared formula error can propagate identically through all three numeric backends and remain invisible to the arithmetic comparison.

**Required disposition:** Build a separately derived five-year benchmark with independently implemented formula sequencing or independently reviewed exact expected outputs. Include all statements, schedules, signs, bounds, negative earnings, zero denominators, debt edge cases, funding shortfalls and multi-period propagation. The arithmetic oracle and model oracle must remain conceptually separate.

### EA-06 — The proposed reconciliation tolerance is not yet justified as a model policy

**Severity:** Major  
**Evidence:** `VALIDATION_MATRIX.md` proposes (10^{-6} + 10^{-8}\times\max(|A|,|B|,1)). The policy has not been approved against the final model unit, numeric backend, schedule depth or materiality.

**Risk:** A relative tolerance grows with balance size. At sufficiently large amounts it may permit a residual that is numerically small relative to the balance but financially material in absolute currency terms. Conversely, an absolute tolerance changes its economic meaning when the model scale changes from units to thousands or millions.

**Required disposition:** Separate at least (a) arithmetic/backend error budget, (b) accounting identity tolerance, (c) schedule roll-forward tolerance and (d) workbook display tolerance. Express monetary tolerances in explicit model units and show their converted currency meaning. Test the proposed policy at the minimum and maximum supported scale before approval. Do not weaken a gate to fit observed results.

### EA-07 — Deserialization diagnostics depend on error-message text

**Severity:** Major  
**Evidence:** `validation.rs` maps Serde failures by checking substrings such as `DECIMAL_STRING_REQUIRED`, `DECIMAL_SCALE_EXCEEDED`, `unknown field` and `missing field`. Some deserialization failures return no JSON Pointer.

**Risk:** A Serde upgrade or wording change can alter classification. Consumers cannot reliably locate every rejected field from a structured pointer.

**Required disposition:** Introduce a typed deserialization boundary with stable error variants and path capture. Review any new dependency before adoption. Keep external diagnostic codes independent of third-party error strings. Add tests for nested fields, array indices, missing fields, unknown fields and invalid enum values.

### EA-08 — Schema and Rust contract can drift

**Severity:** Major  
**Evidence:** The Draft 2020-12 JSON Schema and Rust Serde/domain constraints are separate artifacts and are tested by separate workflows.

**Risk:** A payload can satisfy one contract but fail the other; duplicated required fields, patterns or enum values can drift unnoticed.

**Required disposition:** Maintain a shared positive/negative contract corpus and run it against both schema validation and the Rust runtime. Document intentional differences between structural/lexical schema checks and cross-field semantic checks.

### EA-09 — Negative cash needs a non-misleading report state

**Severity:** Major  
**Evidence:** PR #11's high-CapEx diagnostic has negative calculated closing cash and an algebraically zero Balance Sheet residual. The PR's updated model-convention proposal correctly states that negative diagnostic cash is not a valid reportable cash asset.

**Risk:** A mathematically reconciled schedule could be presented as a feasible Balance Sheet even though it has no approved source of funding.

**Required disposition:** Keep calculated diagnostic cash and positive funding-shortfall amount visible; mark Scenario Feasibility FAIL. Do not publish negative diagnostic cash as an ordinary cash asset, force it to zero, or create an undisclosed overdraft. The final output contract must distinguish calculation completion, structural integrity and financial-statement reportability.

### EA-10 — Model-run provenance and calculation lineage are not implemented

**Severity:** Major  
**Evidence:** The current public result is an input ValidationReport. There is no production ModelRun manifest, canonical input hash, engine/model-policy fingerprint, formula lineage or output hash.

**Risk:** A reviewer cannot prove which input, model version, numeric policy and engine build produced an output or trace a reported metric to its assumptions and formula.

**Required disposition:** Add a deterministic run manifest and a metric-level formula/assumption lineage contract. Keep timestamps and environment details outside the deterministic canonical result hash where they would otherwise destroy repeatability.

### EA-11 — XLSX export and reconciliation are not implemented

**Severity:** Major  
**Evidence:** The architecture defines the reporting boundary, but the current CLI exposes only `validate`; no production workbook generator exists.

**Risk:** A workbook developed before canonical output semantics are fixed can introduce a second calculation engine or imply precision Excel cannot retain.

**Required disposition:** Generate all workbook values from a completed Rust ModelRun. Do not implement financial formulas in Excel. Publish a display policy, preserve canonical JSON/CSV, and test every displayed statement and schedule against canonical outputs.

### EA-12 — Test depth is not yet sufficient for a forecast model

**Severity:** Major  
**Evidence:** The production semantic validation integration suite has 28 ordinary tests. The numeric POC has its own 12-test suite. The production suite does not yet contain the full forecast property/golden/export tests specified by the validation matrix.

**Risk:** Successful parsing and a small set of fixed examples do not establish invariant behaviour over the supported numeric envelope.

**Required disposition:** Add unit, integration, property-based, golden, boundary, negative, determinism and export tests. Persist shrunk failing property cases as regression fixtures. Keep expected outputs independently reviewed.

### EA-13 — CLI input handling is intentionally minimal

**Severity:** Moderate  
**Evidence at audited main baseline:** `ofrl_cli` implemented `validate <path>` and used `fs::read_to_string` without an explicit byte limit.

**Risk:** The initial CLI could allocate memory proportional to an untrusted file before JSON parsing. It also has no production run/export interface yet.

**Remediation on this enterprise-foundation branch:** the CLI now reads at most 1 MiB + 1 byte, rejects documents over 1 MiB before full allocation, rejects invalid UTF-8 and includes four focused reader regression tests. This change is not part of main until its PR is reviewed and merged.

**Remaining disposition:** Before broader input distribution, define output-path behaviour, overwrite protection, deterministic exit codes and commands for validate/run/export. Do not add a large CLI dependency solely for cosmetic features.

### EA-14 — Some main-branch documentation is stale

**Severity:** Major documentation-integrity issue  
**Evidence:** On the audited `main`, `DECISION_LOG.md` still says runtime semantic validation is pending/unimplemented, and `SYNTHETIC_REFERENCE_CASE.md` still describes schema/runtime validation as pending. PR #10 has already delivered the runtime validator.

**Risk:** The repository tells contributors two different implementation stories and can cause duplicated work or incorrect status reporting.

**Required disposition:** Correct these status statements and retain the explicit distinction between delivered historical validation and the not-yet-built forecast engine.

## 5. Numeric POC interpretation

PR #11 provides useful evidence:

- Canonical Case A compares 160 forecast metric values per numeric backend.
- Both candidates have zero mismatches at the POC's provisional two-decimal comparison boundary.
- The measured unrounded Balance Sheet residual differs between candidates.
- The (10^{18}) stress point is supported by both candidates in the tested path.
- The (10^{22}) stress point produces checked overflow in rust_decimal and completes in the tested BigDecimal context.
- The high-CapEx case demonstrates a funding shortfall without adding automatic debt.

These observations are not production selection criteria by themselves. They do not establish throughput, memory cost, exact overflow threshold, full dependency acceptability, real-world company coverage, formula correctness or five-year accumulated materiality.

## 6. Release-blocking decision register

| Gate | Required evidence | Current disposition |
|---|---|---|
| Numeric operating envelope | Approved amount/rate/day bounds, intermediate headroom and scale policy | OPEN |
| Numeric backend | Comparative range, precision, performance, memory and dependency decision | OPEN |
| Error budget | Formula-chain propagation and approved monetary tolerance by purpose | OPEN |
| Rust SBOM/security/licences | Machine-readable inventories and reviewed exceptions | OPEN |
| Independent model oracle | Five-year independently derived benchmark and formula review | OPEN |
| Stable diagnostic contract | Typed errors, field paths and regression coverage | OPEN |
| Forecast engine | Typed production schedules, statements and calculation results | NOT STARTED |
| Forecast validation | SI/SF checks over all five periods and failure scenarios | NOT STARTED |
| Canonical run manifest | Input/model/policy/engine identity and deterministic fingerprints | NOT STARTED |
| XLSX output | Generated from canonical outputs and reconciled | NOT STARTED |
| Release reproduction | Documented clean-environment reproduction and review | NOT STARTED |

## 7. Audit conclusion

The project is ready for controlled engineering preparation, not for an un-gated production forecast implementation.

The preferred sequence is:

1. Correct the main-branch documentation drift.
2. Ratify the numeric operating envelope and error budget.
3. Complete the Rust dependency/SBOM/licence/security review.
4. Complete the independent multi-period formula benchmark.
5. Finalize stable typed diagnostics and the ModelRun output contract.
6. Implement the canonical Rust forecast engine.
7. Add structural-integrity and scenario-feasibility validation.
8. Generate and reconcile the analyst-facing workbook.
9. Complete release reproducibility and independent review.

No audit finding authorizes changes outside OFRL-IB-001. A successful build or isolated POC must not be represented as production readiness.
