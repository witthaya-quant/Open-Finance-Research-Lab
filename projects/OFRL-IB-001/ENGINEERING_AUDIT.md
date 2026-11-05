<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Engineering Audit

**Audit version:** 0.7.0  
**Audit date:** 2026-11-05  
**Production baseline:** `main` at `cc2d01b3ffd1d63d32fe87430967a4a86ecbffaa`  
**Research evidence:** PR #11, head `120ccd3414fb7018011ea3991800728f86736319`  
**Schema-tooling evidence:** PR #12, head `c38f9ad50ff4ff39f92d6cf7514a6a917e6971cd`  
**System-design, CLI hardening and Numeric Profile tooling:** PR #13 remains open Draft; code-tested baseline `044f0224fbeda4b3ef305a0f66aeb52e9ac92d8a` passed all six required contexts and the 17-test exact Numeric Envelope job; latest head `c2f5cf3ee286eb2b17ef3d086ad074cebfefd765` passed all six required contexts plus the additional Numeric Envelope Derivation check  
**Status:** Evidence-based audit at main `cc2d01b`; merged governance and supply-chain controls are distinguished from branch-local OFRL-IB-001 implementation evidence

## 1. Executive assessment

OFRL-IB-001 has progressed beyond a basic input-validator prototype. The main branch contains a versioned input contract, a strict Decimal-string boundary, historical accounting validation, a deterministic synthetic case, a Rust library/CLI and a two-toolchain CI matrix.

The open research PR #11 materially improves the numeric evidence base: it contains a five-year independent exact-rational baseline, four independent edge scenarios, a standard-library Python reference that checks 800 exact expected outputs, candidate arithmetic comparisons and point-in-time CycloneDX/RustSec evidence. Its runtime-cost harness is observational and non-gating. PR #11 remains Draft and does not select a production arithmetic backend. PR #12 remains Draft and contains the isolated schema-validator toolchain lockfile.

Repository governance and supply-chain controls are now on `main` following PR #14, which integrated the reviewed PR #18 / #20 / #22 sequence. The active Ruleset and its six required checks were re-read after merge. Documentation-only and project-code smoke PRs (#24 and #27) each reported and passed all six exact required contexts, then were closed without merge. These changes improve repository controls; they do not create a production forecast engine.

The system is **not yet a production forecast model**. The production Rust workspace still validates inputs and historical relationships only. It does not calculate the five forecast years, produce a canonical ModelRun, evaluate Structural Integrity / Scenario Feasibility for calculated results, produce deterministic provenance, or generate an XLSX report. PR #13 remains Draft and contains branch-local input hardening, shared contract corpus, system design and numeric derivation work.

The principal remaining risk is not simply a lack of code. It is the unresolved boundary between (a) evidence that software follows the currently written equations and (b) approval that those equations, conventions, numeric limits and reporting treatments are appropriate for the declared modelling purpose. The implementation must preserve that distinction.

## 2. Scope, branch state and evidence rules

This audit covers only OFRL-IB-001 in `witthaya-quant/Open-Finance-Research-Lab`. No other project is included.

