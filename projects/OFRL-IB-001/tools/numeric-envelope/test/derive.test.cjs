"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const {
  Fraction,
  createDerivation,
  parseNonNegativeDecimal,
  parseSignedDecimal,
  roundOutward,
  validateProfile
} = require("../derive.cjs");

const fixturePath = path.join(
  __dirname,
  "fixtures",
  "synthetic-profile.test-only.json"
);

function readFixture() {
  return JSON.parse(fs.readFileSync(fixturePath, "utf8"));
}

function node(result, id) {
  const found = result.nodes.find(item => item.id === id);
  assert.ok(found, "missing derived node: " + id);
  return found;
}

test("fraction arithmetic stays exact and decimal output rounds outward", () => {
  const oneThird = new Fraction(1n, 3n);
  assert.equal(oneThird.add(oneThird).add(oneThird).toString(), "1");
  assert.equal(new Fraction(1n, 365n).toString(), "1/365");
  assert.equal(roundOutward(new Fraction(1n, 3n), 6), "0.333334");
  assert.equal(roundOutward(new Fraction(0n), 4), "0.0000");
});

test("zero fractional source digits are a valid explicit precision limit", () => {
  const profile = readFixture();
  profile.source_precision.monetary_max_fraction_digits = 0;
  profile.source_precision.ratio_max_fraction_digits = 0;
  profile.source_precision.days_max_fraction_digits = 0;
  for (const group of [
    profile.opening_balance_sheet,
    profile.historical_period.income_statement,
    profile.historical_period.balance_sheet,
    profile.historical_period.cash_flow_statement
  ]) {
    for (const key of Object.keys(group)) {
      group[key] = BigInt(group[key].split(".")[0]).toString();
    }
  }
  for (const key of Object.keys(profile.forecast_drivers)) {
    profile.forecast_drivers[key] = BigInt(profile.forecast_drivers[key].split(".")[0]).toString();
  }
  validateProfile(profile);
});

test("decimal bound parser rejects non-canonical or signed inputs", () => {
  assert.equal(parseNonNegativeDecimal("125.50", "test").value.toString(), "251/2");
  for (const value of ["-1", "+1", "01", "1.", ".1", "1e3", " 1", "NaN"]) {
    assert.throws(() => parseNonNegativeDecimal(value, "test"));
  }
});
test("revenue growth syntax is signed while the V1 -100% floor is enforced by profile validation", () => {
  assert.equal(parseSignedDecimal("-0.05", "growth").value.toString(), "-1/20");
  assert.equal(parseSignedDecimal("-1", "growth").value.toString(), "-1");
  assert.equal(parseSignedDecimal("-1.01", "growth").value.toString(), "-101/100");
  assert.throws(() => parseSignedDecimal("-0.0", "growth"));

  const profile = readFixture();
  profile.forecast_drivers.revenue_growth_max = "-0.05";
  validateProfile(profile);
  assert.equal(
    node(createDerivation(profile), "forecast.year_1.revenue").exact_upper_bound,
    "2565/2"
  );

  const belowFloor = readFixture();
  belowFloor.forecast_drivers.revenue_growth_max = "-1.01";
  assert.throws(() => validateProfile(belowFloor), /cannot be below the V1 floor/);
});
 
test("profile rejects bounds that exceed declared source precision", () => {
  const profile = readFixture();
  profile.forecast_drivers.cogs_pct_revenue_max = "0.40000";
  assert.throws(() => validateProfile(profile), /source precision/);

  const monetary = readFixture();
  monetary.forecast_drivers.debt_drawdowns_max_per_period = "1000000000";
  assert.throws(() => validateProfile(monetary), /source precision/);
});

test("profile rejects impossible PP&E bound relationships", () => {
  const profile = readFixture();
  profile.opening_balance_sheet.accumulated_depreciation_max = "601.0";
  assert.throws(() => validateProfile(profile), /cannot exceed gross PP&E/);
});

test("synthetic fixture is explicitly test-only and contains no approval claim", () => {
  const profile = readFixture();
  validateProfile(profile);
  assert.equal(profile.approval_state, "test_only");
  assert.equal(profile.approvals, null);
  const result = createDerivation(profile);
  assert.equal(result.decision, "TEST_ONLY__NOT_A_MODEL_OWNER_APPROVAL");
  assert.equal(result.derivation.candidate_backend_selected, false);
  assert.equal(result.derivation.production_engine_authorized, false);
  assert.equal(result.derivation.division_policy_status, "UNRESOLVED");
  assert.equal(result.derivation.reporting_policy_status, "UNRESOLVED");
  assert.equal(
    node(result, "forecast.year_1.accounts_receivable").post_rounding_scale,
    null
  );
});

test("five-year magnitude and scale recurrences use exact operation order", () => {
  const result = createDerivation(readFixture());
  assert.equal(
    node(result, "forecast.year_1.revenue").exact_upper_bound,
    "2835/2"
  );
  assert.equal(
    node(result, "forecast.year_5.revenue").pre_normalization_scale,
    22
  );
  assert.equal(
    node(result, "forecast.year_5.cogs").pre_normalization_scale,
    26
  );
  assert.equal(
    node(result, "forecast.year_1.accounts_receivable_numerator").exact_upper_bound,
    "62370"
  );
  assert.equal(
    node(result, "forecast.year_1.accounts_receivable").exact_upper_bound,
    "12474/73"
  );
  assert.equal(
    node(result, "forecast.year_1.accounts_receivable").division_context_required,
    true
  );
});

