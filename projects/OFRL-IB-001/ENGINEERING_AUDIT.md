<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Engineering Audit

**Audit version:** 1.2.4  
**Audit date:** 2026-11-09  
**Production baseline:** `main` at `a02fe250cffac1e8efe86059da422f688443a2db`  
**Research evidence:** PR #11, head `120ccd3414fb7018011ea3991800728f86736319`  
**Schema-tooling evidence:** PR #12, head `c38f9ad50ff4ff39f92d6cf7514a6a917e6971cd`  
**System design, CLI hardening, typed diagnostics and Numeric Profile tooling:** PR #13 merged to `main` at `593d04d125522e5ec42bea06da6b94c3c2abf604` from validated source head `17b77e385f9980ff00c6ab1466361a54a5d34d23`. All six required Ruleset contexts, Numeric Envelope derivation and Release Candidate Preview passed on that exact source head. Workflows: `41980695313`, `41980695585`, `41980695359`, `41980695504`; artifacts: Numeric Envelope `13393328236`, Release Candidate Preview `13394285657`. The source checkpoint and merge commit are distinct and recorded separately.  
**Status:** Post-merge evidence-based audit at `main` `a02fe250cffac1e8efe86059da422f688443a2db`; prior input hardening and test-only numeric tooling remain merged, and PR #43 has added feature-aware dependency-path evidence. Numeric Profile approval, independent Finance/Accounting review, package-level legal acceptance, backend selection and production forecast authorization remain separate open gates.

## 1. Executive assessment

OFRL-IB-001 has progressed beyond a basic input-validator prototype. The main branch contains a versioned input contract, a strict Decimal-string boundary, historical accounting validation, a deterministic synthetic case, a Rust library/CLI and a two-toolchain CI matrix.

The open research PR #11 materially improves the numeric evidence base: it contains a five-year independent exact-rational baseline, four independent edge scenarios, a standard-library Python reference that checks 800 exact expected outputs, candidate arithmetic comparisons and point-in-time CycloneDX/RustSec evidence. Its runtime-cost harness is observational and non-gating. PR #11 remains Draft and does not select a production arithmetic backend. PR #12 remains Draft and contains the isolated schema-validator toolchain lockfile.

Repository governance and supply-chain controls are now on `main` following PR #14, which integrated the reviewed PR #18 / #20 / #22 sequence. The active Ruleset and its six required checks were re-read after merge. Documentation-only and project-code smoke PRs (#24 and #27) each reported and passed all six exact required contexts, then were closed without merge. These changes improve repository controls; they do not create a production forecast engine.

The system is **not yet a production forecast model**. The production Rust workspace validates inputs and historical relationships, including bounded CLI input transport, typed deserialization diagnostics and the shared Schema/Rust contract corpus. It does not calculate the five forecast years, produce a canonical ModelRun, evaluate Structural Integrity / Scenario Feasibility for calculated results, produce deterministic provenance, or generate an XLSX report. PR #13 delivered the merged engineering foundation; Numeric Profile approval and forecast implementation remain gated.

The principal remaining risk is not simply a lack of code. It is the unresolved boundary between (a) evidence that software follows the currently written equations and (b) approval that those equations, conventions, numeric limits and reporting treatments are appropriate for the declared modelling purpose. The implementation must preserve that distinction.

## 2. Scope, branch state and evidence rules

This audit covers only OFRL-IB-001 in `witthaya-quant/Open-Finance-Research-Lab`. No other project is included.