| Evidence | Current status | Interpretation |
|---|---|---|
| `main` | Governance merge `cc2d01b` | Input/history validator plus active Ruleset, repository policy workflow, portfolio/data controls, unfiltered OFRL CI, isolated Decimal POC lockfile and preview-only Linux CLI SBOM |
| Ruleset / required contexts | Active; exactly six required contexts | Read back from Rulesets API after merge; no bypass actors; strict up-to-date and conversation-resolution rules |
| PR #24 documentation smoke test | Closed without merge | All six required contexts passed on a documentation-only PR targeting current main |
| PR #27 project-code smoke test | Closed without merge | All six required contexts passed on a project-code PR targeting current main |
| PR #11 — numeric research | Open, Draft; head `120ccd3414` | 800 independent exact-rational expected outputs, candidate arithmetic comparison, point-in-time SBOM/RustSec and non-gating timing; no production backend approval |
| PR #12 — schema tooling | Open, Draft; head `c38f9ad50` | Pinned Node/AJV toolchain and package lockfile; tooling change remains branch-local |
| PR #13 — audit/design/CLI/numeric derivation | Open, Draft; synchronized with current main | Engineering audit, system design, roadmap, bounded CLI reader, shared corpus and symbolic Numeric Profile derivation; all six required contexts passed on the synchronized code branch; changes remain unmerged |
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
| Repository governance | Active main Ruleset, PR requirement, no bypass actors, required conversation resolution and six exact required contexts |
| Repository policy | Always-running `repository-policy` workflow validates local Markdown links, JSON/YAML, project manifests/registry and policy contracts |
| Portfolio/data controls | Project lifecycle/standards, project manifest schema, repository registry checks and OFRL-IB-001 data manifest |
| OFRL validation CI | Decimal and semantic validation workflows execute for all Pull Requests on Rust 1.85.0 and 1.99.1 |
| Decimal POC supply chain | Isolated POC Cargo.lock and locked metadata/tree/Clippy/test/docs commands |
| CLI SBOM preview | Preview-only CycloneDX 1.5 SBOM and checksum bundle for the current Linux CLI input validator; not a forecast-model release |
| Calculation engine | Not implemented |
| Canonical ModelRun / provenance | Not implemented |
| Forecast integrity / feasibility | Not implemented |
| XLSX reporting | Not implemented |

The bounded CLI input reader (1 MiB limit, max-plus-one byte read, UTF-8 rejection and regression tests) and the versioned shared Schema/Rust corpus exist on PR #13's branch only. They are not on `main` until PR #13 is synchronized, reviewed, passes all six required contexts and is merged.

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

The isolated POC compares `rust_decimal 1.43.0`, `bigdecimal 0.4.11` and Exact Rational across the same five-year formula path. It records division behaviour, checked overflow and output differences. The current comparison covers selected value magnitudes more strongly than source-scale propagation and arbitrary-precision coefficient growth; these are now explicit Numeric Profile requirements.

Observed stress evidence:

- The POC stress helper multiplies every monetary input field by (10^n); the (10^{18}) and (10^{22}) values are **input scaling multipliers**, not exact intermediate magnitudes or approved company-size limits.
- It also replaces assumptions with deliberately aggressive values: 100% annual growth, 500% COGS / revenue, 500% operating expenses / revenue, 3650 days for DSO/DIO/DPO, 500% CapEx / revenue, and 100% depreciation, interest, tax and dividend rates.
- With FY2025 revenue of 1,350 model units, the FY2026 AR numerator is (2 × 1,350 × 3,650 × 10^n) = (9,855,000 × 10^n): (9.855 × 10^24) at the (10^{18}) multiplier and (9.855 × 10^28) at the (10^{22}) multiplier. The latter exceeds rust_decimal's 96-bit maximum mantissa and triggers checked multiplication overflow at the FY2026 AR numerator; BigDecimal completes the tested path.
- These exact fixture points do not locate the full transition boundary or define a supported business domain. The provisional two-decimal comparison is a POC observation, not the approved reporting or reconciliation policy.

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

**Evidence:** The Numeric Profile remains unapproved and Issue #28 remains open. The current conventions still do not define finite upper bounds for revenue growth, COGS ratio, operating-expense ratio, DSO/DIO/DPO, CapEx ratio, debt drawdowns or interest rate. PR #13 now contains a versioned closed input schema and a candidate-neutral exact-rational derivation tool. Its latest code-tested head emitted 332 opening/historical/forecast formula nodes, translated separate error budgets to reporting currency and derived the /365 error-propagation path; the Numeric Envelope CI job passed 17/17 tests. The committed fixture is explicitly test-only and is not an approved domain.

**Risk:** A candidate may pass selected fixtures and still overflow or lose material precision elsewhere in the admitted domain. An arbitrary-precision type does not solve an unbounded input domain or remove the need for finite CPU, memory, scale and output limits.

**Disposition:** Obtain Model Owner decisions for every unbounded input and opening/historical amount class. Approve source-scale limits separately for monetary values, rates/ratios and days. Derive five-year magnitude, scale and coefficient bounds for every formula node, then complete exact boundary tests and reviewer sign-offs. The 10^18 and 10^22 monetary-input multipliers are artificial capacity probes and must not become business limits by inference.

