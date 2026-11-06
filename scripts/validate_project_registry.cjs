#!/usr/bin/env node
"use strict";

/**
 * Portfolio registry gate.
 *
 * Checks the published JSON Schema, every project manifest, cross-file
 * requirements, type-specific documents, declared CI workflows, lifecycle
 * consistency, and the human-readable project catalog.
 *
 * This gate does not replace each project's domain-specific tests,
 * mathematical validation, security assessment, or research review.
 */

const fs = require("node:fs");
const path = require("node:path");

const ROOT = path.resolve(__dirname, "..");
const PROJECTS_ROOT = path.join(ROOT, "projects");
const CATALOG_PATH = path.join(ROOT, "PROJECT_CATALOG.md");
const SCHEMA_PATH = path.join(ROOT, "schemas", "ofrl-project-manifest-1.1.0.schema.json");
const NODE_MODULES = path.join(__dirname, "schema-validator", "node_modules");

const AjvModule = require(path.join(NODE_MODULES, "ajv", "dist", "2020"));
const Ajv2020 = AjvModule.default || AjvModule;
const FormatsModule = require(path.join(NODE_MODULES, "ajv-formats"));
const addFormats = FormatsModule.default || FormatsModule;

const ALLOWED_TYPES = new Set([
  "research",
  "quantitative_research",
  "financial_model",
  "software",
  "data",
  "benchmark",
  "prototype",
  "infrastructure",
]);

const ALLOWED_STATUSES = new Set([
  "proposed",
  "approved",
  "in_development",
  "validation",
  "released",
  "maintenance",
  "deprecated",
  "archived",
]);

const ALLOWED_RISK_TIERS = new Set(["R0", "R1", "R2", "R3"]);
const ALLOWED_RELEASE_STATES = new Set([
  "not_released",
  "pre_release",
  "released",
  "withdrawn",
]);

const BASE_REQUIRED_GROUPS = [
  { alternatives: ["README.md"] },
  { alternatives: ["DECISION_LOG.md"] },
  { alternatives: ["VALIDATION_MATRIX.md", "VALIDATION_PLAN.md"] },
];

const ALLOWED_VALIDATION_PROFILES = new Set([
  "research",
  "quantitative",
  "financial_model",
  "software",
  "data",
  "benchmark",
  "prototype",
  "security",
  "privacy",
  "performance",
  "integration",
]);

const TYPE_REQUIREMENTS = {
  research: [["RESEARCH_SPECIFICATION.md", "PROJECT_SPECIFICATION.md"]],
  quantitative_research: [["RESEARCH_SPECIFICATION.md", "PROJECT_SPECIFICATION.md"]],
  financial_model: [
    ["MODEL_CONVENTIONS.md"],
    ["NUMERIC_ENVELOPE.md"],
  ],
  software: [["ENGINEERING_ARCHITECTURE.md"]],
  data: [["DATA_MANIFEST.md"]],
  benchmark: [["BENCHMARK_SPECIFICATION.md"]],
  prototype: [["PROTOTYPE_PLAN.md"]],
  infrastructure: [
    ["ENGINEERING_ARCHITECTURE.md"],
    ["THREAT_MODEL.md"],
  ],
};

const TYPE_VALIDATION_PROFILES = {
  research: ["research"],
  quantitative_research: ["research", "quantitative"],
  financial_model: ["financial_model"],
  software: ["software"],
  data: ["data"],
  benchmark: ["benchmark"],
  prototype: ["prototype"],
  infrastructure: ["integration", "security"],
};

const TYPE_LABELS = {
  research: "Research",
  quantitative_research: "Quantitative Research",
  financial_model: "Financial Model",
  software: "Software",
  data: "Data",
  benchmark: "Benchmark",
  prototype: "Prototype",
  infrastructure: "Infrastructure",
};

const STATUS_LABELS = {
  proposed: "Proposed",
  approved: "Approved",
  in_development: "In Development",
  validation: "Validation",
  released: "Released",
  maintenance: "Maintenance",
  deprecated: "Deprecated",
  archived: "Archived",
};

const MATURITY_LABELS = {
  0: "Proposed",
  1: "Exploratory",
  2: "Reproducible",
  3: "Robustness Validated",
  4: "Independently Replicated",
};

const errors = [];

function fail(message) {
  errors.push(message);
}

function isObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function missingBaseDocumentGroups(requiredDocuments) {
  const documents = Array.isArray(requiredDocuments) ? requiredDocuments : [];
  return BASE_REQUIRED_GROUPS.filter(
    (group) => !group.alternatives.some((document) => documents.includes(document))
  );
}

function missingTypeValidationProfiles(projectTypes, validationProfiles) {
  const profiles = Array.isArray(validationProfiles) ? validationProfiles : [];
  const types = Array.isArray(projectTypes) ? projectTypes : [];
  const missing = [];

  for (const type of types) {
    for (const profile of TYPE_VALIDATION_PROFILES[type] || []) {
      if (!profiles.includes(profile)) missing.push({ type, profile });
    }
  }
  return missing;
}

