#!/usr/bin/env python3
"""Independent, standard-library-only exact-rational reference for OFRL-IB-001.

This is a verification oracle, not a production runtime. It does not import,
call, or parse outputs from the Rust forecast implementation.
"""
from __future__ import annotations
import argparse
import copy
import hashlib
import json
import sys
from decimal import Decimal
from fractions import Fraction
from pathlib import Path
from typing import Any

FIXTURE_RELATIVE = Path("fixtures/synthetic/ofrl-ib-001-case-a.input.v0.1.0.json")
FIXTURE_BLOB_SHA1 = "157584f3289fb2827f5a98af055532277cc40911"
METRIC_COUNT = 32
FORECAST_YEARS = 5


def repository_root() -> Path:
    for path in Path(__file__).resolve().parents:
        if (path / FIXTURE_RELATIVE).is_file():
            return path
    raise RuntimeError(f"cannot locate repository root containing {FIXTURE_RELATIVE}")


ROOT = repository_root()
GOLDEN = ROOT / "projects/OFRL-IB-001/poc/model-numeric-envelope/golden"


def q(value: Any) -> Fraction:
    if isinstance(value, bool) or value is None:
        raise TypeError(f"expected decimal text, got {value!r}")
    return Fraction(Decimal(str(value)))


def render(value: Fraction) -> str:
    return f"{value.numerator}/{value.denominator}"


def exact_forecast(data: dict[str, Any]) -> list[dict[str, Any]]:
    """Apply V1 equations independently with fractions.Fraction."""
    historical = data["historical_periods"][-1]
    bs = historical["balance_sheet"]
    is_ = historical["income_statement"]

    cash = q(bs["cash"])
    ar_previous = q(bs["accounts_receivable"])
    inventory_previous = q(bs["inventory"])
    gross_ppe = q(bs["gross_ppe"])
    accumulated_depreciation = q(bs["accumulated_depreciation"])
    ap_previous = q(bs["accounts_payable"])
    debt = q(bs["interest_bearing_debt"])
    share_capital = q(bs["share_capital"])
    retained_earnings = q(bs["retained_earnings"])
    previous_revenue = q(is_["revenue"])
    previous_nwc = ar_previous + inventory_previous - ap_previous

    output = []
    for period in data["forecast_periods"]:
        period_id = period["period_id"]
        a = period["assumptions"]

        revenue = previous_revenue * (1 + q(a["revenue_growth"]))
        cogs = revenue * q(a["cogs_pct_revenue"])
        operating_expenses = revenue * q(a["operating_expenses_pct_revenue"])
        capex = revenue * q(a["capex_pct_revenue"])

        closing_gross_ppe = gross_ppe + capex
        average_gross_ppe = (gross_ppe + closing_gross_ppe) / 2
        opening_net_ppe = gross_ppe - accumulated_depreciation
        depreciation = min(
            average_gross_ppe * q(a["depreciation_rate"]),
            opening_net_ppe + capex,
        )
        closing_accumulated_depreciation = accumulated_depreciation + depreciation
        net_ppe = closing_gross_ppe - closing_accumulated_depreciation

        drawdowns = q(a["debt_drawdowns"])
        repayments = q(a["principal_repayments"])
        closing_debt = debt + drawdowns - repayments
        average_debt = (debt + closing_debt) / 2
        interest = average_debt * q(a["interest_rate"])

        gross_profit = revenue - cogs
        ebitda = gross_profit - operating_expenses
        ebit = ebitda - depreciation
        ebt = ebit - interest
        tax = max(Fraction(0), ebt * q(a["tax_rate"]))
        net_income = ebt - tax

        ar = revenue * q(a["dso"]) / 365
        inventory = cogs * q(a["dio"]) / 365
        ap = cogs * q(a["dpo"]) / 365
        nwc = ar + inventory - ap
        change_nwc = nwc - previous_nwc

        dividends = max(Fraction(0), net_income) * q(a["dividend_payout_ratio"])
        cfo = net_income + depreciation - change_nwc
        cfi = -capex
        cff = drawdowns - repayments - dividends
        change_cash = cfo + cfi + cff
        closing_cash = cash + change_cash

        closing_re = retained_earnings + net_income - dividends
        assets = closing_cash + ar + inventory + net_ppe
        liabilities_equity = ap + closing_debt + share_capital + closing_re
        balance_difference = assets - liabilities_equity

        values = {
            "revenue": revenue,
            "cogs": cogs,
            "gross_profit": gross_profit,
            "operating_expenses": operating_expenses,
            "ebitda": ebitda,
            "depreciation": depreciation,
            "ebit": ebit,
            "interest_expense": interest,
            "ebt": ebt,
            "income_tax_expense": tax,
            "net_income": net_income,
            "accounts_receivable": ar,
            "inventory": inventory,
            "accounts_payable": ap,
            "net_working_capital": nwc,
            "change_in_nwc": change_nwc,
            "capex": capex,
            "gross_ppe": closing_gross_ppe,
            "accumulated_depreciation": closing_accumulated_depreciation,
            "net_ppe": net_ppe,
            "debt_drawdowns": drawdowns,
            "principal_repayments": repayments,
            "closing_debt": closing_debt,
            "dividends": dividends,
            "cash_from_operations": cfo,
            "cash_from_investing": cfi,
            "cash_from_financing": cff,
            "net_change_in_cash": change_cash,
            "closing_cash": closing_cash,
            "total_assets": assets,
            "total_liabilities_equity": liabilities_equity,
            "balance_difference": balance_difference,
        }
        if len(values) != METRIC_COUNT:
            raise AssertionError(f"{period_id}: expected {METRIC_COUNT} metrics")

        output.append({
            "period_id": period_id,
            "values": {name: render(value) for name, value in values.items()},
        })

        cash = closing_cash
        ar_previous = ar
        inventory_previous = inventory
        gross_ppe = closing_gross_ppe
        accumulated_depreciation = closing_accumulated_depreciation
        ap_previous = ap
        debt = closing_debt
        retained_earnings = closing_re
        previous_revenue = revenue
        previous_nwc = nwc

    return output


