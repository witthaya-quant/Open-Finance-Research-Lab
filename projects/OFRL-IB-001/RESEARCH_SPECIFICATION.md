<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Research Specification

**Project:** Integrated Three-Statement Financial Model  
**Division:** Financial Modelling & Valuation  
**Specification version:** 0.1.0  
**Status:** Proposed — requires founder approval before implementation  
**Evidence maturity:** Level 0 — Proposed  
**Owner:** Kenny, Founder

---

## 1. Executive summary

OFRL-IB-001 will develop a driver-based financial model for a generic non-financial operating company. It will integrate the Income Statement, Balance Sheet, and indirect Cash Flow Statement through clearly defined operating assumptions and supporting schedules.

The project treats model integrity as a first-class requirement. Every material line should have a traceable source, assumption, or calculation. The model must reconcile across statements and pass pre-defined structural and numerical tests before its outputs are described as internally consistent.

The first release will use three annual historical periods and five annual forecast periods. A synthetic case will provide a lawful, deterministic baseline for testing and reproduction. Real-company data integration is deliberately excluded from the initial release.

The project is Rust-first. A Rust-native calculation engine is the canonical implementation of financial mechanics, and a Rust validation suite is the primary engineering deliverable. A Rust CLI will run deterministic scenarios from versioned JSON/CSV inputs. An analyst-facing Excel workbook will be generated as an interoperability and reporting artifact from canonical Rust outputs. Python is not required by the first release. No Rust code, workbook, calculation engine, or empirical result is claimed to exist at this stage.

## 2. Research question

**Primary research question**

Can a compact, driver-based financial model produce internally consistent projected financial statements while making its assumptions, schedules, formula dependencies, and reconciliation checks sufficiently transparent for independent review?

**Secondary questions**

1. Can operating drivers explain the forecast movement of revenue, margins, working capital, capital expenditure, depreciation, and debt?
2. Can the three statements be linked without hidden balancing plugs or circular references?
3. Can automated invariants identify broken links, invalid inputs, and economically infeasible funding outcomes?
4. Can a synthetic example and documented test suite allow an independent reviewer to reproduce the model outputs?

This is a model-design and validation project. It is not a hypothesis that the resulting projections will predict actual company performance.

## 3. Objectives and acceptance intent

The project will:

- Build an explicit, driver-based operating forecast.
- Link operating performance to working capital, PP&E, debt, retained earnings, and cash.
- Produce all three projected financial statements from a common assumption set.
- Preserve a visible distinction between input data, assumptions, formulas, and outputs.
- Test accounting identities and schedule roll-forwards automatically.
- Detect negative cash and other funding gaps without silently inserting financing plugs.
- Document limitations and excluded accounting complexity.
- Provide a reproducible synthetic case and independent validation procedure.

The model must not be made to balance by inserting an unexplained “Other”, “Cash Plug”, or “Balance Sheet Adjustment” line.

## 4. Scope

### 4.1 Entity profile

The baseline entity is one generic, non-financial operating company with a conventional operating cycle, trade receivables, inventory, trade payables, property, plant and equipment, and interest-bearing debt.

The baseline is not designed for banks, insurers, REITs, funds, holding companies with complex investments, or businesses whose economics require materially different statement structures.

### 4.2 Periods and units

| Dimension | Proposed baseline |
|---|---|
| Historical periods | 3 annual periods |
| Forecast periods | 5 annual periods |
| Frequency | Annual |
| Currency | Configurable; USD is the illustrative default |
| Display scale | Configurable; USD thousands is the illustrative default |
| Day-count convention | 365 days for working-capital calculations |
| Forecast start | Immediately after the latest historical balance-sheet date |
| Example dataset | Synthetic, generated specifically for this project |

The model should store currency and scale as explicit metadata. It must not mix currencies or unit scales within a calculation.

### 4.3 In-scope schedules

1. Revenue and operating-cost forecast.
2. Working capital: accounts receivable, inventory, and accounts payable.
3. PP&E: gross asset cost, accumulated depreciation, net PP&E, CapEx, and depreciation.
4. Debt: opening balance, drawdowns, principal repayments, closing balance, and interest expense.
5. Income tax under the simplified assumptions in this specification.
6. Dividends and retained earnings.
7. Integrated Income Statement, Balance Sheet, and indirect Cash Flow Statement.
8. Model integrity checks, scenario checks, and audit outputs.

### 4.4 Explicit exclusions

The first release excludes:

