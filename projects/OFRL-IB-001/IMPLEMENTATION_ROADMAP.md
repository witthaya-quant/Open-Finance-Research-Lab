<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Implementation Roadmap

**Roadmap version:** 0.2.0  
**Status:** Proposed execution plan with evidence-linked gates  
**Date:** 2026-11-04

## 1. Delivery principles

Deliver in small, reviewable increments. Each increment must preserve a reproducible repository state, have a bounded scope, and distinguish research evidence from production capability.

The V1 scope remains one generic non-financial operating company, three annual historical periods, five annual forecast periods, one reporting currency and one model-level unit scale. The first release does not include financial institutions, consolidation, multi-currency translation, statutory reporting claims, DCF/LBO valuation or automatic funding mechanisms.

## 2. Current implementation baseline

| Capability | Status | Evidence / boundary |
|---|---|---|
| Versioned JSON input contract | Implemented on main | Draft 2020-12 schema and closed objects |
| Synthetic Case A input | Implemented on main | Canonical three-history/five-forecast fixture |
| Rust historical semantic validator | Implemented on main | Opening/historical statement and roll-forward checks |
| Rust CLI input hardening | Implemented in open PR #13 | 1 MiB bound, max-plus-one read, UTF-8 rejection and tests; not on main |
| Decimal string interchange POC | Implemented in prior work | Isolated proof of boundary behaviour |
| Full-formula numeric candidate comparison | Implemented in open PR #11 | rust_decimal, BigDecimal and Exact Rational research path |
| Independent multi-period expected values | Implemented in open PR #11 | BM-002 160 values + BM-003 640 values = 800 exact outputs |
| Independent Python reference | Implemented in open PR #11 | Standard-library Decimal/Fraction; verifies fixture identity and goldens |
| Rust SBOM/RustSec automation | Implemented in open PR #11 | CycloneDX + advisory scan; one point-in-time scan reports zero findings |
| Runtime-cost observation harness | Implemented in latest open PR #11 commit | Non-gating release-mode timing; latest CI/artifact result pending |
| Schema validator dependency lock | Implemented in open PR #12 | AJV / Node toolchain lock; shared Rust/schema corpus still open |
| Model-specific Numeric Profile | Not approved | Bounds, headroom, error budget, tolerance, rounding remain open |
| Production numeric backend | Not selected | Candidate evidence is not an adoption decision |
| Independent finance/accounting sign-off | Not evidenced | Arithmetic benchmark is not an accounting opinion |
| Production forecast engine | Not started | POC is isolated and not the production library |
| Forecast SI/SF validation | Not implemented | No production forecast results |
| Canonical ModelRun and provenance | Not implemented | No run manifest or canonical result hash |
| XLSX generation/reconciliation | Not implemented | No workbook exporter |

**Branch interpretation:** PR #11, #12 and #13 are open Draft changes. Their code and CI evidence are not merged production capabilities. Main-branch documentation and status claims must retain this distinction.

## 3. Delivery phases

### Phase 0 — Input and historical-validation foundation

**Status:** Delivered on main.

Evidence includes the versioned input schema, deterministic synthetic Case A, Rust typed historical validator, production Cargo.lock and two-toolchain CI.

Exit condition: malformed and historically inconsistent input is rejected before a future calculation stage.

### Phase 1 — Audit, architecture and input-bound hardening

**Status:** Implemented in open PR #13; review and merge remain pending.

Deliverables:
- Engineering Audit with severity-ranked findings.
- System Design and result/status contracts.
- This gated roadmap.
- Corrected project status and reference-case documentation.
- CLI input bound and UTF-8 rejection tests.

Exit conditions:
- Documentation accurately separates main-branch capability from open-PR evidence.
- One Rust calculation authority is preserved.
- No numeric backend is selected by assumption.
- Input size and encoding boundary is explicit.
- No files outside OFRL-IB-001 are changed.

### Phase 2 — Numeric, dependency and model-policy acceptance

**Status:** Partially evidenced; approval gates remain OPEN.

#### Workstream A — Numeric Profile

