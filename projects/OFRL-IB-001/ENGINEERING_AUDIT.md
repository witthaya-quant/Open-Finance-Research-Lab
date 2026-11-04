<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Engineering Audit

**Audit version:** 0.2.0  
**Audit date:** 2026-11-04  
**Production baseline:** `main` at `323d4da36bd5386d88a64fb67ff8ca77e73c5905`  
**Research evidence:** PR #11, head `2b93a966978966843bd6f78ae7d719b4c35f6d9d`  
**Schema-tooling evidence:** PR #12, head `c38f9ad50ff4ff39f92d6cf7514a6a917e6971cd`  
**System-design and CLI changes:** PR #13, head `8085fe4bfdce7ee4d5cbc7d5a0493693264fbaeb`  
**Status:** Evidence-based audit; all open PR evidence is distinguished from merged production capability

## 1. Executive assessment

OFRL-IB-001 has progressed beyond a basic input-validator prototype. The main branch contains a versioned input contract, a strict Decimal-string boundary, historical accounting validation, a deterministic synthetic case, a Rust library/CLI and a two-toolchain CI matrix.

The open research PR #11 materially improves the evidence base: it contains a five-year independent exact-rational baseline, four independent edge scenarios, a standard-library Python reference that checks 800 exact expected outputs, and project-scoped CycloneDX/RustSec automation. A new runtime-cost observation harness has also been added to that branch. These are meaningful research deliverables, but they remain unmerged and do not select a production arithmetic backend.

The system is **not yet a production forecast model**. The production Rust workspace still validates inputs and historical relationships only. It does not calculate the five forecast years, produce a canonical ModelRun, evaluate Structural Integrity / Scenario Feasibility for calculated results, produce deterministic provenance, or generate an XLSX report.

The principal remaining risk is not simply a lack of code. It is the unresolved boundary between (a) evidence that software follows the currently written equations and (b) approval that those equations, conventions, numeric limits and reporting treatments are appropriate for the declared modelling purpose. The implementation must preserve that distinction.

## 2. Scope, branch state and evidence rules

This audit covers only OFRL-IB-001 in `witthaya-quant/Open-Finance-Research-Lab`. No other project is included.

| Evidence | Current status | Interpretation |
|---|---|---|
| `main` | Merged baseline | Authoritative production code currently available on the default branch |
| PR #11 — numeric research | Open, Draft | Contains research code, independent benchmarks, SBOM workflow and the new timing harness; not yet production capability |
| PR #12 — schema tooling | Open, Draft | Contains a pinned Node/AJV validator toolchain and lockfile; does not yet prove parity with Rust |
| PR #13 — audit/design/CLI | Open, Draft | Contains the audit, design, roadmap and bounded CLI reader; not merged |
| Finance/accounting review | Not evidenced | No independent sign-off is claimed |
| Production Forecast Engine | Not implemented | Must remain gated |

A CI pass applies to the exact commit and workflow scope that ran. It does not transfer automatically to a later commit, and it does not approve a financial policy. PR #11's earlier research head `d9788253e8e06f374bcf89add71e793c3348286c` passed its four workflow groups. The later timing-harness commits exposed formatter differences; those were corrected in the current branch head, but the latest commit's full CI result must be checked before the timing artifact can be treated as verified evidence.

## 3. Capabilities already implemented on main

| Area | Assessment |
|---|---|
| Input contract | Versioned Draft 2020-12 JSON Schema, closed objects and canonical synthetic fixture |
| Decimal input boundary | Decimal values are JSON strings; non-string tokens, non-canonical text, negative zero, excess fractional scale and parse/format changes are rejected |
| Typed input | Rust structures reject unknown fields and constrain declared enum values |
| Period validation | Three historical and five forecast periods, Gregorian dates, sequence, contiguity, annual duration and hand-offs |
| Historical accounting | Opening/historical Balance Sheet, Income Statement, Cash Flow Statement and selected NWC, PP&E, debt and retained-earnings roll-forwards |
| CLI baseline | `validate <input.json>`; no production forecast or export command |
| Build reproducibility | Production Cargo.lock and two-toolchain CI |
| Calculation engine | Not implemented |
| Canonical ModelRun / provenance | Not implemented |
| Forecast integrity / feasibility | Not implemented |
| XLSX reporting | Not implemented |

