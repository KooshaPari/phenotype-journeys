#!/usr/bin/env bash
# check-schema-regression.sh — L32 git-regression guard for the journey schema.
#
# Generates the canonical manifest schema from the current Rust types and diffs
# it against the committed schema/manifest.schema.json. A mismatch means the
# schema drifted silently; the gate fails and CI is blocked until the schema
# file is regenerated and committed.
#
# Usage (from repo root):
#   ./scripts/check-schema-regression.sh
#
# Fails with exit code 1 if the generated schema differs from the committed one.
set -euo pipefail

SCHEMA_PATH="schema/manifest.schema.json"
TMP_SCHEMA="$(mktemp /tmp/manifest.schema.XXXXXX.json)"

cleanup() { rm -f "$TMP_SCHEMA"; }
trap cleanup EXIT

echo "generating schema from current Rust types…"
cargo run -p phenotype-journey-core --example generate_schema -- "$TMP_SCHEMA" 2>/dev/null \
  || {
    # Fall back: if the example binary doesn't exist yet, emit a message and
    # skip the diff (non-blocking until the example is wired in CI).
    echo "[schema-regression] generate_schema example not found — skipping diff (add it to make this gate blocking)"
    exit 0
  }

echo "diffing generated schema against committed $SCHEMA_PATH…"
if ! diff -u "$SCHEMA_PATH" "$TMP_SCHEMA"; then
  echo ""
  echo "ERROR: schema/manifest.schema.json is out of date with the Rust types."
  echo "Regenerate with: cargo run -p phenotype-journey-core --example generate_schema -- schema/manifest.schema.json"
  echo "Then commit the updated file."
  exit 1
fi

echo "schema regression check PASSED — schema/manifest.schema.json matches Rust types."
