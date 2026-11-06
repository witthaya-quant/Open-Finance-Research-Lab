<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Implementation Roadmap

**Roadmap version:** 0.9.6  
**Status:** Canonical project delivery register; execution remains gated by recorded decisions  
**Audit baseline:** main at cdbf345c6e27f1cd82f3419b164eb4a65575c09b  
**Date:** 2026-11-06

## 1. Delivery principles

Deliver in small, reviewable increments. Each increment must preserve a reproducible repository state, have a bounded scope, and distinguish research evidence from production capability.

The V1 scope remains one generic non-financial operating company, three annual historical periods, five annual forecast periods, one reporting currency and one model-level unit scale. The first release does not include financial institutions, consolidation, multi-currency translation, statutory reporting claims, DCF/LBO valuation or automatic funding mechanisms.

## 1.1 Authority and single-source-of-truth rules

**This file is the sole project-level execution source of truth for OFRL-IB-001.** It owns current workstream status, phase gates, stable task IDs, dependencies, blockers and next permitted actions.

| Information | Authoritative location | Must not be duplicated in |
|---|---|---|
| Project scope, phases, task status, blockers and next actions | This Implementation Roadmap | A second roadmap, dashboard, task board, chat handoff file or copied Issue checklist |
| Project identity, lifecycle, risk, maturity and release status | project.json and root PROJECT_CATALOG.md | A second manifest or informal status table |
| Model behaviour, formulas and exclusions | RESEARCH_SPECIFICATION.md and MODEL_CONVENTIONS.md | Roadmap prose or code comments redefining financial policy |
| Engineering design and component boundaries | ENGINEERING_ARCHITECTURE.md and SYSTEM_DESIGN.md | Ad hoc architecture files or duplicate diagrams |
| Accepted/superseded decisions and rationale | DECISION_LOG.md | Roadmap task descriptions or PR summaries as historical authority |
| Acceptance criteria and validation evidence | VALIDATION_MATRIX.md and linked workflow/test evidence | A parallel test tracker |
| Model Owner numeric inputs awaiting approval | NUMERIC_PROFILE_WORKSHEET.md and Issue #28 | Fixtures, benchmark values, stress-test values or chat guesses |
| Audit findings and residual controls | ENGINEERING_AUDIT.md | A second findings register |
| IB-001 dependency source, licence, notice and advisory evidence | DEPENDENCY_REVIEW.md | A duplicate task board or portfolio-wide findings register |

GitHub Issues and Pull Requests provide discussion, review and implementation evidence. They do not replace this task register. README is navigation and a concise public status summary, not a competing roadmap.

