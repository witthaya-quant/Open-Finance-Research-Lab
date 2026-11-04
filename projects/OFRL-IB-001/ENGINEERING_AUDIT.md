<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Engineering Audit

**Audit version:** 0.3.0  
**Audit date:** 2026-11-04  
**Production baseline:** `main` at `abd5d5f439f85805ac732d147f5d42d48cabc2aa`  
**Research evidence:** PR #11, head `120ccd3414fb7018011ea3991800728f86736319`  
**Schema-tooling evidence:** PR #12, head `c38f9ad50ff4ff39f92d6cf7514a6a917e6971cd`  
**System-design and CLI changes:** PR #13, open Draft, head `ff34f9aa73fbc04f5341078c020a7d0a78ac1524`  
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
| `main` | Merged baseline at `abd5d5f` | Authoritative production code currently available on the default branch; input and historical validation only |
| PR #11 — numeric research | Open, Draft; head `120ccd3414` | 800 independent exact-rational expected outputs, candidate arithmetic comparison, SBOM/RustSec research and non-gating timing; not production approval |
| PR #12 — schema tooling | Open, Draft; head `c38f9ad50` | Reproducible Node/AJV toolchain and package lockfile; schema/Rust parity is supplied separately in PR #13 |
| PR #13 — audit/design/CLI | Open, Draft; head `ff34f9aa` | Engineering audit, system design, roadmap, Numeric Profile worksheet, bounded CLI reader and shared contract corpus; current CI rerun is not yet confirmed complete |
| PR #14 — repository foundation | Open, non-Draft; base `main` | Introduces governance and the always-running `repository-policy` workflow; all six expected workflow groups passed on its inspected head |
| PR #18 — portfolio controls | Open, non-Draft; stacked on PR #14 | Hardens registry/data controls, removes path filters from project workflows and removes mutable Rust bootstrap fallback; all six required checks passed on its inspected head |
| PR #20 — Decimal POC lockfile | Open, non-Draft; stacked on PR #18 | Adds the isolated POC lockfile and locked CI commands; all six required checks passed on its inspected head |
| PR #22 — SBOM preview | Open, non-Draft; stacked on PR #20 | Adds a Linux-targeted CLI SBOM/checksum preview only; seven checks/jobs passed on its inspected head; not an official release |
| Finance/accounting review | Not evidenced | No independent sign-off is claimed |
| Production Forecast Engine | Not implemented | Must remain gated |

A CI pass applies only to the exact commit and workflow scope that ran. It does not transfer automatically to a later commit, approve a financial policy, or imply that an open PR has merged. The PR #11 timing-harness evidence remains observational; PR #20 and PR #22 improve dependency and preview evidence but do not complete source-level licence/security acceptance or authorize a release.

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

