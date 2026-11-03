<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Model Conventions

**Version:** 0.1.0  
**Status:** Proposed  
**Purpose:** Fix the initial model mechanics so the same assumptions produce the same statement linkages and validation expectations.

## 1. Period, currency, and unit conventions

- Annual periods; three historical periods followed by five forecast periods.
- One reporting currency and one display scale per model instance.
- Illustrative defaults: USD and USD thousands.
- Working-capital day calculations use 365 days.
- Period labels and fiscal year-end dates must be explicit.
- All monetary calculations use unrounded values; display rounding must not feed back into formulas.

## 2. Sign conventions

| Item | Internal convention |
|---|---|
| Revenue | Positive |
| COGS and operating expenses | Positive expenses, subtracted in profit calculations |
| Depreciation | Positive expense and positive accumulated-depreciation movement |
| CapEx | Positive investment amount; shown as a negative cash flow |
| Debt drawdown | Positive financing inflow |
| Principal repayment | Positive repayment amount; subtracted from debt and cash |
| Dividends | Positive distribution amount; subtracted from retained earnings and cash |
| Change in net working capital | Positive increase consumes cash |
| Cash flow line items | Positive for inflows, negative for outflows |
| Balance Sheet balances | Positive asset, liability, and equity balances |

A user-facing workbook may present expenses in parentheses, but a single, documented internal convention must be applied consistently.

## 3. Operating forecast

Revenue is projected using a period-specific growth assumption. COGS and operating expenses are forecast using percentages of revenue.

- Revenue = prior-period revenue × (1 + growth rate).
- COGS = revenue × COGS percentage.
- Operating expenses = revenue × operating-expense percentage.
- EBITDA = revenue − COGS − operating expenses.
- EBIT = EBITDA − depreciation.
- EBT = EBIT − interest expense.
- Net income = EBT − tax expense.

COGS excludes depreciation and amortisation. Operating expenses also exclude depreciation and amortisation, interest, and income tax. Interest expense excludes interest income; interest income is not modelled in this release.

COGS and operating expense percentages are non-negative. They are not capped at 100%, because a loss-making operating period is a valid model outcome. Revenue growth may be negative, but revenue cannot become negative.

## 4. Working-capital definitions

- DSO = accounts receivable / revenue × 365, where revenue is positive.
- DIO = inventory / COGS × 365, where COGS is positive.
- DPO = accounts payable / COGS × 365, where COGS is positive.

Forecast balances are calculated from drivers:

- Accounts receivable = revenue × DSO / 365.
- Inventory = COGS × DIO / 365.
- Accounts payable = COGS × DPO / 365.
- Net working capital = accounts receivable + inventory − accounts payable.

If a denominator is zero, any historical derived ratio is reported as N/A rather than infinity or zero. Forecast balances remain calculated from the direct driver equations.

The Cash Flow Statement uses the change in operating net working capital between the latest closing historical position and each forecast closing position.

## 5. PP&E and depreciation

The baseline PP&E schedule is aggregated:

- CapEx = revenue × CapEx percentage.
- Closing gross PP&E = opening gross PP&E + CapEx.
- Average gross PP&E = average of opening and closing gross PP&E.
- Depreciation = the lower of (average gross PP&E × depreciation rate) and (opening net PP&E + CapEx).
- Closing accumulated depreciation = opening accumulated depreciation + depreciation.
- Closing net PP&E = closing gross PP&E − closing accumulated depreciation.

This method is a simplified forecasting convention. It is not an asset-vintage method and does not model disposals, asset classes, impairments, or revaluations.

Depreciation rate and accumulated depreciation must be defined so that accumulated depreciation never exceeds gross PP&E and net PP&E never becomes negative beyond numerical tolerance.

## 6. Debt and interest

- Closing debt = opening debt + drawdowns − principal repayments.
- Principal repayments cannot exceed opening debt plus drawdowns.
- Average debt = (opening debt + closing debt) / 2.
- Interest expense = average debt × interest rate.

The average-balance method approximates intra-year timing by assuming debt movements occur evenly over the period. The first release excludes compounding within the year, fees, capitalised interest, interest payable, and circular cash-sweep mechanics.

Interest expense is assumed paid in the same period. The baseline Cash Flow Statement classifies it within operating cash flow because it is already reflected in net income. This is a disclosed model convention, not a framework-compliance conclusion.

## 7. Tax

- Tax rate is an explicit assumption between 0% and 100%.
- Tax expense = max(0, EBT × tax rate).
- Cash tax paid is assumed equal to tax expense in the same period.