**File and directory control:** do not create another roadmap, task register, status dashboard, agent handoff summary, folder, crate, workflow, benchmark or schema unless a stable task ID or approved Decision Log entry authorizes the exact path and explains why existing locations cannot serve the purpose. Update the existing owner document instead of creating a second copy. Every agent must read this file first, verify live Git state and task dependencies, and update this register through a reviewable change before handoff.

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
| Rust CLI input hardening | Implemented on main through PR #13 | 1 MiB bound, max-plus-one read, UTF-8 rejection and regression tests; source head `17b77e3`, merge `593d04d` |
| Full-formula numeric candidate comparison | PR #11 open Draft; branch stale/diverged | Old head 120ccd3414fb7018011ea3991800728f86736319; compare main...head was 77 ahead / 371 behind. Its eight checks passed only on that historical SHA. Salvage required files into a fresh main-based PR; do not merge this branch directly. |
| Independent multi-period expected values | PR #11 branch only; not on main | BM-002/BM-003 claims total 800 exact outputs. Associated POC/golden files are not in the main project tree; treat as unmerged evidence until selectively reviewed and revalidated. |
| Independent Python reference | PR #11 branch only; not on main | Its exact-rational reference is described on the unmerged branch; scope and independence must be re-audited before relying on results. |
| Rust SBOM/RustSec research | PR #11 branch only; not on main | Candidate inventory/advisory snapshots require freshness review and do not satisfy official release review. |
| Runtime-cost observation harness | PR #11 branch only; not on main | Single-run synthetic timing is directional only; memory, allocation, tail-latency, concurrency and representative workload evidence remain open. |
| Schema validator dependency lock | Implemented on main; PR #12 closed unmerged | package.json, package-lock.json and schema-validation shell script are on main. PR #12 was closed without merge because the core file blobs matched main exactly. Issue #9 is closed completed. |
| Shared Schema/Rust corpus | Implemented on main through PR #13 | Versioned positive/negative fixtures, expected schema/runtime outcomes, diagnostic codes and fixture path-safety checks; six required contexts passed on source head `17b77e3` |
| Typed deserialization diagnostics | Implemented on main through PR #13 | Contract-owned preflight error kinds, shared Decimal parser, RFC 6901 pointers, escaped path segments, array indices, period context and nine versioned mutation fixtures; source head `17b77e3` passed full CI |
| Numeric Profile schema, worksheet and exact derivation tool | Implemented on main through PR #13 | Closed profile schema, exact BigInt-rational derivation and division-error propagation; test-only fixture emits 287 arithmetic nodes; no owner-approved profile values |
| Candidate-specific capacity evaluator | Implemented on main through PR #13 | Exact rust_decimal 96-bit coefficient / scale 0–28 screen and BigDecimal profile-ceiling screen; test-only fixture yields rust_decimal `STATIC_BOUND_EXCEEDS_LIMIT` (37 nodes) and BigDecimal CONDITIONAL (50 nodes); not a runtime verdict or production decision |
| Model-specific Numeric Profile | Not approved | Finite input bounds, headroom, error budgets, tolerance and rounding remain open |
| Production numeric backend | Not selected | Candidate evidence is not an adoption decision |
| Independent finance/accounting sign-off | Not evidenced | Arithmetic benchmarks are not an accounting opinion |
| Production forecast engine | Not started | POC is isolated and not the production library |
| Forecast SI/SF validation | Not implemented | No production forecast results |
| Canonical ModelRun and provenance | Not implemented | No run manifest or canonical result hash |
| XLSX generation/reconciliation | Not implemented | No workbook exporter |

**Live branch interpretation (2026-11-06):** PR #14 and PR #13 are merged. PR #13 merged as 593d04d125522e5ec42bea06da6b94c3c2abf604 from validated source head 17b77e385f9980ff00c6ab1466361a54a5d34d23; PR #30 subsequently advanced main to 5f8f578c387150214a1f438c3d67da48652eb4cb. PR #11 remains Draft but is 77 commits ahead / 371 behind main at its old head; its CI is historical, not current main evidence. PR #12 is closed without merge because its core tooling is already on main. No PR #11 branch result is represented as a main capability. The PR #13 merge does not approve a Numeric Profile, select a backend or authorize the Forecast Engine.

## 3. Delivery phases

### Phase 0 — Input and historical-validation foundation

**Status:** Delivered on main.

Evidence includes the versioned input schema, deterministic synthetic Case A, Rust typed historical validator, production Cargo.lock and two-toolchain CI.

Exit condition: malformed and historically inconsistent input is rejected before a future calculation stage.

### Phase 1 — Audit, architecture and input-bound hardening

**Status:** Delivered and merged to `main` through PR #13 at `593d04d125522e5ec42bea06da6b94c3c2abf604`. The validated source head `17b77e385f9980ff00c6ab1466361a54a5d34d23` passed all six required Ruleset contexts, Numeric Envelope derivation and Release Candidate Preview (runs `41980695313`, `41980695585`, `41980695359`, `41980695504`). This phase delivers the audit, design, roadmap, bounded CLI reader, shared Schema/Rust corpus, typed diagnostics and Numeric Profile decision tooling. Numeric Profile owner decisions remain open and continue to gate backend selection and production forecast-engine authorization.

