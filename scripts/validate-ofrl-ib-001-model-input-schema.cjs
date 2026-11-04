#!/usr/bin/env node
"use strict";

const fs = require("node:fs");
const path = require("node:path");

const ajvModule = require("ajv/dist/2020");
const Ajv2020 = ajvModule.default || ajvModule;
const formatsModule = require("ajv-formats");
const addFormats = formatsModule.default || formatsModule;

const rootDir = process.argv[2]
  ? path.resolve(process.argv[2])
  : path.resolve(__dirname, "..");

const schemaPath = path.join(
  rootDir,
  "schemas",
  "ofrl-ib-001-model-input-0.1.0.schema.json"
);
const corpusPath = path.join(
  rootDir,
  "fixtures",
  "contracts",
  "ofrl-ib-001-shared-corpus.v0.1.0.json"
);

function readJson(filePath) {
  try {
    return JSON.parse(fs.readFileSync(filePath, "utf8"));
  } catch (error) {
    throw new Error(
      "Unable to read valid JSON from " + filePath + ": " + error.message
    );
  }
}

function printErrors(errors) {
  return (errors || []).map((error) => ({
    instancePath: error.instancePath,
    schemaPath: error.schemaPath,
    keyword: error.keyword,
    message: error.message,
    params: error.params
  }));
}

function assertExpectedKeys(value, expectedKeys, label) {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error(label + " must be a JSON object");
  }

  const actualKeys = Object.keys(value).sort();
  const sortedExpected = [...expectedKeys].sort();
  if (
    actualKeys.length !== sortedExpected.length ||
    actualKeys.some((key, index) => key !== sortedExpected[index])
  ) {
    throw new Error(
      label + " has an invalid property set; expected " +
      sortedExpected.join(", ") + "; received " + actualKeys.join(", ")
    );
  }
}

function isWithin(parentPath, candidatePath) {
  const relative = path.relative(parentPath, candidatePath);
  return relative === "" ||
    (!relative.startsWith(".." + path.sep) &&
      relative !== ".." &&
      !path.isAbsolute(relative));
}

function resolveFixturePath(rootPath, fixturesPath, input, caseId) {
  if (
    typeof input !== "string" ||
    input.length === 0 ||
    input.includes("\\") ||
    path.posix.isAbsolute(input)
  ) {
    throw new Error(caseId + " must use a non-empty relative POSIX fixture path");
  }

  const segments = input.split("/");
  if (
    segments[0] !== "fixtures" ||
    segments.some((segment) => segment === "" || segment === "." || segment === "..")
  ) {
    throw new Error(caseId + " input must remain under fixtures/ without traversal segments");
  }

  let resolvedPath;
  try {
    resolvedPath = fs.realpathSync(path.resolve(rootPath, input));
  } catch (error) {
    throw new Error(caseId + " input cannot be resolved: " + error.message);
  }

  if (!isWithin(rootPath, resolvedPath) || !isWithin(fixturesPath, resolvedPath)) {
    throw new Error(caseId + " input resolves outside the repository fixtures directory");
  }
  if (!fs.statSync(resolvedPath).isFile()) {
    throw new Error(caseId + " input does not resolve to a regular file");
  }

  return resolvedPath;
}

function main() {
  const realRoot = fs.realpathSync(rootDir);
  const realFixtures = fs.realpathSync(path.join(realRoot, "fixtures"));
  const schema = readJson(schemaPath);
  const corpus = readJson(corpusPath);

  if (schema.$schema !== "https://json-schema.org/draft/2020-12/schema") {
    throw new Error("Input schema must declare JSON Schema Draft 2020-12");
  }
  if (
    corpus.corpus_version !== "0.1.0" ||
    !Array.isArray(corpus.cases) ||
    corpus.cases.length === 0
  ) {
    throw new Error("Shared contract corpus must have version 0.1.0 and a non-empty cases array");
  }
  if (corpus.schema_id !== schema.$id) {
    throw new Error("Shared contract corpus schema_id does not match the input schema $id");
  }

  const ajv = new Ajv2020({
    allErrors: true,
    strict: true,
    validateFormats: true
  });
  addFormats(ajv);

  console.log("Ajv version: " + require("ajv/package.json").version);
  console.log("Schema: " + path.relative(realRoot, schemaPath));
  console.log("Corpus: " + path.relative(realRoot, corpusPath));

  // Schema compilation is a gate: unsupported or malformed schema constructs
  // must stop the job before any payload can be accepted.
  const validate = ajv.compile(schema);
  console.log("PASS: Draft 2020-12 schema compiled");

  const seenIds = new Set();
  const seenInputs = new Set();
  const expectedCaseKeys = [
    "id",
    "input",
    "schema_valid",
    "runtime_status",
    "diagnostic_codes"
  ];

  for (const item of corpus.cases) {
    assertExpectedKeys(item, expectedCaseKeys, "Shared corpus case");

    if (typeof item.id !== "string" || item.id.length === 0 || seenIds.has(item.id)) {
      throw new Error("Every shared corpus case must have a unique non-empty id");
    }
    seenIds.add(item.id);

    if (
      typeof item.schema_valid !== "boolean" ||
      !["accepted", "rejected"].includes(item.runtime_status) ||
      !Array.isArray(item.diagnostic_codes) ||
      !item.diagnostic_codes.every((code) => typeof code === "string" && code.length > 0)
    ) {
      throw new Error("Corpus case " + item.id + " has invalid expected results");
    }

    if (!item.schema_valid && item.runtime_status !== "rejected") {
      throw new Error(
        "Schema-invalid case " + item.id + " must also be rejected by the Rust runtime"
      );
    }

    if (new Set(item.diagnostic_codes).size !== item.diagnostic_codes.length) {
      throw new Error("Corpus case " + item.id + " contains duplicate diagnostic codes");
    }

    const inputPath = resolveFixturePath(realRoot, realFixtures, item.input, item.id);
    if (seenInputs.has(inputPath)) {
      throw new Error("Shared corpus input path is duplicated: " + item.input);
    }
    seenInputs.add(inputPath);

    const payload = readJson(inputPath);
    const actualSchemaValidity = validate(payload);
    if (actualSchemaValidity !== item.schema_valid) {
      throw new Error(
        "Schema expectation mismatch for " + item.id + ": expected " +
        item.schema_valid + ", received " + actualSchemaValidity + "\\n" +
        JSON.stringify(printErrors(validate.errors), null, 2)
      );
    }

    console.log(
      "PASS: " + item.id + " — JSON Schema " +
      (actualSchemaValidity ? "ACCEPT" : "REJECT")
    );
  }

  console.log("PASS: all " + corpus.cases.length + " shared contract corpus cases completed");
}

try {
  main();
} catch (error) {
  console.error("FAIL: " + error.message);
  process.exitCode = 1;
}