function dataPolicyViolations(manifest) {
  if (!manifest || manifest.data_classification === "none") return [];

  const violations = [];
  const profiles = Array.isArray(manifest.validation_profiles) ? manifest.validation_profiles : [];
  const documents = Array.isArray(manifest.required_documents) ? manifest.required_documents : [];

  if (!profiles.includes("data")) {
    violations.push("non-none data classification requires the 'data' validation profile");
  }
  if (!documents.includes("DATA_MANIFEST.md")) {
    violations.push("non-none data classification requires DATA_MANIFEST.md in required_documents");
  }
  return violations;
}

function releaseStateViolations(manifest) {
  const violations = [];
  const lifecycle = manifest?.lifecycle_status;
  const release = manifest?.release_status;
  const versionIsPresent = typeof manifest?.version === "string" && manifest.version.trim().length > 0;

  if (release === "pre_release" && lifecycle !== "validation") {
    violations.push("pre_release artifacts require lifecycle_status 'validation'");
  }

  if (release === "released") {
    if (!["released", "maintenance", "deprecated", "archived"].includes(lifecycle)) {
      violations.push("release_status 'released' requires lifecycle_status released, maintenance, deprecated, or archived");
    }
    if (!versionIsPresent) {
      violations.push("release_status 'released' requires a non-empty version");
    }
  }

  if (["released", "maintenance"].includes(lifecycle) && release !== "released") {
    violations.push("released/maintenance lifecycle requires release_status 'released'");
  }

  if (release === "withdrawn" && !["deprecated", "archived"].includes(lifecycle)) {
    violations.push("withdrawn releases must be deprecated or archived");
  }

  return violations;
}

function riskTierPolicyViolations(manifest, threatModelExists) {
  if (!manifest || manifest.risk_tier !== "R3") return [];

  const violations = [];
  const profiles = Array.isArray(manifest.validation_profiles) ? manifest.validation_profiles : [];
  const documents = Array.isArray(manifest.required_documents) ? manifest.required_documents : [];

  if (!profiles.includes("security")) {
    violations.push("R3 projects must include the security validation profile");
  }
  if (!threatModelExists) {
    violations.push("R3 projects require THREAT_MODEL.md");
  } else if (!documents.includes("THREAT_MODEL.md")) {
    violations.push("R3 projects must list THREAT_MODEL.md in required_documents");
  }
  return violations;
}

const REQUIRED_RELEASE_GATE_IDS = new Set([
  "scope_and_claims",
  "evidence_and_limitations",
  "data_rights",
  "licence_and_ip",
  "reproducibility",
  "domain_review",
  "numeric_and_financial_controls",
  "dependency_and_sbom",
  "independent_domain_review",
  "security_review",
  "privacy_legal_review",
  "operational_safeguards",
  "incident_response",
  "rollback_recovery",
  "founder_approval",
  "maintenance_and_corrections",
]);

function assessmentRequiresGateInventory(status) {
  return status === "BLOCKED" || status === "ELIGIBLE_FOR_AUTHORIZATION";
}

function requiredReleaseGateIds(manifest) {
  const ids = [
    "scope_and_claims",
    "evidence_and_limitations",
    "licence_and_ip",
    "founder_approval",
  ];
  const types = new Set(Array.isArray(manifest?.project_types) ? manifest.project_types : []);

  if (manifest?.risk_tier && manifest.risk_tier !== "R0") {
    ids.push("reproducibility", "domain_review");
  }
  if (manifest?.data_classification && manifest.data_classification !== "none") {
    ids.push("data_rights", "privacy_legal_review");
  }
  if (types.has("financial_model")) {
    ids.push("numeric_and_financial_controls");
  }
  if (["software", "infrastructure"].some((type) => types.has(type))) {
    ids.push("dependency_and_sbom");
  }
  if (manifest?.release_readiness?.independent_assurance_required === true) {
    ids.push("independent_domain_review");
  }
  if (manifest?.lifecycle_status === "maintenance") {
    ids.push("maintenance_and_corrections");
  }
  if (manifest?.risk_tier === "R3") {
    ids.push(
      "independent_domain_review",
      "security_review",
      "privacy_legal_review",
      "operational_safeguards",
      "incident_response",
      "rollback_recovery"
    );
  }
  return [...new Set(ids)];
}

function isSafeEvidenceReference(reference, projectDir) {
  if (typeof reference !== "string" || reference.trim() !== reference || reference.length === 0) {
    return false;
  }
  try {
    const url = new URL(reference);
    return url.protocol === "https:" && Boolean(url.hostname) && !url.username && !url.password;
  } catch {
    return Boolean(resolveFileWithin(projectDir, reference));
  }
}

