# OFRL-IB-001 — Integrated Three-Statement Financial Model

> **Division:** Financial Modelling & Valuation  
> **Project ID:** OFRL-IB-001  
> **Status:** Proposed — specification stage  
> **Evidence maturity:** 0 / Proposed  
> **Specification version:** 0.1.0  
> **Founder:** Kenny

## Purpose

Develop a transparent, auditable three-statement model for a generic non-financial operating company. The model will connect the Income Statement, Balance Sheet, and Cash Flow Statement through explicit operating assumptions and supporting schedules.

The objective is not merely to produce a workbook that calculates. It is to make the financial mechanics traceable, the assumptions inspectable, and the accounting relationships testable.

## Research question

Can a compact, driver-based financial model produce internally consistent projected financial statements while exposing every material assumption, schedule linkage, and reconciliation check required for independent review?

## Scope at a glance

- Three annual historical periods supplied as reported or explicitly synthetic inputs.
- Five annual forecast periods.
- One generic non-financial operating entity.
- Revenue and operating-cost drivers.
- Working-capital schedule using DSO, DIO, and DPO.
- Aggregate PP&E, CapEx, and depreciation schedule.
- Debt drawdown, repayment, and interest schedule.
- Simplified tax and dividend assumptions.
- Integrated Income Statement, Balance Sheet, and indirect Cash Flow Statement.
- A synthetic example dataset for reproducible testing.

The model is an analytical and educational framework. It is not a statutory reporting engine, audit opinion, tax calculation system, or representation that a company complies with a particular accounting framework.

## Project documents

1. [Research Specification](RESEARCH_SPECIFICATION.md) — question, scope, equations, assumptions, and proposed deliverables.
2. [Model Conventions](MODEL_CONVENTIONS.md) — accounting mechanics, sign conventions, and formula definitions.
3. [Validation Matrix](VALIDATION_MATRIX.md) — pre-defined structural, numerical, scenario, and release gates.
4. [Decision Log](DECISION_LOG.md) — explicit decisions, open questions, and changes.

## Current phase

**Specification only. No model code or workbook has been released.**

The next gate is founder review of the scope and accounting conventions. Implementation should begin only after the specification and pass/fail criteria are accepted.

## Out of scope for the first release

Banks, insurers, REITs, consolidated groups, multi-currency operations, leases, pensions, share-based compensation, deferred tax, tax-loss carryforwards, acquisitions, disposals, revolvers, cash sweeps, and DCF/LBO/M&A valuation are excluded from the first release. These require separate accounting and modelling choices and should not be hidden inside a generic model.

## Data and licensing

The first reproducible example will use synthetic data created for this project. No company financial statements or licensed market-data feeds will be redistributed. This repository currently has no explicit content or software licence; see the root-level LICENSE_POLICY.md.

## Project status language

- Proposed: specification and acceptance criteria are under review.
- Exploratory: implementation exists, but results are provisional.
- Reproducible: documented steps reproduce the reported outputs.
- Robustness Validated: defined robustness and failure tests are complete.
- Independently Replicated: an independent reviewer reproduces the principal outputs.

A status label describes evidence maturity, not profitability or accounting-framework compliance.