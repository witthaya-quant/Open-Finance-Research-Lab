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
const SCHEMA_PATH = path.join(ROOT, "schemas", "ofrl-project-manifest-1.0.0.schema.json");
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

const BASE_REQUIRED = [
  "README.md",
  "DECISION_LOG.md",
  "VALIDATION_MATRIX.md",
];

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
  prototype: [],
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

  if (manifest.schema_version !== "1.0.0") {
    fail(relativeDir + "/project.json: unsupported schema_version; expected 1.0.0");
  }

  if (manifest.$schema !== "../../schemas/ofrl-project-manifest-1.0.0.schema.json") {
    fail(relativeDir + "/project.json: $schema must reference the repository project-manifest schema");
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
    for (const type of Array.isArray(manifest.project_types) ? manifest.project_types : []) {
      for (const profile of TYPE_VALIDATION_PROFILES[type] || []) {
        if (!manifest.validation_profiles.includes(profile)) {
          fail(relativeDir + "/project.json: project type '" + type + "' requires validation profile '" + profile + "'");
        }
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

    for (const required of BASE_REQUIRED) {
      if (!manifest.required_documents.includes(required)) {
        fail(relativeDir + "/project.json: required_documents must include " + required);
      }
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

  if (manifest.risk_tier === "R3") {
    if (!Array.isArray(manifest.validation_profiles) || !manifest.validation_profiles.includes("security")) {
      fail(relativeDir + ": R3 projects must include the security validation profile");
    }

    if (!resolveFileWithin(projectDir, "THREAT_MODEL.md")) {
      fail(relativeDir + ": R3 projects require THREAT_MODEL.md");
    } else if (
      Array.isArray(manifest.required_documents) &&
      !manifest.required_documents.includes("THREAT_MODEL.md")
    ) {
      fail(relativeDir + "/project.json: R3 projects must list THREAT_MODEL.md in required_documents");
    }
  }

  if (["public_source", "licensed", "restricted", "mixed"].includes(manifest.data_classification)) {
    if (!Array.isArray(manifest.validation_profiles) || !manifest.validation_profiles.includes("data")) {
      fail(relativeDir + ": non-synthetic data requires the data validation profile");
    }
  }

  if (!ALLOWED_RELEASE_STATES.has(manifest.release_status)) {
    fail(relativeDir + "/project.json: invalid release_status '" + manifest.release_status + "'");
  }

  if (manifest.release_status === "pre_release" && manifest.lifecycle_status !== "validation") {
    fail(relativeDir + ": pre_release artifacts require lifecycle_status 'validation'");
  }

  if (["released", "maintenance"].includes(manifest.lifecycle_status)) {
    if (manifest.release_status !== "released") {
      fail(relativeDir + ": released/maintenance lifecycle requires release_status 'released'");
    }
    if (typeof manifest.version !== "string" || manifest.version.trim().length === 0) {
      fail(relativeDir + ": released/maintenance lifecycle requires a version");
    }
  }

  if (manifest.release_status === "released" && typeof manifest.version !== "string") {
    fail(relativeDir + ": release_status 'released' requires a version");
  }

  if (
    manifest.release_status === "withdrawn" &&
    !["deprecated", "archived"].includes(manifest.lifecycle_status)
  ) {
    fail(relativeDir + ": withdrawn releases must be deprecated or archived");
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

main();