function residualRiskAssessmentViolations(readiness, projectDir) {
  const violations = [];
  const assessment = readiness?.residual_risk_assessment;
  if (!isObject(assessment)) {
    return ["project.json: residual_risk_assessment must be an object"];
  }

  const risks = Array.isArray(assessment.risks) ? assessment.risks : [];
  if (assessment.status === "NOT_ASSESSED") {
    if (assessment.outcome !== "NOT_ASSESSED" ||
        assessment.reviewed_by !== null ||
        assessment.reviewer_role !== null ||
        assessment.reviewed_at !== null ||
        assessment.next_review_at !== null ||
        (assessment.evidence?.length ?? 0) !== 0 ||
        risks.length !== 0) {
      violations.push("project.json: NOT_ASSESSED residual-risk state must not contain an assessment outcome, reviewer, dates, evidence or risk records");
    }
    return violations;
  }

  if (assessment.status !== "ASSESSED") {
    violations.push("project.json: residual-risk assessment status must be NOT_ASSESSED or ASSESSED");
    return violations;
  }
  if (assessment.outcome === "NOT_ASSESSED") {
    violations.push("project.json: an ASSESSED residual-risk review requires an explicit outcome");
  }
  if (typeof assessment.reviewed_by !== "string" ||
      typeof assessment.reviewer_role !== "string" ||
      typeof assessment.reviewed_at !== "string" ||
      typeof assessment.next_review_at !== "string") {
    violations.push("project.json: ASSESSED residual-risk review requires a named reviewer, role and review dates");
  } else if (Date.parse(assessment.next_review_at) < Date.parse(assessment.reviewed_at)) {
    violations.push("project.json: residual-risk next_review_at must not precede reviewed_at");
  }
  if (!Array.isArray(assessment.evidence) || assessment.evidence.length === 0) {
    violations.push("project.json: ASSESSED residual-risk review requires at least one evidence reference");
  }

  for (const reference of assessment.evidence ?? []) {
    if (!isSafeEvidenceReference(reference, projectDir)) {
      violations.push("project.json: residual-risk assessment has an unsafe or unresolved evidence reference");
    }
  }

  if (assessment.outcome === "NO_RESIDUAL_RISKS" && risks.length !== 0) {
    violations.push("project.json: NO_RESIDUAL_RISKS outcome must have an empty risks array");
  }
  if (assessment.outcome === "RESIDUAL_RISKS_RECORDED" && risks.length === 0) {
    violations.push("project.json: RESIDUAL_RISKS_RECORDED outcome requires at least one risk record");
  }

  const seenRiskIds = new Set();
  for (const risk of risks) {
    if (!isObject(risk) || typeof risk.risk_id !== "string") continue;
    if (seenRiskIds.has(risk.risk_id)) {
      violations.push("project.json: duplicate residual-risk id '" + risk.risk_id + "'");
    }
    seenRiskIds.add(risk.risk_id);

    if (Date.parse(risk.review_due_at) > Date.parse(risk.expires_at)) {
      violations.push("project.json: residual risk '" + risk.risk_id + "' review_due_at must not be later than expires_at");
    }
    if (typeof assessment.next_review_at === "string" &&
        Date.parse(risk.review_due_at) > Date.parse(assessment.next_review_at)) {
      violations.push("project.json: residual risk '" + risk.risk_id + "' review_due_at must not be later than the assessment next_review_at");
    }
    if (typeof assessment.reviewed_at === "string" &&
        Date.parse(risk.expires_at) < Date.parse(assessment.reviewed_at)) {
      violations.push("project.json: residual risk '" + risk.risk_id + "' was already expired when the assessment was recorded");
    }

    for (const reference of risk.evidence ?? []) {
      if (!isSafeEvidenceReference(reference, projectDir)) {
        violations.push("project.json: residual risk '" + risk.risk_id + "' has an unsafe or unresolved evidence reference");
      }
    }

    if (risk.decision === "ACCEPTED") {
      if (typeof risk.accepted_by !== "string" ||
          typeof risk.accepted_by_role !== "string" ||
          typeof risk.accepted_at !== "string" ||
          typeof risk.decision_reference !== "string") {
        violations.push("project.json: ACCEPTED residual risk '" + risk.risk_id + "' requires a named accepting authority, role, date and decision reference");
      } else if (!isSafeEvidenceReference(risk.decision_reference, projectDir)) {
        violations.push("project.json: ACCEPTED residual risk '" + risk.risk_id + "' requires a resolvable acceptance decision reference");
      }
      if (!Array.isArray(risk.evidence) || risk.evidence.length === 0) {
        violations.push("project.json: ACCEPTED residual risk '" + risk.risk_id + "' requires supporting evidence");
      }
      if (typeof risk.accepted_at === "string" && Date.parse(risk.accepted_at) > Date.parse(risk.expires_at)) {
        violations.push("project.json: accepted residual risk '" + risk.risk_id + "' cannot be accepted after its expiry date");
      }
    } else if (risk.accepted_by !== null || risk.accepted_by_role !== null || risk.accepted_at !== null) {
      violations.push("project.json: non-ACCEPTED residual risk '" + risk.risk_id + "' must not carry acceptance fields");
    }

    if (["MITIGATED", "TRANSFERRED", "CLOSED"].includes(risk.decision)) {
      if (!Array.isArray(risk.evidence) || risk.evidence.length === 0 ||
          typeof risk.decision_reference !== "string" ||
          !isSafeEvidenceReference(risk.decision_reference, projectDir)) {
        violations.push("project.json: " + risk.decision + " residual risk '" + risk.risk_id + "' requires evidence and a resolvable decision reference");
      }
    }
  }

  return violations;
}

