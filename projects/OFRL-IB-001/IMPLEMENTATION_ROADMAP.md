<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Enterprise Implementation Roadmap

**Roadmap version:** 0.1.0  
**Status:** Proposed execution plan  
**Date:** 2026-11-04

## 1. Delivery strategy

Deliver the first release in gated, reviewable increments. Each increment must leave the repository in a reproducible state and must not claim that an isolated POC is a production capability.

The plan preserves the founder-approved V1 scope. It does not expand into financial institutions, consolidation, multi-currency, statutory reporting, DCF or automated funding.

### Current baseline

| Capability | Status |
|---|---|
| V1 scope and model conventions | Defined; conventions remain subject to final implementation review |
| Versioned JSON input schema | Implemented on main |
| Synthetic Case A input | Implemented on main |
| Rust historical semantic validator | Implemented and CI-verified on main |
| Decimal JSON/CSV contract POC | Merged; isolated POC evidence |
| Full-formula numeric comparison POC | Open Draft PR #11; CI evidence on its current head |
| Schema validator pinned toolchain | Open Draft PR #12; CI evidence on its current head |
| Model-specific numeric envelope | Open |
| Rust SBOM/licence/security review | Open |
| Independent five-year financial formula benchmark | Open |
| Production forecast engine | Not started |
| Forecast structural/feasibility validation | Not started |
| Canonical ModelRun and provenance | Not started |
| XLSX reporting and reconciliation | Not started |

## 2. Phase plan

### Phase 0 — Input and historical-validation foundation

**Status:** Delivered

Evidence:
- PR #8: versioned JSON Schema and synthetic Case A.
- PR #10: Rust typed input and historical semantic validation.
- Cargo.lock and locked CI for the production Rust workspace.

Exit evidence: the canonical Case A input passes the schema and Rust runtime validation; negative cases reject malformed input and inconsistent historical statements.

### Phase 1 — Enterprise architecture and audit baseline

**Status:** In progress

Deliverables:
- Engineering audit and severity-ranked finding register.
- Enterprise target architecture and result/status contract.
- Delivery roadmap and PR sequence.
- Corrected implementation-status documents.
- Explicit statement of the existing validator's numeric coupling and diagnostic limitations.
- CLI input-read bound (1 MiB), UTF-8 validation and regression tests, implemented on the enterprise-foundation branch.

Exit criteria:
- Main-branch documentation matches delivered code.
- Architecture preserves a single Rust calculation authority.
- No production numeric backend is selected by assumption.
- CLI transport boundary rejects oversized and invalid UTF-8 documents deterministically.
- Blockers and owners/evidence are visible.

### Phase 2 — Numeric, dependency and independent-model gates

**Status:** Blocked on evidence and policy decisions

Workstream A — Numeric Profile:
- Set maximum/minimum monetary inputs by model unit.
- Set maximum rate/day and assumption bounds.
- Calculate maximum intermediate values for every formula chain.
- Define supported source fractional scale and internal division behaviour.
- Quantify accumulated error across five forecast periods.
- Define separate accounting, schedule and display tolerances.
- Define final rounding mode and display scale.
- Test range headroom, overflow and near-boundary behaviour.

Workstream B — Numeric backend:
- Compare fixed Decimal and arbitrary-precision candidates against the approved profile.
- Measure runtime, memory, allocation behaviour and operational complexity.
- Verify serialization, context control, determinism and error reporting.
- Decide whether to retain rust_decimal or migrate the application boundary.
- Record the decision in DECISION_LOG.md before production engine work.

Workstream C — Dependency/SBOM:
- Inventory the production Cargo workspace and every isolated POC independently.
- Produce machine-readable SBOMs.
- Review direct/transitive licences, notices, source, maintenance and advisories.
- Pin approved versions and document exceptions.
- Re-run advisory checks at release.

Workstream D — Independent formula validation:
- Separately derive the full five-year Case A expected values.
- Compare every statement and supporting schedule, not just selected headline metrics.
- Include loss-making periods, zero denominators, zero debt, full repayment, invalid over-repayment, zero CapEx, cash shortfall and rate/day boundaries.
- Obtain an accounting/formula review before production implementation is accepted.

**Phase 2 exit:** Numeric Profile, production backend, dependency policy and independent benchmark are approved and versioned. Any missing item blocks Phase 3.

### Phase 3 — Canonical Rust calculation engine

**Status:** Not started; gated by Phase 2

Implement inside the existing `ofrl_model` crate:

1. Strong domain types and numeric boundary.
2. Operating forecast.
3. Working-capital schedule.
4. Aggregate PP&E schedule.
5. Debt and interest schedule.
6. Tax, dividends and retained earnings.
7. Income Statement.
8. Cash Flow Statement.
9. Balance Sheet diagnostic state.
10. Structural Integrity and Scenario Feasibility checks.
11. Typed calculation diagnostics.
12. Canonical ModelRun result.

