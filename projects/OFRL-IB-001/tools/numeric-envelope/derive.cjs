#!/usr/bin/env node
"use strict";

// Candidate-neutral OFRL-IB-001 Numeric Profile envelope derivation.
// This tool uses exact BigInt fractions. It is analysis tooling only: it is
// not part of the Rust model runtime and it does not approve a profile/backend.

const fs = require("node:fs");
const path = require("node:path");

const PROFILE_SCHEMA_ID =
  "urn:ofrl:schema:ofrl-ib-001-numeric-profile:0.1.0";
const MAX_DECIMAL_SCALE = 28;
const UNIT_FACTORS = {
  units: 1n,
  thousands: 1_000n,
  millions: 1_000_000n
};

class Fraction {
  constructor(numerator, denominator = 1n) {
    if (denominator === 0n) throw new Error("fraction denominator cannot be zero");
    let n = BigInt(numerator);
    let d = BigInt(denominator);
    if (d < 0n) { n = -n; d = -d; }
    const divisor = gcd(abs(n), d);
    this.n = n / divisor;
    this.d = d / divisor;
  }

  add(other) {
    return new Fraction(this.n * other.d + other.n * this.d, this.d * other.d);
  }

  mul(other) {
    return new Fraction(this.n * other.n, this.d * other.d);
  }

  div(other) {
    if (other.n === 0n) throw new Error("division by zero in envelope derivation");
    return new Fraction(this.n * other.d, this.d * other.n);
  }

  abs() { return new Fraction(abs(this.n), this.d); }

  compare(other) {
    const left = this.n * other.d;
    const right = other.n * this.d;
    return left < right ? -1 : left > right ? 1 : 0;
  }

  toString() {
    return this.d === 1n ? this.n.toString() : this.n.toString() + "/" + this.d.toString();
  }
}

function abs(value) { return value < 0n ? -value : value; }

function gcd(a, b) {
  let x = abs(a);
  let y = abs(b);
  while (y !== 0n) [x, y] = [y, x % y];
  return x === 0n ? 1n : x;
}

function pow10(scale) {
  if (!Number.isInteger(scale) || scale < 0 || scale > 10000) {
    throw new Error("decimal scale is outside the derivation tool's safe operational bound");
  }
  return 10n ** BigInt(scale);
}

function parseNonNegativeDecimal(value, label) {
  if (typeof value !== "string" ||
      !/^(?:0|[1-9][0-9]*)(?:\.[0-9]+)?$/.test(value)) {
    throw new Error(label + " must be a canonical non-negative decimal string");
  }
  const [whole, fractional = ""] = value.split(".");
  const digits = whole + fractional;
  return {
    value: new Fraction(BigInt(digits), pow10(fractional.length)),
    scale: fractional.length,
    text: value
  };
}

function decimalDigits(value) {
  const n = abs(BigInt(value));
  return n === 0n ? 1 : n.toString().length;
}

function ceilPositive(numerator, denominator) {
  return numerator === 0n ? 0n : (numerator + denominator - 1n) / denominator;
}

function roundOutward(value, fractionDigits = 18) {
  const positive = value.abs();
  const scaled = ceilPositive(positive.n * pow10(fractionDigits), positive.d);
  const digits = scaled.toString().padStart(fractionDigits + 1, "0");
  if (fractionDigits === 0) return digits;
  return digits.slice(0, -fractionDigits) + "." + digits.slice(-fractionDigits);
}

function terminatingScale(denominator) {
  let d = BigInt(denominator);
  let twos = 0;
  let fives = 0;
  while (d % 2n === 0n) { d /= 2n; twos += 1; }
  while (d % 5n === 0n) { d /= 5n; fives += 1; }
  return d === 1n ? Math.max(twos, fives) : null;
}

function exactCoefficientDigits(value, scale) {
  if (scale === null) return null;
  const scaledNumerator = value.n * pow10(scale);
  if (scaledNumerator % value.d !== 0n) return null;
  return decimalDigits(scaledNumerator / value.d);
}

function assertObject(value, label, expectedKeys) {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error(label + " must be an object");
  }
  const actual = Object.keys(value).sort();
  const expected = [...expectedKeys].sort();
  if (actual.length !== expected.length ||
      actual.some((key, index) => key !== expected[index])) {
    throw new Error(
      label + " fields must be exactly [" + expected.join(", ") +
      "]; received [" + actual.join(", ") + "]"
    );
  }
}