function releaseReadinessViolations(manifest, projectDir) {
  const violations = [];
  const readiness = manifest?.release_readiness;
  if (!isObject(readiness)) {
    return ["project.json: release_readiness must be an object under manifest schema 1.1.0"];
  }

  for (const violation of residualRiskAssessmentViolations(readiness, projectDir)) {
    violations.push(violation);
  }

  const gates = Array.isArray(readiness.gates) ? readiness.gates : [];
  const gatesById = new Map();
  for (const gate of gates) {
    if (!isObject(gate) || typeof gate.gate_id !== "string") continue;
    if (gatesById.has(gate.gate_id)) {
      violations.push("project.json: duplicate release-readiness gate '" + gate.gate_id + "'");
    } else {
      gatesById.set(gate.gate_id, gate);
    }
    if (!REQUIRED_RELEASE_GATE_IDS.has(gate.gate_id)) {
      violations.push("project.json: unsupported release-readiness gate '" + gate.gate_id + "'");
      continue;
    }

    if (Array.isArray(gate.evidence)) {
      for (const reference of gate.evidence) {
        if (!isSafeEvidenceReference(reference, projectDir)) {
          violations.push("project.json: release gate '" + gate.gate_id + "' has an unsafe or unresolved evidence reference");
        }
      }
    }

    if (gate.status === "PASS") {
      if (!isObject(gate.assessor)) {
        violations.push("project.json: PASS gate '" + gate.gate_id + "' requires a named assessor");
        continue;
      }
      if (!isSafeEvidenceReference(gate.assessor.review_reference, projectDir)) {
        violations.push("project.json: PASS gate '" + gate.gate_id + "' requires a resolvable reviewer/decision reference");
      }
      if (gate.assessor.independent === true && gate.assessor.conflict_status !== "NONE_DECLARED") {
        violations.push("project.json: assessor for '" + gate.gate_id + "' cannot claim independence while a conflict is disclosed");
      }
      const ownerIdentity = String(manifest.owner || "").toLocaleLowerCase().replace(/[^a-z0-9]/g, "");
      const assessorIdentity = String(gate.assessor.identity || "").toLocaleLowerCase().replace(/[^a-z0-9]/g, "");
      if (gate.assessor.independent === true && ownerIdentity && assessorIdentity === ownerIdentity) {
        violations.push("project.json: project owner cannot be recorded as an independent assessor");
      }
      const requiresIndependentAssessor =
        gate.gate_id === "independent_domain_review" ||
        (gate.gate_id === "security_review" && manifest.risk_tier === "R3");
      if (requiresIndependentAssessor && gate.assessor.independent !== true) {
        violations.push("project.json: " + gate.gate_id + " PASS requires an independent assessor");
      }
      if (requiresIndependentAssessor && gate.assessor.conflict_status !== "NONE_DECLARED") {
        violations.push("project.json: " + gate.gate_id + " cannot count as independent PASS when a reviewer conflict is disclosed");
      }
      if (gate.gate_id === "founder_approval" && gate.assessor.independent === true) {
        violations.push("project.json: founder_approval must not be represented as independent review");
      }
    }

    if (gate.status === "NOT APPLICABLE" && (typeof gate.rationale !== "string" || gate.rationale.trim().length < 30)) {
      violations.push("project.json: NOT APPLICABLE gate '" + gate.gate_id + "' requires a substantive rationale");
    }
  }

  const required = requiredReleaseGateIds(manifest);
  if (assessmentRequiresGateInventory(readiness.assessment_status)) {
    for (const gateId of required) {
      if (!gatesById.has(gateId)) {
        violations.push("project.json: release readiness is missing required gate '" + gateId + "'");
      }
    }
  }

  if (manifest.risk_tier === "R2" &&
      Array.isArray(manifest.project_types) &&
      manifest.project_types.includes("financial_model") &&
      readiness.independent_assurance_required !== true) {
    violations.push("R2 financial models must require independent domain assurance before release");
  }

  if (manifest.risk_tier === "R3" && readiness.independent_assurance_required !== true) {
    violations.push("R3 projects must require independent assurance");
  }

  const assessment = readiness.assessment_status;
  const authorization = readiness.authorization;
  const residualRiskAssessment = readiness.residual_risk_assessment;

  if (assessment === "NOT_ASSESSED" && residualRiskAssessment?.status !== "NOT_ASSESSED") {
    violations.push("project.json: NOT_ASSESSED release readiness must keep residual-risk assessment NOT_ASSESSED");
  }
  if (assessment === "NOT_ASSESSED" && gates.length !== 0) {
    violations.push("project.json: NOT_ASSESSED release readiness must not contain gate dispositions");
  }
  if (assessment === "ELIGIBLE_FOR_AUTHORIZATION") {
    if (residualRiskAssessment?.status !== "ASSESSED" ||
        !["NO_RESIDUAL_RISKS", "RESIDUAL_RISKS_RECORDED"].includes(residualRiskAssessment?.outcome)) {
      violations.push("project.json: ELIGIBLE_FOR_AUTHORIZATION requires a completed residual-risk assessment");
    }
    if ((residualRiskAssessment?.risks ?? []).some((risk) => risk.decision === "OPEN")) {
      violations.push("project.json: ELIGIBLE_FOR_AUTHORIZATION cannot contain OPEN residual risks");
    }
  }
  const allRequiredSatisfied = required.every((gateId) => {
    const gate = gatesById.get(gateId);
    if (!gate) return false;
    if (gate.status === "PASS") return true;
    if (gate.status !== "NOT APPLICABLE") return false;
    return gateId === "privacy_legal_review";
  });
  const hasUnresolvedGate =
    required.some((gateId) => {
      const gate = gatesById.get(gateId);
      return !gate || !["PASS", "NOT APPLICABLE"].includes(gate.status);
    }) ||
    gates.some((gate) => ["FAIL", "NOT RUN", "BLOCKED"].includes(gate.status));

  if (assessment === "ELIGIBLE_FOR_AUTHORIZATION") {
    if (!allRequiredSatisfied) {
      violations.push("project.json: ELIGIBLE_FOR_AUTHORIZATION requires every applicable risk-tier gate to PASS or have a justified NOT APPLICABLE state");
    }
    if (gates.some((gate) => gate.status === "FAIL" || gate.status === "NOT RUN" || gate.status === "BLOCKED")) {
      violations.push("project.json: eligible release readiness cannot contain FAIL, NOT RUN, or BLOCKED gates");
    }
  }

  if (assessment === "BLOCKED" && !hasUnresolvedGate) {
    violations.push("project.json: BLOCKED release readiness must identify at least one unresolved required gate");
  }

  if (authorization?.status === "AUTHORIZED" && assessment !== "ELIGIBLE_FOR_AUTHORIZATION") {
    violations.push("project.json: authorization cannot be AUTHORIZED before release readiness is eligible");
  }

  if (authorization?.status === "AUTHORIZED") {
    const founderGate = gatesById.get("founder_approval");
    if (!founderGate || founderGate.status !== "PASS") {
      violations.push("project.json: AUTHORIZED release requires a PASS founder_approval gate");
    } else if (founderGate.assessor?.identity !== authorization.authority) {
      violations.push("project.json: release authority must match the recorded founder_approval assessor");
    }
  }

  const requiresAuthorization =
    manifest.release_status === "pre_release" ||
    manifest.release_status === "released" ||
    readiness.operational_mode === "LIVE_EXTERNAL";

  if (requiresAuthorization && authorization?.status !== "AUTHORIZED") {
    violations.push("project.json: pre_release, released, and LIVE_EXTERNAL states require explicit AUTHORIZED release readiness");
  }

  if (manifest.release_status === "withdrawn" && authorization?.status !== "WITHDRAWN") {
    violations.push("project.json: withdrawn release_status requires WITHDRAWN release authorization");
  }
  if (authorization?.status === "WITHDRAWN" && manifest.release_status !== "withdrawn") {
    violations.push("project.json: WITHDRAWN authorization requires release_status 'withdrawn'");
  }

  if (readiness.operational_mode === "LIVE_EXTERNAL") {
    if (manifest.risk_tier !== "R3") {
      violations.push("project.json: LIVE_EXTERNAL operation is restricted to R3 projects");
    }
    if (manifest.release_status !== "released") {
      violations.push("project.json: LIVE_EXTERNAL operation requires release_status 'released'");
    }
  }

  if (authorization?.status === "AUTHORIZED" && !isSafeEvidenceReference(authorization.decision_record, projectDir)) {
    violations.push("project.json: AUTHORIZED release requires a resolvable decision_record reference");
  }

  if (authorization?.status === "AUTHORIZED" && typeof authorization.source_commit !== "string") {
    violations.push("project.json: AUTHORIZED release requires the exact 40-character source_commit");
  }

  return violations;
}

