<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — System Design and Control Architecture

**Design version:** 0.2.0  
**Status:** Proposed target architecture; existing V1 decisions remain authoritative  
**Date:** 2026-11-04  
**Implementation language:** Rust  
**System of record:** Canonical Rust model output plus versioned JSON/CSV

## 1. Design objective

Build a deterministic, auditable and maintainable three-statement modelling system for one generic non-financial operating company, with three annual historical periods and five annual forecast periods.

The system must provide strong financial-model controls without creating unnecessary infrastructure. The first release remains a single Rust Cargo workspace with a model library and a thin CLI. The design does not require a service, database, cloud deployment, plugin framework, model API, Python runtime or separate microservices.

The core invariant is:

> One accepted input package + one versioned model policy + one engine build must produce one reproducible canonical result. Every report and workbook is a projection of that result, not another calculation engine.

## 2. Non-negotiable architecture principles

1. **One calculation authority:** Rust owns every financial formula and authoritative validation.
2. **Immutable inputs:** source history and assumptions are never silently overwritten or inferred.
3. **Typed domain:** money, rates, days, dates, periods, currency and unit scale are not interchangeable primitive values.
4. **Fail closed:** invalid inputs, unsupported policies and arithmetic failures block the stages that depend on them.
5. **No silent repair:** no missing-value defaults, cash plugs, invented debt/equity, hidden clamping or automatic unit conversion.
6. **Separation of outcomes:** input acceptance, calculation completion, structural integrity, scenario feasibility and reportability are separate results.
7. **Deterministic evidence:** canonical outputs are reproducible and have a stable input/model/engine fingerprint.
8. **Audit lineage:** each material output can be traced to its formula, assumptions, prior-period values and source inputs.
9. **Adapter isolation:** JSON, CSV and XLSX are adapters around the domain result; they do not own financial logic.
10. **Small first-release architecture:** keep modules in the existing two crates until an independently reusable boundary justifies extraction.

## 3. Context and trust boundaries

```text
                  ┌────────────────────────┐
                  │ Versioned input package│
                  │ JSON / CSV + provenance│
                  └────────────┬───────────┘
                               │
                  ┌────────────▼───────────┐
                  │ Contract and parsing   │
                  │ lexical + schema gates │
                  └────────────┬───────────┘
                               │
                  ┌────────────▼───────────┐
                  │ Semantic input checks  │
                  │ history + assumptions │
                  └────────────┬───────────┘
                               │
                  ┌────────────▼───────────┐
                  │ Numeric policy gate    │
                  │ range + precision      │
                  └────────────┬───────────┘
                               │
                  ┌────────────▼───────────┐
                  │ Canonical Rust engine  │
                  │ schedules + statements│
                  └────────────┬───────────┘
                               │
              ┌────────────────┴────────────────┐
              │                                 │
    ┌─────────▼─────────┐             ┌─────────▼─────────┐
    │ Structural checks │             │ Feasibility checks│
    │ identity/rollforward│            │ funding/no plug   │
    └─────────┬─────────┘             └─────────┬─────────┘
              └────────────────┬────────────────┘
                               │
                  ┌────────────▼───────────┐
                  │ Canonical ModelRun     │
                  │ values + status + audit│
                  └────────────┬───────────┘
                               │
                ┌──────────────┴──────────────┐
                │                             │
       ┌────────▼────────┐           ┌────────▼────────┐
       │ JSON / CSV      │           │ XLSX report     │
       │ canonical data  │           │ presentation    │
       └─────────────────┘           └─────────────────┘
```

### Trust boundaries

- External JSON/CSV is untrusted input.
- The JSON Schema is a published structural/lexical contract, not proof of accounting validity.
- The Rust typed domain and semantic validator are the runtime calculation boundary.
- Assumptions are user/model inputs, not facts. Their source, owner and rationale must remain visible.
- The calculation engine is the only authority for forecast outputs.
- XLSX is generated output. It must not execute independent forecast formulas.
- No network access is required by the V1 calculation path.