- Set approved minimum/maximum monetary input values by model unit.
- Set maximum source fractional scale and accepted lexical form.
- Set rate, day and operating assumption bounds.
- Calculate intermediate bounds for every material formula chain, including multiplication before division.
- Establish representation headroom and overflow rejection behaviour.
- Define deterministic division precision/context and rounding semantics.
- Quantify worst-case five-year propagation against exact rational results.
- Define distinct budgets for arithmetic approximation, accounting identity, schedule roll-forward and workbook display.
- Translate tolerances into reporting currency amounts at the supported range endpoints.
- Obtain Finance/Accounting and engineering approval; version the final policy.

The worksheet is `NUMERIC_PROFILE_WORKSHEET.md`. It intentionally contains unresolved fields rather than fabricated limits.

#### Workstream B — Production backend decision

- Evaluate candidates only against the accepted Numeric Profile.
- Review checked arithmetic semantics, serialization and parse/format fidelity.
- Make BigDecimal context explicit and deterministic if it remains a candidate.
- Measure wall time, memory, allocation behaviour and operational resource limits in a controlled environment.
- Consider production validator migration cost because its current domain types use rust_decimal.
- Review error types, determinism, support/MSRV and dependency graph.
- Record the selected backend and rejected alternatives in the Decision Log.

**No forecast implementation should begin until Workstreams A and B have accepted decisions.**

#### Workstream C — Dependency and source review

- Retain production and selected POC lockfiles or formally retire the relevant POC.
- Review direct/transitive package source, duplicate versions, licences, notices and maintenance.
- Triage advisory findings from the release-date database.
- Approve an explicit licence/exception policy.
- Record source obligations and distribution-notice requirements.
- Preserve SBOMs, checksums and exact source/build identity.

The PR #11 workflow provides an initial inventory and advisory-scan mechanism; it does not itself close legal or source review.

#### Workstream D — Independent finance and accounting review

- Review the V1 model purpose, scope, classifications, sign conventions and equations.
- Check working-capital mechanics and day-count convention.
- Check aggregate PP&E depreciation cap and roll-forward.
- Check average-debt interest timing and debt movement.
- Check tax/dividend floors and same-period cash settlement.
- Check negative cash, funding shortfall, structural integrity and reportability.
- Record reviewer scope, findings, decisions and unresolved limitations.

BM-002 and BM-003 give 800 exact expected arithmetic outputs. They are not a substitute for this review.

**Phase 2 exit:** Numeric Profile, production backend, dependency/source/licence disposition and finance/accounting review are approved, versioned and linked to evidence. Any unresolved blocker keeps Phase 3 closed.

### Phase 3 — Canonical Rust forecast engine

**Status:** Not started; gated by Phase 2.

Implement inside the existing production `ofrl_model` crate:

1. Numeric/domain boundary and typed calculation errors.
2. Operating forecast.
3. Working-capital schedule.
4. Aggregate PP&E schedule.
5. Debt and interest schedule.
6. Tax, dividends and retained earnings.
7. Income Statement.
8. Cash Flow Statement.
9. Balance Sheet result and diagnostic cash.
10. Structural Integrity and Scenario Feasibility checks.
11. Formula identifiers and metric lineage.
12. Canonical ModelRun.

Acceptance:
- Every material output has a stable formula ID.
- All monetary operations use the approved arithmetic backend and checked error handling.
- No binary floating-point in authoritative calculations.
- No hidden cash, debt, equity or balancing plugs.
- Negative calculated cash is a feasibility failure and not a reportable asset.
- All formulas match independently reviewed expected values under the accepted error policy.
- Unit, integration, property, boundary and negative tests pass.

### Phase 4 — ModelRun, deterministic provenance and CLI

**Status:** Not started; depends on Phase 3.

Deliver:
- `validate`, `run`, `inspect`, and later `export` commands.
- Versioned canonical JSON/CSV output contracts.
- SHA-256 input and output digests.
- Model, schema, conventions, Numeric Profile, backend/context and engine identity.
- Formula/assumption/source lineage.
- Stable exit codes and deterministic machine-readable diagnostics.
- Atomic output writes and safe path/overwrite semantics.