function relativeName(filePath) {
  return path.relative(ROOT, filePath).split(path.sep).join("/");
}

function resolveFileWithin(baseDirectory, relativePath) {
  if (typeof relativePath !== "string" || relativePath.length === 0) return null;
  if (relativePath.startsWith("/") || relativePath.includes(String.fromCharCode(92))) return null;

  const base = path.resolve(baseDirectory);
  const resolved = path.resolve(base, relativePath);
  const relative = path.relative(base, resolved);

  if (
    relative === "" ||
    relative === ".." ||
    relative.startsWith(".." + path.sep) ||
    path.isAbsolute(relative)
  ) {
    return null;
  }

  if (!fs.existsSync(resolved)) return null;

  try {
    const stat = fs.lstatSync(resolved);
    if (!stat.isFile() || stat.isSymbolicLink()) return null;

    const realBase = fs.realpathSync(base);
    const realFile = fs.realpathSync(resolved);
    const realRelative = path.relative(realBase, realFile);

    if (
      realRelative === ".." ||
      realRelative.startsWith(".." + path.sep) ||
      path.isAbsolute(realRelative)
    ) {
      return null;
    }
  } catch {
    return null;
  }

  return resolved;
}

function validateStrictDate(value) {
  if (typeof value !== "string" || !/^\d{4}-\d{2}-\d{2}$/.test(value)) return false;
  const parsed = new Date(value + "T00:00:00Z");
  return !Number.isNaN(parsed.getTime()) && parsed.toISOString().slice(0, 10) === value;
}

