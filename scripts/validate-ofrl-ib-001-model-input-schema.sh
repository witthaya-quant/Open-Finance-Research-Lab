#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

SCHEMA="schemas/ofrl-ib-001-model-input-0.1.0.schema.json"
FIXTURE="fixtures/synthetic/ofrl-ib-001-case-a.input.v0.1.0.json"

for command_name in node npm npx jq; do
  if ! command -v "$command_name" >/dev/null 2>&1; then
    echo "ERROR: required command not found: $command_name" >&2
    exit 2
  fi
done

echo "Node: $(node --version)"
echo "npm: $(npm --version)"

AJV="npx --yes --package=ajv-cli@5.0.0 --package=ajv-formats@3.0.1 -- ajv"

echo "1/6 Compile Draft 2020-12 schema"
$AJV compile --spec=draft2020 -c ajv-formats -s "$SCHEMA"

echo "2/6 Validate canonical synthetic fixture"
$AJV validate --spec=draft2020 -c ajv-formats -s "$SCHEMA" -d "$FIXTURE"

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

jq '.historical_periods[0].income_statement.revenue = 1000' "$FIXTURE" > "$TMP_DIR/json-number.json"
jq '.unexpected_field = "must be rejected"' "$FIXTURE" > "$TMP_DIR/unknown-field.json"
jq '.historical_periods[0].income_statement.net_income = "-0"' "$FIXTURE" > "$TMP_DIR/negative-zero.json"
jq '.forecast_periods[0].assumptions.dso = "1.00000000000000000000000000000"' "$FIXTURE" > "$TMP_DIR/over-scale.json"

expect_invalid() {
  local label="$1"
  local file="$2"
  if $AJV validate --spec=draft2020 -c ajv-formats -s "$SCHEMA" -d "$file"; then
    echo "FAIL: invalid payload unexpectedly passed: $label" >&2
    exit 1
  fi
  echo "PASS: invalid payload rejected: $label"
}

echo "3/6 Reject JSON numeric token for a Decimal field"
expect_invalid "JSON numeric token" "$TMP_DIR/json-number.json"

echo "4/6 Reject undeclared fields"
expect_invalid "unknown field" "$TMP_DIR/unknown-field.json"

echo "5/6 Reject negative zero"
expect_invalid "negative zero" "$TMP_DIR/negative-zero.json"

echo "6/6 Reject Decimal scale above 28"
expect_invalid "over-scale Decimal" "$TMP_DIR/over-scale.json"

echo "PASS: model-input JSON Schema contract checks completed"