| Evidence | Current status | Interpretation |
|---|---|---|
| `main` | PR #13 merge `593d04d` | Input/history validator, bounded CLI input reader, typed diagnostics, shared Schema/Rust corpus, Numeric Profile schema/derivation tools, active Ruleset, repository policy workflow, portfolio/data controls, isolated Decimal POC lockfile and preview-only Linux CLI SBOM |
| Ruleset / required contexts | Active; exactly six required contexts | Read back from Rulesets API after merge; no bypass actors; strict up-to-date and conversation-resolution rules |
| PR #24 documentation smoke test | Closed without merge | All six required contexts passed on a documentation-only PR targeting current main |
| PR #27 project-code smoke test | Closed without merge | All six required contexts passed on a project-code PR targeting current main |
| PR #11 — numeric research | Open, Draft; head `120ccd3414` | 800 independent exact-rational expected outputs, candidate arithmetic comparison, point-in-time SBOM/RustSec and non-gating timing; no production backend approval |
| PR #12 — schema tooling | Open, Draft; head `c38f9ad50` | Pinned Node/AJV toolchain and package lockfile; tooling change remains branch-local |
| PR #13 — audit/design/CLI/numeric derivation | Merged to `main` at `593d04d` | 42 changed files; source head `17b77e3`; six required contexts, Numeric Envelope and Release Candidate Preview passed on that source head; Founder Stage 0 disposition recorded transparently in the PR conversation |
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
| CLI baseline | `validate <input.json>` with 1 MiB max-plus-one input bound and invalid UTF-8 rejection; no production forecast or export command |
| Build reproducibility | Production Cargo.lock and two-toolchain CI |\n| Deserialization diagnostics | Contract-owned diagnostic codes, shared Decimal parser, RFC 6901 JSON Pointers and nine versioned mutation fixtures; merged and CI-verified |
| Repository governance | Active main Ruleset, PR requirement, no bypass actors, required conversation resolution and six exact required contexts |
| Repository policy | Always-running `repository-policy` workflow validates local Markdown links, JSON/YAML, project manifests/registry and policy contracts |
| Portfolio/data controls | Project lifecycle/standards, project manifest schema, repository registry checks and OFRL-IB-001 data manifest |
| OFRL validation CI | Decimal and semantic validation workflows execute for all Pull Requests on Rust 1.85.0 and 1.99.1 |
| Decimal POC supply chain | Isolated POC Cargo.lock and locked metadata/tree/Clippy/test/docs commands |
| CLI SBOM preview | Preview-only CycloneDX 1.5 SBOM and checksum bundle for the current Linux CLI input validator; not a forecast-model release |\n| Schema/Rust contract corpus | Versioned positive/negative corpus consumed by AJV and Rust, with fixture path-safety checks; merged and CI-verified |
| Calculation engine | Not implemented |
| Canonical ModelRun / provenance | Not implemented |
| Forecast integrity / feasibility | Not implemented |
| XLSX reporting | Not implemented |

The bounded CLI input reader (1 MiB limit, max-plus-one byte read, UTF-8 rejection and regression tests), typed deserialization diagnostics and versioned shared Schema/Rust corpus are merged on `main` through PR #13. The merge source head was `17b77e3`; all six required contexts, Numeric Envelope derivation and Release Candidate Preview passed on that exact source head. This does not implement forecast calculation or authorize the Numeric Profile.

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