## 4. Target Cargo and module layout

Retain the current Cargo workspace and two-crate design. Add modules only as the implementation reaches them.

```text
projects/OFRL-IB-001/
├── Cargo.toml
├── Cargo.lock
├── ENGINEERING_AUDIT.md
├── SYSTEM_DESIGN.md
├── IMPLEMENTATION_ROADMAP.md
├── MODEL_CONVENTIONS.md
├── MODEL_INPUT_CONTRACT.md
├── NUMERIC_ENVELOPE.md
├── VALIDATION_MATRIX.md
├── VALIDATION_ERROR_CONTRACT.md
├── RUNTIME_VALIDATOR.md
├── crates/
│   ├── ofrl_model/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── domain/
│   │   │   ├── contract/
│   │   │   ├── validation/
│   │   │   ├── numeric/
│   │   │   ├── calculation/
│   │   │   ├── schedules/
│   │   │   ├── statements/
│   │   │   ├── integrity/
│   │   │   ├── results/
│   │   │   └── serialization/
│   │   └── tests/
│   │       ├── unit/
│   │       ├── integration/
│   │       ├── properties/
│   │       ├── golden/
│   │       └── boundary/
│   └── ofrl_cli/
│       └── src/main.rs
└── poc/
    ├── decimal-contract/
    └── model-numeric-envelope/   # isolated research only
```

The layout is a target, not a request to create empty directories. The current validator modules may be reorganized when stable interfaces are known. The numeric POC must remain explicitly excluded from the production workspace.

## 5. Domain model

### 5.1 Strong types

The calculation layer should not accept ambiguous `Decimal` values without domain meaning. Introduce small, reviewed newtypes after the numeric backend is approved:

| Type | Meaning | Invariant |
|---|---|---|
| `Money` | Monetary amount in the model's unit scale | Carries no implicit currency conversion |
| `Rate` | Fractional ratio, e.g. 0.20 means 20% | Domain checked by the relevant assumption |
| `Days` | Day-count driver such as DSO/DIO/DPO | Non-negative and bounded by the approved Numeric Profile |
| `PeriodId` | Stable period identifier | Unique, ordered and mapped to explicit dates |
| `CurrencyCode` | Model reporting currency | Validated against the supported registry when that registry is adopted |
| `UnitScale` | units / thousands / millions | One value per model instance |
| `ModelVersion` | Version of formula/convention set | Explicit and reproducible |

Retained earnings may be negative. Other baseline balance-sheet balances follow the approved V1 domain rules. Negative forecast cash is a calculated diagnostic outcome and must not be accepted as ordinary input cash.

### 5.2 Numeric backend boundary

The production backend remains undecided. Do not let the calculation architecture depend directly on a research-only POC type.

After the numeric decision, the production model should expose a narrow internal arithmetic boundary with:

- parse from canonical decimal text;
- zero and integer constants;
- checked add, subtract, multiply and divide;
- comparison and sign checks;
- canonical decimal-string output;
- explicit scale/precision observations where supported;
- typed overflow, division and scale errors.

Do not assume that a generic arithmetic trait makes two numeric types semantically interchangeable. Division context, scale preservation, formatting and range must be tested for the selected backend. The exact-rational implementation remains a research oracle, not the production monetary type.

## 6. Deterministic execution pipeline

The engine is an ordered pipeline. Each stage has an explicit precondition and status.

