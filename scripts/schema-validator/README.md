# OFRL JSON Schema Validator Tooling

This directory contains the isolated Node.js tooling used to compile and validate the repository's JSON Schema contracts.

It is separate from every project's runtime and build dependencies. Node.js is a CI/development tool for schema validation; it is not a runtime dependency of OFRL software.

## Pinned toolchain

- Node.js: 22.23.4
- npm: 10.9.9
- Ajv: 8.20.0
- ajv-formats: 3.0.1

The package manifest pins direct dependencies. The committed lockfile records the resolved transitive graph and integrity hashes.

## Reproduction

From the repository root:

```bash
npm ci --prefix scripts/schema-validator --ignore-scripts --no-audit --no-fund
node scripts/validate_project_registry.cjs
bash scripts/validate-ofrl-ib-001-model-input-schema.sh
```

The repository policy workflow installs this isolated tooling with `npm ci` before compiling and validating project manifests. The OFRL-IB-001 workflow uses the same package for its financial model input schema and fixture checks.

## Dependency changes

1. Review package metadata, licences, security advisories, runtime support, and package provenance.
2. Change exact direct versions in `package.json`.
3. Regenerate and inspect the complete lockfile using the declared Node/npm toolchain.
4. Run both the project-manifest registry validator and OFRL-IB-001 model-input schema tests.
5. Update this document and the dependency review when the toolchain or dependency graph changes.

Do not add these packages to any project's runtime dependencies. Schema validation does not replace domain-specific semantic, mathematical, or accounting validation.
