"use strict";

const assert = require("node:assert/strict");
const { test } = require("node:test");
const {
  BIGDECIMAL,
  RUST_DECIMAL,
  evaluateCandidateCapacity
} = require("../candidate-capacity.cjs");

const MAX_RUST_DECIMAL_COEFFICIENT =
  "79228162514264337593543950335";

function derivation({
  scale = 0,
  coefficient = "1",
  coefficientDigits = coefficient.length,
  maxScale = 28,
  maxDigits = 29,
  divisionNodes = [],
  divisionScale = 18,
  divisionRounding = "half_even"
} = {}) {
  return {
    profile_id: "TEST-ONLY-CAPACITY",
    profile_version: "0.0.0",
    approval_state: "test_only",
    resource_limits: {
      max_intermediate_scale: maxScale,
      max_coefficient_digits: maxDigits
    },
    arithmetic_policy: {
      division_fractional_scale: divisionScale,
      division_rounding_mode: divisionRounding
    },
    derivation: {
      division_context_required_nodes: divisionNodes,
      candidate_backend_selected: false,
      production_engine_authorized: false
    },
    nodes: [{
      id: "test.node",
      pre_normalization_scale: scale,
      post_rounding_scale: null,
      coefficient_integer_upper_bound: coefficient,
      coefficient_digits_upper_bound: coefficientDigits,
      division_context_required: false
    }]
  };
}

test("rust_decimal accepts the exact maximum 96-bit coefficient at scale zero", () => {
  const result = evaluateCandidateCapacity(
    derivation({
      coefficient: MAX_RUST_DECIMAL_COEFFICIENT,
      coefficientDigits: 29
    }),
    RUST_DECIMAL.id
  );

  assert.equal(result.static_capacity_status, "STATIC_CAPACITY_WITHIN_DECLARED_PROFILE");
  assert.equal(result.failure_count, 0);
  assert.equal(result.checks[0].status, "WITHIN_STATIC_LIMITS");
});

test("rust_decimal rejects a 29-digit coefficient one unit above the 96-bit maximum", () => {
  const oneAbove = (BigInt(MAX_RUST_DECIMAL_COEFFICIENT) + 1n).toString();
  const result = evaluateCandidateCapacity(
    derivation({ coefficient: oneAbove, coefficientDigits: 29 }),
    RUST_DECIMAL.id
  );

  assert.equal(result.static_capacity_status, "FAIL");
  assert.ok(result.checks[0].reasons.includes("rust_decimal_96_bit_mantissa_exceeded"));
});

test("rust_decimal does not use digit count as a substitute for exact mantissa comparison", () => {
  const below = (BigInt(MAX_RUST_DECIMAL_COEFFICIENT) - 1n).toString();
  assert.equal(below.length, 29);

  const result = evaluateCandidateCapacity(
    derivation({ coefficient: below, coefficientDigits: 29 }),
    RUST_DECIMAL.id
  );
  assert.equal(result.static_capacity_status, "STATIC_CAPACITY_WITHIN_DECLARED_PROFILE");
});

test("rust_decimal rejects an intermediate scale above 28 even when the coefficient is small", () => {
  const result = evaluateCandidateCapacity(
    derivation({ scale: 29, coefficient: "1", coefficientDigits: 1 }),
    RUST_DECIMAL.id
  );

  assert.equal(result.static_capacity_status, "FAIL");
  assert.ok(result.checks[0].reasons.includes("rust_decimal_scale_exceeds_28"));
});

test("candidate checks fail closed when the profile coefficient-digit ceiling is exceeded", () => {
  const result = evaluateCandidateCapacity(
    derivation({
      coefficient: "100000",
      coefficientDigits: 6,
      maxDigits: 5
    }),
    BIGDECIMAL.id
  );

  assert.equal(result.static_capacity_status, "FAIL");
  assert.ok(result.checks[0].reasons.includes("profile_coefficient_digit_limit_exceeded"));
});

test("unresolved scale and coefficient bounds remain conditional, not pass", () => {
  const input = derivation();
  input.nodes[0].pre_normalization_scale = null;
  input.nodes[0].coefficient_integer_upper_bound = null;
  input.nodes[0].coefficient_digits_upper_bound = null;

  const result = evaluateCandidateCapacity(input, BIGDECIMAL.id);
  assert.equal(result.static_capacity_status, "CONDITIONAL");
  assert.equal(result.conditional_count, 1);
  assert.equal(result.checks[0].status, "CONDITIONAL");
});

test("BigDecimal uses independent profile scale and coefficient ceilings", () => {
  const result = evaluateCandidateCapacity(
    derivation({
      scale: 42,
      coefficient: "123456789012345678901234567890",
      coefficientDigits: 30,
      maxScale: 48,
      maxDigits: 32,
      divisionNodes: ["forecast.year_1.accounts_receivable"]
    }),
    BIGDECIMAL.id
  );

  assert.equal(result.static_capacity_status, "STATIC_CAPACITY_WITHIN_DECLARED_PROFILE");
  assert.equal(result.division_policy_status, "SPECIFIED__CANDIDATE_OPERATION_SEMANTICS_STILL_REQUIRE_TESTS");
  assert.deepEqual(result.division_nodes_requiring_candidate_verification, [
    "forecast.year_1.accounts_receivable"
  ]);
});

test("candidate capacity output never selects a backend or authorizes the engine", () => {
  for (const candidate of [RUST_DECIMAL.id, BIGDECIMAL.id]) {
    const result = evaluateCandidateCapacity(derivation(), candidate);
    assert.equal(result.backend_selected, false);
    assert.equal(result.production_engine_authorized, false);
    assert.equal(result.runtime_correctness_status, "NOT_EXECUTED");
    assert.equal(result.operational_resource_status, "NOT_MEASURED");
  }
});

test("candidate evaluator rejects duplicate node identifiers", () => {
  const input = derivation();
  input.nodes.push({ ...input.nodes[0] });
  assert.throws(
    () => evaluateCandidateCapacity(input, RUST_DECIMAL.id),
    /unique non-empty id/
  );
});

test("candidate evaluator rejects malformed coefficient integers", () => {
  const input = derivation({ coefficient: "01", coefficientDigits: 2 });
  assert.throws(
    () => evaluateCandidateCapacity(input, RUST_DECIMAL.id),
    /invalid coefficient integer bound/
  );
});

test("candidate evaluator rejects a coefficient digit count inconsistent with its exact integer", () => {
  const input = derivation({ coefficient: "12345", coefficientDigits: 4 });
  assert.throws(
    () => evaluateCandidateCapacity(input, RUST_DECIMAL.id),
    /digit count does not match/
  );
});

