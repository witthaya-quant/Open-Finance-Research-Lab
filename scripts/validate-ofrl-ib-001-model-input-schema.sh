#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

for command_name in node npm; do
  if ! command -v "$command_name" >/dev/null 2>&1; then
    echo "ERROR: required command not found: $command_name" >&2
    exit 2
  fi
done

echo "Node: $(node --version)"
echo "npm:  $(npm --version)"

VALIDATOR_DIR="$ROOT_DIR/scripts/schema-validator"

if [[ ! -f "$VALIDATOR_DIR/package.json" || ! -f "$VALIDATOR_DIR/package-lock.json" ]]; then
  echo "ERROR: the schema-validator package manifest or lockfile is missing" >&2
  exit 2
fi

# npm ci removes any existing node_modules and installs the exact dependency
# graph recorded by package-lock.json. Lifecycle scripts are disabled because
# this validator is a pure JavaScript tooling dependency set.
npm ci \
  --prefix "$VALIDATOR_DIR" \
  --ignore-scripts \
  --no-audit \
  --no-fund

NODE_PATH="$VALIDATOR_DIR/node_modules" node scripts/validate-ofrl-ib-001-model-input-schema.cjs "$ROOT_DIR"