Deliverables:
- Engineering Audit with severity-ranked findings.
- System Design and result/status contracts.
- This gated roadmap.
- Corrected project status and reference-case documentation.
- Added a versioned shared Schema/Rust contract corpus with expected acceptance states and diagnostic codes; metadata, fixture containment, path traversal and symlink checks are implemented and covered by CI.
- Replaced Serde-message substring classification with a typed structural preflight and shared Decimal parser. RFC 6901 pointers cover nested fields, escaped object keys, array indices and period context; tests also cover root type errors and pathless malformed JSON. The semantic-validator matrix passed on Rust 1.85.0 and 1.99.1 on source head `17b77e3`; the implementation is merged to `main` and future contract changes must preserve these guarantees.
- CLI input bound and UTF-8 rejection tests.

Exit conditions:
- Documentation accurately separates main-branch capability from open-PR evidence.
- One Rust calculation authority is preserved.
- No numeric backend is selected by assumption.
- Input size and encoding boundary is explicit.
- No unrelated project or portfolio scope is changed. Shared contract fixtures and the schema-validation script are included only as integration surfaces required to test the OFRL-IB-001 contract across AJV and Rust.

### Phase 2 — Numeric, dependency and model-policy acceptance

**Status:** Derivation and test tooling implemented and merged to `main` through PR #13; Model Owner, Finance/Accounting and Engineering Numeric Profile decisions remain OPEN.

#### Workstream A — Numeric Profile

- Approve finite minimum/maximum monetary input and calculated-balance values by model unit and field class.
- Approve source integer/fractional-digit limits separately for monetary values, rates/ratios and days; retain canonical lexical rules.
- Set finite rate, day and operating-assumption bounds without accidentally excluding valid loss-making cases.
- Derive five-year maximum magnitude bounds at every formula node, including multiplication before division.
- Derive a separate pre-normalization scale graph and coefficient-digit envelope for every material formula node and recurrence.
- Prove representation headroom, scale behaviour and overflow/reduction rejection at exact boundaries and just outside them.
- Define deterministic division precision/context, rounding semantics and canonical output formatting.
- Quantify worst-case five-year arithmetic-error propagation against exact-rational results and distinguish it from accounting and schedule residuals.
- Define separate budgets for arithmetic approximation, accounting identity, schedule roll-forward and display/export.
- Translate each budget into reporting-currency amounts at approved domain endpoints.
- Set finite CPU, memory, allocation, concurrency, input-size and run-resource limits.
- Obtain Finance/Accounting and Engineering review; version the final policy.

The worksheet is `NUMERIC_PROFILE_WORKSHEET.md`. The symbolic formula-chain and scale derivation is `NUMERIC_PROFILE_DERIVATION.md`. It identifies currently unbounded inputs and derives maximum-path formulas once Model Owner bounds are supplied. It deliberately preserves TBD values rather than fabricate limits.

The main-branch derivation tool (tools/numeric-envelope/derive.cjs), versioned input schema and candidate-capacity evaluator were merged through PR #13. The validated source-head artifact contains 287 arithmetic nodes and is explicitly test-only. On that synthetic profile, the static evaluator reports 37 rust_decimal nodes with conservative bounds above the 96-bit coefficient ceiling (including 19 scale-related reasons that overlap coefficient reasons) and 50 BigDecimal nodes conditional on unresolved division policy. These are static screening outputs, not runtime overflow observations, intended business-domain limits or candidate selection. The older 474-bound-count statement belonged to an earlier derivation revision and is not current approval evidence. The separate 800-output BM-002/BM-003 research and Python reference remain on unmerged PR #11.

**Current domain finding:** V1 has no finite upper bound for revenue growth, COGS / revenue, Opex / revenue, DSO/DIO/DPO, CapEx / revenue, debt drawdowns or interest rate. The current Decimal parser ceiling of 28 fractional digits is only a source-field grammar limit; it is not proof that five forecast years of multiplications preserve that scale. A finite magnitude, scale and coefficient envelope cannot be certified until the owner approves finite bounds for all relevant drivers and opening/historical amounts.

#### Workstream B — Production backend decision

Use [`NUMERIC_BACKEND_EVALUATION.md`](NUMERIC_BACKEND_EVALUATION.md) as the candidate acceptance and measurement protocol. Candidates must pass all hard gates; a weighted score cannot compensate for a correctness, domain, determinism, resource or dependency failure.

