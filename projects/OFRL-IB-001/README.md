<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Integrated Three-Statement Financial Model

> **Division:** Financial Modelling & Valuation  
> **Project ID:** OFRL-IB-001  
> **Status:** Input contract and runtime semantic validator CI-verified; comparative numeric POC CI-verified; production numeric foundation not selected; forecast engine not started  
> **Evidence maturity:** 0 / Proposed  
> **Specification version:** 0.1.0  
> **Project owner:** Kenny

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

- [Research specification](RESEARCH_SPECIFICATION.md)
- [Model conventions](MODEL_CONVENTIONS.md)
- [Validation matrix](VALIDATION_MATRIX.md)
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
- [Full-formula numeric foundation comparison POC](poc/model-numeric-envelope/README.md)


## Current stage

**No production financial calculation engine or analyst-facing workbook has been released.** The Rust input-validation library and CLI are implemented and pass the required CI matrix on Rust 1.85.0 and 1.99.1. This is an input-acceptance and historical-accounting validation milestone, not a completed forecast model.

The Draft 2020-12 schema, deterministic synthetic input fixture, Rust runtime validator, and CLI are implemented. The comparative numeric POC tests candidate arithmetic, not the financial formula implementation independently. The production calculation engine, forecast statement generation, Scenario Feasibility evaluation, and workbook remain future milestones.

## Out of scope for the first release

Banks, insurers, REITs, consolidated groups, multi-currency operations, leases, pensions, share-based compensation, deferred tax, tax-loss carryforwards, acquisitions, disposals, revolving credit facilities, cash sweeps, and DCF/LBO/M&A valuation.

## Data and licensing

The first reproducible case will use synthetic data created specifically for this project. No company financial statements or licensed market-data feeds will be redistributed. Code is licensed under Apache-2.0; project documentation is licensed under CC BY 4.0. See the repository licensing and data policies.