- Banking, insurance, REIT, fund, and other financial-institution statement structures.
- Group consolidation, minority interests, complex segment reporting, and intercompany eliminations.
- Multi-currency translation and foreign-exchange remeasurement.
- Leases, pensions, share-based compensation, goodwill impairment, and complex intangible-asset schedules.
- Deferred tax, tax-loss carryforwards, uncertain tax positions, and detailed tax depreciation.
- Acquisitions, disposals, asset revaluations, and purchase-price allocation.
- Revolving credit facilities, cash sweeps, minimum-cash debt funding loops, and circular interest calculations.
- Equity issuance, share buybacks, and complex dividend policies.
- Quarterly or monthly modelling.
- DCF, DDM, LBO, M&A, and comparable-company valuation.
- Live data connections or redistribution of third-party financial datasets.

An excluded item must not be approximated silently. If later introduced, it requires an explicit scope extension and additional validation criteria.

## 5. Model architecture

The proposed logical dependency chain is:

1. **Historical inputs and opening balances**
2. **Operating assumptions**
3. **Revenue and operating expense forecast**
4. **Working-capital schedule**
5. **PP&E and depreciation schedule**
6. **Debt and interest schedule**
7. **Income Statement**
8. **Cash Flow Statement**
9. **Balance Sheet**
10. **Integrity, liquidity, and audit checks**

The statements must be outputs of linked schedules and assumptions. The model must not use the Balance Sheet as an arbitrary balancing mechanism.

The calculation engine must be implemented in Rust. Spreadsheet formulas, if introduced in a later phase, are secondary representations and must reconcile to the canonical Rust calculation engine.

### 5.1 Input classification

Every model value must belong to one of these categories:

- Historical reported input.
- Synthetic example input.
- Forecast assumption.
- Formula-derived value.
- Validation output.

These categories should be visually and programmatically distinguishable in the eventual implementation.

## 6. Proposed mathematical and financial framework

The following equations define the baseline mechanics. All expense and balance amounts are stored as positive values unless a formula explicitly represents a cash outflow or change. Presentation may show expenses in parentheses, but the underlying sign convention must remain consistent.

### 6.1 Revenue and operating performance

For forecast year t:

- Revenue_t = Revenue_(t-1) × (1 + RevenueGrowth_t)
- COGS_t = Revenue_t × COGS_as_pct_of_Revenue_t
- GrossProfit_t = Revenue_t − COGS_t
- OperatingExpenses_t = Revenue_t × OperatingExpense_as_pct_of_Revenue_t
- EBITDA_t = GrossProfit_t − OperatingExpenses_t
- EBIT_t = EBITDA_t − Depreciation_t
- InterestExpense_t = AverageDebt_t × InterestRate_t
- EBT_t = EBIT_t − InterestExpense_t
- TaxExpense_t = max(0, EBT_t × TaxRate_t)
- NetIncome_t = EBT_t − TaxExpense_t

COGS and operating expenses are positive expense values in the calculation layer. Negative growth and negative EBITDA are permitted outcomes. Revenue itself must not be negative in the baseline model.

### 6.2 Working capital

Let DSO, DIO, and DPO denote days sales outstanding, days inventory outstanding, and days payable outstanding.

- AccountsReceivable_t = Revenue_t × DSO_t / 365
- Inventory_t = COGS_t × DIO_t / 365
- AccountsPayable_t = COGS_t × DPO_t / 365
- NetWorkingCapital_t = AccountsReceivable_t + Inventory_t − AccountsPayable_t
- ChangeInNWC_t = NetWorkingCapital_t − NetWorkingCapital_(t-1)

The indirect Cash Flow Statement reflects the working-capital cash impact as negative ChangeInNWC. Positive growth in operating net working capital consumes cash; a reduction releases cash.

### 6.3 PP&E and depreciation

The baseline uses an aggregate PP&E schedule, not an asset-by-asset fixed-asset register.

- CapEx_t = Revenue_t × CapEx_as_pct_of_Revenue_t
- GrossPPE_t = GrossPPE_(t-1) + CapEx_t
- AverageGrossPPE_t = (GrossPPE_(t-1) + GrossPPE_t) / 2
- Depreciation_t = min(AverageGrossPPE_t × DepreciationRate_t, NetPPE_(t-1) + CapEx_t)
- AccumulatedDepreciation_t = AccumulatedDepreciation_(t-1) + Depreciation_t
- NetPPE_t = GrossPPE_t − AccumulatedDepreciation_t

The cap on depreciation prevents the aggregate schedule from depreciating more than the available opening net book value plus additions. It is a simplified modelling convention, not a substitute for a statutory fixed-asset register or asset-vintage depreciation calculation.

