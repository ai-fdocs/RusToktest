#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RUNNER="$REPO_ROOT/scripts/verify/run-control-plane-remediation-minimal.sh"

if [[ ! -x "$RUNNER" ]]; then
  echo "runner not executable: $RUNNER" >&2
  exit 1
fi

# smoke: syntax should be valid
bash -n "$RUNNER"

# lock guard: pre-acquire lock and assert runner exits with lock message
LOCK_FILE="$REPO_ROOT/target/.control-plane-remediation-minimal.lock"
mkdir -p "$REPO_ROOT/target"
exec 8>"$LOCK_FILE"
flock -n 8

LOCK_OUTPUT="$(mktemp)"
if env RUSTOK_VERIFY_SKIP_FMT=1 timeout 5s "$RUNNER" >"$LOCK_OUTPUT" 2>&1; then
  echo "runner unexpectedly succeeded while lock is held" >&2
  cat "$LOCK_OUTPUT" >&2
  exit 1
fi
if ! rg -q "Another remediation verification run is already active" "$LOCK_OUTPUT"; then
  echo "runner did not report active lock" >&2
  cat "$LOCK_OUTPUT" >&2
  exit 1
fi

# release lock and ensure runner enters migration step in skip-fmt mode
flock -u 8
STEP_OUTPUT="$(mktemp)"
if env RUSTOK_VERIFY_SKIP_FMT=1 timeout 8s "$RUNNER" >"$STEP_OUTPUT" 2>&1; then
  :
fi
if ! rg -q "Skipping format check because RUSTOK_VERIFY_SKIP_FMT=1" "$STEP_OUTPUT"; then
  echo "skip-fmt mode message missing" >&2
  cat "$STEP_OUTPUT" >&2
  exit 1
fi
if ! rg -q "==> migration tests" "$STEP_OUTPUT"; then
  echo "runner did not reach migration step" >&2
  cat "$STEP_OUTPUT" >&2
  exit 1
fi

echo "control_plane_remediation_minimal_runner_test.sh: PASS"
