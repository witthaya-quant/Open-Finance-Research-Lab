# OFRL JSON Schema Validator Tooling

This directory contains only the Node.js tooling used to compile and validate the OFRL-IB-001 Draft 2020-12 model-input JSON Schema and its positive/negative fixtures.

It is deliberately separate from the Rust financial calculation workspace. Node.js is a development/CI tool for the schema gate; it is not a runtime dependency of the model or its CLI.

## Pinned toolchain

- Node.js: 22.23.4
- npm: 10.9.9
- Ajv: 8.20.0
- ajv-formats: 3.0.1

The package manifest declares exact direct dependency versions. The committed \`package-lock.json\` pins the complete resolved dependency graph and integrity hashes.

## Reproduction

From the repository root:

\`\`\`bash
npm ci --prefix scripts/schema-validator --ignore-scripts --no-audit --no-fund
bash scripts/validate-ofrl-ib-001-model-input-schema.sh
\`\`\`

The validation script uses \`npm ci\`, which requires the committed package lock and rejects manifest/lockfile drift. Lifecycle scripts are disabled during installation.

## Intentional dependency updates

1. Review the current package metadata, licences, security advisories, and Node/npm support policy.
2. Edit exact versions in \`package.json\`.
3. Regenerate the lockfile using Node.js 22.23.4 / npm 10.9.9:

   \`\`\`bash
   npm install --package-lock-only --ignore-scripts --no-audit --no-fund
   \`\`\`

4. Review the complete lockfile diff and dependency graph.
5. Run the schema validation script and the GitHub Actions workflow.
6. Update this README and the related engineering issue if the runtime/tooling baseline changes.

Do not add these dependencies to the Rust Cargo workspace. Do not treat JSON Schema validation as a replacement for the Rust semantic and accounting validator.