No deferred tax, tax payable, tax-loss carryforward, permanent difference, uncertain tax position, or jurisdiction-specific tax computation is included.

## 8. Dividends and equity

- Dividend payout ratio is an explicit assumption between 0% and 100%.
- Dividends = payout ratio × max(0, net income).
- Retained earnings = opening retained earnings + net income − dividends.
- Share capital remains constant.

No equity issuance, repurchases, other comprehensive income, minority interest, or direct-to-equity adjustment is modelled in the first release.

## 9. Cash Flow Statement

The baseline indirect Cash Flow Statement is:

**Cash from operations**
- Net income
- Add back depreciation
- Less increase / add decrease in operating net working capital

**Cash from investing**
- Capital expenditure as a cash outflow

**Cash from financing**
- Debt drawdowns
- Less principal repayments
- Less dividends

Net change in cash is the sum of operating, investing, and financing cash flows. Closing cash equals opening cash plus net change in cash.

The model does not automatically fund a cash deficit. Negative cash is an explicit funding-feasibility failure unless a later approved scope extension introduces a defined funding source.

## 10. Balance Sheet presentation

The baseline Balance Sheet contains only:

**Assets**
- Cash
- Accounts receivable
- Inventory
- Net PP&E

**Liabilities**
- Accounts payable
- Interest-bearing debt

**Equity**
- Share capital
- Retained earnings

Total assets must equal total liabilities plus total equity. Additional accounts may not be inserted as unexplained balancing items.

## 11. Accounting-framework boundary

The model is a simplified analytical model with an explicit presentation policy. It does not claim to comply with IFRS, US GAAP, Thai Financial Reporting Standards, or another statutory reporting framework.

Interest and dividend cash-flow classifications, tax mechanics, lease accounting, depreciation policy, and other areas can differ by applicable reporting framework and entity facts. A future framework-specific version must identify the authoritative standard, effective date, policy choices, and required review.

## 12. Input domains and validation rules

The initial implementation must validate these proposed input domains before calculation:

| Input / balance | Permitted domain |
|---|---|
| Revenue | Greater than or equal to zero |
| Revenue growth | Greater than or equal to -100% |
| COGS as % of revenue | Greater than or equal to zero; no 100% cap |
| Operating expenses as % of revenue | Greater than or equal to zero; no 100% cap |
| DSO, DIO, DPO | Greater than or equal to zero |
| CapEx as % of revenue | Greater than or equal to zero |
| Depreciation rate | 0% to 100% |
| Debt drawdowns | Greater than or equal to zero |
| Principal repayments | Greater than or equal to zero and no greater than opening debt plus drawdowns |
| Interest rate | Greater than or equal to zero in this release |
| Tax rate | 0% to 100% |
| Dividend payout ratio | 0% to 100% |
| Cash, receivables, inventory, gross PP&E, debt, payables, and share capital | Non-negative balances, except that forecast cash may be negative as a reported funding-feasibility failure |
| Accumulated depreciation | Greater than or equal to zero and no greater than gross PP&E |
| Retained earnings | May be negative |
| All numeric inputs | Finite; NaN and positive / negative infinity are rejected |

A zero denominator in a historical ratio produces N/A with an explanatory note. The model must not silently replace invalid or missing numeric inputs with zero.

## 13. Numerical conventions

- Calculations use full available precision.
- Rounding is presentation-only.
- Monetary reconciliation uses the tolerance in VALIDATION_MATRIX.md.
- Undefined ratios use N/A and a reason.
- Inputs must reject NaN, positive infinity, and negative infinity.
- Model outputs must not silently coerce invalid inputs to zero.

## 14. Canonical numeric and Excel interoperability policy

The Rust calculation engine is authoritative. Monetary calculations use the approved fixed-point Decimal representation and checked operations. Intermediate calculations are not rounded.

Canonical JSON/CSV inputs and outputs represent monetary values and rates as decimal strings, accompanied by currency, scale, period, and schema-version metadata. This is the authoritative reproducibility path.

The Excel workbook is generated for analyst review and presentation. Excel stores and calculates numeric values with 15 significant digits of precision. Therefore:

- The XLSX artifact must declare its display precision and rounding policy.
- The workbook must not be treated as the authoritative store of full-precision model values.
- The export process must not silently convert high-precision Rust Decimal values to binary floating-point and then claim exact equality.
- Canonical decimal-string outputs must be retained alongside the workbook.
- Any future formula-driven workbook must be reconciled against the Rust engine using an explicit tolerance and a documented spreadsheet calculation engine.
- The model must never enable a "precision as displayed" setting as a hidden workaround.

This boundary is a deliberate design decision, not a defect to conceal.