| Stage | Input | Output | Failure behaviour |
|---|---|---|---|
| 1. Read | Input path/bytes | Bounded raw document | File/size/encoding error; calculation not run |
| 2. Parse | Raw JSON | Parsed document | Syntax error; calculation not run |
| 3. Contract | Parsed document + schema version | Structurally accepted document | Schema/version failure; calculation not run |
| 4. Deserialize | Contract document | Typed input model | Typed field/path diagnostic; calculation not run |
| 5. Semantic validation | Typed input | Accepted historical data + assumptions | Semantic errors; calculation not run |
| 6. Numeric profile | Accepted input + versioned Numeric Profile | Numeric-safe model context | Outside-envelope or insufficient-headroom error; calculation not run |
| 7. Calculation | Immutable model context | Schedules and statements | Typed arithmetic/calculation failure |
| 8. Structural validation | Calculated statements | SI findings | Calculation can complete; Structural Integrity may FAIL |
| 9. Scenario validation | Calculated cash and explicit financing | SF findings and funding shortfall | Calculation can complete; Scenario Feasibility may FAIL |
| 10. Result assembly | All stage outputs | Canonical ModelRun | Incomplete stage remains NOT_RUN; never imply PASS |
| 11. Export | Canonical ModelRun | JSON/CSV/XLSX artifacts | Export failure does not rewrite canonical calculation results |

A downstream stage cannot run when its prerequisite failed. A warning does not become an error unless a versioned policy says so.

## 7. Calculation module boundaries

All functions should be deterministic, side-effect-free calculations over explicit inputs. Do not hide formulas in the CLI or reporting adapter.

### 7.1 Operating forecast

- Revenue = prior-period revenue × (1 + revenue growth).
- COGS = revenue × COGS percentage.
- Operating expenses = revenue × operating-expense percentage.
- EBITDA = revenue − COGS − operating expenses.
- EBIT = EBITDA − depreciation.
- EBT = EBIT − interest expense.
- Net income = EBT − income tax expense.

Negative EBITDA, EBIT, EBT and net income are valid economic outcomes. They must not be treated as malformed input.

### 7.2 Working-capital schedule

- Accounts receivable = revenue × DSO / 365.
- Inventory = COGS × DIO / 365.
- Accounts payable = COGS × DPO / 365.
- Net working capital = receivables + inventory − payables.
- Change in NWC = current closing NWC − prior closing NWC.

A zero historical denominator produces N/A with a reason. It must not produce infinity or a misleading zero.

### 7.3 PP&E schedule

- CapEx = revenue × CapEx percentage.
- Closing gross PP&E = opening gross PP&E + CapEx.
- Average gross PP&E = (opening gross PP&E + closing gross PP&E) / 2.
- Depreciation = min(average gross PP&E × depreciation rate, opening net PP&E + CapEx).
- Closing accumulated depreciation = opening accumulated depreciation + depreciation.
- Closing net PP&E = closing gross PP&E − closing accumulated depreciation.

This remains an aggregate simplified schedule. No disposals, impairments, revaluations or asset-vintage accounting are implied.

### 7.4 Debt and interest

- Closing debt = opening debt + explicit drawdowns − explicit principal repayments.
- Repayment cannot exceed debt available before repayment.
- Average debt = (opening debt + closing debt) / 2.
- Interest expense = average debt × interest rate.

No revolver, cash sweep, circular funding or invented debt is permitted in V1.

### 7.5 Tax, dividends and equity

- Tax expense = max(0, EBT × tax rate).
- Dividends = payout ratio × max(0, net income).
- Closing retained earnings = opening retained earnings + net income − dividends.
- Share capital remains constant under the current V1 scope.

These are explicit simplified model conventions, not statutory tax or accounting conclusions.

### 7.6 Cash flow and balance sheet

- CFO = net income + depreciation addback − change in operating NWC.
- CFI = −CapEx.
- CFF = debt drawdowns − repayments − dividends.
- Net change in cash = CFO + CFI + CFF.
- Diagnostic closing cash = opening cash + net change in cash.

The Balance Sheet must use the same underlying schedule state and must not insert an unexplained balancing account.

## 8. Result and status contract

The current `ValidationReport` remains an input-validation result. It must not be expanded by simply adding loosely related boolean fields. A separate canonical `ModelRun` should represent completed calculation evidence.

Target top-level result dimensions:

