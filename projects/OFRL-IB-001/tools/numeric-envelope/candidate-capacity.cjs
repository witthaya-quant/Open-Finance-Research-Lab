#!/usr/bin/env node
"use strict";

// Candidate-specific static representation-capacity check.
// This tool does not execute a financial forecast or authorize production use.

const fs = require("node:fs");
const path = require("node:path");

const RUST_DECIMAL = Object.freeze({
  id: "rust_decimal_1_43_0",
  max_scale: 28,
  max_coefficient: 79228162514264337593543950335n
});

const BIGDECIMAL = Object.freeze({
  id: "bigdecimal_0_4_11"
});

function effectiveScale(node) {
  if (node.post_rounding_scale !== null && node.post_rounding_scale !== undefined) {
    return node.post_rounding_scale;
  }
  return node.pre_normalization_scale ?? null;
}

function validateDerivation(derivation) {
  if (!derivation || typeof derivation !== "object" || Array.isArray(derivation)) {
    throw new Error("derivation input must be a JSON object");
  }
  if (
    typeof derivation.profile_id !== "string" ||
    typeof derivation.profile_version !== "string" ||
    !Array.isArray(derivation.nodes) ||
    derivation.nodes.length === 0 ||
    !derivation.resource_limits ||
    !derivation.derivation
  ) {
    throw new Error("derivation input is missing profile identity, nodes, resource limits, or derivation metadata");
  }

  const seen = new Set();
  for (const node of derivation.nodes) {
    if (!node || typeof node.id !== "string" || node.id.length === 0 || seen.has(node.id)) {
      throw new Error("every derivation node must have a unique non-empty id");
    }
    seen.add(node.id);

    const scale = effectiveScale(node);
    if (scale !== null && (!Number.isSafeInteger(scale) || scale < 0)) {
      throw new Error("node " + node.id + " has an invalid scale bound");
    }
    const coefficient = node.coefficient_integer_upper_bound;
    if (coefficient !== null && coefficient !== undefined) {
      if (typeof coefficient !== "string" || !/^(0|[1-9][0-9]*)$/.test(coefficient)) {
        throw new Error("node " + node.id + " has an invalid coefficient integer bound");
      }
    }
    const digits = node.coefficient_digits_upper_bound;
    if (digits !== null && digits !== undefined && (!Number.isSafeInteger(digits) || digits < 1)) {
      throw new Error("node " + node.id + " has an invalid coefficient digit bound");
    }
    if (coefficient !== null && coefficient !== undefined &&
        digits !== null && digits !== undefined &&
        digits !== coefficient.length) {
      throw new Error("node " + node.id + " coefficient digit count does not match its exact integer bound");
    }
  }

  for (const key of ["max_intermediate_scale", "max_coefficient_digits"]) {
    const value = derivation.resource_limits[key];
    if (!Number.isSafeInteger(value) || value < 0) {
      throw new Error("derivation resource limit " + key + " must be a non-negative safe integer");
    }
  }
}