A dependency-free release-mode timing executable has completed for Case A and the (10^{18}) stress input. Workflow run [41784590895](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41784590895) passed on Rust 1.85.0 and 1.99.1 and uploaded two artifacts. Median time per five-year forecast was: Rust 1.85.0 — Case A 42.117 µs / stress 43.032 µs for rust_decimal, 87.877 µs / 64.566 µs for BigDecimal, and 316.808 µs / 367.050 µs for Exact Rational; Rust 1.99.1 — Case A 37.235 µs / stress 38.158 µs, 82.277 µs / 63.559 µs, and 324.448 µs / 392.920 µs respectively. BigDecimal was 1.50×–2.21× the fixed Decimal median in this run; Exact Rational was 7.52×–10.30×. This is one 4-vCPU AMD EPYC 7763 runner environment using 5 warm-ups, 9 samples and 40 forecasts per sample. The result is non-gating and does not measure memory, allocations, tail latency, concurrency, file I/O, validation, serialization or workbook costs. CSV and runner metadata checksums are recorded in [BENCHMARKS_PERFORMANCE.md](https://github.com/witthaya-quant/Open-Finance-Research-Lab/blob/research/ofrl-ib-001-numeric-envelope/projects/OFRL-IB-001/poc/model-numeric-envelope/BENCHMARKS_PERFORMANCE.md) on PR #11.

**Disposition:** no production numeric library is selected by this audit. Production input types and historical validation currently use `rust_decimal::Decimal`; selecting a different backend would require an explicit input/validation boundary migration, not only swapping the calculation implementation.

### 4.3 Dependency and supply-chain evidence

PR #11 supplies the candidate-level CycloneDX/RustSec research workflow and 2026-11-04 point-in-time scan evidence. PR #20 adds a separate Cargo-generated lockfile for the Decimal Contract POC (56 package records: the local package plus 55 registry packages), with `cargo metadata --locked`, `cargo tree --locked`, locked Clippy/tests/docs and a publisher-declared licence-metadata check. It does not perform source-level legal review.

PR #22 adds a **preview-only** CycloneDX 1.5 SBOM and SHA-256 bundle for the implemented `ofrl_cli` input validator on `x86_64-unknown-linux-gnu`. Its inspected run compared 41 SBOM components exactly with target-filtered Cargo metadata (40 registry packages plus local `ofrl_model`), verified package hashes/licence metadata and four per-file checksums, and recorded the artifact archive digest. It is not a release of the unfinished three-statement model and does not establish Windows, macOS, ARM or other target support.

The preview generator is pinned to `cargo-cyclonedx 0.5.9`; its published lockfile includes yanked `xml-rs 0.8.19`. This is recorded as a time-bounded, preview-only build-tool exception with review due by 2027-02-04 or earlier if upstream removes the dependency. The reason for the yank was not established; absence of a surfaced advisory is not proof of safety.

Open items remain:

- Source provenance and source-level licence/notice review for dependencies intended for distribution.
- Security-advisory review and explicit disposition of every applicable finding/exception at release time.
- Review or re-approval of the preview-generator exception before any official release.
- Reproducible builds and artifact evidence for the exact reviewed release tag and each supported target.
- Broader build provenance/attestation and non-Rust/system dependency coverage where relevant.

A package's publisher-declared SPDX expression is inventory metadata, not legal acceptance. A clean advisory scan is time-bounded evidence, not proof of no vulnerabilities or application defects.

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

**Evidence:** PR #20 adds a committed, isolated Decimal Contract POC lockfile and locked CI on its open branch. PR #22 adds a checksum-verified preview SBOM for the implemented CLI on one Linux target. Neither PR completes source-level licence/notice acceptance or security disposition for an official distributable release.

**Risk:** A lockfile, publisher-declared licence field, SBOM or clean point-in-time advisory scan does not establish source trust, legal compatibility, notice obligations, exploitability, maintainability or absence of defects.

**Disposition:** Review each direct/transitive component intended for distribution, source provenance, duplicate versions, original licence text and notice requirements; record advisory findings and exceptions with owners/expiry; resolve or re-approve the yanked `xml-rs 0.8.19` preview-generator exception before official release. Preserve exact tag, target, lockfile, SBOM and artifact checksum identity.

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

**Evidence:** PR #13 now contains a versioned shared positive/negative corpus consumed by both AJV and the Rust runtime. Cases declare schema validity, runtime status and stable Rust diagnostic codes. The Rust test now validates corpus metadata, unique case IDs/fixture paths, canonical fixture paths under `fixtures/`, and rejects path traversal/symlink escape. AJV validation checks schema identity, corpus shape, expected outcome types and fixture containment. CI on the latest hardening commit is still being re-run; this evidence remains branch-local and is not merged to `main`.

**Risk:** A shared fixture set can still be incomplete or can drift if one side does not enforce the same metadata and expected outcome contract. Schema-invalid fixtures must never be accepted by the runtime; schema-valid fixtures may still correctly fail cross-field/accounting semantic checks.

**Disposition:** Complete the current two-toolchain CI run, preserve the intentional structural-versus-semantic distinction, and add future cases whenever the versioned contract changes. Do not claim parity for all possible JSON documents from a finite fixture set.

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

### EA-15 — Required repository-policy check is not yet available on the current main branch

**Severity:** Major merge-sequencing blocker

**Evidence:** The active main Ruleset (ID `26516806`) is configured with exactly six required GitHub Actions contexts, including `repository-policy`. The `.github/workflows/repository-policy.yml` workflow is present on PR #14's branch and the hardened workflow changes are on stacked PR #18, but the workflow file is not yet present on the current `main` commit. PR #13 consequently reports the five OFRL-specific contexts but no `repository-policy` check on its current branch head.

**Risk:** A PR targeting `main` cannot demonstrate every required check while the required workflow has not yet been integrated into the protected base branch. Weakening the Ruleset or renaming/removing the required context would conceal a publication-order problem rather than solve it.

**Disposition:** Preserve all six required checks. Complete the reviewed governance/supply-chain branch sequence so the always-running repository policy workflow is integrated into `main`; then verify a fresh documentation-only and project-code PR both report all six exact contexts. Keep PR #13 Draft until the base-branch workflow dependency and its own review/acceptance gates are satisfied.

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
| Numeric Profile | Approved input/intermediate bounds, scale, precision, rounding and separate error budgets | OPEN — worksheet remains TBD |
| Production arithmetic backend | Profile-based comparison, explicit arithmetic context, serialization and cost/resource review | OPEN — no backend selected |
| Independent finance/accounting review | Written independent review and disposition of equations, conventions and edge cases | OPEN — not evidenced |
| Rust dependencies | Locked graphs, target-scoped SBOM, source/licence/notice review and advisory disposition | PARTIAL — PR #20/#22 open; official release review open |
| Independent arithmetic goldens | BM-002/BM-003: 800 exact outputs in open PR #11 | IMPLEMENTED IN OPEN PR — research evidence only |
| Schema/Rust parity | Shared versioned corpus consumed by AJV and Rust | IMPLEMENTED IN OPEN PR #13 — latest hardening CI pending |
| Typed diagnostics | Stable error variants and JSON Pointer for deserialization failures | OPEN — message-substring mapping remains |
| Repository required checks | All six Ruleset contexts available on main and reported on fresh PRs | OPEN — repository-policy workflow not yet on main |
| Production forecast engine | Typed Rust schedules and three statements | NOT STARTED — gated |
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
5. Complete and maintain the shared Schema/Rust contract corpus in PR #13; separately replace message-substring deserialization classification with typed error/path mapping.
6. Freeze the production calculation/result contracts only after all Phase 2 gates, then implement the Rust schedules and statements.
7. Add SI/SF/reportability checks and property/boundary tests.
8. Implement canonical ModelRun, deterministic fingerprints and formula lineage.
9. Generate XLSX solely from canonical outputs and reconcile it.
10. Complete clean-checkout reproduction and release review.

No research POC or green CI run is to be represented as production financial-model approval. This audit authorizes work only within OFRL-IB-001.