No disposals, revaluations, impairments, or asset-class lives are included in the first release.

### 6.4 Debt and interest

- AverageDebt_t = (OpeningDebt_t + ClosingDebt_t) / 2
- ClosingDebt_t = OpeningDebt_t + DebtDrawdowns_t − PrincipalRepayments_t
- InterestExpense_t = AverageDebt_t × InterestRate_t

Debt drawdowns and repayments are explicit assumptions. Principal repayments cannot exceed opening debt plus current-year drawdowns. The average-balance method assumes financing changes occur evenly through the year.

The model has no automatic revolver, cash sweep, or circular funding mechanism. If projected cash becomes negative, the model must report a funding-feasibility failure rather than inventing debt or equity.

### 6.5 Income tax and dividends

- TaxExpense_t = max(0, EBT_t × TaxRate_t)
- Dividends_t = DividendPayoutRatio_t × max(0, NetIncome_t)
- RetainedEarnings_t = RetainedEarnings_(t-1) + NetIncome_t − Dividends_t

The baseline assumes cash tax paid equals tax expense in the same period. It does not model tax payable, deferred tax, or loss carryforwards. Dividends are limited to the chosen payout ratio applied to positive net income; no discretionary distribution in loss-making periods is modelled in this version.

### 6.6 Indirect Cash Flow Statement

- CashFromOperations_t = NetIncome_t + Depreciation_t − ChangeInNWC_t
- CashFromInvesting_t = −CapEx_t
- CashFromFinancing_t = DebtDrawdowns_t − PrincipalRepayments_t − Dividends_t
- NetChangeInCash_t = CashFromOperations_t + CashFromInvesting_t + CashFromFinancing_t
- ClosingCash_t = OpeningCash_t + NetChangeInCash_t

The baseline classifies interest expense and its assumed same-period cash payment within operating activities because Net Income already includes the interest expense. Dividends are financing cash flows. This is an explicit model policy, not a claim that the model complies with every presentation choice available under IFRS or US GAAP.

### 6.7 Balance Sheet

Baseline assets:

- Cash
- Accounts receivable
- Inventory
- Net PP&E

Baseline liabilities:

- Accounts payable
- Interest-bearing debt

Baseline equity:

- Share capital
- Retained earnings

The core balance-sheet invariant is:

**TotalAssets_t − TotalLiabilities_t − TotalEquity_t = 0**

Share capital is held constant in the first release. Retained earnings are rolled forward using net income and dividends. No unexplained balancing account is permitted.

## 7. Historical data and forecast hand-off

Three historical periods will be supplied as normalized inputs. Each historical period must include the relevant Income Statement, Balance Sheet, and Cash Flow Statement lines needed by the baseline model.

Historical statements are not recalculated from forecast assumptions. They are treated as source inputs and subjected to validation:

- Historical Balance Sheet balances.
- Net income agrees between the Income Statement and Cash Flow Statement.
- Closing cash on the Cash Flow Statement agrees with Balance Sheet cash.
- Cash-flow opening, movement, and closing balances reconcile.
- Required line items and units are consistently mapped.

The latest historical closing Balance Sheet becomes the opening position for the first forecast year. The first forecast period must use the same currency, scale, fiscal-year convention, and account definitions.

If real-company inputs are added in a later version, source, reporting framework, restatement status, units, and normalization decisions must be recorded. The initial synthetic case must be labelled synthetic wherever it appears.

## 8. Key assumptions and controls

All assumptions must be explicit and period-specific where applicable:

- Revenue growth.
- COGS as a percentage of revenue.
- Operating expenses as a percentage of revenue.
- DSO, DIO, and DPO.
- CapEx as a percentage of revenue.
- Depreciation rate.
- Debt drawdowns and principal repayments.
- Interest rate.
- Tax rate.
- Dividend payout ratio.

Input validation must reject non-finite values, invalid period mappings, negative operating balances where not permitted, impossible debt repayments, and percentages outside their defined domains. Negative revenue growth, negative EBIT, and negative net income remain valid economic outcomes.

## 9. Outputs

The first release should provide:

- Three projected financial statements.
- Supporting operating, working-capital, PP&E, and debt schedules.
- Revenue growth, gross margin, EBITDA margin, EBIT margin, and net margin.
- DSO, DIO, DPO, and net working capital.
- Closing cash, total debt, net debt, and Net Debt / EBITDA where mathematically meaningful.
- Statement-integrity checks and a funding-feasibility flag.
- An assumptions register and a clear source / formula / output distinction.

