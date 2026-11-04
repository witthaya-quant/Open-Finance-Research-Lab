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
- [Shared Schema/Rust contract corpus](../../fixtures/contracts/ofrl-ib-001-shared-corpus.v0.1.0.json)

## Current branch and approval boundaries

The current `main` branch contains the input contract, historical semantic validator, repository governance controls, project registry/data controls, unfiltered Rust validation workflows, the isolated Decimal Contract POC lockfile, and a preview-only SBOM/checksum workflow for the existing Linux CLI input validator. The preview workflow does not represent a released forecast model.

PR #13 remains an open Draft. It contains the bounded CLI input reader, shared Schema/Rust contract corpus, engineering audit, system design, roadmap, Numeric Profile worksheet and symbolic numeric-envelope derivation. Those changes are not merged production capability. The branch has now been synchronized with current `main`, content conflicts were reconciled, and the refreshed code branch passed all six required checks. PR #13 remains Draft and unmerged pending review.

The Ruleset on `main` is active with six exact required contexts. Documentation-only and project-code smoke PRs (#24 and #27) each passed all six contexts and were closed without merging.

No production arithmetic backend or Numeric Profile has been approved. Independent Finance/Accounting review and source-level dependency licence/security acceptance remain open. The Production Forecast Engine, forecast Structural Integrity / Scenario Feasibility, canonical ModelRun provenance and XLSX reporting remain unimplemented.