The bounded input reader (1 MiB limit, max-plus-one byte read, UTF-8 rejection and four tests) exists on PR #13's branch. It is not on `main` until reviewed and merged.

## 4. Updated assessment of research evidence in PR #11

### 4.1 Independent arithmetic and formula-output verification

PR #11 now contains:

- BM-001: an independently derived FY2026 benchmark covering 32 output metrics.
- BM-002: FY2026–FY2030, 32 metrics per year, 160 exact rational expected values.
- BM-003: four named edge scenarios × five years × 32 metrics, 640 exact rational expected values.
- A Python standard-library reference using `Decimal` and `Fraction`; it does not import, invoke or read Rust forecast output. It verifies the canonical fixture Git blob identity and regenerates the committed golden data.
- Rust integration tests comparing against immutable goldens without display quantization or tolerance.

The combined BM-002/BM-003 reference check reports **800 exact expected outputs PASS** on the inspected PR #11 CI runs. The edge set includes high-CapEx funding deficit, loss-making operations, full debt repayment followed by a later redraw, and zero revenue/zero CapEx.

**What this proves:** the tested Rust POC output matches separately generated exact expected values for the documented synthetic cases, and the independent reference regenerates the committed golden datasets.

**What it does not prove:** the model conventions are appropriate accounting policy, all formula choices are correct for every intended entity, the model is IFRS/US GAAP/statutory compliant, or all valid input combinations have been covered. Independent finance/accounting review remains open.

### 4.2 Numeric candidate evidence

The isolated POC compares `rust_decimal 1.43.0`, `bigdecimal 0.4.11` and Exact Rational across the same five-year formula path. It records division behaviour, checked overflow and output differences.

Observed stress evidence:

- At the selected (10^{18}) monetary scaling point, both Decimal candidates completed the tested path and matched the exact-rational reference for the compared outputs.
- At the selected (10^{22}) capacity point, `rust_decimal` raised a checked multiplication overflow in the FY2026 accounts-receivable numerator; BigDecimal completed the tested path under the POC's 80-significant-digit HalfEven context.
- The values (10^{18}) and (10^{22}) are two chosen test points, not the exact transition boundary, a supported company-size envelope or approved production limits.
- The provisional two-decimal comparison is a POC observation, not the approved reporting or reconciliation policy.

A dependency-free release-mode timing executable has been added to PR #11 for Case A and the (10^{18}) stress input. It reports min/median/max per-forecast elapsed time and runner/toolchain metadata. It is explicitly non-gating. The newest branch commit must complete CI before its timing artifact can be cited as verified. Memory, allocations, tail latency, concurrent workloads and workbook costs remain unmeasured.

**Disposition:** no production numeric library is selected by this audit. Production input types and historical validation currently use `rust_decimal::Decimal`; selecting a different backend would require an explicit input/validation boundary migration, not only swapping the calculation implementation.

### 4.3 Dependency and supply-chain evidence

PR #11 adds a project-scoped workflow that creates separate CycloneDX 1.5 inventories for the production model, CLI and isolated POCs. It pins `cargo-cyclonedx 0.5.9` and `cargo-audit 0.22.2` release binaries by SHA-256, runs RustSec with `--deny warnings`, preserves JSON reports and checksums, and verifies committed lockfiles remain unchanged.

The inspected 2026-11-04 scan reported zero vulnerability findings and zero warnings for the production graph (54 package dependencies), Numeric Envelope POC (58), and Decimal Contract POC snapshot (56). This is a point-in-time advisory-database result, not proof that software has no vulnerabilities or application defects.

Open items remain:

- Decimal Contract POC has no committed Cargo.lock; its scan uses an ephemeral lock snapshot and is not reproducible dependency evidence.
- Package source provenance, duplicate versions, licence texts, legal alternatives, notices and distribution/source obligations require review.
- A declared SPDX expression is inventory metadata, not legal acceptance.
- Broader build provenance, artifact attestation and non-Rust/system dependency coverage are not established by this workflow.