| Dimension | Values | Meaning |
|---|---|---|
| `input_status` | accepted / rejected | Whether the input contract and semantic gates passed |
| `calculation_status` | not_run / completed / failed | Whether all required forecast calculations completed |
| `structural_integrity_status` | not_run / pass / fail | Whether SI hard gates passed |
| `scenario_feasibility_status` | not_run / pass / fail | Whether funding assumptions support the scenario |
| `reportability_status` | not_run / reportable / not_reportable | Whether financial statements can be presented without misleading representation |
| `release_readiness_status` | not_run / pass / fail | Research/reproducibility and release gates; not a scenario result |

A single overall PASS must never hide a failed dimension.

### Negative-cash treatment

When calculated closing cash is negative:

1. Preserve the unadjusted calculated value in a diagnostic field.
2. Calculate funding shortfall as `max(0, -diagnostic_closing_cash)`.
3. Mark Scenario Feasibility FAIL.
4. Do not label negative diagnostic cash as a valid cash asset.
5. Do not force the asset to zero or insert unapproved debt/equity.
6. Mark the forecast Balance Sheet presentation not reportable until a valid funding mechanism is modelled under a versioned scope change.

This separates mathematical traceability from financial statement presentation.

## 9. Structural and scenario validation

Structural checks must be independent of whether a scenario is economically attractive:

- historical and forecast Balance Sheet identities;
- cash roll-forward and cash cross-statement ties;
- net income links;
- NWC cash impact;
- PP&E roll-forward and bounds;
- debt roll-forward and repayment bounds;
- retained-earnings and dividend bounds;
- input completeness, units, domain and formula coverage;
- determinism and absence of circular dependencies;
- explicit prohibition of balancing plugs.

Scenario checks include non-negative funded closing cash and adherence to explicit financing assumptions. A scenario failure is reportable as a model outcome; it is not silently repaired.

Use the Validation Matrix as the source of hard-gate IDs. Any change to a threshold requires a dated, versioned rationale and a complete rerun of affected tests.

## 10. Diagnostic design

External diagnostics must be stable and useful to a finance practitioner and a software consumer.

Each diagnostic should include:

- stable code and category;
- severity;
- concise message;
- JSON Pointer for input errors where possible;
- period identifier and field where relevant;
- expected and actual values when meaningful;
- stage identifier and stable formula identifier for calculation/model findings.

Implementation requirements:

- Map internal typed errors to public codes explicitly.
- Do not classify errors by substring matching third-party messages.
- Capture nested field paths and array indices.
- Keep error messages free of secrets, personal data not needed to identify the issue, and environment-specific absolute paths.
- Sort diagnostics deterministically by stage, period, code and field.
- Keep input rejection distinct from calculation failure and post-calculation findings.

## 11. Formula catalogue and lineage

Each material calculated output should map to a stable formula ID, for example:

| Formula ID | Output | Dependencies |
|---|---|---|
| OP-REV-001 | Revenue | Prior revenue, revenue growth |
| OP-PROFIT-001 | EBITDA | Revenue, COGS, operating expenses |
| WC-AR-001 | Accounts receivable | Revenue, DSO, day-count basis |
| WC-NWC-001 | Net working capital | AR, inventory, AP |
| PPE-DEP-001 | Depreciation | Opening PP&E, CapEx, depreciation rate |
| DEBT-INT-001 | Interest expense | Opening debt, drawdowns, repayments, interest rate |
| TAX-001 | Income tax expense | EBT, tax rate |
| EQ-RE-001 | Retained earnings | Opening retained earnings, net income, dividends |
| CF-CASH-001 | Closing diagnostic cash | Opening cash, CFO, CFI, CFF |
| BS-IDENTITY-001 | Balance Sheet difference | Assets, liabilities, equity |

Lineage should identify the period, formula version, source field or prior-period output, assumption field and canonical result. The formula catalogue is an audit contract, not a second implementation of the formulas.

## 12. Canonical run manifest and determinism

A completed run should include a manifest with at least:

- schema version;
- model ID and model version;
- model conventions version;
- Numeric Profile ID and version;
- numeric backend and arithmetic-context identifier;
- engine version and source commit;
- canonical input SHA-256;
- canonical output SHA-256;
- ordered period set;
- validation and result statuses;
- build/toolchain identity needed for reproduction.

