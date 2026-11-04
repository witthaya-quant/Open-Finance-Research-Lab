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
const fixturePath = path.join(
  rootDir,
  "fixtures",
  "synthetic",
  "ofrl-ib-001-case-a.input.v0.1.0.json"
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
  const fixture = readJson(fixturePath);

  if (schema.$schema !== "https://json-schema.org/draft/2020-12/schema") {
    throw new Error("Input schema must declare JSON Schema Draft 2020-12");
  }

  const ajv = new Ajv2020({
    allErrors: true,
    strict: true,
    validateFormats: true
  });
  addFormats(ajv);

  console.log("Ajv version: " + require("ajv/package.json").version);
  console.log("Schema: " + path.relative(rootDir, schemaPath));
  console.log("Fixture: " + path.relative(rootDir, fixturePath));

  // Compilation is itself a validation gate: malformed or unsupported
  // schema constructs must stop the job before any payload is accepted.
  const validate = ajv.compile(schema);
  console.log("PASS: Draft 2020-12 schema compiled");

  assertValid(validate, "canonical synthetic Case A fixture", fixture);

  const maximumScale = clone(fixture);
  maximumScale.forecast_periods[0].assumptions.dso =
    "0.1234567890123456789012345678";
  assertValid(
    validate,
    "28-digit fractional scale remains schema-valid",
    maximumScale
  );

  const numericToken = clone(fixture);
  numericToken.historical_periods[0].income_statement.revenue = 1000;
  assertInvalid(validate, "JSON numeric token for Decimal field", numericToken);

  const unknownRootField = clone(fixture);
  unknownRootField.unexpected_field = "must be rejected";
  assertInvalid(validate, "unknown root field", unknownRootField);

  const unknownNestedField = clone(fixture);
  unknownNestedField.forecast_periods[0].assumptions.unapproved_driver = "1";
  assertInvalid(validate, "unknown nested assumption", unknownNestedField);

  const negativeZero = clone(fixture);
  negativeZero.historical_periods[0].income_statement.net_income = "-0";
  assertInvalid(validate, "negative zero", negativeZero);

  const excessiveScale = clone(fixture);
  excessiveScale.forecast_periods[0].assumptions.dso =
    "1.00000000000000000000000000000";
  assertInvalid(validate, "29 fractional digits", excessiveScale);

  const nonSyntheticSource = clone(fixture);
  nonSyntheticSource.historical_periods[0].source.classification =
    "public_filing";
  assertInvalid(validate, "non-synthetic source in V1 schema", nonSyntheticSource);

  const missingSourceReference = clone(fixture);
  delete missingSourceReference.historical_periods[0].source.source_reference;
  assertInvalid(
    validate,
    "missing historical source reference",
    missingSourceReference
  );

  const invalidCalendarDate = clone(fixture);
  invalidCalendarDate.historical_periods[0].end_date = "2023-02-30";
  assertInvalid(validate, "invalid calendar date", invalidCalendarDate);

  console.log("PASS: all model-input JSON Schema checks completed");
}

try {
  main();
} catch (error) {
  console.error("FAIL: " + error.message);
  process.exitCode = 1;
}
