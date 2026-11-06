#!/usr/bin/env node
"use strict";

const test = require("node:test");
const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");

const registry = require("../validate_project_registry.cjs");
const ROOT = path.resolve(__dirname, "../..");

test("every supported project type has explicit document and validation-profile rules", () => {
  const types = [...registry.ALLOWED_TYPES].sort();
  assert.deepEqual(Object.keys(registry.TYPE_REQUIREMENTS).sort(), types);
  assert.deepEqual(Object.keys(registry.TYPE_VALIDATION_PROFILES).sort(), types);

  for (const type of types) {
    assert.ok(registry.TYPE_REQUIREMENTS[type].length > 0, type + " must define document requirements");
    for (const alternatives of registry.TYPE_REQUIREMENTS[type]) {
      assert.ok(alternatives.length > 0, type + " has an empty document alternative group");
      const hasTemplate = alternatives.some((document) => {
        const template = document.replace(/\.md$/i, "_TEMPLATE.md");
        return fs.existsSync(path.join(ROOT, "templates", template));
      });
      assert.ok(hasTemplate, type + " has no matching starting template");
    }
    for (const profile of registry.TYPE_VALIDATION_PROFILES[type]) {
      assert.ok(registry.ALLOWED_VALIDATION_PROFILES.has(profile), type + " references unknown profile " + profile);
    }
  }
});

test("the manifest schema and validator agree on allowed validation profiles", () => {
  const schemaPath = path.join(ROOT, "schemas", "ofrl-project-manifest-1.1.0.schema.json");
  const schema = JSON.parse(fs.readFileSync(schemaPath, "utf8"));
  const schemaProfiles = new Set(schema.properties.validation_profiles.items.enum);
  assert.deepEqual([...registry.ALLOWED_VALIDATION_PROFILES].sort(), [...schemaProfiles].sort());
});

test("the universal validation document accepts either a matrix or an explicit plan", () => {
  assert.equal(registry.missingBaseDocumentGroups(["README.md", "DECISION_LOG.md", "VALIDATION_MATRIX.md"]).length, 0);
  assert.equal(registry.missingBaseDocumentGroups(["README.md", "DECISION_LOG.md", "VALIDATION_PLAN.md"]).length, 0);
  assert.equal(registry.missingBaseDocumentGroups(["README.md", "DECISION_LOG.md"]).length, 1);
});

test("every non-none data classification requires a data profile and Data Manifest", () => {
  const synthetic = {
    data_classification: "synthetic",
    validation_profiles: ["research"],
    required_documents: ["README.md", "DATA_MANIFEST.md"],
  };
  assert.deepEqual(registry.dataPolicyViolations(synthetic), [
    "non-none data classification requires the 'data' validation profile",
  ]);

  synthetic.validation_profiles.push("data");
  assert.deepEqual(registry.dataPolicyViolations(synthetic), []);

  const noData = { data_classification: "none", validation_profiles: [], required_documents: [] };
  assert.deepEqual(registry.dataPolicyViolations(noData), []);
});

test("released artifacts cannot be attached to pre-release lifecycle states", () => {
  const invalid = { lifecycle_status: "in_development", release_status: "released", version: "0.1.0" };
  assert.ok(registry.releaseStateViolations(invalid).some((message) => message.includes("requires lifecycle_status")));

  const validReleased = { lifecycle_status: "released", release_status: "released", version: "1.0.0" };
  assert.deepEqual(registry.releaseStateViolations(validReleased), []);

  const validDeprecated = { lifecycle_status: "deprecated", release_status: "released", version: "1.0.0" };
  assert.deepEqual(registry.releaseStateViolations(validDeprecated), []);

  const invalidMaintenance = { lifecycle_status: "maintenance", release_status: "not_released" };
  assert.ok(registry.releaseStateViolations(invalidMaintenance).some((message) => message.includes("released/maintenance")));
});

test("prototype projects have a dedicated validation profile", () => {
  assert.deepEqual(registry.TYPE_VALIDATION_PROFILES.prototype, ["prototype"]);
  assert.ok(registry.ALLOWED_VALIDATION_PROFILES.has("prototype"));
});

