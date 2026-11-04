#!/usr/bin/env node
"use strict";

/**
 * Enforce the common project-registration contract for every immediate
 * directory under /projects. No third-party runtime dependencies are needed.
 *
 * JSON Schema remains the published external contract. This validator adds
 * cross-file and repository rules that are not expressible in the schema:
 * directory/ID consistency, required project artifacts, type-specific
 * documents, catalog registration, and lifecycle/release consistency.
 */

const fs = require("node:fs");
const path = require("node:path");

const ROOT = path.resolve(__dirname, "..");
const PROJECTS_ROOT = path.join(ROOT, "projects");
const CATALOG_PATH = path.join(ROOT, "PROJECT_CATALOG.md");

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
const ALLOWED_MATURITY = new Set([0, 1, 2, 3, 4]);
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
  quantitative_research: [
    ["RESEARCH_SPECIFICATION.md", "PROJECT_SPECIFICATION.md"],
    ["VALIDATION_MATRIX.md"],
  ],
  financial_model: [
    ["MODEL_CONVENTIONS.md"],
    ["VALIDATION_MATRIX.md"],
    ["NUMERIC_ENVELOPE.md"],
  ],
  software: [
    ["ENGINEERING_ARCHITECTURE.md"],
    ["VALIDATION_MATRIX.md"],
  ],
  data: [
    ["DATA_MANIFEST.md"],
    ["VALIDATION_MATRIX.md"],
  ],
  benchmark: [
    ["BENCHMARK_SPECIFICATION.md", "RESEARCH_SPECIFICATION.md"],
    ["VALIDATION_MATRIX.md"],
  ],
  prototype: [["README.md"]],
  infrastructure: [
    ["ENGINEERING_ARCHITECTURE.md"],
    ["THREAT_MODEL.md"],
  ],
};

const TYPE_VALIDATION_PROFILES = {
  research: ["research"],
  quantitative_research: ["quantitative"],
  financial_model: ["financial_model"],
  software: ["software"],
  data: ["data"],
  benchmark: ["benchmark"],
  prototype: [],
  infrastructure: ["integration", "security"],
};

const errors = [];

function fail(message) {
  errors.push(message);
}

function isObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function existsWithin(projectDir, relativePath) {
  if (typeof relativePath !== "string" || relativePath.length === 0) return false;
  if (relativePath.startsWith("/") || relativePath.includes(String.fromCharCode(92))) return false;

  const resolved = path.resolve(projectDir, relativePath);
  const relative = path.relative(projectDir, resolved);
  if (relative === ".." || relative.startsWith(`..${path.sep}`) || path.isAbsolute(relative)) {
    return false;
  }

  return fs.existsSync(resolved) && fs.statSync(resolved).isFile();
}

