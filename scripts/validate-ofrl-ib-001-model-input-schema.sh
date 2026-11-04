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

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

# Pinned direct validator packages, isolated from the Rust application runtime.
npm install \
  --prefix "$TMP_DIR" \
  --no-save \
  --package-lock=false \
  --ignore-scripts \
  --no-audit \
  --no-fund \
  ajv@8.20.0 \
  ajv-formats@3.0.1

NODE_PATH="$TMP_DIR/node_modules" node scripts/validate-ofrl-ib-001-model-input-schema.cjs "$ROOT_DIR"