test("risk-tier enum is aligned with the manifest schema", () => {
  const schemaPath = path.join(ROOT, "schemas", "ofrl-project-manifest-1.1.0.schema.json");
  const schema = JSON.parse(fs.readFileSync(schemaPath, "utf8"));
  assert.deepEqual([...registry.ALLOWED_RISK_TIERS].sort(), [...schema.properties.risk_tier.enum].sort());
  assert.deepEqual([...registry.ALLOWED_RISK_TIERS].sort(), ["R0", "R1", "R2", "R3"]);
});

test("R3 registration requires security profile and a declared threat model", () => {
  const complete = {
    risk_tier: "R3",
    validation_profiles: ["security"],
    required_documents: ["THREAT_MODEL.md"],
  };
  assert.deepEqual(registry.riskTierPolicyViolations(complete, true), []);

  const missingSecurity = { ...complete, validation_profiles: ["research"] };
  assert.ok(registry.riskTierPolicyViolations(missingSecurity, true).includes(
    "R3 projects must include the security validation profile"
  ));

  const missingFile = registry.riskTierPolicyViolations(complete, false);
  assert.ok(missingFile.includes("R3 projects require THREAT_MODEL.md"));

  const missingManifestEntry = { ...complete, required_documents: [] };
  assert.ok(registry.riskTierPolicyViolations(missingManifestEntry, true).includes(
    "R3 projects must list THREAT_MODEL.md in required_documents"
  ));

  assert.deepEqual(registry.riskTierPolicyViolations(
    { ...complete, risk_tier: "R2" }, false
  ), []);
});


function makeReadiness(overrides = {}) {
  return {
    assessment_version: "1.0.0",
    assessment_status: "NOT_ASSESSED",
    operational_mode: "RESEARCH_ONLY",
    independent_assurance_required: false,
    independent_assurance_rationale: "Current scope does not require an external independent reviewer before research-only work.",
    gates: [],
    authorization: {
      status: "NOT_AUTHORIZED",
      authority: null,
      authority_role: null,
      authorized_at: null,
      decision_record: null,
      source_commit: null,
    },
    ...overrides,
  };
}

function makeGate(gateId, overrides = {}) {
  const independent = ["independent_domain_review", "security_review"].includes(gateId);
  const identity = gateId === "founder_approval" ? "Kenny" : "Reviewer One";
  return {
    gate_id: gateId,
    status: "PASS",
    rationale: "The defined scope was reviewed and the recorded evidence supports this gate disposition.",
    evidence: ["https://github.com/witthaya-quant/Open-Finance-Research-Lab/issues/19"],
    recorded_by: identity,
    recorded_at: "2026-11-06",
    assessor: {
      identity,
      role: gateId === "founder_approval" ? "Founder / Principal Maintainer" : "Domain reviewer",
      independent,
      conflict_status: "NONE_DECLARED",
      review_reference: "https://github.com/witthaya-quant/Open-Finance-Research-Lab/pull/31",
    },
    ...overrides,
  };
}

test("manifest schema 1.1.0 declares a separate release-readiness and authorization contract", () => {
  const schemaPath = path.join(ROOT, "schemas", "ofrl-project-manifest-1.1.0.schema.json");
  const schema = JSON.parse(fs.readFileSync(schemaPath, "utf8"));
  assert.equal(schema.properties.schema_version.const, "1.1.0");
  assert.ok(schema.required.includes("release_readiness"));
  assert.deepEqual(
    schema.properties.release_readiness.properties.assessment_status.enum,
    ["NOT_ASSESSED", "BLOCKED", "ELIGIBLE_FOR_AUTHORIZATION"]
  );
  assert.deepEqual(
    schema.properties.release_readiness.properties.authorization.properties.status.enum,
    ["NOT_AUTHORIZED", "AUTHORIZED", "WITHDRAWN"]
  );
});

test("NOT_ASSESSED project can remain in research or development without fabricated gate evidence", () => {
  const manifest = {
    risk_tier: "R2",
    project_types: ["research", "financial_model"],
    data_classification: "synthetic",
    lifecycle_status: "in_development",
    release_status: "not_released",
    release_readiness: makeReadiness({
      independent_assurance_required: true,
      independent_assurance_rationale: "Financial-model outputs may inform business decisions and require independent domain assurance before release.",
    }),
  };
  assert.deepEqual(registry.releaseReadinessViolations(manifest, ROOT), []);
});