- Evaluate candidates only against the accepted magnitude, scale/coefficient and resource profile; no candidate can be selected before Workstream A closes.
- Review checked arithmetic, scale reduction, operation-specific rounding, parse/format fidelity and canonical serialization.
- Use exact coefficient-integer upper bounds at each declared intermediate scale; compare the resulting integer directly with the exact fixed-width mantissa limit. Digit count alone is not a capacity proof.
- The merged candidate screen checks `rust_decimal 1.43.0` against scale 0–28 and maximum coefficient `79228162514264337593543950335`; the 30-test suite covers at, above and below the mantissa boundary. Static screening is not runtime representability proof.
- A test-only profile currently yields `rust_decimal` `STATIC_BOUND_EXCEEDS_LIMIT` on 37/287 arithmetic nodes: 37 conservative coefficient upper bounds exceed the 96-bit maximum and 19 exceed scale 28 (the sets overlap). This means capacity is not proven for those conservative bounds; it is not an observed runtime overflow and does not establish that every feasible input fails. It does not establish the intended V1 business-domain decision.
- BigDecimal reports CONDITIONAL on 50 nodes because the test-only profile leaves division scale unresolved. Its 80-significant-digit compile-time POC context is not equivalent to the Numeric Profile's fixed fractional-scale division policy. Define an explicit run-scoped adapter or deliberately version the profile's precision semantics; do not assume equivalence.
- Bound BigDecimal multiplication coefficient growth independently of division precision, and prove resource behaviour with approved limits.
- Keep Exact Rational as a correctness oracle unless a separate operational resource case justifies evaluating it as a production candidate.
- Treat the 1.50×–2.21× and 7.52×–10.30× timing observations mentioned in PR #11 as historical, non-gating synthetic observations on an unmerged branch only. They are not main-branch evidence and are not production workload measurements. Do not use them to select a backend.
- Measure latency, throughput, allocations, peak memory/RSS, tail latency, concurrency and long-running process behaviour under a controlled workload generated from the approved profile.
- Measure migration cost because production input/history domain types currently use `rust_decimal`.
- Review typed failure semantics, determinism, MSRV/platform support, dependency source/licence/advisory status and per-run provenance.
- Record the selected backend, explicit rejected alternatives, context, known limitations and evidence references in the Decision Log.

**No forecast implementation should begin until Workstreams A and B have accepted decisions and Workstream D has documented the required formula/accounting review. A finite magnitude bound without a finite scale/coefficient/resource bound is not a completed Numeric Profile.**

#### Workstream C — Dependency and source review

- Retain production and selected POC lockfiles or formally retire the relevant POC.
- Review direct/transitive package source, duplicate versions, licences, notices and maintenance.
- Triage advisory findings from the release-date database.
- Approve an explicit licence/exception policy.
- Record source obligations and distribution-notice requirements.
- Preserve SBOMs, checksums and exact source/build identity.

PR #11's dependency inventory and advisory results remain on its unmerged branch and are point-in-time evidence only. The isolated Decimal Contract POC lockfile and locked CI, plus the preview-only CLI SBOM/checksum workflow, are on main through PRs #20/#22. The main preview pipeline does not close source-level licence/notice review, advisory disposition, multi-platform release evidence or official-release authorization. Issue #16 remains open for these release gates. The yanked xml-rs 0.8.19 preview-generator exception is time-bounded and must be reviewed before official release.

