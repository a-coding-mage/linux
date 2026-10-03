#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-only
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
cat "$here/launch.json"
# These values may be changed only in the exact owner-reviewed setup revision.
# This is a recorded approval gate, not a way for software to grant permission.
if ! jq -e '.schema==1 and .status=="reviewed-pilot" and .public_source_review=="approved" and .owner_pilot_approval=="one-logs-only-execution-approved" and .evidence_mode=="logs-only"' "$here/launch.json" >/dev/null; then
  printf '%s\n' 'LAUNCH_BLOCKED: exact public draft and one logs-only execution lack owner approval.' >&2
  exit 78
fi
