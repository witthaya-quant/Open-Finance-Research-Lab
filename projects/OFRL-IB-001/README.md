<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Integrated Three-Statement Financial Model

> **Division:** Financial Modelling & Valuation  
> **Project ID:** OFRL-IB-001  
> **Status:** Rust runtime semantic validator implemented and CI-verified; forecast calculation engine not started  
> **Evidence maturity:** 0 / Proposed  
> **Specification version:** 0.1.0  
> **Project owner:** Kenny  
> **Project manifest:** [project.json](project.json)  
> **Project standards:** [Portfolio-wide requirements](../../PROJECT_STANDARDS.md)

## Purpose

Build an integrated financial model for a generic non-financial operating company. The model will connect the Income Statement, Balance Sheet, and Cash Flow Statement through explicit assumptions and supporting schedules.

The calculation engine and validation suite will be implemented in Rust. The model is intended to be useful to finance practitioners as well as software developers: the financial logic should be easy to follow, while the implementation should be testable and reproducible.

## Research question

Can a driver-based financial model produce internally consistent projected financial statements while making its assumptions, calculation flow, and reconciliation checks clear enough for another person to review?

## Planned implementation

- **Rust model library:** financial calculations, schedules, and statement integration.
- **Rust validation suite:** unit, integration, property-based, boundary, and golden tests.
- **Rust command-line tool:** run a scenario, validate it, and produce a report.
- **JSON / CSV:** documented input and output formats, with decimal-string monetary values.
- **Excel:** an analyst-facing report generated from the model results.
- **Additional tools:** considered only when they serve a defined purpose, such as independent comparison or data interoperability.

The Rust implementation is the reference for financial calculations. Excel is a reporting interface, not a second source of truth.

## Scope

- Three annual historical periods and five annual forecast periods.
- One generic non-financial operating company.
- Revenue and operating-cost drivers.
- Working-capital schedule using DSO, DIO, and DPO.
- Aggregate PP&E, CapEx, and depreciation schedule.
- Debt drawdown, repayment, and interest schedule.
- Simplified tax and dividend assumptions.
- Integrated Income Statement, Balance Sheet, and indirect Cash Flow Statement.
- Synthetic example inputs for deterministic testing.

The model is not a statutory reporting engine, audit opinion, tax calculation system, or representation of compliance with a particular accounting framework.

## Project documents

- [Project manifest](project.json)
- [Project lifecycle](../../PROJECT_LIFECYCLE.md)
- [Project standards](../../PROJECT_STANDARDS.md)

- [Research specification](RESEARCH_SPECIFICATION.md)
- [Model conventions](MODEL_CONVENTIONS.md)
- [Validation matrix](VALIDATION_MATRIX.md)
- [Data manifest](DATA_MANIFEST.md)
- [Decision log](DECISION_LOG.md)
- [Engineering architecture](ENGINEERING_ARCHITECTURE.md)
- [Model input contract](MODEL_INPUT_CONTRACT.md)
- [Numeric envelope](NUMERIC_ENVELOPE.md)
- [Numeric Profile worksheet](NUMERIC_PROFILE_WORKSHEET.md)
- [Numeric Profile derivation](NUMERIC_PROFILE_DERIVATION.md)
- [Numeric backend evaluation protocol](NUMERIC_BACKEND_EVALUATION.md)
- [Numeric Profile input schema](../../schemas/ofrl-ib-001-numeric-profile-0.1.0.schema.json)
- [Numeric envelope derivation tool](tools/numeric-envelope/derive.cjs) (analysis tooling only; not part of the Rust runtime)
- [Validation and error contract](VALIDATION_ERROR_CONTRACT.md)
- [Rust runtime semantic validator](RUNTIME_VALIDATOR.md)
- [Synthetic reference case A](SYNTHETIC_REFERENCE_CASE.md)
- [Model input JSON Schema](../../schemas/ofrl-ib-001-model-input-0.1.0.schema.json)
- [Synthetic Case A input fixture](../../fixtures/synthetic/ofrl-ib-001-case-a.input.v0.1.0.json)
- [Schema validation script](../../scripts/validate-ofrl-ib-001-model-input-schema.sh)
- [Decimal contract POC](poc/decimal-contract/README.md)
- [Dependency review](poc/decimal-contract/DEPENDENCY_REVIEW.md)
- [Release candidate preview workflow](../../.github/workflows/ofrl-ib-001-release-candidate.yml) (preview artifacts only; not a public release)


## Current stage

**No production financial calculation engine or analyst-facing workbook has been released.** The Rust input-validation library and CLI are implemented and pass the required CI matrix on Rust 1.85.0 and 1.99.1. This is an input-acceptance and historical-accounting validation milestone, not a completed forecast model.