Acceptance:
- Identical canonical input plus identical model/policy/engine identity produces identical canonical output and fingerprint.
- Time, host and local paths are separated from deterministic hashes.
- Invalid input, calculation failure, Structural Integrity failure and Scenario Feasibility failure are distinct statuses.

### Phase 5 — Analyst-facing XLSX

**Status:** Not started; depends on Phases 3 and 4.

Sheets:
1. Cover and Run Status.
2. Executive Summary.
3. Assumptions Register.
4. Income Statement.
5. Balance Sheet.
6. Cash Flow Statement.
7. Working Capital.
8. PP&E.
9. Debt and Interest.
10. Structural Integrity and Scenario Feasibility.
11. Diagnostics and Provenance.

Acceptance:
- Workbook is generated only from canonical ModelRun values.
- No independent forecast formulas recalculate outputs.
- Currency, scale, sign convention and precision are explicit.
- Negative diagnostic cash is presented as a funding shortfall, not a cash asset.
- Every material displayed value reconciles to canonical output.
- Canonical JSON/CSV strings remain the authoritative precision record.

### Phase 6 — Release hardening and independent reproduction

**Status:** Not started.

Deliver:
- Complete property, boundary, golden, negative and determinism test evidence.
- Shared JSON Schema/Rust contract corpus.
- Final SBOM, licence/source/advisory disposition and release artifact hashes.
- Reproducible clean-checkout build/run instructions.
- XLSX structure and display reconciliation report.
- Known-limitations and model-assumptions register.
- Independent reproduction and documented review.

Release statuses:
- Structural Integrity PASS only if every SI hard gate passes.
- Scenario Feasibility PASS only if every SF hard gate passes.
- Research Release PASS only if its release gates pass.
- A scenario can be reproducible while infeasible; it must not be presented as funded or reportable.

## 4. Pull-request sequence and merge controls

| Work item | Branch / PR | Merge/acceptance condition |
|---|---|---|
| Numeric candidate and independent arithmetic research | PR #11 | POC checks pass; findings reviewed; production adoption remains separate |
| Schema tooling lockfile | PR #12 | Review and CI pass; shared corpus follow-up remains required |
| Audit, system design, bounded CLI | PR #13 | Documentation and scope review; all changes limited to OFRL-IB-001 |
| Numeric Profile/backend decision | New OFRL-only PR | Phase 2 evidence complete |
| Production forecast engine | New OFRL-only PR | Phase 2 exit accepted |
| ModelRun/provenance/CLI run | New OFRL-only PR | Production calculations and invariants accepted |
| XLSX reporting | New OFRL-only PR | ModelRun contract stable and reconciled |

Keep research, production formula and reporting changes in separately reviewable PRs. A Draft PR remains Draft when project-level gates are unresolved, even if its defined CI scope is green. No PR is merged by this roadmap without the required review.

## 5. Required test layers

| Layer | Purpose | Required evidence |
|---|---|---|
| Unit | Formula/schedule mechanics | Reviewed expected values and edge boundaries |
| Integration | Complete model paths | All five years, three statements and supporting schedules |
| Property | Invariants over generated inputs | Reproducible seeds and minimized regression cases |
| Golden | Independent expected outputs | Versioned reviewed expected values |
| Negative | Unsupported inputs/scenarios | Stable diagnostic codes and JSON Pointer |
| Numeric | Bounds, scale and accumulated error | Approved Numeric Profile endpoints |
| Contract parity | Schema vs Rust behaviour | Same positive/negative fixture corpus |
| Determinism | Same input/policy/build | Canonical output and SHA-256 reproducibility |
| Export | Workbook fidelity | Structural checks and period-by-period reconciliation |
| Dependency | Package/source risk | SBOM, advisory and licence/source disposition |
| Reproduction | Independent repeatability | Clean-checkout reproduction report |

## 6. Change control and scope

Every material change records the prior decision, new decision, rationale, affected formula IDs/inputs/outputs, version impact, required migration and tests to rerun.

Never change a tolerance after observing a failed result merely to produce PASS. Any threshold change requires a dated rationale, policy version change and complete rerun.

This roadmap authorizes work only in `projects/OFRL-IB-001/` within `witthaya-quant/Open-Finance-Research-Lab`. It does not authorize edits to other projects or unrelated repository files.