test("required release gates scale by risk, project type and data classification", () => {
  assert.deepEqual(
    registry.requiredReleaseGateIds({
      risk_tier: "R0",
      project_types: ["research"],
      data_classification: "none",
      lifecycle_status: "in_development",
      release_readiness: { independent_assurance_required: false },
    }),
    ["scope_and_claims", "evidence_and_limitations", "licence_and_ip", "founder_approval"]
  );

  const r2Financial = registry.requiredReleaseGateIds({
    risk_tier: "R2",
    project_types: ["financial_model", "software"],
    data_classification: "synthetic",
    lifecycle_status: "in_development",
    release_readiness: { independent_assurance_required: true },
  });
  for (const required of [
    "scope_and_claims", "evidence_and_limitations", "licence_and_ip", "founder_approval",
    "reproducibility", "domain_review", "data_rights", "numeric_and_financial_controls",
    "dependency_and_sbom", "independent_domain_review",
  ]) assert.ok(r2Financial.includes(required), required);

  const r3 = registry.requiredReleaseGateIds({
    risk_tier: "R3",
    project_types: ["software"],
    data_classification: "restricted",
    lifecycle_status: "in_development",
    release_readiness: { independent_assurance_required: true },
  });
  for (const required of [
    "independent_domain_review", "security_review", "privacy_legal_review",
    "operational_safeguards", "incident_response", "rollback_recovery", "founder_approval",
  ]) assert.ok(r3.includes(required), required);
});

test("R2 financial models must explicitly require independent domain assurance", () => {
  const manifest = {
    risk_tier: "R2",
    project_types: ["financial_model"],
    data_classification: "none",
    lifecycle_status: "in_development",
    release_status: "not_released",
    release_readiness: makeReadiness({
      independent_assurance_required: false,
      independent_assurance_rationale: "No independent reviewer is planned for this financial model.",
    }),
  };
  assert.ok(registry.releaseReadinessViolations(manifest, ROOT).some((x) => x.includes("R2 financial models must require independent")));
});

test("ELIGIBLE_FOR_AUTHORIZATION requires every applicable gate and substantive N/A rationale", () => {
  const required = ["scope_and_claims", "evidence_and_limitations", "licence_and_ip", "founder_approval"];
  const manifest = {
    risk_tier: "R0",
    project_types: ["research"],
    data_classification: "none",
    lifecycle_status: "validation",
    release_status: "not_released",
    release_readiness: makeReadiness({
      assessment_status: "ELIGIBLE_FOR_AUTHORIZATION",
      gates: required.filter((x) => x !== "founder_approval").map((x) => makeGate(x)).concat([makeGate("founder_approval")]),
    }),
  };
  assert.deepEqual(registry.releaseReadinessViolations(manifest, ROOT), []);

  manifest.release_readiness.gates.pop();
  assert.ok(registry.releaseReadinessViolations(manifest, ROOT).some((x) => x.includes("missing required gate 'founder_approval'")));

  manifest.release_readiness.gates.push(makeGate("founder_approval", { status: "NOT APPLICABLE", assessor: undefined, evidence: [], rationale: "Not applicable." }));
  assert.ok(registry.releaseReadinessViolations(manifest, ROOT).some((x) => x.includes("NOT APPLICABLE gate 'founder_approval' requires a substantive rationale")));
});

test("independent domain review and R3 security review require independent assessors", () => {
  const independentGate = makeGate("independent_domain_review", {
    assessor: { identity: "Owner", role: "Founder", independent: false, conflict_status: "NONE_DECLARED", review_reference: "https://github.com/example/review" },
  });
  const manifest = {
    risk_tier: "R2",
    project_types: ["research"],
    data_classification: "none",
    lifecycle_status: "in_development",
    release_status: "not_released",
    release_readiness: makeReadiness({
      independent_assurance_required: true,
      independent_assurance_rationale: "This claim requires independent domain assurance before it can be released.",
      gates: [independentGate],
    }),
  };
  assert.ok(registry.releaseReadinessViolations(manifest, ROOT).some((x) => x.includes("independent_domain_review PASS requires an independent assessor")));
});

