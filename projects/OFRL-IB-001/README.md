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

## Current branch and approval boundaries

The current `main` branch contains the input contract, historical semantic validator, repository governance controls, project registry/data controls, unfiltered Rust validation workflows, the isolated Decimal Contract POC lockfile, and a preview-only SBOM/checksum workflow for the existing Linux CLI input validator. The preview workflow does not represent a released forecast model.

PR #13 is Open / Ready for Review and remains unmerged. It contains the bounded CLI input reader, shared Schema/Rust contract corpus, typed deserialization preflight with RFC 6901 JSON Pointer diagnostics, engineering audit, system design, roadmap, Numeric Profile schema/worksheet, exact candidate-neutral derivation tool, division-error propagation, a candidate-specific static capacity evaluator and the Numeric Backend Evaluation Protocol.

The latest code-tested candidate-capacity increment passed all six required Ruleset contexts and 30/30 Numeric Envelope tests on the branch. The test-only fixture emits 287 arithmetic nodes. Static screening reports `rust_decimal 1.43.0` `STATIC_BOUND_EXCEEDS_LIMIT` on 37 nodes (conservative coefficient and/or scale 0–28 upper bounds) and `bigdecimal 0.4.11` CONDITIONAL on 50 nodes because the fixture leaves division scale unresolved. The Rust result means capacity is not proven for the conservative bound; it is not an observed runtime overflow. These are test-only static-capacity observations, not a production-domain or backend decision. The BigDecimal POC's 80-significant-digit compile-time context is not equivalent to the current fixed-fractional-scale Numeric Profile policy.

The typed-diagnostics implementation and nine versioned mutation fixtures passed the full semantic-validator matrix on latest code head `516a434610d98f3b92b8d2356d596d078e3b50de`, including formatting, Clippy, tests, documentation, canonical Case A CLI acceptance and unsupported-schema CLI rejection on Rust 1.85.0 and 1.99.1. All six required Ruleset contexts, Numeric Envelope derivation and the separate Release Candidate Preview also passed on that exact head. The preview generated a binary-scoped CycloneDX SBOM and immutable build/checksum bundle; it is a preview artifact, not an official forecast-model release.

OFRL is in Governance Stage 0 — Founder-maintained. Under `GOVERNANCE.md` sections 3 and 6, the Founder / Principal Maintainer may review and merge their own PR when no independent Maintainer exists; that must be recorded as a single-maintainer review, not an independent approval. PR #13 is Ready for Review and remains branch-local until merged.

The Ruleset on `main` is active with six exact required contexts. Documentation-only and project-code smoke PRs (#24 and #27) each passed all six contexts and were closed without merging.

No production arithmetic backend or Numeric Profile has been approved. Independent Finance/Accounting review and source-level dependency licence/security acceptance remain open. The Production Forecast Engine, forecast Structural Integrity / Scenario Feasibility, canonical ModelRun provenance and XLSX reporting remain unimplemented.
