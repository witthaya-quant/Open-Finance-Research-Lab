<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Independent Edge-Scenario Benchmarks

**Benchmark ID:** OFRL-IB-001-BM-003  
**Version:** 1.0.0  
**Baseline fixture:** Synthetic Case A, schema 0.1.0  
**Coverage:** 4 scenarios × 5 forecast years × 32 metrics = 640 exact outputs  
**Status:** Implemented; independent finance/accounting sign-off remains open

## 1. Purpose

BM-003 extends BM-002 beyond the profitable baseline case and exercises four important V1 boundary behaviors:

- Funding infeasibility under deliberately high capital expenditure.
- Loss-making operations with tax and dividend floors.
- Full repayment of opening debt followed by a later drawdown.
- Exact-zero revenue and CapEx across the forecast horizon.

Every scenario has 160 immutable reduced-rational expected values. The Rust POC is checked against this separate golden dataset with no display rounding or tolerance.

## 2. Independent reference implementation

The reproducible reference is `reference/independent_reference.py`. It uses only Python standard-library `decimal.Decimal`, `fractions.Fraction`, `json`, and `copy`. It does not import, execute, or share code with the Rust forecast function. Input decimal strings are converted to exact fractions, each formula is separately evaluated using rational arithmetic, and outputs are stored as reduced numerator/denominator strings.

Run from the repository root:

```bash
python3 projects/OFRL-IB-001/poc/model-numeric-envelope/reference/independent_reference.py --check
```

The `--check` mode regenerates BM-002 and BM-003 in memory and compares the exact objects with their committed golden files. `--write` is an explicit maintainer action following a reviewed methodology change; CI never runs `--write`.

Python is a verification-only tool and is not a production runtime dependency or used by the Rust library/CLI.

## 3. Scenario definitions

| Case ID | Input mutations | Behavior under test |
|---|---|---|
| `high_capex_funding_deficit` | CapEx / revenue = 5.00 in all five forecast periods | Negative cash is visible; no automatic borrowing or balancing plug |
| `loss_making_no_tax_no_dividend` | COGS / revenue = 0.65; operating expenses / revenue = 0.45 | EBITDA and Net Income remain negative; tax and dividends floor at zero |
| `full_debt_repayment` | FY2026 repayment = 180; later repayments = 0; canonical FY2027 drawdown of 20 remains | Debt reaches zero, then a later drawdown restores 20; average-balance interest follows each opening/closing debt pair |
| `zero_revenue_zero_capex` | FY2026 growth = -1; subsequent growth = 0; CapEx, debt draws/repayments and interest = 0 | Exact-zero operating lines and working capital; PP&E depreciation continues |

These are synthetic mechanics tests, not forecasts or investment scenarios.

## 4. Selected exact outcomes

All figures are USD thousands.

### High-CapEx funding deficit

- FY2026 closing cash: `-1531866007/292000` (approximately -5,246.116462).
- FY2026 closing debt: `140/1`, i.e. opening debt 180 less explicit repayment 40, with no invented borrowing.
- FY2026 Balance Sheet difference: `0/1`.

The scenario deliberately becomes extreme in later periods; it is not a reportable feasible company position. A mathematically reconciled statement is not a Scenario Feasibility PASS.

### Loss-making operating case

- FY2026 EBITDA: `-567/4`.
- FY2026 Net Income: `-7787081/40000`.
- FY2026 income-tax expense: `0/1`.
- FY2026 dividends: `0/1`.

Negative operating and net income must not be clamped to zero. The test checks `Tax = max(0, EBT × tax rate)` and `Dividends = payout × max(0, Net Income)`.

### Full repayment and redraw

- FY2026 closing debt: `0/1`; interest: `9/2` = (180 + 0) / 2 × 5%.
- FY2027 drawdown: 20; closing debt: `20/1`; interest: `1/2` = (0 + 20) / 2 × 5%.
- FY2028 closing debt: `20/1`; interest: `1/1` = 20 × 5%.

### Zero revenue and CapEx

- Revenue, COGS, gross profit, operating expenses, EBITDA, AR, inventory, AP, NWC and CapEx equal exactly `0/1` in every forecast year.
- FY2026 Net Income equals `-429/10` due to continuing aggregate PP&E depreciation.
- The first-year cash movement includes release of opening working capital as receivables, inventory and payables become zero.

## 5. Automated acceptance checks

`tests/independent_edge_scenarios.rs` enforces:
1. Four named cases in fixed order.
2. Exactly five ordered forecast years per case.
3. Exactly 32 output metrics per period and 640 in total.
4. Exact equality of every reduced rational value.
5. Zero algebraic Balance Sheet difference for every case/year.
6. Negative Net Income retained with zero tax and dividend floors.
7. Debt extinguishment, redraw and average-balance interest.
8. Exact-zero revenue, working capital and CapEx behavior.
9. High-CapEx funding deficit without automatic debt creation.

Numeric Envelope CI runs the independent Python reference in `--check` mode. The Python reference verifies the committed golden dataset, while the Rust integration test verifies Rust output against the same constants.

## 6. Limitations

BM-003 covers four synthetic cases under the published V1 mechanics. It does not settle the production numeric backend, scale, precision/overflow envelope, accumulated-error budget, production input-domain boundaries, independent accounting review, or IFRS/US GAAP/statutory compliance. It is not property-based coverage of all allowed input combinations.

Production Forecast Engine remains gated by the accepted Numeric Profile, dependency/legal review and finance/accounting acceptance.