function validateProfile(p) {
  assertObject(p, "profile", [
    "schema_id", "profile_id", "profile_version", "convention_version",
    "approval_state", "reporting_currency", "unit_scale", "day_count_basis",
    "source_precision", "opening_balance_sheet", "historical_period",
    "forecast_drivers", "resource_limits", "error_budgets", "approvals"
  ]);

  if (p.schema_id !== PROFILE_SCHEMA_ID) throw new Error("unsupported numeric profile schema_id");
  for (const key of ["profile_id", "profile_version", "convention_version"]) {
    if (typeof p[key] !== "string" || p[key].trim() === "") {
      throw new Error(key + " must be a non-empty string");
    }
  }
  if (!/^[A-Z]{3}$/.test(p.reporting_currency)) {
    throw new Error("reporting_currency must be three uppercase ASCII letters");
  }
  if (!Object.hasOwn(UNIT_FACTORS, p.unit_scale)) {
    throw new Error("unit_scale must be units, thousands, or millions");
  }
  if (p.day_count_basis !== 365) {
    throw new Error("V1 currently requires day_count_basis = 365");
  }
  if (!["test_only", "approved"].includes(p.approval_state)) {
    throw new Error("approval_state must be test_only or approved");
  }

  const precisionKeys = [
    "monetary_max_integer_digits", "monetary_max_fraction_digits",
    "ratio_max_integer_digits", "ratio_max_fraction_digits",
    "days_max_integer_digits", "days_max_fraction_digits"
  ];
  assertObject(p.source_precision, "source_precision", precisionKeys);
  for (const key of precisionKeys) {
    const value = p.source_precision[key];
    if (!Number.isInteger(value) || value < 1 ||
        value > (key.endsWith("fraction_digits") ? MAX_DECIMAL_SCALE : 1000)) {
      throw new Error("source_precision." + key + " is outside its allowed integer range");
    }
  }

  const openingKeys = [
    "cash_max", "accounts_receivable_max", "inventory_max", "gross_ppe_max",
    "accumulated_depreciation_max", "accounts_payable_max",
    "interest_bearing_debt_max", "share_capital_max",
    "retained_earnings_absolute_max"
  ];
  assertObject(p.opening_balance_sheet, "opening_balance_sheet", openingKeys);

  const historicalKeys = {
    income_statement: [
      "revenue_absolute_max", "cogs_absolute_max", "operating_expenses_absolute_max",
      "depreciation_absolute_max", "interest_expense_absolute_max",
      "income_tax_expense_absolute_max", "net_income_absolute_max"
    ],
    balance_sheet: openingKeys,
    cash_flow_statement: [
      "net_income_absolute_max", "depreciation_addback_absolute_max",
      "change_in_operating_nwc_absolute_max", "cash_from_operations_absolute_max",
      "capex_absolute_max", "cash_from_investing_absolute_max",
      "debt_drawdowns_absolute_max", "principal_repayments_absolute_max",
      "dividends_absolute_max", "cash_from_financing_absolute_max",
      "net_change_in_cash_absolute_max", "opening_cash_absolute_max",
      "closing_cash_absolute_max"
    ]
  };
  assertObject(p.historical_period, "historical_period", Object.keys(historicalKeys));
  for (const [group, keys] of Object.entries(historicalKeys)) {
    assertObject(p.historical_period[group], "historical_period." + group, keys);
  }

  const driverKeys = [
    "revenue_growth_max", "cogs_pct_revenue_max",
    "operating_expenses_pct_revenue_max", "dso_max", "dio_max", "dpo_max",
    "capex_pct_revenue_max", "depreciation_rate_max",
    "debt_drawdowns_max_per_period", "interest_rate_max", "tax_rate_max",
    "dividend_payout_ratio_max"
  ];
  assertObject(p.forecast_drivers, "forecast_drivers", driverKeys);

  const resourceKeys = [
    "max_intermediate_scale", "max_coefficient_digits", "division_precision_digits",
    "max_input_bytes", "max_cpu_ms", "max_peak_memory_mib", "max_concurrent_runs"
  ];
  assertObject(p.resource_limits, "resource_limits", resourceKeys);
  for (const key of resourceKeys) {
    if (!Number.isSafeInteger(p.resource_limits[key]) || p.resource_limits[key] <= 0) {
      throw new Error("resource_limits." + key + " must be a positive safe integer");
    }
  }
  if (p.resource_limits.max_input_bytes > 1_048_576) {
    throw new Error("max_input_bytes cannot exceed the current 1 MiB CLI transport limit");
  }

  const errorKeys = [
    "minimum_material_amount", "arithmetic_approximation_absolute",
    "accounting_identity_absolute", "schedule_rollforward_absolute",
    "display_export_absolute"
  ];
  assertObject(p.error_budgets, "error_budgets", errorKeys);

  const allAmounts = [
    ["opening_balance_sheet", p.opening_balance_sheet],
    ["historical_period.income_statement", p.historical_period.income_statement],
    ["historical_period.balance_sheet", p.historical_period.balance_sheet],
    ["historical_period.cash_flow_statement", p.historical_period.cash_flow_statement],
    ["forecast_drivers", p.forecast_drivers],
    ["error_budgets", p.error_budgets]
  ];
  for (const [group, object] of allAmounts) {
    for (const [key, value] of Object.entries(object)) {
      parseNonNegativeDecimal(value, group + "." + key);
    }
  }

  const growth = parseNonNegativeDecimal(
    p.forecast_drivers.revenue_growth_max, "forecast_drivers.revenue_growth_max"
  );
  if (growth.value.compare(new Fraction(1n)) < 0) {
    // The upper bound may be zero; the lower bound -100% is fixed by convention.
  }
  for (const key of [
    "depreciation_rate_max", "tax_rate_max", "dividend_payout_ratio_max"
  ]) {
    if (parseNonNegativeDecimal(p.forecast_drivers[key], "forecast_drivers." + key)
        .value.compare(new Fraction(1n)) > 0) {
      throw new Error("forecast_drivers." + key + " must be within [0, 1]");
    }
  }

  if (p.approval_state === "test_only") {
    if (p.approvals !== null) throw new Error("test_only profiles must set approvals to null");
  } else {
    assertObject(p.approvals, "approvals", ["model_owner", "finance_accounting", "engineering"]);
    for (const role of ["model_owner", "finance_accounting", "engineering"]) {
      const a = p.approvals[role];
      assertObject(a, "approvals." + role, ["reviewer", "decision", "date", "evidence_ref"]);
      if (typeof a.reviewer !== "string" || a.reviewer.trim() === "" ||
          a.decision !== "approved" ||
          !/^\d{4}-\d{2}-\d{2}$/.test(a.date) ||
          typeof a.evidence_ref !== "string" || a.evidence_ref.trim() === "") {
        throw new Error("approvals." + role + " must contain a completed approved record");
      }
    }
  }
}