def fixture_data() -> tuple[dict[str, Any], str]:
    raw = (ROOT / FIXTURE_RELATIVE).read_bytes()
    blob = hashlib.sha1(b"blob " + str(len(raw)).encode() + b"\0" + raw).hexdigest()
    if blob != FIXTURE_BLOB_SHA1:
        raise AssertionError(
            f"canonical fixture identity changed: expected {FIXTURE_BLOB_SHA1}, got {blob}"
        )
    return json.loads(raw), blob


def set_all(data: dict[str, Any], key: str, value: str) -> None:
    for period in data["forecast_periods"]:
        period["assumptions"][key] = value


def bm002(fixture: dict[str, Any], blob: str) -> dict[str, Any]:
    model = fixture["model"]
    return {
        "benchmark_id": "OFRL-IB-001-BM-002",
        "benchmark_version": "1.0.0",
        "model_id": model["model_id"],
        "schema_version": fixture["schema_version"],
        "fixture_path": FIXTURE_RELATIVE.as_posix(),
        "fixture_git_blob_sha1": blob,
        "reporting_currency": model["reporting_currency"],
        "unit_scale": model["unit_scale"],
        "day_count_basis": model["day_count_basis"],
        "arithmetic": "exact rational; each value is a reduced numerator/denominator string",
        "periods": exact_forecast(fixture),
    }