## 5. Findings and required disposition

Severity definitions: **Blocker** must close before production forecast acceptance; **Major** must close before a reproducible V1 release; **Moderate** should close before broader distribution or real-data intake.

### EA-01 — Production Numeric Profile is not approved

**Severity:** Blocker

**Evidence:** `NUMERIC_ENVELOPE.md` remains Proposed. Stress tests exercise selected monetary scaling points but do not define business-approved maxima/minima, rate/day bounds, intermediate headroom, source scale, division context, five-year error budget or display precision.

**Risk:** A candidate may pass the selected examples and still overflow or lose material precision elsewhere in the permitted domain. An arbitrary-precision type does not solve unlimited resource use or remove the need for a finite supported input contract.

**Disposition:** Complete the versioned Numeric Profile worksheet, derive formula-by-formula intermediate bounds, quantify error propagation and separate arithmetic, accounting, schedule and display tolerances. No production limits may be invented from the stress points.

### EA-02 — Production numeric backend remains undecided

**Severity:** Blocker

**Evidence:** The POC is isolated. Production input and historical validation types use `rust_decimal::Decimal`. BigDecimal's POC division relies on a configured default environment context; production use would need explicit, deterministic arithmetic-context ownership and provenance.

**Risk:** Early engine implementation could hardwire a representation and create costly migrations. Arbitrary precision can also introduce CPU/memory growth and less predictable latency.

**Disposition:** Decide only after EA-01, controlled performance/memory evidence, serialization review, error-semantics review and dependency/license approval. Record the decision and migration impact before forecast implementation.

### EA-03 — Independent accounting/formula sign-off remains open

**Severity:** Blocker

**Evidence:** BM-002/BM-003 provide 800 exact synthetic expected values and independent computation. The benchmark methodology itself states that it implements the published V1 equations; it is not an accounting opinion.

**Risk:** A consistently implemented but conceptually inappropriate convention can still reconcile perfectly. Examples requiring explicit review include average-debt interest timing, same-period cash settlement, tax floor, dividend floor, aggregate PP&E depreciation cap, working-capital day basis and treatment of funding shortfall.

**Disposition:** Obtain a documented independent review of model assumptions, equations, statement classifications, sign conventions, edge outcomes and stated non-compliance boundary. Review must identify reviewer role, scope, findings and disposition. Do not describe the work as independent review until it occurs.

### EA-04 — Dependency/licence/source acceptance is incomplete

**Severity:** Blocker before production dependency approval

**Evidence:** PR #11 automates SBOM and RustSec inventory, with a point-in-time clean advisory scan. The Decimal Contract POC lockfile is ephemeral. Licence and source review is explicitly open.

**Risk:** A clean advisory scan does not establish source trust, exploitability, licence compatibility, notice obligations, maintainability or absence of defects.

**Disposition:** Commit or retire the Decimal Contract POC; review each direct/transitive component, source, duplicate versions, SPDX expression and original licence text; approve an allow/exception policy; record notices and distribution obligations; retain release-date evidence.

### EA-05 — Forecast engine and result contract are not implemented

**Severity:** Blocker

**Evidence:** Production `ofrl_model` exposes input validation and historical semantic checks only. POC forecast code is isolated and consumes generic JSON values rather than the production typed domain.

**Risk:** Promoting POC code directly would couple production to research abstractions, omit typed domain validation and bypass a stable calculation result/error contract.

**Disposition:** After EA-01 through EA-04 are accepted, design a production typed numeric boundary, typed schedules, typed calculation errors, formula identifiers and canonical ModelRun. Port formulas into production in small reviewed increments, with independently checked expected values.

### EA-06 — Deserialization classification is based on error-message text

**Severity:** Major

**Evidence:** `validation.rs` maps Serde failures by substring inspection (for example, `unknown field`, `missing field` and custom Decimal markers); failures do not consistently expose JSON Pointer paths.

**Risk:** Dependency wording changes can silently alter public diagnostic codes. Callers cannot locate nested fields and array indices reliably.