Acceptance:
- Every formula has a stable ID.
- No binary floating-point in authoritative monetary calculations.
- Checked arithmetic failures are typed and surfaced.
- No hidden funding or balancing plugs.
- All SI/SF checks have deterministic tests.
- Golden and property tests pass over the approved Numeric Profile.
- Exact outputs match the independently reviewed benchmark within the approved error policy.

### Phase 4 — Deterministic run manifest and CLI

**Status:** Not started; depends on Phase 3

Deliver:
- `validate`, `run`, `inspect` and later `export` commands.
- Canonical input and output serialization.
- SHA-256 input/output fingerprint and run manifest.
- Engine, model, schema and Numeric Profile versions.
- Stable exit codes and typed diagnostic output.
- Bounded input handling and safe output writes.

Acceptance:
- Identical canonical inputs and policy/build identity reproduce identical canonical output.
- Wall-clock metadata is separated from deterministic hashes.
- Invalid inputs and arithmetic failures produce distinct status/exit semantics.

### Phase 5 — Analyst-facing XLSX

**Status:** Not started; depends on Phase 3 and Phase 4

Deliver:
- Cover and run status.
- Assumptions register.
- Income Statement, Balance Sheet and Cash Flow Statement.
- Working capital, PP&E, debt and KPI schedules.
- Structural Integrity and Scenario Feasibility.
- Diagnostics, provenance and limitations.

Acceptance:
- Workbook contains no independent financial calculations.
- Currency, scale, sign and display precision are explicit.
- Negative cash is represented as a funding-shortfall diagnostic, not a valid cash asset.
- Every material displayed value reconciles to canonical ModelRun outputs.
- Canonical JSON/CSV is retained independently of workbook precision.

### Phase 6 — Release hardening and independent reproduction

**Status:** Not started

Deliver:
- Full CI matrix and reproducible build instructions.
- SBOM/licence/security review snapshot.
- Property, boundary, golden and negative test evidence.
- Workbook reconciliation report.
- Known-limitations and model-assumptions register.
- Independent reviewer reproduction.

Release claims:
- **Structural Integrity PASS** only if every SI hard gate passes.
- **Scenario Feasibility PASS** only if every SF hard gate passes.
- **Research Release PASS** only if all release gates pass.
- A failed scenario may still be reproducible, but it must never be presented as funded or feasible.

## 3. PR sequencing and merge controls

| Work item | Intended branch / PR | Merge condition |
|---|---|---|
| Decimal/full-formula research | Existing PR #11 | POC scope reviewed; production adoption remains separate |
| Schema-tooling lockfile | Existing PR #12 | Review and CI complete |
| Enterprise audit/design package | `work/ofrl-ib-001-enterprise-foundation` | Documentation review and scope check |
| Numeric Profile/backend decision | Separate OFRL-only PR | Phase 2 evidence complete |
| Production forecast core | Separate OFRL-only PR | Phase 2 exit criteria PASS |
| ModelRun/CLI | Separate OFRL-only PR | Core formulas and invariants accepted |
| XLSX export | Separate OFRL-only PR | Canonical output contract stable |

Keep PRs small enough to review. Do not combine research dependency changes, production formula changes and workbook presentation changes into one unreviewable patch.

No Draft PR should be made ready merely because CI is green. A PR may pass its defined test scope while its project-level acceptance gates remain open.

## 4. Required test layers

| Layer | Purpose | Required evidence |
|---|---|---|
| Unit | Formula and schedule mechanics | Formula-specific expected values and boundary tests |
| Integration | Full model paths | All five periods, all three statements and schedules |
| Property | Invariants over generated valid inputs | Reproducible seeds and shrunk regression cases |
| Golden | Independent expected outputs | Reviewed exact/decimal expected values by period |
| Negative | Invalid inputs and unsupported scenarios | Stable diagnostic code and path |
| Numeric | Range, scale and accumulated error | Approved Numeric Profile boundaries |
| Determinism | Same input/model/build identity | Stable canonical run hash |
| Export | Workbook structure and displayed values | Period-by-period canonical reconciliation |
| Dependency | Package provenance and risk | SBOM, licence inventory and advisory snapshot |
| Reproduction | Independent repeatability | Clean-checkout instructions and recorded environment |

## 5. Change control

Every material change must record:
- the previous decision and new decision;
- the reason and evidence;
- affected formula IDs, inputs and outputs;
- schema/model/numeric policy version impact;
- tests that must be rerun;
- whether a migration is required.

Never change a tolerance after observing a failed result merely to produce PASS. Any acceptance-criterion change requires a rationale, a versioned policy update and rerunning the affected suite.

## 6. Scope guard

This roadmap authorizes work only in OFRL-IB-001 within `witthaya-quant/Open-Finance-Research-Lab`. It does not authorize work in other repositories or projects. Any proposed dependency or shared repository-level change must be directly required by OFRL-IB-001 and listed explicitly in its PR.
