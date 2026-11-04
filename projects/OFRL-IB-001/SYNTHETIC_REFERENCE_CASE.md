<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Synthetic Reference Case A

**Case ID:** OFRL-IB-001-CASE-A  
**Status:** Canonical input fixture, Draft 2020-12 schema validation and Rust historical semantic validation implemented/CI-verified; production forecast outputs and golden results remain pending  
**Data class:** Synthetic; generated specifically for OFRL validation  
**Reporting currency:** USD  
**Unit scale:** Thousands  
**Periods:** FY2023–FY2025 historical; FY2026–FY2030 forecast

## 1. Purpose

Case A is a small, transparent non-financial operating-company example designed to exercise the V1 statement links and schedule roll-forwards.

It is not based on a real issuer, does not represent expected market returns, and must never be labelled as reported financial data.

The case is intentionally simple: one entity, one currency, one scale, no disposals, no acquisitions, no leases, no tax-loss carryforwards, and no automatic funding mechanism.

## 2. Opening Balance Sheet

Opening position as at 2022-12-31, before FY2023:

| Account | USD thousands |
|---|---:|
| Cash | 1,000.0 |
| Accounts receivable | 120.0 |
| Inventory | 80.0 |
| Gross PP&E | 600.0 |
| Accumulated depreciation | 200.0 |
| Net PP&E | 400.0 |
| Accounts payable | 50.0 |
| Interest-bearing debt | 300.0 |
| Share capital | 800.0 |
| Retained earnings | 450.0 |

Opening Balance Sheet check:

- Total assets = 1,000 + 120 + 80 + 400 = 1,600.
- Total liabilities = 50 + 300 = 350.
- Total equity = 800 + 450 = 1,250.
- Liabilities plus equity = 1,600; the opening Balance Sheet balances.

## 3. Historical Income Statement

All expense values are positive inputs and subtracted by the model.

| USD thousands | FY2023 | FY2024 | FY2025 |
|---|---:|---:|---:|
| Revenue | 1,000.0 | 1,200.0 | 1,350.0 |
| COGS | 400.0 | 480.0 | 540.0 |
| Operating expenses | 250.0 | 300.0 | 337.5 |
| Depreciation | 30.0 | 35.0 | 40.0 |
| Interest expense | 15.0 | 14.0 | 11.5 |
| Income tax expense | 61.0 | 74.2 | 84.2 |
| Net income | 244.0 | 296.8 | 336.8 |

Net income is equal to revenue less COGS, operating expenses, depreciation, interest expense, and income tax expense for each historical year.

## 4. Historical Balance Sheet

| USD thousands | FY2023 | FY2024 | FY2025 |
|---|---:|---:|---:|
| Cash | 1,154.0 | 1,317.8 | 1,493.6 |
| Accounts receivable | 130.0 | 144.0 | 162.0 |
| Inventory | 90.0 | 96.0 | 108.0 |
| Gross PP&E | 650.0 | 710.0 | 780.0 |
| Accumulated depreciation | 230.0 | 265.0 | 305.0 |
| Net PP&E | 420.0 | 445.0 | 475.0 |
| Accounts payable | 60.0 | 72.0 | 81.0 |
| Interest-bearing debt | 280.0 | 230.0 | 180.0 |
| Share capital | 800.0 | 800.0 | 800.0 |
| Retained earnings | 654.0 | 900.8 | 1,177.6 |

Balance Sheet tie-outs:

- FY2023 assets = 1,154 + 130 + 90 + 420 = 1,794; liabilities and equity = 60 + 280 + 800 + 654 = 1,794.
- FY2024 assets = 1,317.8 + 144 + 96 + 445 = 2,002.8; liabilities and equity = 72 + 230 + 800 + 900.8 = 2,002.8.
- FY2025 assets = 1,493.6 + 162 + 108 + 475 = 2,238.6; liabilities and equity = 81 + 180 + 800 + 1,177.6 = 2,238.6.

## 5. Historical movement inputs