function validateManifest(projectDir, folderName, catalogText, seenIds) {
  const manifestPath = path.join(projectDir, "project.json");
  const relativeDir = path.relative(ROOT, projectDir).split(path.sep).join("/");

  if (!fs.existsSync(manifestPath)) {
    fail(`${relativeDir}: missing project.json manifest`);
    return;
  }

  let manifest;
  try {
    manifest = JSON.parse(fs.readFileSync(manifestPath, "utf8"));
  } catch (error) {
    fail(`${relativeDir}/project.json: invalid JSON: ${error.message}`);
    return;
  }

  if (!isObject(manifest)) {
    fail(`${relativeDir}/project.json: manifest must be a JSON object`);
    return;
  }

  const requiredFields = [
    "schema_version",
    "project_id",
    "title",
    "lifecycle_status",
    "project_types",
    "risk_tier",
    "owner",
    "created_at",
    "updated_at",
    "summary",
    "evidence_maturity",
    "license_profile",
    "data_classification",
    "validation_profiles",
    "required_documents",
    "release_status",
  ];

  for (const field of requiredFields) {
    if (!(field in manifest)) fail(`${relativeDir}/project.json: missing required field '${field}'`);
  }

  if (manifest.schema_version !== "1.0.0") {
    fail(`${relativeDir}/project.json: unsupported schema_version; expected 1.0.0`);
  }

  if (manifest.$schema !== "../../schemas/ofrl-project-manifest-1.0.0.schema.json") {
    fail(`${relativeDir}/project.json: $schema must reference the repository project-manifest schema`);
  }

  const id = manifest.project_id;
  if (typeof id !== "string" || !/^OFRL-[A-Z0-9]{2,8}-[0-9]{3}$/.test(id)) {
    fail(`${relativeDir}/project.json: invalid project_id format`);
  } else {
    if (id !== folderName) fail(`${relativeDir}: folder name must exactly match project_id '${id}'`);
    if (seenIds.has(id)) fail(`${relativeDir}: duplicate project_id '${id}'`);
    seenIds.add(id);
    if (!catalogText.includes(`projects/${id}/README.md`)) {
      fail(`${relativeDir}: project is missing from PROJECT_CATALOG.md`);
    }
  }

  if (typeof manifest.title !== "string" || manifest.title.trim().length < 5) {
    fail(`${relativeDir}/project.json: title must contain at least 5 characters`);
  }

  if (!ALLOWED_STATUSES.has(manifest.lifecycle_status)) {
    fail(`${relativeDir}/project.json: invalid lifecycle_status '${manifest.lifecycle_status}'`);
  }

  if (!Array.isArray(manifest.project_types) || manifest.project_types.length === 0) {
    fail(`${relativeDir}/project.json: project_types must be a non-empty array`);
  } else {
    const unique = new Set(manifest.project_types);
    if (unique.size !== manifest.project_types.length) {
      fail(`${relativeDir}/project.json: project_types contains duplicates`);
    }
    for (const type of manifest.project_types) {
      if (!ALLOWED_TYPES.has(type)) fail(`${relativeDir}/project.json: unsupported project type '${type}'`);
    }
  }

  if (!ALLOWED_RISK_TIERS.has(manifest.risk_tier)) {
    fail(`${relativeDir}/project.json: invalid risk_tier '${manifest.risk_tier}'`);
  }

  if (typeof manifest.owner !== "string" || manifest.owner.trim().length < 2) {
    fail(`${relativeDir}/project.json: owner is required`);
  }

  for (const field of ["created_at", "updated_at"]) {
    const value = manifest[field];
    const parsedDate = typeof value === "string" ? new Date(`${value}T00:00:00Z`) : null;
    if (typeof value !== "string" || value.length !== 10 || value[4] !== "-" || value[7] !== "-" || !parsedDate || Number.isNaN(parsedDate.getTime()) || parsedDate.toISOString().slice(0, 10) !== value) {
      fail(`${relativeDir}/project.json: ${field} must use YYYY-MM-DD`);
    }
  }

  if (typeof manifest.summary !== "string" || manifest.summary.trim().length < 40) {
    fail(`${relativeDir}/project.json: summary must contain at least 40 characters`);
  }

  if (!ALLOWED_MATURITY.has(manifest.evidence_maturity)) {
    fail(`${relativeDir}/project.json: evidence_maturity must be an integer from 0 to 4`);
  }

  if (typeof manifest.license_profile !== "string" || manifest.license_profile.trim().length < 3) {
    fail(`${relativeDir}/project.json: license_profile is required`);
  }

  const dataClasses = new Set(["none", "synthetic", "public_source", "licensed", "restricted", "mixed"]);
  if (!dataClasses.has(manifest.data_classification)) {
    fail(`${relativeDir}/project.json: invalid data_classification`);
  }

  const validationProfiles = new Set([
    "research", "quantitative", "financial_model", "software",
    "data", "benchmark", "security", "privacy", "performance", "integration",
  ]);
  if (!Array.isArray(manifest.validation_profiles) || manifest.validation_profiles.length === 0) {
    fail(`${relativeDir}/project.json: validation_profiles must be a non-empty array`);
  } else {
    for (const profile of manifest.validation_profiles) {
      if (!validationProfiles.has(profile)) {
        fail(`${relativeDir}/project.json: unsupported validation profile '${profile}'`);
      }
    }
  }

  if (!Array.isArray(manifest.required_documents)) {
    fail(`${relativeDir}/project.json: required_documents must be an array`);
  } else {
    const uniqueDocs = new Set(manifest.required_documents);
    if (uniqueDocs.size !== manifest.required_documents.length) {
      fail(`${relativeDir}/project.json: required_documents contains duplicates`);
    }

    for (const required of BASE_REQUIRED) {
      if (!manifest.required_documents.includes(required)) {
        fail(`${relativeDir}/project.json: required_documents must include ${required}`);
      }
    }

    for (const required of manifest.required_documents) {
      if (!existsWithin(projectDir, required)) {
        fail(`${relativeDir}/project.json: required document does not exist or escapes the project: ${required}`);
      }
    }
  }

  for (const type of Array.isArray(manifest.project_types) ? manifest.project_types : []) {
    const requirements = TYPE_REQUIREMENTS[type] || [];
    for (const alternatives of requirements) {
      const selected = alternatives.find((file) => existsWithin(projectDir, file));
      if (!selected) {
        fail(`${relativeDir}: project type '${type}' requires one of: ${alternatives.join(", ")}`);
      } else if (Array.isArray(manifest.required_documents) && !manifest.required_documents.includes(selected)) {
        fail(`${relativeDir}/project.json: required_documents must list type-specific document '${selected}'`);
      }
    }

    for (const profile of TYPE_VALIDATION_PROFILES[type] || []) {
      if (!Array.isArray(manifest.validation_profiles) || !manifest.validation_profiles.includes(profile)) {
        fail(`${relativeDir}/project.json: project type '${type}' requires validation profile '${profile}'`);
      }
    }
  }

  if (manifest.risk_tier === "R3") {
    if (!Array.isArray(manifest.validation_profiles) || !manifest.validation_profiles.includes("security")) {
      fail(`${relativeDir}: R3 projects must include the security validation profile`);
    }
  }

  if (["public_source", "licensed", "restricted", "mixed"].includes(manifest.data_classification)) {
    if (!Array.isArray(manifest.validation_profiles) || !manifest.validation_profiles.includes("data")) {
      fail(`${relativeDir}: licensed/restricted/mixed data requires the data validation profile`);
    }
  }

  if (["released", "maintenance"].includes(manifest.lifecycle_status)) {
    if (manifest.release_status !== "released") {
      fail(`${relativeDir}: released/maintenance lifecycle requires release_status 'released'`);
    }
    if (typeof manifest.version !== "string" || manifest.version.trim().length === 0) {
      fail(`${relativeDir}: released/maintenance lifecycle requires a version`);
    }
  }
}

