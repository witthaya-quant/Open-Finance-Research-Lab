<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Decision Log

This log records material scope and modelling decisions. Proposed items are not final until approved by the project owner.

| ID | Decision | Status | Rationale / consequence |
|---|---|---|---|
| D-001 | Model a generic non-financial operating company first | Proposed | Avoid mixing fundamentally different financial-statement structures in the first release |
| D-002 | Use three annual historical periods and five annual forecast periods | Proposed | Provides historical context while keeping the first model manageable |
| D-003 | Use a synthetic example dataset initially | Proposed | Avoids licensing, redistribution, and issuer-data normalization issues during model design |
| D-004 | Use an explicit single-currency and single-scale model instance | Proposed | Prevents silent unit and currency mixing |
| D-005 | Use an aggregate PP&E schedule without disposals | Proposed | Keeps the first release focused on statement integration; asset-vintage accounting is deferred |
| D-006 | Use average debt for interest calculation | Proposed | A transparent approximation for intra-year drawdowns and repayments |
| D-007 | Use same-period cash settlement for interest and tax | Proposed | Avoids interest payable and tax payable schedules in the baseline |
| D-008 | Do not auto-fund negative cash | Proposed | Exposes a funding gap instead of hiding it through a circular debt plug |
| D-009 | Separate Structural Integrity from Scenario Feasibility | Proposed | A model can balance arithmetically yet remain unfunded |
| D-010 | Do not claim IFRS, US GAAP, or local statutory compliance | Proposed | The model is an analytical learning framework, not a framework-specific accounting system |
| D-011 | Implement the financial calculation engine, validation logic, and command-line tool in Rust; generate Excel reports from model outputs | Proposed | Build practical Rust engineering capability through a real financial modelling problem while keeping the model useful to finance practitioners |
| D-012 | Use versioned JSON/CSV with decimal-string amounts as the canonical first-release input/output contract | Proposed | Avoids introducing Excel binary floating-point precision boundaries into the authoritative calculation path |
| D-013 | Keep the calculation engine and authoritative validation suite in Rust for the initial release | Proposed | Maintain one reference implementation for the model's financial logic; use other tools only where a specific task justifies them |
| D-014 | Start with one Rust library crate and one CLI crate in a Cargo workspace | Proposed | Keep architecture extensible without premature multi-crate fragmentation |
| D-015 | Treat crate selection as a proof-of-concept decision, not a popularity contest | Proposed | Validate precision, MSRV, licence, maintenance, security, and compatibility before adoption |

## Open questions for approval

1. Are the proposed three historical and five forecast annual periods appropriate for the first release?
2. Should the first workbook use USD thousands as its illustrative default, while remaining configurable?
3. Is the generic non-financial operating-company profile the right baseline entity?
4. Are the simplified tax, interest-payment, dividend, and PP&E assumptions acceptable for a teaching and modelling framework?
5. Is the Rust calculation engine the canonical model, with Excel generated as an analyst-facing report and no Python requirement in v1?
6. Are JSON/CSV decimal-string inputs and outputs acceptable as the canonical reproducible data contract?
7. Should the initial Cargo workspace begin with one library crate and one CLI crate, with further crate extraction deferred until stable boundaries emerge?

## Change control

When a proposed decision is approved, change its status and record the approval date. When a decision changes, document the previous position, new position, reason, and affected validation tests.