**Disposition:** Introduce a typed deserialization/error-path boundary with stable code mapping and JSON Pointer capture. Review any additional dependency and lockfile/SBOM impact before adoption. Add regression tests for nested values, missing/unknown fields, invalid enums, array indices and malformed Decimal values.

### EA-07 — JSON Schema and Rust contract parity is not enforced

**Severity:** Major

**Evidence:** Schema validation and Rust semantic validation run in separate jobs. PR #12 makes the schema toolchain reproducible but does not yet run one shared positive/negative corpus through both validators.

**Risk:** Structural rules, field names, enum values and lexical constraints can drift. Passing schema validation does not guarantee Rust acceptance, and vice versa.

**Disposition:** Create a versioned shared contract corpus with case ID, JSON document, expected schema result and expected Rust result/code. Run the same files through AJV and Rust in CI. Document intentional differences between structural checks and cross-field/accounting semantics.

### EA-08 — Reconciliation tolerances are proposed, not approved

**Severity:** Major

**Evidence:** `VALIDATION_MATRIX.md` proposes (10^{-6}+10^{-8}\times\max(|A|,|B|,1)). The policy is not tied to accepted scale, materiality or approved numeric representation.

**Risk:** A relative tolerance grows with the magnitude of the balance. For a value of (10^{12}) model units, the relative component alone is (10^4) model units. Under a thousands unit scale, that corresponds to 10 million currency units. Such a threshold may be unacceptable even though the percentage appears small.

**Disposition:** Define independent policies for backend error, accounting identities, schedule roll-forwards and workbook display. Express each in model units and converted currency amounts at the supported range endpoints. Establish the policy before looking at production pass/fail results; do not widen it to make tests pass.

### EA-09 — Forecast Structural Integrity and Scenario Feasibility are not implemented

**Severity:** Blocker before accepting forecast outputs

**Evidence:** The validator checks historical statements and forecast assumption domains only. The high-CapEx POC demonstrates negative calculated closing cash with an algebraic Balance Sheet residual of zero.

**Risk:** Algebraic reconciliation can be confused with an economically funded/reportable forecast.

**Disposition:** For every calculated period, expose separate status dimensions for calculation completion, Structural Integrity, Scenario Feasibility and reportability. Preserve diagnostic cash and funding shortfall, mark feasibility FAIL when cash is negative, and never insert hidden financing or show negative diagnostic cash as a valid reportable asset.

### EA-10 — Test coverage is not yet sufficient for production domain claims

**Severity:** Major

**Evidence:** The independent goldens cover one baseline plus four edge scenarios (800 outputs). Production validation tests cover fixed input cases. No production forecast property suite exists; broad rate/day boundaries, zero denominator ratios, invalid debt paths, near-overflow bounds and generated combinations remain incomplete.

**Risk:** A finite set of fixed fixtures cannot establish invariants across the complete supported input domain.

**Disposition:** After Numeric Profile approval, add property-based tests with reproducible seeds, boundary partitions, invalid-input generators, shrunk regression fixtures, formula-level tests and complete five-year integration cases. Keep the independent goldens separate from the implementation generating actual outputs.

### EA-11 — Deterministic ModelRun and audit lineage are not implemented

**Severity:** Major

**Evidence:** The public result is currently a ValidationReport. There is no canonical run manifest, input/output SHA-256, model/conventions/numeric-policy identity, formula lineage or deterministic run fingerprint.

**Risk:** A reviewer cannot establish which policy and exact input generated a figure or reproduce a result after an engine change.

**Disposition:** Version canonical JSON serialization, key ordering, Decimal formatting and null/omission rules. Hash canonical input/output and model/engine identity. Keep timestamps and machine-specific execution details outside the deterministic fingerprint. Link every material metric to formula ID, period, driver and prior-period dependency.

### EA-12 — XLSX generation and reconciliation are not implemented

**Severity:** Major

**Evidence:** No production exporter exists. The workbook design requires Rust canonical outputs and acknowledges Excel's finite numeric precision.

**Risk:** Formula duplication or display precision can make workbook figures disagree with authoritative results.

