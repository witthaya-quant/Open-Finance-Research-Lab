#!/usr/bin/env node
"use strict";

const fs = require("node:fs");
const path = require("node:path");

const ROOT = path.resolve(__dirname, "..");
const VENDOR = path.join(ROOT, "schemas", "vendor", "cyclonedx", "1.7");
const NODE_MODULES = path.join(__dirname, "schema-validator", "node_modules");
const PACKAGE_MANIFEST = path.join(
  ROOT,
  "projects",
  "OFRL-IB-001",
  "crates",
  "ofrl_cli",
  "Cargo.toml"
);

function fail(message) {
  process.stderr.write("SBOM validation FAIL: " + message + "\n");
  process.exit(1);
}

function readJson(filePath) {
  try {
    return JSON.parse(fs.readFileSync(filePath, "utf8"));
  } catch (error) {
    fail("cannot read valid JSON from " + filePath + ": " + error.message);
  }
}

function readPackageField(toml, field) {
  const start = toml.indexOf("[package]");
  if (start < 0) fail("Cargo package manifest has no [package] section");
  const nextTable = toml.indexOf("\n[", start + 1);
  const packageText = toml.slice(start, nextTable < 0 ? toml.length : nextTable);
  const match = packageText.match(new RegExp("^" + field + '\\s*=\\s*"([^"]+)"\\s*$', "m"));
  if (!match) fail("Cargo package manifest has no literal " + field + " value");
  return match[1];
}

function validate() {
  const [sbomPath, targetTriple] = process.argv.slice(2);
  if (!sbomPath || !targetTriple) {
    fail("usage: node scripts/validate-cyclonedx-sbom.cjs <sbom.json> <target-triple>");
  }

  const absoluteSbomPath = path.resolve(sbomPath);
  if (!fs.existsSync(absoluteSbomPath) || !fs.statSync(absoluteSbomPath).isFile()) {
    fail("SBOM file does not exist: " + absoluteSbomPath);
  }
  if (!path.basename(absoluteSbomPath).includes(targetTriple)) {
    fail("SBOM filename does not identify the expected target triple " + targetTriple);
  }

  let Ajv;
  let addFormats;
  try {
    Ajv = require(path.join(NODE_MODULES, "ajv"));
    const formatsModule = require(path.join(NODE_MODULES, "ajv-formats"));
    addFormats = formatsModule.default || formatsModule;
  } catch (error) {
    fail("pinned schema-validator tooling is not installed: " + error.message);
  }

  const schemaFiles = [
    "spdx.schema.json",
    "jsf-0.82.schema.json",
    "cryptography-defs.schema.json",
    "bom-1.7.schema.json",
  ];
  const schemas = Object.fromEntries(
    schemaFiles.map((name) => [name, readJson(path.join(VENDOR, name))])
  );

  const ajv = new Ajv({
    allErrors: true,
    strict: false,
    validateFormats: true,
  });
  addFormats(ajv);

  for (const name of schemaFiles.slice(0, 3)) {
    ajv.addSchema(schemas[name]);
  }
  const validateSchema = ajv.compile(schemas["bom-1.7.schema.json"]);
  const bom = readJson(absoluteSbomPath);

  if (!validateSchema(bom)) {
    const errors = (validateSchema.errors || []).slice(0, 30).map((error) => ({
      instancePath: error.instancePath,
      keyword: error.keyword,
      message: error.message,
      params: error.params,
    }));
    process.stderr.write(JSON.stringify(errors, null, 2) + "\n");
    fail("CycloneDX 1.7.2 JSON Schema validation failed");
  }

  if (bom.bomFormat !== "CycloneDX" || bom.specVersion !== "1.7") {
    fail("expected bomFormat=CycloneDX and specVersion=1.7");
  }
  if (bom.serialNumber !== undefined && bom.serialNumber !== null) {
    fail("reproducible candidate SBOM must omit the random serialNumber");
  }
  if (!bom.metadata || typeof bom.metadata.timestamp !== "string") {
    fail("SBOM metadata.timestamp is missing; SOURCE_DATE_EPOCH may not have been applied");
  }

  const packageToml = fs.readFileSync(PACKAGE_MANIFEST, "utf8");
  const expectedName = readPackageField(packageToml, "name");
  const expectedVersion = readPackageField(packageToml, "version");
  const rootComponent = bom.metadata.component;
  if (!rootComponent || rootComponent.name !== expectedName || rootComponent.version !== expectedVersion) {
    fail(
      "root component mismatch; expected " +
      expectedName + "@" + expectedVersion +
      ", received " + JSON.stringify(rootComponent)
    );
  }
  if (typeof rootComponent["bom-ref"] !== "string" || rootComponent["bom-ref"].length === 0) {
    fail("root component has no bom-ref");
  }

  const components = Array.isArray(bom.components) ? bom.components : [];
  const refs = new Set([rootComponent["bom-ref"]]);
  for (const component of components) {
    if (typeof component["bom-ref"] !== "string" || component["bom-ref"].length === 0) {
      fail("component " + JSON.stringify(component.name) + " has no bom-ref");
    }
    if (refs.has(component["bom-ref"])) {
      fail("duplicate bom-ref: " + component["bom-ref"]);
    }
    refs.add(component["bom-ref"]);
  }

  const requiredComponents = ["ofrl_model", "rust_decimal", "serde", "serde_json"];
  const componentNames = new Set(components.map((component) => component.name));
  const missing = requiredComponents.filter((name) => !componentNames.has(name));
  if (missing.length > 0) {
    fail("expected runtime dependency components are missing: " + missing.join(", "));
  }

  const registryComponents = components.filter(
    (component) =>
      typeof component.purl === "string" &&
      component.purl.startsWith("pkg:cargo/") &&
      component.name !== "ofrl_model"
  );
  if (registryComponents.length < 5) {
    fail("SBOM contains too few registry package components: " + registryComponents.length);
  }

  for (const component of registryComponents) {
    const hasSha256 = Array.isArray(component.hashes) && component.hashes.some(
      (hash) =>
        hash.alg === "SHA-256" &&
        typeof hash.content === "string" &&
        /^[a-f0-9]{64}$/i.test(hash.content)
    );
    if (!hasSha256) fail("registry component lacks a Cargo.lock SHA-256: " + component.name);

    if (!Array.isArray(component.licenses) || component.licenses.length === 0) {
      fail("registry component lacks declared licence information: " + component.name);
    }
  }

  const dependencyEntries = Array.isArray(bom.dependencies) ? bom.dependencies : [];
  const rootDependency = dependencyEntries.find((entry) => entry.ref === rootComponent["bom-ref"]);
  if (!rootDependency || !Array.isArray(rootDependency.dependsOn) || rootDependency.dependsOn.length === 0) {
    fail("root dependency entry is missing or has no dependencies");
  }
  for (const entry of dependencyEntries) {
    if (!refs.has(entry.ref)) fail("dependency entry references an unknown component: " + entry.ref);
    for (const dependencyRef of entry.dependsOn || []) {
      if (!refs.has(dependencyRef)) {
        fail("dependency graph points to an unknown component: " + dependencyRef);
      }
    }
  }

  process.stdout.write(
    "SBOM validation PASS: CycloneDX 1.7 schema; root=" +
    expectedName + "@" + expectedVersion +
    "; target=" + targetTriple +
    "; registry components=" + registryComponents.length +
    "; dependency references=" + dependencyEntries.length +
    "; reproducible serialNumber omitted\n"
  );
}

try {
  validate();
} catch (error) {
  fail(error.stack || error.message || String(error));
}