PR #34 adds a locked source-archive inventory and current-head RustSec audit workflow, plus the project-scoped dependency review record. After merge, the workflow will repeat source and RustSec review weekly on the default branch (Monday 06:17 UTC) and remains manually dispatchable; it does not add a required Ruleset context. Corrected schema 1.2.0 workflow run [42186000627](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/42186000627) passed on PR source head `a47b68c808cb996f932f0fc475e85807f87e1fae`: 107/107 locked registry archives matched Cargo.lock SHA-256, 107/107 source manifest metadata cross-checks matched, and all 107 package records had licence-text candidates under filename/embedded-heading detection. `r-efi 5.3.0` was correctly detected through four licence headings embedded in `AUTHORS`. Both committed lockfiles passed RustSec with 0 vulnerabilities / 0 warnings against DB commit `26fd2d43f7280a1fb925a3743ec463017afe7119` (2026-11-05T09:50:49+01:00). Artifact ID `13487721458`, archive SHA-256 `5b72fefda65438758c115c04b8bde13d6fab4ceea0cf791a4792461ecc604d23`. This is evidence from an unmerged PR head, not main. All 107 human licence/source reviews remain PENDING; the preview-generator exception and official-release gates remain open. The dependency review record is the specialist evidence source for IB-T005; this roadmap remains the sole task/status register.

**Main-branch baseline verified 2026-11-06 (`cdbf345c6e27f1cd82f3419b164eb4a65575c09b`):** the application lockfile contains 54 package records (52 registry, two local) and no Git sources; the separate Decimal POC lockfile contains 56 records (55 registry, one local). The application lockfile blob is identical to the one used by the PR #22 preview run, but that run was triggered by `pull_request` on source `3ca98dd26de97098f68fa0ca45b92fbb466f1d19`, not by the current `main` commit. The four workflows on `main` do not currently run `cargo audit` / RustSec. PR #11's scan remains unmerged and cannot satisfy the current advisory gate. Dependabot schedules weekly updates for Cargo, GitHub Actions and npm; this is update automation rather than advisory or licence clearance. Exact source-archive review and package-level licence/notice dispositions remain NOT COMPLETE.


#### Workstream D — Independent finance and accounting review

- Review the V1 model purpose, scope, classifications, sign conventions and equations.
- Check working-capital mechanics and day-count convention.
- Check aggregate PP&E depreciation cap and roll-forward.
- Check average-debt interest timing and debt movement.
- Check tax/dividend floors and same-period cash settlement.
- Check negative cash, funding shortfall, structural integrity and reportability.
- Record reviewer scope, findings, decisions and unresolved limitations.

PR #11 describes BM-002/BM-003 with 800 exact expected arithmetic outputs on its unmerged branch. Even after independent revalidation and main integration, arithmetic goldens are not a substitute for this finance/accounting review.

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
- Maintain the merged Schema/Rust contract corpus and add regression cases whenever the versioned input contract changes.
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
| Numeric candidate and independent arithmetic research | PR #11, Draft, stale/diverged | Old head 120ccd3414fb7018011ea3991800728f86736319; 77 ahead / 371 behind main; exact-old-head checks passed historically. Do not merge directly. Selectively salvage valuable research into a fresh main-based PR after source review, then rerun all checks. |
| Schema tooling lockfile | PR #12, closed without merge | Core package.json, lockfile and validation script blobs are identical on main; Issue #9 is closed completed. The old branch was not merged. |
| Audit, system design, bounded CLI, shared corpus, typed diagnostics and Numeric Profile derivation | PR #13, merged to main | Merge `593d04d`; source head `17b77e3` passed all six required contexts, Numeric Envelope and Release Candidate Preview; Stage 0 single-maintainer disposition recorded; no independent approval claimed |
| Numeric Profile/backend decision | New OFRL-only PR | Finite input domain, error budgets, resource and Finance/Accounting gates accepted |
| Production forecast engine | New OFRL-only PR | Phase 2 exit accepted |
| ModelRun/provenance/CLI run | New OFRL-only PR | Production calculations and invariants accepted |
| XLSX reporting | New OFRL-only PR | ModelRun contract stable and reconciled |

**Governance status:** the active Ruleset still requires the six exact contexts. After PR #14 reached main, both a documentation-only PR (#24) and a project-code PR (#27) reported and passed all six contexts. Both smoke PRs were closed without merging. Do not remove or relax any required check.

