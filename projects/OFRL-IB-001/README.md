<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: Apache-2.0 -->

# OFRL-IB-001 — Integrated Three-Statement Financial Model

> **Division:** Financial Modelling & Valuation  
> **Project ID:** OFRL-IB-001  
> **Status:** Proposed — specification stage  
> **Evidence maturity:** 0 / Proposed  
> **Specification version:** 0.1.0  
> **Founder:** Kenny

## Purpose

Develop a transparent, driver-based three-statement model for a generic non-financial operating company, with a Rust-native calculation engine at its centre.

The objective is to make financial mechanics traceable, assumptions inspectable, and accounting relationships testable. Rust is the canonical calculation and validation implementation—not a secondary port of a spreadsheet model.

## Research question

Can a Rust-native, deterministic calculation engine generate internally consistent projected financial statements while exposing the assumptions, schedule linkages, and reconciliation checks required for independent review?

## Rust-first delivery model

- **Rust calculation engine:** canonical implementation of financial formulas and statement integration.
- **Rust validation suite:** unit, integration, property-based, boundary, and golden tests.
- **Rust CLI:** deterministic scenario execution and validation reporting.
- **JSON / CSV:** canonical first-release input and exact decimal-string output formats.
- **Excel workbook:** analyst-facing reporting and interoperability artifact generated from the Rust model.
- **Python:** not required by the first release.

The spreadsheet must not become an undocumented second source of truth. Any future interactive spreadsheet calculation must be reconciled against the Rust engine.

## Scope at a glance

- Three annual historical periods and five annual forecast periods.
- One generic non-financial operating entity.
- Revenue and operating-cost drivers.
- Working-capital schedule using DSO, DIO, and DPO.
- Aggregate PP&E, CapEx, and depreciation schedule.
- Debt drawdown, repayment, and interest schedule.
- Simplified tax and dividend assumptions.
- Integrated Income Statement, Balance Sheet, and indirect Cash Flow Statement.
- Synthetic example inputs for deterministic testing.

The model is not a statutory reporting engine, audit opinion, tax calculation system, or representation of compliance with a particular accounting framework.

## Project documents

1. [Research Specification](RESEARCH_SPECIFICATION.md)
2. [Model Conventions](MODEL_CONVENTIONS.md)
3. [Rust-First Technical Architecture](RUST_ARCHITECTURE.md)
4. [Validation Matrix](VALIDATION_MATRIX.md)
5. [Decision Log](DECISION_LOG.md)

## Current phase

**Specification only. No Rust code, calculation engine, or workbook has been released.**

The next gate is approval of the financial rules and architecture, followed by a small Rust Decimal and serialization proof-of-concept before the model engine is implemented.

## Out of scope for the first release

Banks, insurers, REITs, consolidated groups, multi-currency operations, leases, pensions, share-based compensation, deferred tax, tax-loss carryforwards, acquisitions, disposals, revolvers, cash sweeps, and DCF/LBO/M&A valuation.

## Data and licensing

The first reproducible case will use synthetic data created for this project. No company financial statements or licensed market-data feeds will be redistributed. This repository currently has no explicit content or software licence; see the root-level LICENSE_POLICY.md.