function evaluateCandidateCapacity(derivation, candidateId) {
  validateDerivation(derivation);

  if (![RUST_DECIMAL.id, BIGDECIMAL.id].includes(candidateId)) {
    throw new Error("unsupported candidate: " + candidateId);
  }

  const checks = [];
  let hasBoundExceedance = false;
  let hasConditional = false;

  for (const node of derivation.nodes) {
    const scale = effectiveScale(node);
    const coefficientText = node.coefficient_integer_upper_bound ?? null;
    const coefficientDigits = node.coefficient_digits_upper_bound ?? null;
    const reasons = [];
    let nodeExceeds = false;

    if (scale === null) {
      hasConditional = true;
      reasons.push("scale_bound_unresolved");
    } else if (scale > derivation.resource_limits.max_intermediate_scale) {
      hasBoundExceedance = true;
      nodeExceeds = true;
      reasons.push("derived_scale_bound_exceeds_profile_limit");
    }

    if (coefficientDigits === null) {
      hasConditional = true;
      reasons.push("coefficient_bound_unresolved");
    } else if (coefficientDigits > derivation.resource_limits.max_coefficient_digits) {
      hasBoundExceedance = true;
      nodeExceeds = true;
      reasons.push("derived_coefficient_bound_exceeds_profile_limit");
    }

    if (candidateId === RUST_DECIMAL.id) {
      if (scale !== null && scale > RUST_DECIMAL.max_scale) {
        hasBoundExceedance = true;
        nodeExceeds = true;
        reasons.push("rust_decimal_scale_bound_exceeds_28");
      }
      if (coefficientText === null) {
        hasConditional = true;
        reasons.push("rust_decimal_mantissa_bound_unresolved");
      } else if (BigInt(coefficientText) > RUST_DECIMAL.max_coefficient) {
        hasBoundExceedance = true;
        nodeExceeds = true;
        reasons.push("rust_decimal_coefficient_upper_bound_exceeds_96_bit_maximum");
      }
    }

    checks.push({
      node: node.id,
      effective_scale_bound: scale,
      coefficient_integer_upper_bound: coefficientText,
      coefficient_digits_upper_bound: coefficientDigits,
      status: nodeExceeds
        ? "BOUND_EXCEEDS_LIMIT"
        : reasons.length > 0
          ? "CONDITIONAL"
          : "WITHIN_STATIC_LIMITS",
      reasons
    });
  }

  const divisionNodes = Array.isArray(derivation.derivation.division_context_required_nodes)
    ? derivation.derivation.division_context_required_nodes
    : [];
  const divisionPolicySpecified =
    derivation.arithmetic_policy?.division_fractional_scale !== null &&
    derivation.arithmetic_policy?.division_fractional_scale !== undefined &&
    typeof derivation.arithmetic_policy?.division_rounding_mode === "string";

  let staticCapacityStatus;
  if (hasBoundExceedance) {
    staticCapacityStatus = "STATIC_BOUND_EXCEEDS_LIMIT";
  } else if (hasConditional || !divisionPolicySpecified) {
    staticCapacityStatus = "CONDITIONAL";
  } else {
    staticCapacityStatus = "STATIC_CAPACITY_WITHIN_DECLARED_PROFILE";
  }

  const candidateSpecific = candidateId === RUST_DECIMAL.id
    ? {
        representation: "96-bit unsigned coefficient magnitude; separate sign bit; scale 0..28",
        maximum_coefficient_integer: RUST_DECIMAL.max_coefficient.toString(),
        maximum_scale: RUST_DECIMAL.max_scale,
        exact_mantissa_comparison: "performed per derivation node"
      }
    : {
        representation: "arbitrary-precision decimal coefficient",
        coefficient_ceiling: derivation.resource_limits.max_coefficient_digits + " decimal digits (profile target)",
        scale_ceiling: derivation.resource_limits.max_intermediate_scale + " fractional digits (profile target)",
        exact_package_capacity: "not fixed by this static profile check",
        division_context_compatibility:
          "POC 80-significant-digit compile-time context is not equivalent to a fixed fractional-scale, immutable per-run context; a production adapter must prove the mapping"
      };

  function reasonCounts(status) {
    const counts = {};
    for (const item of checks.filter(check => check.status === status)) {
      for (const reason of item.reasons) {
        counts[reason] = (counts[reason] || 0) + 1;
      }
    }
    return Object.fromEntries(Object.entries(counts).sort(([a], [b]) => a.localeCompare(b)));
  }

  return {
    profile_id: derivation.profile_id,
    profile_version: derivation.profile_version,
    candidate: candidateId,
    candidate_specific_limits: candidateSpecific,
    static_capacity_status: staticCapacityStatus,
    division_policy_status: divisionPolicySpecified
      ? "SPECIFIED__CANDIDATE_OPERATION_SEMANTICS_STILL_REQUIRE_TESTS"
      : "UNRESOLVED",
    division_nodes_requiring_candidate_verification: divisionNodes,
    runtime_correctness_status: "NOT_EXECUTED",
    operational_resource_status: "NOT_MEASURED",
    backend_selected: false,
    production_engine_authorized: false,
    node_count: checks.length,
    bound_exceeds_limit_count: checks.filter(item => item.status === "BOUND_EXCEEDS_LIMIT").length,
    conditional_count: checks.filter(item => item.status === "CONDITIONAL").length,
    bound_exceedance_reason_counts: reasonCounts("BOUND_EXCEEDS_LIMIT"),
    conditional_reason_counts: reasonCounts("CONDITIONAL"),
    bound_exceedance_examples: checks
      .filter(item => item.status === "BOUND_EXCEEDS_LIMIT")
      .map(item => ({
        node: item.node,
        effective_scale_bound: item.effective_scale_bound,
        coefficient_integer_upper_bound: item.coefficient_integer_upper_bound,
        coefficient_digits_upper_bound: item.coefficient_digits_upper_bound,
        reasons: item.reasons
      })),
    checks
  };
}

function main(argv) {
  const args = argv.slice(2);
  const candidateIndex = args.indexOf("--candidate");
  if (args.length !== 3 || candidateIndex !== 1 || candidateIndex === args.length - 1) {
    throw new Error(
      "Usage: node candidate-capacity.cjs <derivation.json> --candidate " +
      RUST_DECIMAL.id + "|" + BIGDECIMAL.id
    );
  }

  const inputPath = path.resolve(args[0] === "--candidate" ? args[1] : args[0]);
  const candidateId = args[candidateIndex + 1];
  const derivation = JSON.parse(fs.readFileSync(inputPath, "utf8"));
  const result = evaluateCandidateCapacity(derivation, candidateId);
  process.stdout.write(JSON.stringify(result, null, 2) + "\n");
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
  BIGDECIMAL,
  RUST_DECIMAL,
  evaluateCandidateCapacity,
  effectiveScale,
  validateDerivation
};