### EA-02 — Production numeric backend remains undecided

**Severity:** Blocker

**Evidence:** The POC remains isolated and production input/history types use `rust_decimal::Decimal`. BigDecimal 0.4.11 is tested with an 80-digit / HalfEven compile-time default context; this is not a run-scoped production arithmetic policy. BigDecimal's documentation states that multiplication preserves all coefficient digits, so its default division precision does not cap coefficient growth; explicit Context values are required for runtime control of context-aware operations. The current POC operator path does not demonstrate a per-run context across the complete forecast graph. The 10^22 monetary-input multiplier fails rust_decimal's checked multiplication at FY2026 AR numerator, while the 10^18 multiplier passes. These points do not establish the transition boundary. Timing evidence is from one shared 4-vCPU runner and does not measure allocations, peak memory, tail latency or concurrency.

**Risk:** Selecting rust_decimal without approved magnitude and scale envelopes risks checked overflow or precision reduction in a permitted input. Selecting BigDecimal without explicit context ownership, coefficient-digit/scale limits and deterministic serialization risks uncontrolled CPU and memory growth even when division precision is finite. Replacing the existing Decimal type also has a migration cost in the production validator.

**Disposition:** Do not select a production backend yet. After EA-01 and scale propagation are closed, compare candidates against the frozen domain using exact-rational error propagation, operation-specific failure/rounding/scale tests, explicit run-scoped context where applicable, coefficient/resource bounds, controlled memory/allocation/concurrency measurements, serialization fidelity, dependency acceptance and a reviewed migration design. The POC's generic JSON-value forecast routine and string error surface must not be ported directly into production.

### EA-16 — Decimal scale and arbitrary-precision coefficient growth are not bounded

**Severity:** Blocker for Numeric Profile / backend acceptance

**Evidence:** The input grammar permits up to 28 fractional digits per Decimal field, but the formula graph can compound scale and coefficient digits beyond the source limit. PR #13 now has a machine-readable profile contract, exact-rational node derivation and independent scale/coefficient-limit tests. The synthetic test-only fixture demonstrates that the five-year path can reach pre-normalization scale 39 and coefficient counts up to 44 under its illustrative source assumptions; those are tool-validation observations, not production limits. The tool returns explicit FAIL/CONDITIONAL/CANDIDATE-VALIDATION-REQUIRED states rather than silently treating the parser's 28-digit lexical ceiling as a calculation guarantee. BigDecimal's 80-digit POC division setting still does not cap exact multiplication coefficient growth.

**Risk:** A value may remain well below the 96-bit magnitude limit and still exceed the supported scale or require precision reduction. Conversely, BigDecimal may preserve every multiplication digit and grow coefficient size, memory and runtime even when its division context is finite. A magnitude-only Numeric Profile would miss both failure modes.

**Disposition:** Approve separate source-scale limits for monetary values, rates/ratios and days. Derive pre-normalization scale and coefficient-digit bounds at every formula node across all five periods. Test representability and error behaviour just below, at and above scale/mantissa/coefficient boundaries. For BigDecimal, bound coefficient growth independently from division precision and prove resource limits under repeated and concurrent runs. No scale reduction or rounding may be silently treated as exact.

### EA-03 — Independent accounting/formula sign-off remains open

**Severity:** Blocker

**Evidence:** BM-002/BM-003 provide 800 exact synthetic expected values and independent computation. The benchmark methodology itself states that it implements the published V1 equations; it is not an accounting opinion.

**Risk:** A consistently implemented but conceptually inappropriate convention can still reconcile perfectly. Examples requiring explicit review include average-debt interest timing, same-period cash settlement, tax floor, dividend floor, aggregate PP&E depreciation cap, working-capital day basis and treatment of funding shortfall.

**Disposition:** Obtain a documented independent review of model assumptions, equations, statement classifications, sign conventions, edge outcomes and stated non-compliance boundary. Review must identify reviewer role, scope, findings and disposition. Do not describe the work as independent review until it occurs.

### EA-04 — Dependency/licence/source acceptance is incomplete

**Severity:** Blocker before production dependency approval

