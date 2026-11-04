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

function clone(value) {
  return JSON.parse(JSON.stringify(value));
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

function assertValid(validate, label, payload) {
  const valid = validate(payload);
  if (!valid) {
    throw new Error(
      label + " unexpectedly failed:\n" +
      JSON.stringify(printErrors(validate.errors), null, 2)
    );
  }
  console.log("PASS: " + label);
}

function assertInvalid(validate, label, payload) {
  const valid = validate(payload);
  if (valid) {
    throw new Error(label + " unexpectedly passed schema validation");
  }
  console.log(
    "PASS: invalid payload rejected — " + label +
    " (" + (validate.errors || []).length + " validation error(s))"
  );
}

function main() {
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

  const ajv = new Ajv2020({
    allErrors: true,
    strict: true,
    validateFormats: true
  });
  addFormats(ajv);

  console.log("Ajv version: " + require("ajv/package.json").version);
  console.log("Schema: " + path.relative(rootDir, schemaPath));
  console.log("Corpus: " + path.relative(rootDir, corpusPath));

  // Schema compilation is a gate: unsupported or malformed schema constructs
  // must stop the job before any payload can be accepted.
  const validate = ajv.compile(schema);
  console.log("PASS: Draft 2020-12 schema compiled");

  const rootPrefix = rootDir.endsWith(path.sep) ? rootDir : rootDir + path.sep;
  const seenIds = new Set();

  for (const item of corpus.cases) {
    if (!item || typeof item.id !== "string" || !item.id || seenIds.has(item.id)) {
      throw new Error("Every shared corpus case must have a unique non-empty id");
    }
    seenIds.add(item.id);

    if (typeof item.input !== "string" || typeof item.schema_valid !== "boolean") {
      throw new Error("Corpus case " + item.id + " must declare input and schema_valid");
    }

    const inputPath = path.resolve(rootDir, item.input);
    if (!inputPath.startsWith(rootPrefix)) {
      throw new Error("Corpus case " + item.id + " resolves outside the repository root");
    }

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