test("NOT APPLICABLE requires a reason and cannot waive non-applicable R3 hard gates", () => {
  const manifest = {
    risk_tier: "R3",
    project_types: ["software", "infrastructure"],
    data_classification: "none",
    lifecycle_status: "validation",
    release_status: "not_released",
    release_readiness: makeReadiness({
      independent_assurance_required: true,
      independent_assurance_rationale: "R3 requires independent assurance because failures may cause material external harm.",
      assessment_status: "ELIGIBLE_FOR_AUTHORIZATION",
      gates: [
        "scope_and_claims", "evidence_and_limitations", "licence_and_ip", "founder_approval",
        "reproducibility", "domain_review", "dependency_and_sbom", "independent_domain_review", "security_review",
        "privacy_legal_review", "operational_safeguards", "incident_response", "rollback_recovery",
      ].map((id) => makeGate(id, id === "privacy_legal_review"
        ? { status: "NOT APPLICABLE", assessor: undefined, evidence: [], rationale: "The reviewed project scope does not process personal or regulated data; the privacy/legal assessment is therefore not applicable." }
        : {})),
    }),
  };
  const errors = registry.releaseReadinessViolations(manifest, ROOT);
  assert.deepEqual(errors, []);
  manifest.release_readiness.gates.find((x) => x.gate_id === "security_review").status = "NOT APPLICABLE";
  assert.ok(registry.releaseReadinessViolations(manifest, ROOT).some((x) => x.includes("risk-tier gate")));
});

test("LIVE_EXTERNAL is restricted to R3 and requires released, authorized readiness", () => {
  const manifest = {
    risk_tier: "R2",
    project_types: ["software"],
    data_classification: "none",
    lifecycle_status: "released",
    release_status: "released",
    version: "1.0.0",
    release_readiness: makeReadiness({
      operational_mode: "LIVE_EXTERNAL",
      authorization: {
        status: "AUTHORIZED",
        authority: "Kenny",
        authority_role: "Founder",
        authorized_at: "2026-11-06",
        decision_record: "https://github.com/example/decision",
        source_commit: "a".repeat(40),
      },
    }),
  };
  assert.ok(registry.releaseReadinessViolations(manifest, ROOT).some((x) => x.includes("LIVE_EXTERNAL operation is restricted to R3")));
});


test("a disclosed conflict cannot satisfy a required independent domain review", () => {
  const manifest = {
    risk_tier: "R2",
    project_types: ["research"],
    data_classification: "none",
    lifecycle_status: "in_development",
    release_status: "not_released",
    release_readiness: makeReadiness({
      independent_assurance_required: true,
      independent_assurance_rationale: "The material research claim requires independent domain assurance before release.",
      gates: [makeGate("independent_domain_review", {
        assessor: {
          identity: "Reviewer One",
          role: "External domain reviewer",
          independent: true,
          conflict_status: "DISCLOSED",
          conflict_rationale: "The reviewer has a current commercial relationship with the project owner.",
          review_reference: "https://github.com/example/review",
        },
      })],
    }),
  };
  assert.ok(registry.releaseReadinessViolations(manifest, ROOT).some((x) => x.includes("cannot count as independent PASS")));
});

test("PASS evidence and reviewer references must resolve to HTTPS or a safe project-local file", () => {
  const manifest = {
    risk_tier: "R0",
    project_types: ["research"],
    data_classification: "none",
    lifecycle_status: "in_development",
    release_status: "not_released",
    release_readiness: makeReadiness({
      gates: [makeGate("scope_and_claims", {
        evidence: ["http://insecure.example/evidence"],
        assessor: { identity: "Reviewer", role: "Research reviewer", independent: false, conflict_status: "NONE_DECLARED", review_reference: "../outside.md" },
      })],
    }),
  };
  const errors = registry.releaseReadinessViolations(manifest, ROOT);
  assert.ok(errors.some((x) => x.includes("unsafe or unresolved evidence")));
  assert.ok(errors.some((x) => x.includes("reviewer/decision reference")));
});