function parseCatalog(catalogText) {
  const rows = new Map();
  const lines = catalogText.split(/\r?\n/);

  for (const line of lines) {
    if (!/^\|\s*\[OFRL-/.test(line)) continue;

    const match = line.match(/^\|\s*\[(OFRL-[A-Z0-9]{2,8}-[0-9]{3})\]\(projects\/(OFRL-[A-Z0-9]{2,8}-[0-9]{3})\/README\.md\)\s*\|/);
    if (!match) {
      fail("PROJECT_CATALOG.md: malformed project row: " + line);
      continue;
    }

    const id = match[1];
    const linkedId = match[2];

    if (id !== linkedId) {
      fail("PROJECT_CATALOG.md: row ID and README link disagree for " + id);
      continue;
    }
    if (rows.has(id)) {
      fail("PROJECT_CATALOG.md: duplicate row for " + id);
      continue;
    }

    const cells = line.split("|").slice(1, -1).map((cell) => cell.trim());
    if (cells.length < 6) {
      fail("PROJECT_CATALOG.md: project row for " + id + " must contain six columns");
      continue;
    }

    rows.set(id, {
      id,
      title: cells[1],
      types: cells[2].split(",").map((item) => item.trim()).filter(Boolean).sort(),
      status: cells[3],
      risk: cells[4],
      evidence: cells[5],
    });
  }

  return rows;
}

function validateCatalogRow(manifest, catalogRows, relativeDir) {
  const row = catalogRows.get(manifest.project_id);

  if (!row) {
    fail(relativeDir + ": project is missing from PROJECT_CATALOG.md");
    return;
  }

  if (row.title !== manifest.title) {
    fail("PROJECT_CATALOG.md: title drift for " + manifest.project_id);
  }

  const expectedTypes = manifest.project_types
    .map((type) => TYPE_LABELS[type] || type)
    .sort();

  if (JSON.stringify(row.types) !== JSON.stringify(expectedTypes)) {
    fail("PROJECT_CATALOG.md: project type drift for " + manifest.project_id);
  }

  if (row.status !== STATUS_LABELS[manifest.lifecycle_status]) {
    fail("PROJECT_CATALOG.md: lifecycle status drift for " + manifest.project_id);
  }

  if (row.risk !== manifest.risk_tier) {
    fail("PROJECT_CATALOG.md: risk tier drift for " + manifest.project_id);
  }

  const maturityName = MATURITY_LABELS[manifest.evidence_maturity];
  const expectedEvidence = String(manifest.evidence_maturity) + " — " + maturityName;

  if (row.evidence !== expectedEvidence) {
    fail("PROJECT_CATALOG.md: evidence maturity drift for " + manifest.project_id);
  }
}

function validateManifest(projectDir, folderName, catalogRows, seenIds, validateSchema) {
  const manifestPath = path.join(projectDir, "project.json");
  const relativeDir = relativeName(projectDir);

  if (!fs.existsSync(manifestPath)) {
    fail(relativeDir + ": missing project.json manifest");
    return;
  }

  let manifest;
  try {
    manifest = JSON.parse(fs.readFileSync(manifestPath, "utf8"));
  } catch (error) {
    fail(relativeDir + "/project.json: invalid JSON: " + error.message);
    return;
  }

  if (!isObject(manifest)) {
    fail(relativeDir + "/project.json: manifest must be a JSON object");
    return;
  }

  if (!validateSchema(manifest)) {
    for (const issue of validateSchema.errors || []) {
      fail(
        relativeDir + "/project.json: schema violation at " +
        (issue.instancePath || "/") + " (" + issue.keyword + "): " + issue.message
      );
    }
  }

  if (manifest.schema_version !== "1.1.0") {
    fail(relativeDir + "/project.json: unsupported schema_version; expected 1.1.0");
  }

  if (manifest.$schema !== "../../schemas/ofrl-project-manifest-1.1.0.schema.json") {
    fail(relativeDir + "/project.json: $schema must reference the repository project-manifest schema 1.1.0");
  }

  const id = manifest.project_id;
  if (typeof id !== "string" || !/^OFRL-[A-Z0-9]{2,8}-[0-9]{3}$/.test(id)) {
    fail(relativeDir + "/project.json: invalid project_id format");
  } else {
    if (id !== folderName) fail(relativeDir + ": folder name must exactly match project_id '" + id + "'");
    if (seenIds.has(id)) fail(relativeDir + ": duplicate project_id '" + id + "'");
    seenIds.add(id);
    validateCatalogRow(manifest, catalogRows, relativeDir);
  }

  if (!ALLOWED_STATUSES.has(manifest.lifecycle_status)) {
    fail(relativeDir + "/project.json: invalid lifecycle_status '" + manifest.lifecycle_status + "'");
  }

  if (!Array.isArray(manifest.project_types) || manifest.project_types.length === 0) {
    fail(relativeDir + "/project.json: project_types must be a non-empty array");
  } else {
    for (const type of manifest.project_types) {
      if (!ALLOWED_TYPES.has(type)) fail(relativeDir + "/project.json: unsupported project type '" + type + "'");
    }
  }

  if (!ALLOWED_RISK_TIERS.has(manifest.risk_tier)) {
    fail(relativeDir + "/project.json: invalid risk_tier '" + manifest.risk_tier + "'");
  }

  if (!validateStrictDate(manifest.created_at)) {
    fail(relativeDir + "/project.json: created_at must be a valid YYYY-MM-DD date");
  }
  if (!validateStrictDate(manifest.updated_at)) {
    fail(relativeDir + "/project.json: updated_at must be a valid YYYY-MM-DD date");
  }
  if (
    validateStrictDate(manifest.created_at) &&
    validateStrictDate(manifest.updated_at) &&
    manifest.updated_at < manifest.created_at
  ) {
    fail(relativeDir + "/project.json: updated_at cannot precede created_at");
  }

  if (!Array.isArray(manifest.validation_profiles) || manifest.validation_profiles.length === 0) {
    fail(relativeDir + "/project.json: validation_profiles must be a non-empty array");
  } else {
    for (const missing of missingTypeValidationProfiles(manifest.project_types, manifest.validation_profiles)) {
      fail(relativeDir + "/project.json: project type '" + missing.type + "' requires validation profile '" + missing.profile + "'");
    }

    for (const profile of manifest.validation_profiles) {
      if (!ALLOWED_VALIDATION_PROFILES.has(profile)) {
        fail(relativeDir + "/project.json: unsupported validation profile '" + profile + "'");
      }
    }
  }

  if (!Array.isArray(manifest.required_documents)) {
    fail(relativeDir + "/project.json: required_documents must be an array");
  } else {
    const uniqueDocs = new Set(manifest.required_documents);
    if (uniqueDocs.size !== manifest.required_documents.length) {
      fail(relativeDir + "/project.json: required_documents contains duplicates");
    }

    for (const group of missingBaseDocumentGroups(manifest.required_documents)) {
      fail(relativeDir + "/project.json: required_documents must include one of: " + group.alternatives.join(", "));
    }

    for (const required of manifest.required_documents) {
      if (!resolveFileWithin(projectDir, required)) {
        fail(relativeDir + "/project.json: required document missing, unsafe, or outside project: " + required);
      }
    }
  }

  for (const type of Array.isArray(manifest.project_types) ? manifest.project_types : []) {
    for (const alternatives of TYPE_REQUIREMENTS[type] || []) {
      const selected = alternatives.find((file) => resolveFileWithin(projectDir, file));

      if (!selected) {
        fail(relativeDir + ": project type '" + type + "' requires one of: " + alternatives.join(", "));
      } else if (
        Array.isArray(manifest.required_documents) &&
        !manifest.required_documents.includes(selected)
      ) {
        fail(relativeDir + "/project.json: required_documents must list type-specific document '" + selected + "'");
      }
    }
  }

  const riskViolations = riskTierPolicyViolations(
    manifest,
    Boolean(resolveFileWithin(projectDir, "THREAT_MODEL.md"))
  );
  for (const violation of riskViolations) {
    fail(relativeDir + ": " + violation);
  }

  for (const violation of dataPolicyViolations(manifest)) {
    fail(relativeDir + ": " + violation);
  }

  if (!ALLOWED_RELEASE_STATES.has(manifest.release_status)) {
    fail(relativeDir + "/project.json: invalid release_status '" + manifest.release_status + "'");
  }

  for (const violation of releaseStateViolations(manifest)) {
    fail(relativeDir + ": " + violation);
  }

  for (const violation of releaseReadinessViolations(manifest, projectDir)) {
    fail(relativeDir + ": " + violation);
  }

  const workflowTypes = ["software", "data", "benchmark", "infrastructure"];
  const requiresWorkflow = Array.isArray(manifest.project_types) &&
    manifest.project_types.some((type) => workflowTypes.includes(type));

  if (
    requiresWorkflow &&
    (!Array.isArray(manifest.validation_workflows) || manifest.validation_workflows.length === 0)
  ) {
    fail(relativeDir + "/project.json: software, data, benchmark, and infrastructure projects must declare validation_workflows");
  }

  if (manifest.validation_workflows !== undefined) {
    if (!Array.isArray(manifest.validation_workflows)) {
      fail(relativeDir + "/project.json: validation_workflows must be an array");
    } else {
      const uniqueWorkflows = new Set(manifest.validation_workflows);

      if (uniqueWorkflows.size !== manifest.validation_workflows.length) {
        fail(relativeDir + "/project.json: validation_workflows contains duplicates");
      }

      for (const workflowPath of manifest.validation_workflows) {
        const hasYamlExtension =
          typeof workflowPath === "string" &&
          (workflowPath.endsWith(".yml") || workflowPath.endsWith(".yaml"));

        if (
          typeof workflowPath !== "string" ||
          !workflowPath.startsWith(".github/workflows/") ||
          !hasYamlExtension
        ) {
          fail(relativeDir + "/project.json: workflow must be a .yml/.yaml file under .github/workflows: " + workflowPath);
          continue;
        }

        const workflowFile = resolveFileWithin(ROOT, workflowPath);
        if (!workflowFile) {
          fail(relativeDir + "/project.json: validation workflow does not exist or is unsafe: " + workflowPath);
          continue;
        }

        const workflowSource = fs.readFileSync(workflowFile, "utf8");
        if (!/^\s*pull_request\s*:/m.test(workflowSource)) {
          fail(relativeDir + ": declared validation workflow must run on pull_request: " + workflowPath);
        }

        const hasPathFilter = /^\s*paths\s*:/m.test(workflowSource);
        if (hasPathFilter && !workflowSource.includes("projects/" + folderName + "/")) {
          fail(relativeDir + ": path-filtered validation workflow must include projects/" + folderName + "/: " + workflowPath);
        }
      }
    }
  }
}

function main() {
  if (!fs.existsSync(PROJECTS_ROOT) || !fs.statSync(PROJECTS_ROOT).isDirectory()) {
    console.error("Project registry validation FAILED: /projects directory not found");
    process.exitCode = 1;
    return;
  }

  if (!fs.existsSync(CATALOG_PATH) || !fs.existsSync(SCHEMA_PATH)) {
    console.error("Project registry validation FAILED: project catalog or manifest schema missing");
    process.exitCode = 1;
    return;
  }

  let schema;
  try {
    schema = JSON.parse(fs.readFileSync(SCHEMA_PATH, "utf8"));
  } catch (error) {
    console.error("Project manifest schema is invalid JSON: " + error.message);
    process.exitCode = 1;
    return;
  }

  const catalogText = fs.readFileSync(CATALOG_PATH, "utf8");
  const catalogRows = parseCatalog(catalogText);
  const ajv = new Ajv2020({
    allErrors: true,
    strict: true,
    validateFormats: true,
  });
  addFormats(ajv);

  let validateSchema;
  try {
    validateSchema = ajv.compile(schema);
  } catch (error) {
    console.error("Project manifest JSON Schema failed to compile: " + error.message);
    process.exitCode = 1;
    return;
  }

  const projectEntries = fs.readdirSync(PROJECTS_ROOT, { withFileTypes: true })
    .filter((entry) => entry.isDirectory() && !entry.name.startsWith("."))
    .sort((a, b) => a.name.localeCompare(b.name));

  const seenIds = new Set();
  const folderNames = new Set(projectEntries.map((entry) => entry.name));

  if (projectEntries.length === 0) fail("/projects must contain at least one registered project");

  for (const entry of projectEntries) {
    validateManifest(
      path.join(PROJECTS_ROOT, entry.name),
      entry.name,
      catalogRows,
      seenIds,
      validateSchema
    );
  }

  for (const id of catalogRows.keys()) {
    if (!folderNames.has(id)) {
      fail("PROJECT_CATALOG.md: stale project entry '" + id + "' has no projects/" + id + "/ directory");
    }
  }

  if (errors.length > 0) {
    console.error("Project registry validation FAILED:");
    for (const error of errors) console.error("- " + error);
    process.exitCode = 1;
    return;
  }

  console.log("Project registry validation PASS");
  console.log("Registered project directories: " + projectEntries.length);
  console.log("Project manifests: JSON Schema validated");
  console.log("Type, risk, lifecycle, document, workflow, and catalog consistency: PASS");
  for (const entry of projectEntries) console.log("- " + entry.name);
}

if (require.main === module) {
  main();
}

module.exports = {
  ALLOWED_TYPES,
  ALLOWED_RISK_TIERS,
  ALLOWED_VALIDATION_PROFILES,
  BASE_REQUIRED_GROUPS,
  TYPE_REQUIREMENTS,
  TYPE_VALIDATION_PROFILES,
  dataPolicyViolations,
  missingBaseDocumentGroups,
  missingTypeValidationProfiles,
  releaseStateViolations,
  releaseReadinessViolations,
  requiredReleaseGateIds,
  isSafeEvidenceReference,
  riskTierPolicyViolations,
};