function createDerivation(profile) {
  const nodes = [];
  const one = new Fraction(1n);
  const zero = new Fraction(0n);
  const amount = (text, label, scaleOverride = null) => {
    const parsed = parseNonNegativeDecimal(text, label);
    return { value: parsed.value, scale: scaleOverride === null ? parsed.scale : scaleOverride };
  };
  const add = (name, operands) => {
    const value = operands.reduce((total, item) => total.add(item.value), zero);
    const known = operands.every(item => item.scale !== null);
    const scale = known ? Math.max(...operands.map(item => item.scale)) : null;
    return record(name, value, scale, "sum_of_absolute_bounds");
  };
  const mul = (name, left, right) => {
    const scale = left.scale === null || right.scale === null
      ? null : left.scale + right.scale;
    return record(name, left.value.mul(right.value), scale, "product_of_absolute_bounds");
  };
  const div = (name, operand, denominator, exactScale = null) => {
    const value = operand.value.div(new Fraction(BigInt(denominator)));
    const scale = exactScale === null ? null : operand.scale + exactScale;
    return record(name, value, scale, "fixed_positive_divisor");
  };
  const maxOf = (name, items) => {
    const value = items.reduce((best, item) =>
      item.value.compare(best) > 0 ? item.value : best, zero);
    const known = items.every(item => item.scale !== null);
    const scale = known ? Math.max(...items.map(item => item.scale)) : null;
    return record(name, value, scale, "maximum_of_non_negative_bounds");
  };
  const minOf = (name, items) => {
    const value = items.reduce((best, item) =>
      item.value.compare(best) < 0 ? item.value : best, items[0].value);
    const known = items.every(item => item.scale !== null);
    const scale = known ? Math.max(...items.map(item => item.scale)) : null;
    return record(name, value, scale, "minimum_of_non_negative_branch_bounds");
  };
  function record(name, value, scale, method) {
    const exactScale = scale === null ? terminatingScale(value.d) : scale;
    const coefficientDigits = exactCoefficientDigits(value, exactScale);
    const node = {
      id: name,
      method,
      exact_upper_bound: value.toString(),
      decimal_upper_bound_18dp: roundOutward(value, 18),
      pre_normalization_scale: scale,
      terminating_decimal_scale: terminatingScale(value.d),
      coefficient_digits_if_exact: coefficientDigits,
      division_context_required: terminatingScale(value.d) === null
    };
    nodes.push(node);
    return { value, scale, node };
  }

  const source = profile.source_precision;
  const monetaryScale = source.monetary_max_fraction_digits;
  const ratioScale = source.ratio_max_fraction_digits;
  const daysScale = source.days_max_fraction_digits;
  const opening = profile.opening_balance_sheet;
  const history = profile.historical_period;
  const drivers = profile.forecast_drivers;

  const O = {};
  for (const [key, value] of Object.entries(opening)) {
    O[key] = amount(value, "opening_balance_sheet." + key, monetaryScale);
  }
  const H = {};
  for (const [group, values] of Object.entries(history)) {
    H[group] = {};
    for (const [key, value] of Object.entries(values)) {
      H[group][key] = amount(value, "historical_period." + group + "." + key, monetaryScale);
    }
  }
  const D = {};
  for (const [key, value] of Object.entries(drivers)) {
    const scale = ["dso_max", "dio_max", "dpo_max"].includes(key) ? daysScale : ratioScale;
    D[key] = amount(value, "forecast_drivers." + key, scale);
  }

  // Historical validator arithmetic: opening Balance Sheet and three
  // independently bounded historical periods. Historical input fields are
  // bounded separately from forecast assumptions; no ratio inference is used.
  let priorNwc = add("history.opening.nwc", [
    O.accounts_receivable_max, O.inventory_max, O.accounts_payable_max
  ]);
  let priorDebt = O.interest_bearing_debt_max;
  let priorGrossPpe = O.gross_ppe_max;
  let priorAccumDep = O.accumulated_depreciation_max;
  let priorRetainedEarnings = O.retained_earnings_absolute_max;
  let priorCash = O.cash_max;
  const hist = H;

  const openingNetPpe = record(
    "history.opening.net_ppe_bound", O.gross_ppe_max.value, monetaryScale,
    "net_ppe_is_bounded_by_gross_ppe_under_input_domain"
  );
  const openingAssets = add("history.opening.total_assets_bound", [
    O.cash_max, O.accounts_receivable_max, O.inventory_max, openingNetPpe
  ]);
  const openingLe = add("history.opening.total_liabilities_equity_bound", [
    O.accounts_payable_max, O.interest_bearing_debt_max,
    O.share_capital_max, O.retained_earnings_absolute_max
  ]);

  for (let index = 1; index <= 3; index += 1) {
    const prefix = "history.period_" + index;
    const i = hist.income_statement;
    const b = hist.balance_sheet;
    const c = hist.cash_flow_statement;

    const historicalNetIncome = add(prefix + ".income_statement.recomputed_net_income_bound", [
      i.revenue_absolute_max, i.cogs_absolute_max, i.operating_expenses_absolute_max,
      i.depreciation_absolute_max, i.interest_expense_absolute_max,
      i.income_tax_expense_absolute_max
    ]);
    const netPpe = record(prefix + ".balance_sheet.net_ppe_bound",
      b.gross_ppe_max.value, monetaryScale,
      "net_ppe_is_bounded_by_gross_ppe_under_input_domain");
    const totalAssets = add(prefix + ".balance_sheet.total_assets_bound", [
      b.cash_max, b.accounts_receivable_max, b.inventory_max, netPpe
    ]);
    const totalLe = add(prefix + ".balance_sheet.total_liabilities_equity_bound", [
      b.accounts_payable_max, b.interest_bearing_debt_max,
      b.share_capital_max, b.retained_earnings_absolute_max
    ]);
    const currentNwc = add(prefix + ".balance_sheet.nwc_bound", [
      b.accounts_receivable_max, b.inventory_max, b.accounts_payable_max
    ]);
    const deltaNwc = add(prefix + ".cash_flow.change_nwc_calculation_bound", [
      currentNwc, priorNwc
    ]);
    const cfo = add(prefix + ".cash_flow.cfo_calculation_bound", [
      c.net_income_absolute_max, c.depreciation_addback_absolute_max, deltaNwc
    ]);
    const cfi = c.capex_absolute_max;
    const cff = add(prefix + ".cash_flow.cff_calculation_bound", [
      c.debt_drawdowns_absolute_max, c.principal_repayments_absolute_max,
      c.dividends_absolute_max
    ]);
    const netCash = add(prefix + ".cash_flow.net_change_calculation_bound", [cfo, cfi, cff]);
    const closingCash = add(prefix + ".cash_flow.closing_cash_calculation_bound", [
      c.opening_cash_absolute_max, netCash
    ]);
    const grossPpeExpected = add(prefix + ".rollforward.gross_ppe_expected_bound", [
      priorGrossPpe, c.capex_absolute_max
    ]);
    const accumDepExpected = add(prefix + ".rollforward.accumulated_depreciation_expected_bound", [
      priorAccumDep, i.depreciation_absolute_max
    ]);
    const debtAvailable = add(prefix + ".rollforward.debt_available_bound", [
      priorDebt, c.debt_drawdowns_absolute_max
    ]);
    const retainedEarningsExpected = add(
      prefix + ".rollforward.retained_earnings_expected_bound", [
        priorRetainedEarnings, i.net_income_absolute_max, c.dividends_absolute_max
      ]
    );

    // Each historical year can independently attain the declared input maxima.
    priorNwc = currentNwc;
    priorDebt = b.interest_bearing_debt_max;
    priorGrossPpe = b.gross_ppe_max;
    priorAccumDep = b.accumulated_depreciation_max;
    priorRetainedEarnings = b.retained_earnings_absolute_max;
    priorCash = b.cash_max;
    void totalAssets; void totalLe; void historicalNetIncome; void grossPpeExpected;
    void accumDepExpected; void debtAvailable; void retainedEarningsExpected; void priorCash;
  }

  // Forecast engine bounds, five periods. Repayment is limited by available
  // debt, hence the conservative debt maximum compounds only by drawdowns.
  const growthFactor = record("forecast.growth_multiplier_bound",
    one.add(D.revenue_growth_max.value), ratioScale,
    "one_plus_maximum_annual_growth");
  let revenue = H.income_statement.revenue_absolute_max;
  let grossPpe = H.balance_sheet.gross_ppe_max;
  let accumulatedDepreciation = H.balance_sheet.accumulated_depreciation_max;
  let debt = H.balance_sheet.interest_bearing_debt_max;
  let cash = H.balance_sheet.cash_max;
  let retainedEarnings = H.balance_sheet.retained_earnings_absolute_max;
  let priorForecastNwc = add("forecast.opening_nwc_bound", [
    H.balance_sheet.accounts_receivable_max,
    H.balance_sheet.inventory_max,
    H.balance_sheet.accounts_payable_max
  ]);

  for (let year = 1; year <= 5; year += 1) {
    const prefix = "forecast.year_" + year;
    revenue = mul(prefix + ".revenue", revenue, growthFactor);
    const cogs = mul(prefix + ".cogs", revenue, D.cogs_pct_revenue_max);
    const opex = mul(prefix + ".operating_expenses", revenue, D.operating_expenses_pct_revenue_max);
    const capex = mul(prefix + ".capex", revenue, D.capex_pct_revenue_max);

    const arNumerator = mul(prefix + ".accounts_receivable_numerator", revenue, D.dso_max);
    const ar = div(prefix + ".accounts_receivable", arNumerator, 365, null);
    const inventoryNumerator = mul(prefix + ".inventory_numerator", cogs, D.dio_max);
    const inventory = div(prefix + ".inventory", inventoryNumerator, 365, null);
    const apNumerator = mul(prefix + ".accounts_payable_numerator", cogs, D.dpo_max);
    const ap = div(prefix + ".accounts_payable", apNumerator, 365, null);

    const closingGrossPpe = add(prefix + ".gross_ppe_closing", [grossPpe, capex]);
    const averageGrossPpeNumerator = add(prefix + ".average_gross_ppe_numerator", [
      grossPpe, closingGrossPpe
    ]);
    const averageGrossPpe = div(prefix + ".average_gross_ppe", averageGrossPpeNumerator, 2, 1);
    const depreciationByRate = mul(prefix + ".depreciation_by_rate", averageGrossPpe, D.depreciation_rate_max);
    const openingNetPpe = record(prefix + ".opening_net_ppe_bound",
      grossPpe.value, grossPpe.scale,
      "net_ppe_is_bounded_by_gross_ppe_under_input_domain");
    const depreciationCapacity = add(prefix + ".depreciation_capacity", [openingNetPpe, capex]);
    const depreciation = minOf(prefix + ".depreciation", [
      // The minimum of two non-negative formula branches is bounded by the
      // minimum of their independent upper bounds.
      depreciationByRate, depreciationCapacity
    ]);
    const closingAccumulatedDepreciation = add(
      prefix + ".accumulated_depreciation_closing", [
        accumulatedDepreciation, depreciation
      ]
    );
    const closingNetPpe = record(prefix + ".net_ppe_closing_bound",
      closingGrossPpe.value, closingGrossPpe.scale,
      "net_ppe_is_bounded_by_gross_ppe_under_input_domain");

    const availableDebt = add(prefix + ".debt_available_after_drawdown", [
      debt, D.debt_drawdowns_max_per_period
    ]);
    const repayment = availableDebt; // maximum permitted repayment equals available debt
    const closingDebt = record(prefix + ".debt_closing", availableDebt.value, availableDebt.scale,
      "closing_debt_is_bounded_by_available_debt");
    const averageDebtNumerator = add(prefix + ".average_debt_numerator", [debt, closingDebt]);
    const averageDebt = div(prefix + ".average_debt", averageDebtNumerator, 2, 1);
    const interest = mul(prefix + ".interest_expense", averageDebt, D.interest_rate_max);

    const grossProfit = add(prefix + ".gross_profit_absolute_bound", [revenue, cogs]);
    const ebitda = add(prefix + ".ebitda_absolute_bound", [grossProfit, opex]);
    const ebit = add(prefix + ".ebit_absolute_bound", [ebitda, depreciation]);
    const ebt = add(prefix + ".ebt_absolute_bound", [ebit, interest]);
    const tax = mul(prefix + ".tax_expense_bound", ebt, D.tax_rate_max);
    const netIncome = add(prefix + ".net_income_absolute_bound", [ebt, tax]);
    const dividends = mul(prefix + ".dividends_bound", netIncome, D.dividend_payout_ratio_max);

    const nwc = add(prefix + ".nwc_absolute_bound", [ar, inventory, ap]);
    const deltaNwc = add(prefix + ".change_nwc_absolute_bound", [nwc, priorForecastNwc]);
    const cfo = add(prefix + ".cfo_absolute_bound", [netIncome, depreciation, deltaNwc]);
    const cfi = capex;
    const cff = add(prefix + ".cff_absolute_bound", [
      D.debt_drawdowns_max_per_period, repayment, dividends
    ]);
    const netCashChange = add(prefix + ".net_change_in_cash_absolute_bound", [cfo, cfi, cff]);
    const closingCash = add(prefix + ".closing_cash_absolute_bound", [cash, netCashChange]);
    const closingRetainedEarnings = add(prefix + ".retained_earnings_closing_bound", [
      retainedEarnings, netIncome, dividends
    ]);
    const totalAssets = add(prefix + ".total_assets_bound", [closingCash, ar, inventory, closingNetPpe]);
    const totalLiabilitiesEquity = add(prefix + ".total_liabilities_equity_bound", [
      ap, closingDebt, H.balance_sheet.share_capital_max, closingRetainedEarnings
    ]);

    grossPpe = closingGrossPpe;
    accumulatedDepreciation = closingAccumulatedDepreciation;
    debt = closingDebt;
    cash = closingCash;
    retainedEarnings = closingRetainedEarnings;
    priorForecastNwc = nwc;

    void totalAssets; void totalLiabilitiesEquity;
  }

  const limitChecks = [];
  for (const node of nodes) {
    if (node.pre_normalization_scale !== null) {
      limitChecks.push({
        node: node.id,
        limit: "max_intermediate_scale",
        actual: node.pre_normalization_scale,
        allowed: profile.resource_limits.max_intermediate_scale,
        status: node.pre_normalization_scale <= profile.resource_limits.max_intermediate_scale ? "PASS" : "FAIL"
      });
    }
    if (node.coefficient_digits_if_exact !== null) {
      limitChecks.push({
        node: node.id,
        limit: "max_coefficient_digits",
        actual: node.coefficient_digits_if_exact,
        allowed: profile.resource_limits.max_coefficient_digits,
        status: node.coefficient_digits_if_exact <= profile.resource_limits.max_coefficient_digits ? "PASS" : "FAIL"
      });
    }
  }

  const failedScale = limitChecks.filter(
    item => item.limit === "max_intermediate_scale" && item.status === "FAIL"
  );
  const failedCoefficient = limitChecks.filter(
    item => item.limit === "max_coefficient_digits" && item.status === "FAIL"
  );
  const failedLimits = limitChecks.filter(item => item.status === "FAIL");
  const divisionNodes = nodes.filter(item => item.division_context_required).map(item => item.id);
  const unitFactor = UNIT_FACTORS[profile.unit_scale];
  const errorBudgets = {};
  for (const [key, value] of Object.entries(profile.error_budgets)) {
    const parsed = parseNonNegativeDecimal(value, "error_budgets." + key);
    errorBudgets[key] = {
      model_units: value,
      reporting_currency_units_exact: parsed.value.mul(new Fraction(unitFactor)).toString(),
      reporting_currency_units_18dp: roundOutward(parsed.value.mul(new Fraction(unitFactor)), 18)
    };
  }

  return {
    schema_id: PROFILE_SCHEMA_ID,
    profile_id: profile.profile_id,
    profile_version: profile.profile_version,
    convention_version: profile.convention_version,
    approval_state: profile.approval_state,
    decision: profile.approval_state === "approved"
      ? "ENGINEERING_DERIVATION_ONLY__SEPARATE_BACKEND_APPROVAL_REQUIRED"
      : "TEST_ONLY__NOT_A_MODEL_OWNER_APPROVAL",
    reporting_currency: profile.reporting_currency,
    unit_scale: profile.unit_scale,
    model_unit_to_currency_factor: unitFactor.toString(),
    day_count_basis: profile.day_count_basis,
    derivation: {
      exact_arithmetic: "BigInt rational; no binary floating point",
      forecast_horizon_periods: 5,
      historical_periods_bounded: 3,
      formula_node_count: nodes.length,
      candidate_backend_selected: false,
      production_engine_authorized: false,
      pre_normalization_scale_limit_status: failedScale.length === 0 ? "PASS" : "FAIL",
      coefficient_digit_limit_status: failedCoefficient.length === 0 ? "PASS" : "FAIL",
      division_context_required_nodes: divisionNodes,
      unresolved_candidate_capacity_nodes: divisionNodes,
      limit_checks: limitChecks
    },
    error_budgets: errorBudgets,
    nodes
  };
}

function main(argv) {
  const args = argv.slice(2);
  const requireApproved = args.includes("--require-approved");
  const positional = args.filter(arg => !arg.startsWith("--"));
  if (positional.length !== 1) {
    throw new Error("Usage: node derive.cjs <numeric-profile.json> [--require-approved]");
  }
  const inputPath = path.resolve(positional[0]);
  const profile = JSON.parse(fs.readFileSync(inputPath, "utf8"));
  validateProfile(profile);
  if (requireApproved && profile.approval_state !== "approved") {
    throw new Error("--require-approved refuses a test_only profile");
  }
  const output = createDerivation(profile);
  process.stdout.write(JSON.stringify(output, null, 2) + "\n");
}

if (require.main === module) {
  try {
    main(process.argv);
  } catch (error) {
    process.stderr.write("FAIL: " + error.message + "\n");
    process.exitCode = 2;
  }
}

module.exports = {
  Fraction,
  createDerivation,
  parseNonNegativeDecimal,
  roundOutward,
  validateProfile
};