| USD thousands | FY2023 | FY2024 | FY2025 |
|---|---:|---:|---:|
| Change in operating NWC | 10.0 | 8.0 | 21.0 |
| Cash from operations | 264.0 | 323.8 | 355.8 |
| CapEx | 50.0 | 60.0 | 70.0 |
| Cash from investing | -50.0 | -60.0 | -70.0 |
| Debt drawdowns | 40.0 | 20.0 | 0.0 |
| Principal repayments | 60.0 | 70.0 | 50.0 |
| Dividends | 40.0 | 50.0 | 60.0 |
| Cash from financing | -60.0 | -100.0 | -110.0 |
| Net change in cash | 154.0 | 163.8 | 175.8 |
| Opening cash | 1,000.0 | 1,154.0 | 1,317.8 |
| Closing cash | 1,154.0 | 1,317.8 | 1,493.6 |

Historical roll-forward controls:

- CFO = net income + depreciation addback − change in operating NWC.
- CFI = negative CapEx.
- CFF = debt drawdowns − principal repayments − dividends.
- Net change in cash = CFO + CFI + CFF.
- Closing cash = opening cash + net change in cash.
- Gross PP&E increases by CapEx; accumulated depreciation increases by depreciation.
- Debt changes by drawdowns less principal repayments.
- Retained earnings changes by net income less dividends.
- Share capital remains unchanged.

## 6. Forecast assumptions for FY2026–FY2030

The base case uses the following stable operating assumptions. Debt drawdowns and repayments are explicit annual amounts, not automatic funding.

| Assumption | FY2026 | FY2027 | FY2028 | FY2029 | FY2030 |
|---|---:|---:|---:|---:|---:|
| Revenue growth | 5.0% | 5.0% | 5.0% | 5.0% | 5.0% |
| COGS / revenue | 40.0% | 40.0% | 40.0% | 40.0% | 40.0% |
| Operating expenses / revenue | 25.0% | 25.0% | 25.0% | 25.0% | 25.0% |
| DSO | 44.0 | 44.0 | 44.0 | 44.0 | 44.0 |
| DIO | 73.0 | 73.0 | 73.0 | 73.0 | 73.0 |
| DPO | 54.75 | 54.75 | 54.75 | 54.75 | 54.75 |
| CapEx / revenue | 5.2% | 5.2% | 5.2% | 5.2% | 5.2% |
| Depreciation rate | 5.5% | 5.5% | 5.5% | 5.5% | 5.5% |
| Debt drawdowns | 0.0 | 20.0 | 0.0 | 0.0 | 0.0 |
| Principal repayments | 40.0 | 30.0 | 30.0 | 30.0 | 30.0 |
| Interest rate | 5.0% | 5.0% | 5.0% | 5.0% | 5.0% |
| Tax rate | 20.0% | 20.0% | 20.0% | 20.0% | 20.0% |
| Dividend payout ratio | 20.0% | 20.0% | 20.0% | 20.0% | 20.0% |

Values in JSON are ratios, e.g. revenue growth 5.0% is represented by the decimal string 0.05, and COGS / revenue 40.0% is represented by 0.40.

## 7. Expected case behaviour

The reference case is intended to exercise:

- Positive revenue and revenue growth.
- Positive operating income and net income.
- Positive working-capital balances and cash investment.
- Debt drawdown and repayment movements.
- Dividend distributions and retained-earnings roll-forward.
- Positive forecast cash under the explicit base-case financing assumptions.
- Statement and schedule reconciliation across all five forecast years.

The final expected forecast outputs must be generated by the Rust reference engine and reviewed before they are committed as golden results. Do not hand-author expected outputs separately from an independently verified calculation.

## 8. Fixture policy

The canonical machine-readable fixture is [ofrl-ib-001-case-a.input.v0.1.0.json](../../fixtures/synthetic/ofrl-ib-001-case-a.input.v0.1.0.json). It must:

1. Use source classification synthetic for every historical period.
2. Use the schema version declared by the input contract.
3. Preserve decimal strings exactly, including meaningful trailing zeros.
4. Contain no derived forecast statements or output balances.
5. Be validated against the JSON Schema and runtime semantic validator.
6. Become a golden fixture only after the independent calculation checks pass.

This document is the design specification for Case A. The canonical JSON input fixture now exists; the golden output remains a separate implementation deliverable and must be generated by the Rust reference engine after it passes the model validation suite.