The Draft 2020-12 schema, deterministic synthetic input fixture, Rust runtime validator, and CLI are implemented. A preview workflow can build the input validator and produce an SBOM/checksum bundle; this does not change the project's `not_released` state. The calculation engine, forecast statement generation, Scenario Feasibility evaluation, and workbook remain future milestones.

## Out of scope for the first release

Banks, insurers, REITs, consolidated groups, multi-currency operations, leases, pensions, share-based compensation, deferred tax, tax-loss carryforwards, acquisitions, disposals, revolving credit facilities, cash sweeps, and DCF/LBO/M&A valuation.

## Data and licensing

The first reproducible case will use synthetic data created specifically for this project. No company financial statements or licensed market-data feeds will be redistributed. Code is licensed under Apache-2.0; project documentation is licensed under CC BY 4.0. See the repository licensing and data policies.


## Engineering and evidence documents

- [Engineering Audit](ENGINEERING_AUDIT.md)
- [System Design and Control Architecture](SYSTEM_DESIGN.md)
- [Implementation Roadmap](IMPLEMENTATION_ROADMAP.md)
- [Numeric Profile Decision Worksheet](NUMERIC_PROFILE_WORKSHEET.md)
- [Numeric Profile Derivation and Backend Decision Basis](NUMERIC_PROFILE_DERIVATION.md)
- [Numeric Backend Evaluation Protocol](NUMERIC_BACKEND_EVALUATION.md)
- [Shared Schema/Rust contract corpus](../../fixtures/contracts/ofrl-ib-001-shared-corpus.v0.1.0.json)

## Current implementation and approval boundaries

The engineering foundation described in this README is merged to `main` through PR #13 at `593d04d125522e5ec42bea06da6b94c3c2abf604`. The validated PR source head was `17b77e385f9980ff00c6ab1466361a54a5d34d23`; all six required Ruleset contexts, Numeric Envelope derivation and Release Candidate Preview passed on that exact source head. The Release Candidate Preview is a CLI input-validator preview only, not a released forecast model.

The merged implementation includes the bounded CLI input reader (1 MiB max-plus-one read and invalid UTF-8 rejection), the versioned shared Schema/Rust contract corpus, typed deserialization diagnostics with RFC 6901 JSON Pointers, the audit/design/roadmap documents, Numeric Profile schema and worksheet, exact candidate-neutral derivation, division-error propagation, candidate-specific static capacity evaluator and Numeric Backend Evaluation Protocol.

### Repository layout

The project files are separated by responsibility rather than collected in one undifferentiated directory:

```text
.github/workflows/                         CI and preview workflows
fixtures/contracts/                        Versioned positive, negative and mutation fixtures
schemas/                                   Machine-readable JSON Schemas
scripts/                                   Schema/corpus validation entry points
projects/OFRL-IB-001/
  *.md                                     Project specifications, audit, decisions and policies
  crates/ofrl_cli/                         CLI transport and command boundary
  crates/ofrl_model/
    src/                                   Typed domain, diagnostics and validation
    tests/                                 Shared-corpus and diagnostic regression tests
  tools/numeric-envelope/
    *.cjs                                  Exact derivation and candidate-capacity tools
    test/                                  Tool tests and test-only synthetic profile
```

The 42 files delivered by PR #13 follow these categories: CI workflows; contract fixtures; project documentation and decision records; Rust CLI/model implementation; Rust regression tests; numeric-envelope tools/tests; the machine-readable profile schema; and the AJV validation script. No unrelated project directory was included.

### Numeric evidence and approval boundary

The validated test-only Numeric Envelope artifact (`13393328236`) contains 287 arithmetic nodes. The `rust_decimal 1.43.0` static screen reports `STATIC_BOUND_EXCEEDS_LIMIT` on 37 conservative nodes (19 scale-related reasons overlap coefficient reasons); BigDecimal remains `CONDITIONAL` on 50 division-dependent nodes. The profile is explicitly `TEST-ONLY-SYNTHETIC-NOT-APPROVED`; runtime correctness is `NOT_EXECUTED`, operational resources are `NOT_MEASURED`, no backend is selected and the Production Forecast Engine is not authorized. These results are tooling evidence, not a business-domain recommendation or an observed runtime overflow.

Issue #23 was automatically closed when PR #13 merged. Issue #28 remains OPEN for Model Owner decisions. Finite input bounds, source precision, intermediate scale/coefficient limits, separate error budgets and CPU/memory/concurrency limits are still unresolved. Independent Finance/Accounting review and source-level dependency licence/security acceptance also remain open.

The Production Forecast Engine, forecast Structural Integrity / Scenario Feasibility, canonical ModelRun provenance and XLSX reporting remain unimplemented.