**Evidence:** PRs #20 and #22 are merged to `main` through PR #14's governance/supply-chain sequence. The isolated Decimal Contract POC lockfile and locked CI are now on main. The preview-only CLI SBOM/checksum workflow is also on main and covers the current CLI input validator for one Linux target. Neither establishes source-level licence/notice acceptance or security disposition for an official distributable release.

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

**Disposition:** Tracked for a separate implementation increment in [Issue #23](https://github.com/witthaya-quant/Open-Finance-Research-Lab/issues/23). Introduce a typed deserialization/error-path boundary with stable code mapping and RFC 6901 JSON Pointer capture. Review any additional dependency and lockfile/SBOM impact before adoption. Add regression tests for nested values, missing/unknown fields, invalid enums, array indices, pointer escaping and malformed Decimal values.

### EA-07 — JSON Schema and Rust contract parity is not enforced

**Severity:** Major

**Evidence:** PR #13 now contains a versioned shared positive/negative corpus consumed by both AJV and the Rust runtime. Cases declare schema validity, runtime status and stable Rust diagnostic codes. The Rust test now validates corpus metadata, unique case IDs/fixture paths, canonical fixture paths under `fixtures/`, and rejects path traversal/symlink escape. AJV validation checks schema identity, corpus shape, expected outcome types and fixture containment. The shared corpus implementation first passed the five project-specific contexts on head `6941b1f`. After governance integration, PR #13 was synchronized with current `main`; the refreshed branch then passed all six required contexts, including `repository-policy`, on Rust 1.85.0 and 1.99.1. The corpus remains branch-local and is not merged to `main`.

**Risk:** A shared fixture set can still be incomplete or can drift if one side does not enforce the same metadata and expected outcome contract. Schema-invalid fixtures must never be accepted by the runtime; schema-valid fixtures may still correctly fail cross-field/accounting semantic checks.

**Disposition:** Complete the current two-toolchain CI run, preserve the intentional structural-versus-semantic distinction, and add future cases whenever the versioned contract changes. Do not claim parity for all possible JSON documents from a finite fixture set.

### EA-08 — Reconciliation tolerances are proposed, not approved

**Severity:** Major

**Evidence:** `VALIDATION_MATRIX.md` proposes (10^{-6}+10^{-8}\times\max(|A|,|B|,1)), but it is not tied to an approved numeric domain or financial materiality. `NUMERIC_PROFILE_DERIVATION.md` now specifies an operation-level error-propagation method (addition/subtraction, multiplication, fixed-constant division, min/max and five-year state recurrence). The existing POC comparison counts two-decimal display mismatches and calculates exact maximum absolute candidate/reference error, but the tests do not assert an approved maximum error threshold.

**Risk:** A relative tolerance grows with balance size. At 10^12 model units the proposed relative component alone is 10^4 model units; with a thousands scale that is 10 million reporting-currency units. Conversely, a two-decimal display match can hide non-zero internal arithmetic error. Neither is proof of financial immateriality.

**Disposition:** After bounds are approved, derive local operation error bounds for each candidate and propagate them through all five years. Define separate budgets for arithmetic approximation, accounting identity, schedule roll-forward and display/export; express each in model units and reporting currency at the approved range endpoints. Freeze the policies before inspecting production PASS/FAIL results and never widen a budget simply to clear a failed test.

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

### EA-15 — Required repository-policy context and main-branch workflow integration

**Severity:** Major — RESOLVED on 2026-11-04

**Evidence:** PR #14 (including the reviewed PR #18 / #20 / #22 sequence) was merged to `main` as `cc2d01b3ffd1d63d32fe87430967a4a86ecbffaa`. The active Ruleset ID `26516806` was re-read after merge and still contains exactly the six intended GitHub Actions contexts. The repository-policy workflow is present on main and the two OFRL project workflows no longer use path filters. Documentation-only PR #24 and project-code PR #27, both targeting the post-merge main, each reported and passed all six required contexts. Both temporary smoke PRs were closed without merging.

**Original risk:** A required context that is absent from the protected base workflow set can leave PRs blocked. Weakening the Ruleset would have concealed the publication-order problem.

**Disposition:** RESOLVED. Keep all six contexts unchanged. Repeat the API read-back and representative documentation/project-code smoke tests after material workflow or Ruleset changes. PR #13 has now synchronized with current main and its refreshed branch passed all six required contexts. It remains Draft pending review; that review is separate from the governance dependency.

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
| Numeric Profile | Approved finite magnitude, source-scale, intermediate-scale/coefficient and resource bounds; formula-chain headroom; precision, rounding and separate error budgets | OPEN — schema and candidate-neutral derivation tooling implemented; Model Owner limits, source-scale decisions and reviewer approvals remain TBD |
| Scale / coefficient propagation | Five-year operation-scale graph and candidate coefficient/resource limits | TOOLING PASS — 332 formula nodes and division-error propagation; test-only fixture is not an approved domain; candidate-specific boundary evidence remains OPEN |
| Production arithmetic backend | Profile-based comparison, explicit run-scoped context, serialization and controlled resource evidence | OPEN — evaluation protocol added; no backend selected; resource measurements and approved profile are still required |
| Independent finance/accounting review | Written independent review and disposition of equations, conventions and edge cases | OPEN — not evidenced |
| Rust dependencies | Locked graphs, target-scoped SBOM, source/licence/notice review and advisory disposition | PARTIAL — lockfile and preview SBOM workflow are on main; source-level licence/security and official release review remain open |
| Independent arithmetic goldens | BM-002/BM-003: 800 exact outputs in open PR #11 | IMPLEMENTED IN OPEN PR — research evidence only |
| Schema/Rust parity | Shared versioned corpus consumed by AJV and Rust | PASS ON PR #13 CODE-TESTED HEAD — shared corpus and path-safety tests passed both Rust toolchains; latest code-tested head passed all six required contexts; unmerged |
| Typed diagnostics | Stable error variants and JSON Pointer for deserialization failures | OPEN — message-substring mapping remains |
| Repository required checks | All six Ruleset contexts available on main and reported on fresh PRs | PASS — post-merge read-back plus documentation-only PR #24 and project-code PR #27 both passed 6/6 |
| Production forecast engine | Typed Rust schedules and three statements | NOT STARTED — gated |
| Forecast integrity/feasibility | SI/SF/reportability results over all periods | NOT STARTED |
| ModelRun provenance | Canonical serialization, SHA-256 and metric lineage | NOT STARTED |
| XLSX | Output-only workbook and canonical reconciliation | NOT STARTED |
| Independent reproduction | Clean-checkout reproduction and reviewer evidence | NOT STARTED |

## 8. Audit conclusion and implementation order

The next implementation increment should not be a speculative production forecast engine. First close the evidence and contract gaps that would otherwise dictate a second rewrite:

1. Obtain Model Owner input-domain decisions for every currently unbounded driver and opening/historical amount class; approve source-scale limits separately for monetary values, rates/ratios and days.
2. Calculate the five-year magnitude, scale and coefficient envelope at every formula node, including multiplication-before-division intermediates.
3. Complete arithmetic error propagation and controlled CPU/memory/allocation/concurrency evidence, then make the backend decision only against the approved profile.
4. Close lockfile, source, licence and advisory dispositions for dependencies intended to remain.
5. Obtain independent finance/accounting review of V1 conventions and formulas.
6. Complete and maintain the shared Schema/Rust contract corpus in PR #13; separately replace message-substring deserialization classification with typed error/path mapping.
7. Freeze the production calculation/result contracts only after all Phase 2 gates, then implement the Rust schedules and statements.
8. Add SI/SF/reportability checks and property/boundary tests.
9. Implement canonical ModelRun, deterministic fingerprints and formula lineage.
10. Generate XLSX solely from canonical outputs and reconcile it.
11. Complete clean-checkout reproduction and release review.

The exact Numeric Profile tool and backend evaluation protocol improve decision readiness, but they do not supply missing business-domain inputs or reviewer approvals. The 17/17 tool tests and 6/6 required checks apply to the code-tested PR #13 head 044f0224; the current documentation-only head bd99fc44 is undergoing its own CI rerun. No research POC, SBOM preview or green CI run is to be represented as production financial-model approval. Governance controls are merged; Numeric Profile, backend selection and independent Finance/Accounting approval remain open. This audit authorizes work only within OFRL-IB-001.