def bm003(fixture: dict[str, Any], blob: str) -> dict[str, Any]:
    high_capex = copy.deepcopy(fixture)
    set_all(high_capex, "capex_pct_revenue", "5")

    loss_making = copy.deepcopy(fixture)
    set_all(loss_making, "cogs_pct_revenue", "0.65")
    set_all(loss_making, "operating_expenses_pct_revenue", "0.45")

    full_repayment = copy.deepcopy(fixture)
    full_repayment["forecast_periods"][0]["assumptions"]["principal_repayments"] = "180"
    for period in full_repayment["forecast_periods"][1:]:
        period["assumptions"]["principal_repayments"] = "0"

    zero_activity = copy.deepcopy(fixture)
    zero_activity["forecast_periods"][0]["assumptions"]["revenue_growth"] = "-1"
    for period in zero_activity["forecast_periods"][1:]:
        period["assumptions"]["revenue_growth"] = "0"
    set_all(zero_activity, "capex_pct_revenue", "0")
    set_all(zero_activity, "debt_drawdowns", "0")
    set_all(zero_activity, "principal_repayments", "0")
    set_all(zero_activity, "interest_rate", "0")

    cases = [
        (
            "high_capex_funding_deficit",
            "CapEx at 500% of revenue in each forecast year; no financing plug or automatic debt issuance.",
            high_capex,
        ),
        (
            "loss_making_no_tax_no_dividend",
            "COGS at 65% and operating expenses at 45% of revenue; EBITDA and net income are negative, tax and dividends floor at zero.",
            loss_making,
        ),
        (
            "full_debt_repayment",
            "Repay the entire FY2026 opening debt balance (180) in FY2026; draw 20 in FY2027 and hold that balance through FY2030 with no later repayment.",
            full_repayment,
        ),
        (
            "zero_revenue_zero_capex",
            "Revenue falls to zero in FY2026 and remains zero; CapEx, debt changes and interest are zero.",
            zero_activity,
        ),
    ]

    return {
        "benchmark_id": "OFRL-IB-001-BM-003",
        "benchmark_version": "1.0.0",
        "model_id": fixture["model"]["model_id"],
        "schema_version": fixture["schema_version"],
        "fixture_git_blob_sha1": blob,
        "arithmetic": "independently implemented exact rational reference; reduced numerator/denominator strings",
        "coverage": {
            "cases": 4,
            "forecast_years_per_case": FORECAST_YEARS,
            "metrics_per_year": METRIC_COUNT,
            "total_expected_values": 4 * FORECAST_YEARS * METRIC_COUNT,
        },
        "cases": [
            {"case_id": case_id, "description": description, "periods": exact_forecast(data)}
            for case_id, description, data in cases
        ],
    }


def verify_or_write(path: Path, value: dict[str, Any], write: bool) -> None:
    text = json.dumps(value, indent=2, ensure_ascii=False) + "\n"
    if write:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
        print(f"WROTE {path.relative_to(ROOT)}")
        return
    if not path.is_file():
        raise AssertionError(f"missing committed golden: {path.relative_to(ROOT)}")
    actual = json.loads(path.read_text(encoding="utf-8"))
    if actual != value:
        raise AssertionError(
            f"{path.relative_to(ROOT)} differs from the independent exact-rational reference"
        )
    print(f"PASS {path.relative_to(ROOT)}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--check", action="store_true", help="verify committed goldens (default)")
    mode.add_argument("--write", action="store_true", help="regenerate goldens after reviewed changes")
    args = parser.parse_args()

    fixture, blob = fixture_data()
    values = [
        (
            GOLDEN / "case-a-five-year.v0.1.0.json",
            bm002(fixture, blob),
            FORECAST_YEARS * METRIC_COUNT,
        ),
        (
            GOLDEN / "case-a-edge-scenarios.v0.1.0.json",
            bm003(fixture, blob),
            4 * FORECAST_YEARS * METRIC_COUNT,
        ),
    ]
    for path, value, count in values:
        verify_or_write(path, value, args.write)
        print(f"  exact expected outputs: {count}")
    print(f"Independent reference PASS: {sum(item[2] for item in values)} exact outputs")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        raise SystemExit(1)