The deterministic run fingerprint should be derived from canonicalized input bytes (or a versioned canonical representation), model version, numeric policy version and engine build identity. Wall-clock timestamps, machine hostnames and file paths must not be included in the canonical deterministic result hash. They may be recorded in a separate execution envelope.

Do not claim reproducibility solely because two JSON files look similar. Define canonical key ordering, string encoding, Decimal formatting, null/omission behaviour and hash inputs.

## 13. Output and workbook architecture

### Canonical outputs

- JSON: complete machine-readable ModelRun, exact Decimal strings, status dimensions, diagnostics and provenance.
- CSV: documented tabular exports for statements, schedules and assumptions, with a stable schema and decimal-string fields.
- XLSX: analyst-facing presentation generated only from canonical ModelRun values.

### XLSX workbook

Target sheets:

1. Cover & Run Status
2. Executive Summary
3. Assumptions Register
4. Income Statement
5. Balance Sheet
6. Cash Flow Statement
7. Working Capital
8. PP&E
9. Debt & Interest
10. Integrity & Feasibility
11. Diagnostics & Provenance

Workbook requirements:

- No forecast formulas that independently recalculate model outputs.
- Clear currency and unit scale on every relevant sheet.
- Consistent sign conventions and display precision.
- N/A with reason for undefined ratios.
- Explicit PASS/FAIL/NOT RUN display.
- Negative cash shown as a funding shortfall diagnostic, not as a reportable asset.
- Canonical decimal-string output retained separately.
- Every displayed key figure traceable to its canonical value.
- Reconciliation tests compare every material displayed item to the canonical ModelRun.
- Excel's numeric precision must not be represented as full-precision canonical storage.

A workbook that merely opens is not a validated workbook.

## 14. CLI design

Keep the CLI small and deterministic. Initial operational commands should be:

```text
ofrl validate <input.json>
ofrl run <input.json> --output <run.json>
ofrl export <run.json> --format xlsx --output <report.xlsx>
ofrl inspect <run.json>
```

The first implementation may deliver commands incrementally. Do not implement export before the ModelRun contract exists.

Requirements:

- stable exit-code contract;
- bounded input size and explicit encoding errors;
- no overwrite of source inputs;
- explicit output-path handling;
- write output to a temporary file then atomically replace the destination after success;
- no network calls or secret material in the calculation path;
- machine-readable diagnostics on standard output and actionable operational errors on standard error.

Avoid adding a CLI framework unless the eventual command surface justifies it.

## 15. Dependency, security and build policy

Before production dependency approval:

- commit Cargo.lock and use `--locked`;
- record direct/transitive package versions and source;
- produce machine-readable SBOMs for the production workspace and isolated POCs;
- review licences, notices, maintenance status and security advisories;
- record all exceptions with owner, rationale, date and expiry/review date;
- avoid unpinned third-party actions and install scripts where feasible;
- keep workflow token permissions least-privileged;
- keep runtime calculation free of network access and secrets.

A point-in-time audit is evidence for the date of the run, not a permanent guarantee.

## 16. Quality and release gates