test("historical validator arithmetic is separately bounded", () => {
  const result = createDerivation(readFixture());
  assert.ok(node(result, "history.opening.balance_sheet.total_assets_bound"));
  assert.ok(node(result, "history.opening.balance_sheet.total_liabilities_equity_bound"));
  assert.ok(node(result, "history.period_1.income_statement.recomputed_net_income"));
  assert.ok(node(result, "history.period_1.income_statement.net_income_comparison_bound"));
  assert.ok(node(result, "history.period_1.balance_sheet.nwc_absolute_bound"));
  assert.ok(node(result, "history.period_1.cash_flow.cfo_after_nwc_subtraction"));
  assert.ok(node(result, "history.period_1.cash_flow.net_change_after_cff"));
  assert.ok(node(result, "history.period_1.cash_flow.opening_cash_tieout_bound"));
  assert.ok(node(result, "history.period_1.rollforward.gross_ppe_opening_plus_capex"));
  assert.ok(node(result, "history.period_1.rollforward.debt_opening_plus_drawdowns"));
  assert.ok(node(result, "history.period_1.rollforward.retained_earnings_after_dividend_subtraction"));
  assert.ok(node(result, "history.period_3.rollforward.gross_ppe_opening_plus_capex"));
});

test("depreciation uses the minimum of rate and capacity upper bounds", () => {
  const result = createDerivation(readFixture());
  const byRate = node(result, "forecast.year_1.depreciation_by_rate");
  const capacity = node(result, "forecast.year_1.depreciation_capacity");
  const depreciation = node(result, "forecast.year_1.depreciation");
  const expected = new Fraction(
    BigInt(byRate.exact_upper_bound.split("/")[0] || byRate.exact_upper_bound),
    BigInt(byRate.exact_upper_bound.split("/")[1] || "1")
  );
  const cap = new Fraction(
    BigInt(capacity.exact_upper_bound.split("/")[0] || capacity.exact_upper_bound),
    BigInt(capacity.exact_upper_bound.split("/")[1] || "1")
  );
  assert.equal(depreciation.exact_upper_bound, expected.compare(cap) <= 0
    ? byRate.exact_upper_bound
    : capacity.exact_upper_bound);
});

test("error budgets are translated through model unit scale without Number arithmetic", () => {
  const result = createDerivation(readFixture());
  assert.equal(
    result.error_budgets.arithmetic_approximation_absolute.reporting_currency_units_exact,
    "1/10"
  );
  assert.equal(
    result.error_budgets.display_export_absolute.reporting_currency_units_exact,
    "10"
  );
});

test("configured magnitude-scale constraints fail closed when a test bound is too small", () => {
  const profile = readFixture();
  profile.resource_limits.max_intermediate_scale = 8;
  const result = createDerivation(profile);
  assert.equal(result.derivation.pre_normalization_scale_limit_status, "FAIL");
  assert.ok(result.derivation.limit_checks.some(
    item => item.limit === "max_intermediate_scale" && item.status === "FAIL"
  ));
});

test("approved state requires three explicit review records and still does not select a backend", () => {
  const profile = readFixture();
  profile.approval_state = "approved";
  profile.arithmetic_policy = {
    division_fractional_scale: 18,
    division_rounding_mode: "half_even",
    reporting_fraction_digits: 2,
    reporting_rounding_mode: "half_up"
  };
  profile.approvals = {
    model_owner: {
      reviewer: "Model Owner",
      decision: "approved",
      date: "2026-11-05",
      evidence_ref: "decision-record/model-owner"
    },
    finance_accounting: {
      reviewer: "Finance Reviewer",
      decision: "approved",
      date: "2026-11-05",
      evidence_ref: "decision-record/finance"
    },
    engineering: {
      reviewer: "Engineering Reviewer",
      decision: "approved",
      date: "2026-11-05",
      evidence_ref: "decision-record/engineering"
    }
  };
  validateProfile(profile);
  const result = createDerivation(profile);
  assert.equal(
    result.decision,
    "ENGINEERING_DERIVATION_ONLY__SEPARATE_BACKEND_APPROVAL_REQUIRED"
  );
  assert.equal(result.derivation.candidate_backend_selected, false);
  assert.equal(result.derivation.production_engine_authorized, false);
  assert.equal(
    result.derivation.division_policy_status,
    "SPECIFIED__CANDIDATE_BEHAVIOUR_NOT_YET_VALIDATED"
  );
  assert.equal(
    result.derivation.reporting_policy_status,
    "SPECIFIED__XLSX_RECONCILIATION_NOT_YET_VALIDATED"
  );
  assert.equal(
    node(result, "forecast.year_1.accounts_receivable").post_rounding_scale,
    18
  );
});

test("CLI cannot silently treat a test-only profile as approved", () => {
  const scriptPath = path.join(__dirname, "..", "derive.cjs");
  const result = spawnSync(process.execPath, [
    scriptPath,
    fixturePath,
    "--require-approved"
  ], { encoding: "utf8" });
  assert.equal(result.status, 2);
  assert.match(result.stderr, /refuses a test_only profile/);
});