**Disposition:** Build XLSX only after ModelRun schema stabilizes. Workbook must be a presentation adapter, contain no independent forecast formulas, expose run status and diagnostics, and reconcile every material displayed value to canonical outputs. Keep JSON/CSV Decimal strings as authoritative evidence.

### EA-13 — CLI operational contract is incomplete

**Severity:** Moderate

**Evidence:** Main supports `validate` only. PR #13 adds bounded input reading and UTF-8 rejection on its branch, but no `run`, `inspect` or `export` commands or safe output-write contract.

**Risk:** A later CLI can overwrite source files, produce partial outputs or confuse invalid input with calculation failure.

**Disposition:** Keep commands incremental. Define stable exit codes, bounded inputs, output destination policy, source/target collision checks and atomic writes before adding run/export commands. Do not add a command framework without a demonstrated need.

### EA-14 — Release evidence is not yet independently reproduced

**Severity:** Major

**Evidence:** CI matrices and artifact checks provide useful project-internal evidence. No independent clean-checkout reproduction/reviewer report is recorded.

**Risk:** The project may overstate reproducibility if instructions depend on hidden local state or unpinned external conditions.

**Disposition:** Add a clean-checkout reproduction script/instructions, identify environment and versions, verify hashes and artifacts, and record an independent reproduction report before a reproducible-release claim.

## 6. Numeric tolerance interpretation

The proposed tolerance must not be treated as approved merely because it is already written in the Validation Matrix. At large balances the relative term can dominate the absolute term. For example:

| Reconciled magnitude (model units) | Relative component at (10^{-8}) | Interpretation |
|---:|---:|---|
| (10^6) | 0.01 | Model units |
| (10^9) | 10 | Model units |
| (10^{12}) | 10,000 | Model units |

The currency meaning then scales with `unit_scale`. The accepted policy should be derived from financial materiality and the approved Numeric Profile, not from a convenient generic epsilon.

## 7. Current release-gate register

| Gate | Evidence required | Status |
|---|---|---|
| Numeric Profile | Approved input/intermediate bounds, scale, precision, rounding and separate error budgets | OPEN |
| Production arithmetic backend | Profile-based comparison, arithmetic context, serialization and cost/complexity review | OPEN |
| Independent finance/accounting review | Written review and disposition of equations, conventions and edge cases | OPEN |
| Rust dependencies | Locked graphs, SBOM, security disposition, source and licence/notice acceptance | PARTIAL |
| Independent arithmetic goldens | BM-002/BM-003: 800 exact outputs in open PR #11 | IMPLEMENTED IN OPEN PR |
| Schema/Rust parity | Shared fixture corpus run against both implementations | OPEN |
| Typed diagnostics | Stable error variants and field-path capture | OPEN |
| Production forecast engine | Typed Rust schedules and three statements | NOT STARTED |
| Forecast integrity/feasibility | SI/SF/reportability results over all periods | NOT STARTED |
| ModelRun provenance | Canonical serialization, SHA-256 and metric lineage | NOT STARTED |
| XLSX | Output-only workbook and canonical reconciliation | NOT STARTED |
| Independent reproduction | Clean-checkout reproduction and reviewer evidence | NOT STARTED |

## 8. Audit conclusion and implementation order

The next implementation increment should not be a speculative production forecast engine. First close the evidence and contract gaps that would otherwise dictate a second rewrite:

1. Complete the Numeric Profile using the worksheet and obtain documented review.
2. Complete the production backend decision using correctness, deterministic context, performance, memory and dependency evidence.
3. Close lockfile, source, licence and advisory dispositions for dependencies intended to remain.
4. Obtain independent finance/accounting review of V1 conventions and formulas.
5. Implement typed deserialization errors and shared Schema/Rust contract tests.
6. Freeze the production calculation/result contracts, then implement the Rust schedules and statements.
7. Add SI/SF/reportability checks and property/boundary tests.
8. Implement canonical ModelRun, deterministic fingerprints and formula lineage.
9. Generate XLSX solely from canonical outputs and reconcile it.
10. Complete clean-checkout reproduction and release review.

No research POC or green CI run is to be represented as production financial-model approval. This audit authorizes work only within OFRL-IB-001.