**Main-branch dependency baseline (initially verified 2026-11-06 at `cdbf345c6e27f1cd82f3419b164eb4a65575c09b`; rechecked at `7228bb37bb7a8e93345891dede319a089e8e0132` before PR #34 merge; main now also includes PR #43 feature-path workflow controls at `a02fe250cffac1e8efe86059da422f688443a2db`)**

The application workspace lockfile `projects/OFRL-IB-001/Cargo.lock` has Git blob `21ac76815662e765ee6502da059a312d2e9bc9eb` and contains 54 package records: 52 crates.io registry packages plus the two local workspace packages (`ofrl_cli`, `ofrl_model`). The isolated Decimal Contract POC lockfile contains 56 records: 55 registry packages plus its local POC package. Neither lockfile contains a Git-sourced package. Five package-name families remain at multiple versions in both lockfiles: `getrandom` (0.2.17 / 0.3.4), `rand` (0.8.8 / 0.9.5), `rand_chacha` (0.3.1 / 0.9.0), `rand_core` (0.6.4 / 0.9.5), and `syn` (2.0.119 / 3.0.9). PR #43's exact-head Cargo tree evidence found no active duplicate package identity in the selected Application or Decimal POC resolved dependency graphs under either captured selected-package feature configuration across all target specifications. Lockfile multiplicity is therefore not an active duplicate in those tested graphs; this does not dispose of optional feature combinations outside the selected package set or the eventual approved production graph. The schema 1.3.0 inventory still reconciles 107 scope records to 55 unique `(name, version, source)` identities / 50 package names (52 shared, zero application-only, three POC-only).

The PR #22 preview run `41838318926` was a `pull_request` run on source commit `3ca98dd26de97098f68fa0ca45b92fbb466f1d19`, not a build run on the current `main` commit. Its application lockfile has the same Git blob SHA as the current `main` lockfile, so its target-specific 41-component inventory (40 registry packages plus local `ofrl_model`) remains relevant to that locked dependency graph. It does not establish a fresh build of the current source commit, multi-platform support, source-level legal acceptance, or an official release.

The isolated Decimal POC remains a distinct dependency boundary: its lockfile Git blob is `eb2ecaed4be37129070271108911ae57f03530ba` and contains 56 package records (55 registry packages plus the local POC package), with no Git-sourced packages. Its existing `DEPENDENCY_REVIEW.md` explicitly limits the review to the POC and keeps source-level licence/security review open; it cannot be used as acceptance evidence for the application workspace.

At the pre-IB-T005 main baseline `7228bb37bb7a8e93345891dede319a089e8e0132` on 2026-11-08, `.github/dependabot.yml` scheduled weekly update proposals for the application Cargo workspace, isolated POC, GitHub Actions and npm schema-validator; this is maintenance automation, not advisory or licence clearance. That pre-merge main did not contain a Cargo/RustSec audit workflow. PR #34 subsequently merged the dedicated schema 1.3.0 source-inventory and isolated RustSec workflow into `main` at `7dcc2463fc8f9617576f970499c67223ee0640d9`. PR #11's point-in-time inventory and advisory results remain on its unmerged Draft branch and are not current main evidence.

**Current disposition:** lockfile identity, registry-source classification, target-specific preview SBOM evidence, update automation, and the current selected-package target/feature graph analysis are partially evidenced. PR #43's four duplicate-only views were empty, so none of the five lockfile-level multi-version families is active in the selected Application/POC graphs captured for this review. This is not permission to consolidate lockfile entries and does not cover every possible optional feature of every transitive dependency or the future production graph. Package-source/repository provenance mapping, exact source-archive licence and notice review, advisory disposition for the eventual production graph, and a complete package-level acceptance record remain open. The yanked `xml-rs 0.8.19` exception remains limited to the preview generator and is due for review by 2027-02-04. No production dependency graph is approved while IB-T004 remains gated, and no official release SBOM/checksum or publication authorization exists.

IB-T005's specialist evidence register and source-archive / RustSec automation are recorded in [DEPENDENCY_REVIEW.md](DEPENDENCY_REVIEW.md). The original PR #34 baseline run [#42475893204](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/42475893204) audited source commit `e04d628f01b18488c76617f00d2f03b375ac482a` and remains historical evidence for schema 1.3.0 source inventory. It verified 107/107 registry archive checksums, 107/107 Cargo metadata cross-checks and 107/107 non-empty licence-text candidates; this was a PR-head run rather than a scan triggered on its merge commit.

**PR #43 current dependency-path and advisory evidence:** PR [#43](https://github.com/witthaya-quant/Open-Finance-Research-Lab/pull/43) added feature-aware dependency graphs and duplicate-only graph checks to the existing workflow, then merged to `main` as `a02fe250cffac1e8efe86059da422f688443a2db`. Exact source head: `74e057e7f191e44b6246bb9afa7dff79994121e9`; Dependency Audit run [#42556567844](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/42556567844). Artifact `13647119934` (24,123 bytes; SHA-256 `c893ecd66c14fed571c84b293e5ac339d7056db48597f64499f9be23822dd77c`) was downloaded and independently verified against GitHub's digest. Its four forward `cargo tree --target all --edges features` views cover Application / Decimal POC and default / all features for the packages selected by each manifest; default and all-selected-package-feature trees are byte-identical within each scope (Application 5,207 bytes; POC 4,441 bytes). Four `cargo tree --target all --duplicates` views are present and empty (zero bytes; `has_active_duplicates=false`) across the same scope/feature selections. No package identity is resolved in multiple versions in the selected Application/POC graphs captured by these views; the five multi-version families remain in both lockfiles and are not removed or consolidated. The result does not cover every optional feature of every transitive crate or a future production graph.

The same PR-head run passed RustSec `cargo-audit 0.22.2` against database commit `7a40350e1b8ba5b656d909f88222a8059a83d88d` (commit time `2026-11-08T16:03:42-07:00`): Application 54 dependencies and Decimal POC 56 dependencies, zero vulnerabilities, zero warnings and exit code 0 for each scope. Audit settings used isolated HOME/work paths, canonical database remote, database fetch enabled, no target filters, no ignored advisories and no project/user audit configuration. The run was triggered on PR source head `74e057e...`; it is not a separate scan triggered on the squash-merge commit. The 55 unique identity-level human source/licence decisions, preview-generator exception and official-release gates remain OPEN; the next default-branch scheduled run remains a separate recurring control.

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

**Evidence:** The Numeric Profile remains unapproved and Issue #28 remains open. The current conventions still do not define finite upper bounds for revenue growth, COGS ratio, operating-expense ratio, DSO/DIO/DPO, CapEx ratio, debt drawdowns or interest rate. PR #13 contains a versioned closed input schema and a candidate-neutral exact-rational derivation tool. The validated e18 evidence artifact records 287 arithmetic formula nodes and 474 bound checks, translates separate error budgets to reporting currency, and derives the /365 error-propagation path; the Numeric Envelope suite passed 30/30 tests on that checkpoint. The committed fixture is explicitly test-only and is not an approved domain.

**Risk:** A candidate may pass selected fixtures and still overflow or lose material precision elsewhere in the admitted domain. An arbitrary-precision type does not solve an unbounded input domain or remove the need for finite CPU, memory, scale and output limits.

**Disposition:** Obtain Model Owner decisions for every unbounded input and opening/historical amount class. Approve source-scale limits separately for monetary values, rates/ratios and days. Derive five-year magnitude, scale and coefficient bounds for every formula node, then complete exact boundary tests and reviewer sign-offs. The 10^18 and 10^22 monetary-input multipliers are artificial capacity probes and must not become business limits by inference.

### EA-02 — Production numeric backend remains undecided

**Severity:** Blocker

**Evidence:** The POC remains isolated and production input/history types use `rust_decimal::Decimal`. The PR #11 workflow explicitly sets `RUST_BIGDECIMAL_DEFAULT_PRECISION=80` and `RUST_BIGDECIMAL_DEFAULT_ROUNDING_MODE=HalfEven` at the job/build environment. Its `ArbitraryDecimal::div` wrapper invokes `&self.0 / &rhs.0` directly and does not pass an explicit Context per ModelRun. The [BigDecimal 0.4.11 documentation](https://docs.rs/crate/bigdecimal/0.4.11) states that compile-time precision governs operations that may produce infinite digits while multiplication preserves all coefficient digits; runtime control requires explicit Context-aware operations. The current POC operator path therefore does not demonstrate a per-run fixed-fractional-scale policy. The [rust_decimal 1.43.0 representation](https://docs.rs/crate/rust_decimal/1.43.0) is a 96-bit unsigned integer mantissa with scale 0–28 and a separate sign bit. The 10^22 monetary-input multiplier fails rust_decimal's checked multiplication at FY2026 AR numerator, while the 10^18 multiplier passes. These two probes do not establish the transition boundary. Timing evidence is from one shared 4-vCPU runner and does not measure allocations, peak memory, tail latency or concurrency.

**Risk:** Selecting rust_decimal without approved magnitude and scale envelopes risks checked overflow or precision reduction in a permitted input. Selecting BigDecimal without explicit context ownership, coefficient-digit/scale limits and deterministic serialization risks uncontrolled CPU and memory growth even when division precision is finite. Replacing the existing Decimal type also has a migration cost in the production validator.

**Disposition:** Do not select a production backend yet. After EA-01 and scale propagation are closed, compare candidates against the frozen domain using exact-rational error propagation, operation-specific failure/rounding/scale tests, explicit run-scoped context where applicable, coefficient/resource bounds, controlled memory/allocation/concurrency measurements, serialization fidelity, dependency acceptance and a reviewed migration design. The POC's generic JSON-value forecast routine and string error surface must not be ported directly into production.

### EA-16 — Decimal scale and arbitrary-precision coefficient growth are not bounded

**Severity:** Blocker for Numeric Profile / backend acceptance

**Evidence:** The input grammar permits up to 28 fractional digits per Decimal field, but the formula graph can compound scale and coefficient digits beyond the source limit. PR #13 now emits exact outward-rounded coefficient-integer upper bounds and digit counts per arithmetic node, then runs a candidate-specific static capacity evaluator. On its explicitly test-only synthetic fixture, the validated e18 CI artifact derives 287 arithmetic nodes. The `rust_decimal 1.43.0` screen returns `STATIC_BOUND_EXCEEDS_LIMIT` on 37 nodes: 37 conservative coefficient upper bounds exceed the exact 96-bit mantissa and 19 exceed scale 28 (these reason counts overlap). This means representability is not proven for the declared conservative bounds; it is not an observed runtime overflow and does not prove that every feasible input fails. Fifty nodes remain CONDITIONAL because division scale/coefficient bounds are unresolved. The `bigdecimal 0.4.11` screen remains CONDITIONAL on the same 50 unresolved division-dependent nodes; no runtime or operational capacity pass is claimed. These are test-fixture observations, not evidence about the intended V1 company domain or an approved candidate decision. The 30/30 Numeric Envelope test suite includes the exact 96-bit maximum, one integer above/below it, coefficient metadata consistency and the comparison-only-node regression. On the validated PR #13 source checkpoint `e18ca49676ffdaed7ace57e52fd0db4c89536e52`, Numeric Envelope CI uploaded artifact `ofrl-ib-001-numeric-envelope-test-only-e18ca49676ffdaed7ace57e52fd0db4c89536e52` (ID `13391434439`, 24,004 bytes; expires 2026-11-19T05:48:28Z). Its ZIP SHA-256 is `7fa5c4f5c9c850a14aa498dcfd276c840a391ffeb0e088dff2c2e35e6186db81`; the archive and all five internal SHA256SUMS entries were verified. The evidence identifies source head SHA separately from the pull_request merge-ref checkout SHA.

**Risk:** A value may remain within an apparent digit-count range yet exceed the exact 96-bit mantissa. A formula path can also exceed scale 28 before the final value is produced. Conversely, a BigDecimal result that remains below the profile's illustrative ceilings is not accepted until division semantics, coefficient growth, CPU and memory are bounded. A magnitude-only or digit-count-only Numeric Profile would miss these failure modes.

**Disposition:** Obtain Model Owner-approved source precision and finite input bounds for monetary values, rates/ratios and days. Derive pre-normalization scale and exact coefficient-integer bounds at every formula node across all five periods. Test representability and error behaviour just below, at and above scale/mantissa/coefficient boundaries. For BigDecimal, resolve fixed-fractional-scale versus significant-digit context semantics, bound coefficient growth independently from division precision and prove resource limits under repeated and concurrent runs. No scale reduction or rounding may be silently treated as exact.

### EA-17 — Numeric envelope arithmetic graph previously counted comparisons as additions

**Severity:** Major for numeric-envelope evidence  
**Status:** RESOLVED IN PR #13; merged to `main` at `593d04d`

**Evidence:** The previous derivation represented several validator comparisons and statement tie-outs using the same `add(...)` operation used for real arithmetic. This could create artificial intermediate sums and overstate representability requirements. It also reported coefficient digit counts without an exact coefficient-integer bound that could be compared to a fixed-width mantissa.

**Correction on PR #13:** Comparison-only nodes were removed from the arithmetic graph. Actual calculations remain represented as arithmetic nodes; the zero-minus-CapEx cash-outflow bound is explicitly represented, and historical closing debt is bounded by opening debt plus drawdowns under the V1 constraint that repayment cannot exceed available debt. Each decimal-capable node now emits an outward-rounded exact coefficient-integer upper bound at its declared scale. The candidate evaluator compares that integer directly with the exact Rust Decimal maximum, `79228162514264337593543950335`, and separately checks scale 0–28. The test suite asserts that comparison/tie-out nodes are not counted as arithmetic operations.

**Risk addressed:** An envelope may be falsely conservative if comparison checks are treated as arithmetic, or falsely permissive if a 29-digit coefficient is assumed safe solely because it has 29 digits. The exact mantissa value, not digit count alone, determines this fixed-width limit.

**Disposition:** The derivation correction is merged to `main` through PR #13 (`593d04d125522e5ec42bea06da6b94c3c2abf604`), using source head `17b77e385f9980ff00c6ab1466361a54a5d34d23`. The Numeric Envelope suite passed 30/30 tests; the exact-source artifact `13393328236` was downloaded and all five internal SHA-256 entries verified. This closes the identified derivation-tool defect for the tested V1 formula graph only; it does not approve model bounds, division policy, candidate semantics or operational resources. Rerun the suite whenever formulas or profile schema change.

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

**Status:** Remediated and merged to `main` through PR #13 (`593d04d`).

**Finding:** The former validator mapped Serde failures by substring inspection and omitted reliable JSON Pointer paths for nested structural/deserialization failures.

**Remediation:** A typed structural preflight now classifies closed-object shape, required/unknown properties, JSON type mismatches, enum values, and Decimal token/text/scale/range errors using contract-owned error kinds. It builds RFC 6901 pointers with `~` and `/` escaping, array indices, root-document pointers and period context where available. The preflight and Serde adapter share the same Decimal parser. Public diagnostic selection does not inspect Serde display text; malformed JSON remains pathless when no unambiguous field location exists. No new dependency was added.

**Evidence:** `crates/ofrl_model/src/typed_deserialization.rs`, `src/domain.rs`, `src/validation.rs`, and `tests/typed_deserialization_diagnostics.rs`. At validated source checkpoint `e18ca49676ffdaed7ace57e52fd0db4c89536e52`, the semantic-validator workflow passed formatting, Clippy, tests, docs, Case A CLI acceptance and unsupported-schema CLI rejection on Rust 1.85.0 and 1.99.1 (workflow `41976312265`); the six required contexts, Numeric Envelope derivation and Release Candidate Preview also passed on that exact source head. The regression suite consumes nine versioned mutation cases.

**Disposition:** The implementation is merged to `main` through PR #13 (`593d04d125522e5ec42bea06da6b94c3c2abf604`); Issue #23 was automatically closed with all nine acceptance criteria checked. The PR conversation records the Stage 0 single-maintainer disposition and explicitly states that the technical assessment was AI-assisted, not independent human review. This closes EA-06 for the current V1 input contract; future contract changes must extend the mutation and shared-corpus suites.

### EA-07 — JSON Schema and Rust contract parity is not enforced

**Severity:** Major

**Evidence:** The versioned shared positive/negative corpus is merged to `main` through PR #13 (`593d04d`). Cases declare schema validity, runtime status and stable Rust diagnostic codes. Rust validates corpus metadata, unique case IDs/fixture paths, canonical fixture paths under `fixtures/`, and rejects path traversal/symlink escape. AJV validation checks schema identity, corpus shape, expected outcome types and fixture containment. All six required contexts passed on source head `17b77e3`, including both Rust toolchains. The merged corpus provides finite regression coverage; it is not a mathematical proof of equivalence for every possible JSON document.

**Risk:** A shared fixture set can still be incomplete or can drift if one side does not enforce the same metadata and expected outcome contract. Schema-invalid fixtures must never be accepted by the runtime; schema-valid fixtures may still correctly fail cross-field/accounting semantic checks.

**Disposition:** Maintain the merged two-toolchain CI gate, preserve the intentional structural-versus-semantic distinction, and add future cases whenever the versioned contract changes. Do not claim parity for all possible JSON documents from a finite fixture set.

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

**Evidence:** Main supports `validate` only. PR #13 merged bounded input reading (1 MiB max-plus-one) and UTF-8 rejection into `main`; no `run`, `inspect` or `export` commands or safe output-write contract are implemented.

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

**Disposition:** RESOLVED. Keep all six contexts unchanged. Repeat the API read-back and representative documentation/project-code smoke tests after material workflow or Ruleset changes. PR #13 subsequently passed all six contexts on source head `17b77e3` and was merged to `main` at `593d04d`. This is separate from the closed governance dependency.

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
| Numeric Profile | Approved finite magnitude, source-scale, intermediate-scale/coefficient and resource bounds; formula-chain headroom; precision, rounding and separate error budgets | OPEN — schema and derivation path implemented; Issue #28 Model Owner limits and Finance/Engineering approvals remain unresolved |
| Scale / coefficient propagation | Five-year operation-scale graph, exact coefficient bounds and candidate representability | TOOLING PASS — 287 arithmetic nodes, 30/30 tests; test-only fixture: rust_decimal `STATIC_BOUND_EXCEEDS_LIMIT` (37 nodes), BigDecimal CONDITIONAL (50 nodes); no runtime failure or business-domain conclusion |
| Production arithmetic backend | Profile-based comparison, explicit run-scoped context, serialization and controlled resource evidence | OPEN — rust_decimal conservative static upper bounds exceed fixed candidate ceilings in this test-only fixture; BigDecimal division context remains unresolved; no backend selected; runtime/resource evidence not measured |
| Independent finance/accounting review | Written independent review and disposition of equations, conventions and edge cases | OPEN — not evidenced |
| Rust dependencies | Locked graphs, target-scoped SBOM, source/licence/notice review and advisory disposition | PARTIAL — PR #34 schema 1.3.0 source inventory verified 107/107 archive checksums, 107/107 Cargo metadata matches and 107/107 non-empty licence-text candidates; 107 scope records reconcile to 55 unique package identities / 50 names. PR #43 run 42556567844 added four selected-package feature-path views and four duplicate-only views across Application/POC and all target specifications; all duplicate-only views are empty, so no multi-version package identity is active in the selected graphs. The five version families remain in both lockfiles; production-graph review after IB-T004 remains required. The PR #43 exact-head RustSec scan passed with 0 vulnerabilities / 0 warnings in both scopes. All 55 human source/licence decisions, the preview-generator exception and official-release evidence remain OPEN; the audit run was PR-triggered and the scheduled default-branch scan remains separate |
| Independent arithmetic goldens | BM-002/BM-003: 800 exact outputs in open PR #11 | IMPLEMENTED IN OPEN PR — research evidence only |
| Schema/Rust parity | Shared versioned corpus consumed by AJV and Rust | PASS — merged to `main` by PR #13; shared corpus and path-safety tests passed both Rust toolchains on source head `17b77e3` |
| Typed diagnostics | Stable error variants and JSON Pointer for deserialization failures | PASS — typed preflight and nine mutation fixtures merged to `main` by PR #13; future contract revisions require expanded regression cases |
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
6. Maintain and extend the merged Schema/Rust contract corpus and typed diagnostic mutation suite as the versioned input contract evolves.
7. Freeze the production calculation/result contracts only after all Phase 2 gates, then implement the Rust schedules and statements.
8. Add SI/SF/reportability checks and property/boundary tests.
9. Implement canonical ModelRun, deterministic fingerprints and formula lineage.
10. Generate XLSX solely from canonical outputs and reconcile it.
11. Complete clean-checkout reproduction and release review.

The exact Numeric Profile derivation and candidate-capacity tools are merged to `main` through PR #13 (`593d04d125522e5ec42bea06da6b94c3c2abf604`) but do not supply missing business-domain inputs or reviewer approvals. On validated source head `17b77e385f9980ff00c6ab1466361a54a5d34d23`, all six required Ruleset contexts, 30/30 Numeric Envelope tests and the separate Release Candidate Preview passed. Numeric Envelope artifact `13393328236` (24,005 bytes; ZIP SHA-256 `c1ecdb255e2b93bd057860a457f7d439456b64b18b7ccac8052c28171684ed36`) was downloaded and all five internal SHA-256 entries verified. Preview artifact `13394285657` (382,543 bytes; ZIP SHA-256 `ae8748218f83484920103e2b6d20ce524c2029d3c5c8758ee710716c5be55b5f`) was also downloaded and verified; its metadata labels it `PREVIEW_ONLY_NOT_A_RELEASE` and limits scope to CLI input-contract and historical semantic validation. The Numeric Envelope artifact records 287 arithmetic nodes, 37 conservative rust_decimal bound exceedances (including 19 overlapping scale reasons), and 50 conditional nodes; BigDecimal remains conditional on unresolved coefficient/scale/division policy. The profile is explicitly `TEST-ONLY-SYNTHETIC-NOT-APPROVED`, runtime correctness is `NOT_EXECUTED`, resources are `NOT_MEASURED`, no backend is selected, and the Forecast Engine is not authorized. These are static test-fixture observations, not observed runtime overflow or an approved V1 operating domain. No research POC, SBOM preview or green CI run is production financial-model approval. Numeric Profile, backend selection and independent Finance/Accounting approval remain open. This audit authorizes work only within OFRL-IB-001.