function main() {
  if (!fs.existsSync(PROJECTS_ROOT) || !fs.statSync(PROJECTS_ROOT).isDirectory()) {
    console.error("Project registry validation FAILED: /projects directory not found");
    process.exitCode = 1;
    return;
  }

  if (!fs.existsSync(CATALOG_PATH)) {
    console.error("Project registry validation FAILED: PROJECT_CATALOG.md not found");
    process.exitCode = 1;
    return;
  }

  const catalogText = fs.readFileSync(CATALOG_PATH, "utf8");
  const projectEntries = fs.readdirSync(PROJECTS_ROOT, { withFileTypes: true })
    .filter((entry) => entry.isDirectory() && !entry.name.startsWith("."))
    .sort((a, b) => a.name.localeCompare(b.name));

  const seenIds = new Set();

  if (projectEntries.length === 0) fail("/projects must contain at least one registered project");

  for (const entry of projectEntries) {
    validateManifest(path.join(PROJECTS_ROOT, entry.name), entry.name, catalogText, seenIds);
  }

  if (errors.length > 0) {
    console.error("Project registry validation FAILED:");
    for (const error of errors) console.error(`- ${error}`);
    process.exitCode = 1;
    return;
  }

  console.log("Project registry validation PASS");
  console.log(`Registered project directories: ${projectEntries.length}`);
  for (const entry of projectEntries) console.log(`- ${entry.name}`);
}

main();