PR #13 is merged to `main` at `593d04d125522e5ec42bea06da6b94c3c2abf604` from source head `17b77e385f9980ff00c6ab1466361a54a5d34d23`. All six required contexts, Numeric Envelope derivation and Release Candidate Preview passed on that exact source head. The Founder Stage 0 single-maintainer disposition is recorded in the PR conversation; it is not independent human approval. Issue #23 was closed by the PR. Numeric Profile boundaries remain a separate Model Owner gate on Issue #28 and are not approved by this merge. Do not authorize the production Forecast Engine until the required Model Owner, Finance/Accounting and Engineering decisions are recorded.

Keep research, production formula and reporting changes in separately reviewable PRs. A Draft PR remains Draft when project-level gates are unresolved, even if its CI is green. No PR is merged by this roadmap without the required review.

## 5. Required test layers

| Layer | Purpose | Required evidence |
|---|---|---|
| Unit | Formula/schedule mechanics | Reviewed expected values and edge boundaries |
| Integration | Complete model paths | All five years, three statements and supporting schedules |
| Property | Invariants over generated inputs | Reproducible seeds and minimized regression cases |
| Golden | Independent expected outputs | Versioned reviewed expected values |
| Negative | Unsupported inputs/scenarios | Stable diagnostic codes and JSON Pointer |
| Numeric | Magnitude, source/intermediate scale, exact coefficient capacity and accumulated error | Approved Numeric Profile endpoints and just-inside/just-outside tests; static candidate screen is not runtime proof |
| Contract parity | Schema vs Rust behaviour | Same positive/negative fixture corpus |
| Determinism | Same input/policy/build | Canonical output and SHA-256 reproducibility |
| Export | Workbook fidelity | Structural checks and period-by-period reconciliation |
| Dependency | Package/source risk | SBOM, advisory and licence/source disposition |
| Reproduction | Independent repeatability | Clean-checkout reproduction report |

## 6. Change control and scope

Every material change records the prior decision, new decision, rationale, affected formula IDs/inputs/outputs, version impact, required migration and tests to rerun.

Never change a tolerance after observing a failed result merely to produce PASS. Any threshold change requires a dated rationale, policy version change and complete rerun.

This roadmap authorizes work only in `projects/OFRL-IB-001/` within `witthaya-quant/Open-Finance-Research-Lab`. It does not authorize edits to other projects or unrelated repository files.


## 7. Canonical active task register

This table is the only active IB task list. Task IDs are stable. A task moves to COMPLETE only when acceptance evidence is linked and the corresponding source documents are updated. GitHub Issues track discussion and evidence; they are not copied as a second checklist.