Minimum automated checks:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo doc --workspace --no-deps
```

The release pipeline must additionally verify:

- shared schema/Rust contract corpus;
- numeric boundaries and headroom;
- property-based invariants;
- independent golden outputs;
- deterministic run fingerprints;
- all SI and SF hard gates;
- canonical JSON/CSV round-trip;
- XLSX workbook structure, precision and reconciliation;
- SBOM, licence and security review;
- documented reproduction from a clean checkout.

The Rust version matrix must be reviewed when dependencies change. A green CI build does not constitute accounting sign-off.

## 17. Explicit non-goals for V1

- Financial institutions, REITs, funds or banking models.
- Consolidation, minority interests, segment consolidation or intercompany elimination.
- Multi-currency translation or FX remeasurement within one model.
- Leases, pensions, share-based compensation, deferred tax or tax-loss carryforwards.
- Asset disposals, impairments, revaluations or asset-vintage depreciation.
- Revolvers, cash sweeps, minimum-cash loops or circular interest.
- Quarterly/monthly forecasting, DCF/DDM/LBO/M&A valuation.
- Real issuer/vendor data ingestion before provenance and data-rights schema are approved.
- A web service, database or multi-tenant deployment.

A non-goal can be added only through a versioned scope change and new validation gates.


## 18. Evidence status and implementation boundary

This design describes the target system, not a claim that all target components exist.

### Implemented on main

- Versioned model input schema, canonical synthetic fixture, Rust typed input and historical semantic validator.
- Bounded CLI input reader (1 MiB max-plus-one read, invalid UTF-8 rejection) and typed deserialization diagnostics with RFC 6901 JSON Pointers.
- Versioned shared Schema/Rust positive/negative corpus, path-safety checks and nine diagnostic mutation fixtures.
- Numeric Profile schema, exact-rational derivation, candidate static-capacity screen and evaluation protocol (tooling only; no profile approval or backend selection).
- Active repository Ruleset with exactly six required GitHub Actions contexts.
- Always-running repository policy workflow and project/portfolio registry validation.
- OFRL validation workflows that run without path filters on Rust 1.85.0 and 1.99.1.
- Isolated Decimal Contract POC Cargo.lock and locked validation commands.
- Preview-only CycloneDX 1.5 SBOM and checksum workflow for the existing Linux CLI input validator. This is not a forecast-model release.

The governance/supply-chain sequence was integrated through PR #14 (including PRs #18, #20 and #22) as main commit `cc2d01b3ffd1d63d32fe87430967a4a86ecbffaa`. Post-merge documentation-only and project-code smoke PRs both reported and passed all six required contexts; both were closed without merging.

### Merged implementation evidence

PR #13 was merged to `main` at `593d04d125522e5ec42bea06da6b94c3c2abf604` from validated source head `17b77e385f9980ff00c6ab1466361a54a5d34d23`. It delivered:

- Engineering Audit and this System Design.
- Bounded CLI input reader (1 MiB limit, max-plus-one read, invalid UTF-8 rejection).
- Versioned positive/negative Schema/Rust shared contract corpus and fixture path-safety checks.
- Typed structural preflight, shared Decimal parser and nine mutation diagnostics fixtures.
- Implementation Roadmap and Decision Log updates.
- Numeric Profile Decision Worksheet, symbolic derivation, division-error propagation and candidate-specific static capacity evaluator.
- Numeric Backend Evaluation Protocol.

All six required Ruleset contexts passed on source head `17b77e3`, together with Numeric Envelope derivation and Release Candidate Preview (workflow runs `41980695313`, `41980695585`, `41980695359`, `41980695504`). The Numeric Envelope artifact was verified against all five internal SHA-256 entries. The preview artifact is marked `PREVIEW_ONLY_NOT_A_RELEASE` and validates CLI input-contract/history only. The source head and merge commit are deliberately recorded as different identifiers.

The Founder Stage 0 single-maintainer disposition is recorded in the PR conversation. It was executed by ChatGPT at the repository owner's explicit instruction and is not independent human approval.

### Open research and approval gates

- PR #11 remains Draft and contains independent exact-rational arithmetic research, candidate comparison, 800 expected outputs, point-in-time SBOM/RustSec research and non-gating timing observations.
- PR #12 remains Draft and contains schema validator tooling/lockfile work.
- No Numeric Profile or production arithmetic backend has been approved.
- Independent Finance/Accounting approval and source-level dependency licence/security acceptance are not evidenced.
- Production forecast calculations, Structural Integrity / Scenario Feasibility evaluation, canonical ModelRun provenance and XLSX generation are not implemented on main.

The Numeric Profile and independent Finance/Accounting review are mandatory before production forecast calculations begin. A green CI run or preview SBOM is not model-policy approval or an official software release.
