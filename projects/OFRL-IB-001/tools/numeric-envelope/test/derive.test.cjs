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

test("decimal bound parser rejects non-canonical or signed inputs", () => {
  assert.equal(parseNonNegativeDecimal("125.50", "test").value.toString(), "251/2");
  for (const value of ["-1", "+1", "01", "1.", ".1", "1e3", " 1", "NaN"]) {
    assert.throws(() => parseNonNegativeDecimal(value, "test"));
  }
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
    "62370/365"
  );
  assert.equal(
    node(result, "forecast.year_1.accounts_receivable").division_context_required,
    true
  );
});

test("historical validator arithmetic is separately bounded", () => {
  const result = createDerivation(readFixture());
  assert.ok(node(result, "history.opening.total_assets_bound"));
  assert.ok(node(result, "history.opening.total_liabilities_equity_bound"));
  assert.ok(node(result, "history.period_1.income_statement.recomputed_net_income_bound"));
  assert.ok(node(result, "history.period_1.balance_sheet.nwc_bound"));
  assert.ok(node(result, "history.period_1.cash_flow.cfo_calculation_bound"));
  assert.ok(node(result, "history.period_1.cash_flow.net_change_calculation_bound"));
  assert.ok(node(result, "history.period_1.rollforward.gross_ppe_expected_bound"));
  assert.ok(node(result, "history.period_1.rollforward.debt_available_bound"));
  assert.ok(node(result, "history.period_1.rollforward.retained_earnings_expected_bound"));
  assert.ok(node(result, "history.period_3.rollforward.gross_ppe_expected_bound"));
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