Undefined ratios must be shown as N/A with a reason, not as infinity, a misleading zero, or an unhandled error.

## 10. Research and implementation method

The project will proceed in controlled stages:

1. Approve this specification, model conventions, and Rust-first architecture.
2. Define a versioned JSON/CSV input schema with decimal-string monetary values and create a deterministic synthetic reference case.
3. Run a focused Rust Decimal and serialization proof-of-concept; document precision, range, rounding, overflow, and conversion boundaries.
4. Implement the Rust domain model, operating forecast, and supporting schedules as deterministic functions.
5. Integrate the three statements and implement typed validation outcomes.
6. Build unit, integration, property-based, golden, and boundary tests in Rust.
7. Build a thin Rust CLI for scenario execution and validation reports.
8. Generate an analyst-facing XLSX report from canonical Rust outputs and reconcile displayed values within documented export tolerances.
9. Document results, limitations, dependency decisions, reproduction steps, and release status.

The Rust workspace, crate selection, schema, and workbook export policy are specified in [RUST_ARCHITECTURE.md](RUST_ARCHITECTURE.md). Dependencies remain provisional until the proof-of-concept and licence/maintenance review are complete.

## 11. Validation and pass/fail criteria

The authoritative acceptance criteria are in [VALIDATION_MATRIX.md](VALIDATION_MATRIX.md). They must be agreed before results are examined and must not be weakened after a test fails merely to obtain a pass.

Three outcomes must be reported separately:

- **Structural Integrity:** do the statements and schedules reconcile?
- **Scenario Feasibility:** does the scenario avoid an unfunded negative-cash position under the stated no-revolver assumptions?
- **Research Release Readiness:** are methodology, data rights, documentation, and reproduction requirements satisfied?

A mathematically balanced model can still fail scenario feasibility. A scenario that produces positive cash can still fail structural integrity.

## 12. Risks and limitations

Principal risks include:

- Simplified working-capital drivers may not represent complex operating cycles.
- Aggregate depreciation does not replace asset-level depreciation accounting.
- Same-period tax and interest cash assumptions may not represent actual payment timing.
- Average-debt interest is an approximation when financing changes materially within a year.
- A single-entity model omits consolidation, segment, currency, and group-structure issues.
- Synthetic data demonstrates mechanics, not forecasting accuracy or external validity.
- A balanced model can still contain incorrect assumptions or economically implausible forecasts.

These limitations must remain visible in the workbook and project report.

## 13. Deliverables

Proposed deliverables:

1. Rust Cargo workspace containing the canonical calculation library and a thin CLI.
2. Versioned JSON/CSV input schema and synthetic reference dataset.
3. Rust implementation of operating forecasts, schedules, and integrated financial statements.
4. Rust validation suite covering unit, integration, property-based, golden, serialization, and boundary tests.
5. Deterministic validation report showing each PASS/FAIL result and any warnings.
6. Analyst-facing XLSX reporting workbook generated from canonical Rust outputs.
7. User documentation, assumptions register, dependency record, and reproduction instructions.
8. Technical report describing methodology, implementation choices, tests, limitations, and future extensions.

No deliverable is considered complete solely because a file exists, the project compiles, or a workbook opens.

## 14. Definition of done

The project may be described as **Reproducible** only when:

- All structural integrity hard gates pass.
- Historical inputs and forecast outputs reconcile.
- The synthetic case is reproducible from documented inputs.
- The documented Rust validation suite passes, including required property and boundary tests.
- cargo fmt --all -- --check, the agreed Clippy command, and the full workspace test command pass in the documented environment.
- Decimal serialization round-trips without precision loss in the canonical JSON/CSV path.
- The XLSX report reconciles to canonical Rust outputs within the declared display tolerance.
- Funding-feasibility results and any failures are disclosed.
- All material assumptions and limitations are documented.
- No restricted or unlicensed data or third-party material is included.
- An independent reviewer can follow the reproduction instructions.

Robustness validation and independent replication are later evidence-maturity stages, not implied by the first release.

## 15. References and accounting basis

The first release will not claim compliance with IFRS, US GAAP, or a national statutory reporting framework. Accounting presentation assumptions are specified in [MODEL_CONVENTIONS.md](MODEL_CONVENTIONS.md). If the project later targets a formal framework, it must cite the authoritative standards, document applicable effective dates, and obtain appropriate technical review.

This specification is a project design document, not accounting, audit, tax, or investment advice.