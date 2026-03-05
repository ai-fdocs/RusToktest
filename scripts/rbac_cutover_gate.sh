#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  scripts/rbac_cutover_gate.sh [options]

Options:
  --staging-artifacts-dir <dir>   Directory with rbac_relation_staging artifacts (default: artifacts/rbac-staging)
  --cutover-artifacts-dir <dir>   Directory with rbac_cutover_baseline artifacts (default: artifacts/rbac-cutover)
  --auth-gate-report <file>       Path to auth_release_gate report artifact (required)
  --help                          Show this message

Gate checks:
  1) Staging artifacts include stage report + pre report + dry-run report
  2) Cutover baseline artifacts include markdown + json report
  3) Baseline json has gate_status=pass
  4) Baseline json deltas mismatch/shadow failures are zero
  5) Auth gate report artifact exists
USAGE
}

STAGING_ARTIFACTS_DIR="artifacts/rbac-staging"
CUTOVER_ARTIFACTS_DIR="artifacts/rbac-cutover"
AUTH_GATE_REPORT=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --staging-artifacts-dir)
      STAGING_ARTIFACTS_DIR="$2"; shift 2 ;;
    --cutover-artifacts-dir)
      CUTOVER_ARTIFACTS_DIR="$2"; shift 2 ;;
    --auth-gate-report)
      AUTH_GATE_REPORT="$2"; shift 2 ;;
    --help)
      usage; exit 0 ;;
    *)
      echo "Unknown option: $1" >&2
      usage
      exit 1 ;;
  esac
done

if [[ -z "$AUTH_GATE_REPORT" ]]; then
  echo "--auth-gate-report is required." >&2
  exit 1
fi

latest_file() {
  local dir="$1"
  local pattern="$2"
  find "$dir" -maxdepth 1 -type f -name "$pattern" | sort | tail -n 1
}

require_file() {
  local path="$1"
  local label="$2"
  if [[ -z "$path" || ! -f "$path" ]]; then
    echo "Missing required artifact: ${label}" >&2
    exit 1
  fi
}

if [[ ! -d "$STAGING_ARTIFACTS_DIR" ]]; then
  echo "Staging artifacts directory does not exist: $STAGING_ARTIFACTS_DIR" >&2
  exit 1
fi
if [[ ! -d "$CUTOVER_ARTIFACTS_DIR" ]]; then
  echo "Cutover artifacts directory does not exist: $CUTOVER_ARTIFACTS_DIR" >&2
  exit 1
fi

stage_report="$(latest_file "$STAGING_ARTIFACTS_DIR" 'rbac_relation_stage_report_*.md')"
stage_pre_json="$(latest_file "$STAGING_ARTIFACTS_DIR" 'rbac_report_pre_*.json')"
stage_dry_json="$(latest_file "$STAGING_ARTIFACTS_DIR" 'rbac_backfill_dry_run_*.json')"

cutover_md="$(latest_file "$CUTOVER_ARTIFACTS_DIR" 'rbac_cutover_baseline_*.md')"
cutover_json="$(latest_file "$CUTOVER_ARTIFACTS_DIR" 'rbac_cutover_baseline_*.json')"

require_file "$stage_report" "staging stage-report markdown"
require_file "$stage_pre_json" "staging pre-check JSON"
require_file "$stage_dry_json" "staging dry-run JSON"
require_file "$cutover_md" "cutover baseline markdown"
require_file "$cutover_json" "cutover baseline JSON"
require_file "$AUTH_GATE_REPORT" "auth release gate report"

python - "$cutover_json" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, 'r', encoding='utf-8') as fh:
    payload = json.load(fh)

status = payload.get('gate_status')
if status != 'pass':
    raise SystemExit(f"baseline gate_status must be 'pass', got: {status!r}")

for key in ('mismatch_delta', 'shadow_compare_failures_delta'):
    value = payload.get(key)
    if not isinstance(value, int):
        raise SystemExit(f"baseline field must be integer: {key}")
    if value != 0:
        raise SystemExit(f"baseline field must be 0 before relation-only cutover: {key}={value}")

if not isinstance(payload.get('total_decisions_delta'), int):
    raise SystemExit('baseline field must be integer: total_decisions_delta')
PY

echo "RBAC cutover gate: PASS"
echo "- staging_report: $stage_report"
echo "- staging_pre_json: $stage_pre_json"
echo "- staging_dry_run_json: $stage_dry_json"
echo "- baseline_md: $cutover_md"
echo "- baseline_json: $cutover_json"
echo "- auth_gate_report: $AUTH_GATE_REPORT"