| Task ID | Workstream | Required outcome | Current status | Dependency / next action |
|---|---|---|---|---|
| IB-T001 | Model Owner domain decision | Finite V1 bounds for operating drivers and opening/historical amounts; units, inclusivity, sign/zero policy, source rationale, integer/fractional precision for monetary/ratio/day classes; confirm retained 0–1 assumptions | **BLOCKED — Kenny / Model Owner input** | Complete the existing Numeric Profile Worksheet and Issue #28. Do not infer values from Case A, fixtures, static capacity or POC stress points. |
| IB-T002 | Finance/Accounting policy review | Review V1 equations, operating assumptions, accounting links, materiality, rounding, and separate arithmetic / accounting-identity / schedule-roll-forward / workbook display error budgets | **BLOCKED — qualified review required** | Follows T001 domain definition; record reviewer, competence/scope, evidence, findings and decision. Do not substitute AI analysis or founder notes for independent review. |
| IB-T003 | Engineering numeric envelope | Derive five-year maximum magnitudes, pre-normalization scales, coefficient bounds at every material formula node, division context/error propagation and finite resource envelope | **BLOCKED — approved input policy required** | After T001/T002 policy inputs, rerun the main-branch exact derivation, derive just-below/at/just-above boundary cases and measure runtime resources. |
| IB-T004 | Production arithmetic backend | Select or reject rust_decimal, BigDecimal or another justified candidate using correctness, representability, precision, determinism, resource, source/licence and migration gates | **NOT STARTED — gated** | T001–T003 plus Workstream D review; no candidate is selected by weighted score if any hard gate fails. |
| IB-T005 | Dependency and source acceptance | Complete package-source/provenance, licence/notice, advisory, exception and reproducible-build disposition for the eventual production dependency graph | **OPEN — PR #34 schema 1.2.0 inventory/advisory automation PASS; human acceptance pending** | Complete human package-level source/licence/notice disposition against the content-specific licensing policy, review exceptions and generator toolchain, then rerun against the approved production graph after IB-T004; official release evidence remains separate. |
| IB-T006 | Canonical Rust forecast engine | Implement operating forecast, supporting schedules and integrated IS/BS/CFS inside existing ofrl_model boundaries | **NOT STARTED — blocked by Phase 2 exit** | No new crate or alternate formula engine. Implement only after T001–T004 and T002 review are accepted. |
| IB-T007 | Forecast validation | Structural Integrity and Scenario Feasibility, formula coverage, edge/boundary/property tests, no-plug and funding-shortfall behaviour | **NOT STARTED — depends on T006** | Use approved numeric policy and the existing Validation Matrix after its tolerance gates are versioned and approved. |
| IB-T008 | ModelRun, provenance and CLI | Deterministic canonical result contract, SHA-256 input/output fingerprints, policy/backend/engine identity, formula lineage and safe atomic writes | **NOT STARTED — depends on T006/T007** | Extend existing ofrl_cli; preserve distinct validation, calculation, integrity and feasibility outcomes. |
| IB-T009 | Analyst-facing XLSX | Generate the workbook from canonical ModelRun only and reconcile every material displayed value | **NOT STARTED — depends on T008** | Do not create spreadsheet formulas that become a second calculation authority. |
| IB-T010 | Release and independent reproduction | Complete R2 release evidence, clean-checkout reproduction, final SBOM/checksum, licence/data review, known limitations, independent domain review and authorized publication | **NOT STARTED — depends on T005–T009 and the IB-specific R2 assessment** | The repository-level risk-tier release control is delivered through PR #32 / Issue #19. This task is still open until OFRL-IB-001's own required gates, independent financial-model review, residual-risk disposition, exact-source release record and Founder authorization are evidenced. A preview artifact is not a release. |

### 7.1 Phase exit criteria

- **Phase 2 exit:** T001–T005 relevant decisions and evidence accepted, including Model Owner domain, Finance/Accounting review, Engineering scale/coefficient/resource proof and explicit backend decision. No Phase 3 implementation before this exit.
- **Phase 3 exit:** complete five-year deterministic three-statement engine with reviewed formulas, hard-gate Structural Integrity and separate Scenario Feasibility results.
- **Phase 4 exit:** canonical ModelRun and CLI determinism/provenance are proven from a clean checkout.
- **Phase 5 exit:** workbook is generated solely from ModelRun and reconciles against canonical output under the approved display policy.
- **Phase 6 exit:** release readiness, dependency/data/IP/security review, R2 domain assurance, reproduction and Founder publication authorization are recorded. Evidence maturity must reflect what was demonstrated; Level 4 requires actual independent replication, not merely release.

## 8. Agent continuation contract

Every new chat, code agent or human contributor must:
1. Read this file first, then read the task's specialist source documents.
2. Fetch current main, active PR metadata, branch head, compare counts and exact-head checks before changing files. Prior GitHub snapshots are historical until re-fetched.
3. Receive one or more existing Task IDs from §7; no task ID means no implementation authorization.
4. Record exact repository, branch, starting SHA, allowed paths, objective, acceptance criteria and required test commands in the task/PR description.
5. Keep changes within authorized existing project paths. Do not create parallel folders, roadmaps or duplicate status documents.
6. Preserve failing, blocked, skipped and not-applicable states. A green workflow does not prove financial correctness.
7. Update this roadmap and the relevant specialist document in the same reviewed change when status or decisions change.
8. Leave unresolved Model Owner, Finance/Accounting, Engineering or release approvals to their actual decision owners.

An agent handoff is complete only when another agent can continue from the task ID, source-of-truth links, exact commit, evidence and next action without relying on this chat transcript.
