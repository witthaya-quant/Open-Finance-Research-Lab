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
  const schemaPath = path.join(ROOT, "schemas", "ofrl-project-manifest-1.0.0.schema.json");
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
  const schemaPath = path.join(ROOT, "schemas", "ofrl-project-manifest-1.0.0.schema.json");
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
