<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Implementation Roadmap

**Roadmap version:** 0.3.0  
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
| Repository policy and branch controls | Implemented on main | Active Ruleset; six exact required contexts; always-running repository-policy workflow |
| Portfolio registry and data controls | Implemented on main | Project manifest validation, data manifest and risk-tier controls |
| OFRL project workflows | Implemented on main | Rust 1.85.0 / 1.99.1 checks run for all PRs without path filters |
| Decimal POC lockfile | Implemented on main | Isolated POC lockfile and locked metadata/tree/Clippy/test/doc commands |
| CLI SBOM/checksum preview | Implemented on main | Preview-only CycloneDX 1.5 artifact for Linux CLI validator; not an official forecast-model release |
| Rust CLI input hardening | Implemented on PR #13 branch | 1 MiB bound, max-plus-one read, UTF-8 rejection and regression tests; not merged |
| Full-formula numeric candidate comparison | Open PR #11, Draft | rust_decimal, BigDecimal and Exact Rational research path |
| Independent multi-period expected values | Open PR #11, Draft | BM-002 160 values + BM-003 640 values = 800 exact outputs |
| Independent Python reference | Open PR #11, Draft | Standard-library Decimal/Fraction; verifies fixture identity and goldens |
| Rust SBOM/RustSec research | Open PR #11, Draft | Candidate/POC inventories and point-in-time advisory evidence |
| Runtime-cost observation harness | Open PR #11, Draft; initial timing CI passed | Non-gating synthetic observations; memory, allocation and production workload evidence remain open |
| Schema validator dependency lock | Open PR #12, Draft | Pinned AJV / Node toolchain and lockfile; shared contract parity is a separate gate |
| Shared Schema/Rust corpus | Open PR #13 branch | Versioned positive/negative fixtures, expected schema/runtime outcomes and diagnostic codes; code baseline passed both Rust toolchains; PR head must be synced with current main |
| Numeric Profile worksheet and symbolic derivation | Open PR #13 branch | Formula-chain envelope documented symbolically; unbounded Model Owner inputs remain TBD |
| Model-specific Numeric Profile | Not approved | Finite input bounds, headroom, error budgets, tolerance and rounding remain open |
| Production numeric backend | Not selected | Candidate evidence is not an adoption decision |
| Independent finance/accounting sign-off | Not evidenced | Arithmetic benchmarks are not an accounting opinion |
| Production forecast engine | Not started | POC is isolated and not the production library |
| Forecast SI/SF validation | Not implemented | No production forecast results |
| Canonical ModelRun and provenance | Not implemented | No run manifest or canonical result hash |
| XLSX generation/reconciliation | Not implemented | No workbook exporter |

**Branch interpretation:** PR #14 was merged to main on 2026-11-04 after PR #18, #20 and #22 were integrated in sequence. PR #11, #12 and #13 remain Draft and unmerged. PR #13's contents are branch-local and must not be represented as merged production capability.

## 3. Delivery phases

### Phase 0 — Input and historical-validation foundation

**Status:** Delivered on main.

Evidence includes the versioned input schema, deterministic synthetic Case A, Rust typed historical validator, production Cargo.lock and two-toolchain CI.

Exit condition: malformed and historically inconsistent input is rejected before a future calculation stage.

### Phase 1 — Audit, architecture and input-bound hardening

**Status:** Implemented on the PR #13 branch; its code baseline passed all five project checks before governance integration. PR #13 remains Draft and behind current `main`; merge-base synchronization and six-context CI on its refreshed head remain pending.

Deliverables:
- Engineering Audit with severity-ranked findings.
- System Design and result/status contracts.
- This gated roadmap.
- Corrected project status and reference-case documentation.
- Added a versioned shared Schema/Rust contract corpus with expected acceptance states and diagnostic codes; metadata, fixture containment, path traversal and symlink checks are implemented and covered by CI.
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

The worksheet is `NUMERIC_PROFILE_WORKSHEET.md`. The symbolic formula-chain derivation is `NUMERIC_PROFILE_DERIVATION.md`. It identifies the currently unbounded inputs and derives maximum-path formulas once Model Owner bounds are supplied. Both documents intentionally preserve unresolved fields rather than fabricate limits.

**Current domain finding:** the V1 conventions provide no finite upper bound for revenue growth, COGS / revenue, Opex / revenue, DSO/DIO/DPO, CapEx / revenue, debt drawdowns or interest rate. A finite five-year numeric envelope cannot be certified until these bounds and absolute opening-balance limits are approved.

#### Workstream B — Production backend decision

- Evaluate candidates only against the accepted Numeric Profile.
- Review checked arithmetic semantics, serialization and parse/format fidelity.
- Make BigDecimal context explicit and deterministic if it remains a candidate.
- Initial CI timing observations now exist: BigDecimal was 1.50×–2.21× the fixed Decimal candidate in the measured synthetic cases; Exact Rational was 7.52×–10.30×. See the artifacts and limitations in `poc/model-numeric-envelope/BENCHMARKS_PERFORMANCE.md` on PR #11.
- Measure wall time, memory, allocation behaviour and operational resource limits in a controlled environment beyond the single shared-runner observation.
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

The PR #11 workflow provides candidate/POC inventory and point-in-time advisory evidence. The isolated Decimal Contract POC lockfile, locked CI and preview-only CLI SBOM/checksum workflow are now on main through PRs #20/#22. They improve dependency reproducibility and artifact inventory but do not close source-level legal review, advisory disposition, multi-platform release evidence or official-release authorization. The yanked `xml-rs 0.8.19` preview-generator exception remains time-bounded and must be re-reviewed before official release.

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
- Complete review and CI acceptance of the shared JSON Schema/Rust contract corpus introduced on PR #13.
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
| Repository foundation, portfolio controls, dependency lock and CLI SBOM preview | PR #14, with #18 → #20 → #22 integrated | MERGED to main as `cc2d01b`; post-merge Ruleset and workflow read-back completed |
| Documentation-only Ruleset smoke test | PR #24 | Closed without merge after all six required contexts passed |
| Project-code Ruleset smoke test | PR #27 | Closed without merge after all six required contexts passed |
| Numeric candidate and independent arithmetic research | PR #11, Draft, targets main | POC checks pass; findings reviewed; production adoption remains separate |
| Schema tooling lockfile | PR #12, Draft, targets main | Review and CI pass; shared corpus remains a separate contract gate |
| Audit, system design, bounded CLI, shared corpus and Numeric Profile derivation | PR #13, Draft, targets main | Sync with current main; run all six required contexts on refreshed head; complete review |
| Numeric Profile/backend decision | New OFRL-only PR | Finite input domain, error budgets, resource and Finance/Accounting gates accepted |
| Production forecast engine | New OFRL-only PR | Phase 2 exit accepted |
| ModelRun/provenance/CLI run | New OFRL-only PR | Production calculations and invariants accepted |
| XLSX reporting | New OFRL-only PR | ModelRun contract stable and reconciled |

**Governance status:** the active Ruleset still requires the six exact contexts. After PR #14 reached main, both a documentation-only PR (#24) and a project-code PR (#27) reported and passed all six contexts. Both smoke PRs were closed without merging. Do not remove or relax any required check.

PR #13 remains Draft and its branch has not yet incorporated the governance merge commit. Refresh that branch and rerun its code and contract suite against the actual current base before any ready-for-review transition.

Keep research, production formula and reporting changes in separately reviewable PRs. A Draft PR remains Draft when project-level gates are unresolved, even if its CI is green. No PR is merged by this roadmap without the required review.

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
