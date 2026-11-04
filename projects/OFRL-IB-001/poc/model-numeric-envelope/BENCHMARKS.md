<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Independent FY2026 Calculation Benchmark

**Benchmark ID:** OFRL-IB-001-BM-001  
**Model version:** V1 synthetic Case A, schema 0.1.0  
**Period:** FY2026  
**Purpose:** Independently check the first forecast period without calling the generic candidate forecast routine to produce expected values.

This benchmark is an arithmetic and formula cross-check for the stated V1 conventions. It is not evidence that the assumptions forecast a real company accurately, nor a statement of IFRS, US GAAP, or statutory-reporting compliance.

## 1. Benchmark inputs

All monetary values are in USD thousands. Rates are fractional values.

### Opening positions (FY2025 closing)

| Input | Value |
|---|---:|
| Revenue | 1,350.000000 |
| Cash | 1,493.600000 |
| Accounts receivable | 162.000000 |
| Inventory | 108.000000 |
| Gross PP&E | 780.000000 |
| Accumulated depreciation | 305.000000 |
| Accounts payable | 81.000000 |
| Interest-bearing debt | 180.000000 |
| Share capital | 800.000000 |
| Retained earnings | 1,177.600000 |
| Opening net working capital | 189.000000 |

Opening net working capital is 162 + 108 − 81 = 189.

### FY2026 assumptions

| Assumption | Value |
|---|---:|
| Revenue growth | 0.05 |
| COGS / revenue | 0.40 |
| Operating expenses / revenue | 0.25 |
| DSO | 44.0 |
| DIO | 73.0 |
| DPO | 54.75 |
| CapEx / revenue | 0.052 |
| Depreciation rate | 0.055 |
| Debt drawdowns | 0 |
| Principal repayments | 40 |
| Interest rate | 0.05 |
| Tax rate | 0.20 |
| Dividend payout ratio | 0.20 |

## 2. Independently derived formula sequence

### Operating statement

Revenue = 1,350 × (1 + 0.05) = 1,417.500000.

COGS = 1,417.5 × 0.40 = 567.000000.

Operating expenses = 1,417.5 × 0.25 = 354.375000.

Gross profit = 1,417.5 − 567 = 850.500000.

EBITDA = 850.5 − 354.375 = 496.125000.

### PP&E and depreciation

CapEx = 1,417.5 × 0.052 = 73.710000.

Closing gross PP&E = 780 + 73.71 = 853.710000.

Average gross PP&E = (780 + 853.71) / 2 = 816.855000.

Opening net PP&E = 780 − 305 = 475.000000.

Depreciation = min(816.855 × 0.055, 475 + 73.71) = 44.927025.

Closing accumulated depreciation = 305 + 44.927025 = 349.927025.

Closing net PP&E = 853.71 − 349.927025 = 503.782975.

### Debt and interest

Closing debt = 180 + 0 − 40 = 140.000000.

Average debt = (180 + 140) / 2 = 160.000000.

Interest expense = 160 × 0.05 = 8.000000.

### Earnings and tax

EBIT = 496.125 − 44.927025 = 451.197975.

EBT = 451.197975 − 8 = 443.197975.

Tax = max(0, 443.197975 × 0.20) = 88.639595.

Net income = 443.197975 − 88.639595 = 354.558380.

### Working capital

Accounts receivable = 1,417.5 × 44 / 365 = 170.876712328767…

Inventory = 567 × 73 / 365 = 113.400000.

Accounts payable = 567 × 54.75 / 365 = 85.050000.

Closing NWC = AR + inventory − AP = 199.226712328767…

Change in NWC = 199.226712328767… − 189 = 10.226712328767…

### Dividends and cash flow

Dividends = 354.55838 × 0.20 = 70.911676.

CFO = net income + depreciation − change in NWC = 389.258692671233…

CFI = −CapEx = −73.710000.

CFF = drawdowns − principal repayments − dividends = −110.911676.

Net change in cash = CFO + CFI + CFF = 204.637016671233…

Closing cash = 1,493.6 + 204.637016671233… = 1,698.237016671233…

### Equity and Balance Sheet

Closing retained earnings = 1,177.6 + 354.55838 − 70.911676 = 1,461.246704.

Total assets = cash + AR + inventory + net PP&E = 2,486.296704.

Total liabilities and equity = AP + debt + share capital + retained earnings = 2,486.296704.

Exact Balance Sheet residual = 0.

## 3. Exact rational golden outputs

The Rust test `case_a_fy2026_matches_independently_derived_exact_benchmark` asserts these exact reduced fractions. No reporting-rounding tolerance is used for these expected values.

| Metric | Exact expected value |
|---|---:|
| Revenue | 2835 / 2 |
| COGS | 567 / 1 |
| Gross profit | 1701 / 2 |
| Operating expenses | 2835 / 8 |
| EBITDA | 3969 / 8 |
| Depreciation | 1797081 / 40000 |
| EBIT | 18047919 / 40000 |
| Interest expense | 8 / 1 |
| EBT | 17727919 / 40000 |
| Income tax expense | 17727919 / 200000 |
| Net income | 17727919 / 50000 |
| Accounts receivable | 12474 / 73 |
| Inventory | 567 / 5 |
| Accounts payable | 1701 / 20 |
| Net working capital | 290871 / 1460 |
| Change in NWC | 14931 / 1460 |
| CapEx | 7371 / 100 |
| Gross PP&E | 85371 / 100 |
| Accumulated depreciation | 13997081 / 40000 |
| Net PP&E | 20151319 / 40000 |
| Debt drawdowns | 0 / 1 |
| Principal repayments | 40 / 1 |
| Closing debt | 140 / 1 |
| Dividends | 17727919 / 250000 |
| Cash from operations | 5683176913 / 14600000 |
| Cash from investing | −7371 / 100 |
| Cash from financing | −27727919 / 250000 |
| Net change in cash | 14938502217 / 73000000 |
| Closing cash | 123971302217 / 73000000 |
| Total assets | 38848386 / 15625 |
| Total liabilities and equity | 38848386 / 15625 |
| Balance Sheet difference | 0 / 1 |

## 4. Independence and limitations

Expected values in this benchmark were derived from the published opening balances, assumptions, and V1 equations separately from the generic `forecast<N>()` routine. The Rust test uses immutable exact fractions; it does not invoke a second backend of the same generic forecast implementation to manufacture expected values.

This is an independent first-period benchmark, not full independent model validation. Further cases are required for negative earnings, zero denominators, boundary inputs, different financing paths, and multi-year propagation. A qualified accounting/model reviewer should review the modelling conventions before the production engine is accepted.

The benchmark remains tied to the V1 synthetic Case A input and the model conventions documented in this project. A change to those inputs or conventions requires a versioned benchmark update and rerunning the affected tests.
