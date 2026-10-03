#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-only
set -euo pipefail
[[ ${PILOT_RUN_ATTEMPT:-${GITHUB_RUN_ATTEMPT:-}} == 1 ]]
stage=${1:?stage}; seconds=${2:?seconds}; shift 2
[[ $stage =~ ^[a-z-]+$ && $seconds =~ ^[1-9][0-9]*$ && $# -gt 0 ]]
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd) || exit 79
work=${PILOT_ROOT:-/work}; logs=${PILOT_LOG_ROOT:-$work/evidence}
mkdir -p "$logs" "$work/O" "$work/evidence" || exit 79
resource_file=$logs/$stage.resources.tsv
printf 'phase\tutc\tfree_bytes\tavailable_memory\tfree_inodes\tO_allocated_bytes\tevidence_allocated_bytes\n' > "$resource_file" || exit 79
telemetry=0
sample() {
  local phase=$1 metrics timestamp disk mem inodes obytes ebytes
  metrics=$(bash "$here/measure.sh" "$work" "$logs") || { telemetry=79; return 79; }
  [[ $metrics =~ ^[0-9]+\ [0-9]+\ [0-9]+\ [0-9]+\ [0-9]+$ ]] || { telemetry=79; return 79; }
  read -r disk mem inodes obytes ebytes <<< "$metrics" || { telemetry=79; return 79; }
  timestamp=$(date -u +%FT%TZ) || { telemetry=79; return 79; }
  [[ $timestamp =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]] || { telemetry=79; return 79; }
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$phase" "$timestamp" "$disk" "$mem" "$inodes" "$obytes" "$ebytes" >> "$resource_file" || { telemetry=79; return 79; }
  (( disk >= 1073741824 && mem >= 536870912 && inodes >= 10000 && obytes <= 1342177280 && ebytes <= 134209536 )) || return 78
  if [[ $phase == admission ]]; then (( disk >= 4294967296 && mem >= 6442450944 )) || return 78; fi
  return 0
}
pid=''; native=125; supervisor=0; cleanup_status=0; interrupted=0; finished=0; waited=0
cleanup_group() {
  [[ -n $pid ]] || return 0
  if kill -0 -- "-$pid" 2>/dev/null; then
    (( supervisor != 0 )) || supervisor=78
    kill -TERM -- "-$pid" 2>/dev/null || cleanup_status=79
    for ((i=0;i<20;i++)); do kill -0 -- "-$pid" 2>/dev/null || break; sleep .1 || cleanup_status=79; done
    if kill -0 -- "-$pid" 2>/dev/null; then kill -KILL -- "-$pid" 2>/dev/null || cleanup_status=79; fi
    for ((i=0;i<20;i++)); do kill -0 -- "-$pid" 2>/dev/null || break; sleep .1 || cleanup_status=79; done
    if kill -0 -- "-$pid" 2>/dev/null; then cleanup_status=79; fi
  fi
}
finish() {
  local incoming=$? terminal_status=0 write_status=0
  (( finished == 0 )) || return
  finished=1; trap - EXIT INT TERM; set +e
  (( incoming == 0 )) || supervisor=$incoming
  cleanup_group
  if [[ -n $pid ]] && (( waited == 0 )); then
    if kill -0 "$pid" 2>/dev/null; then cleanup_status=79
    else wait "$pid"; native=$?; waited=1; fi
  fi
  sample terminal; terminal_status=$?
  (( terminal_status == 0 )) || supervisor=$terminal_status
  printf 'stage=%s native_exit=%s supervisor_exit=%s cleanup_exit=%s interrupted=%s telemetry_exit=%s timeout_seconds=%s\n' "$stage" "$native" "$supervisor" "$cleanup_status" "$interrupted" "$telemetry" "$seconds" > "$logs/$stage.status" || write_status=79
  cat "$logs/$stage.status" || write_status=79
  if (( native == 0 && supervisor == 0 && cleanup_status == 0 && interrupted == 0 && telemetry == 0 && write_status == 0 )); then exit 0; else exit 78; fi
}
trap finish EXIT
trap 'interrupted=130; exit 130' INT
trap 'interrupted=143; exit 143' TERM
sample admission || { supervisor=$?; exit "$supervisor"; }
printf '%q ' "$@" > "$logs/$stage.command" || exit 79
printf '\n' >> "$logs/$stage.command" || exit 79
# The stage ceiling is separate from an unchanged timeout inside its command.
# Direct setsid keeps the original guest timeout900 process as the group leader.
started_seconds=$SECONDS
setsid "$@" > "$logs/$stage.log" 2>&1 & pid=$!
while kill -0 "$pid" 2>/dev/null; do
  if (( SECONDS-started_seconds >= seconds )); then supervisor=124; break; fi
  sample live || { supervisor=$?; break; }
  sleep 5 || { supervisor=79; break; }
done
if (( supervisor == 0 )); then set +e; wait "$pid"; native=$?; waited=1; set -e; fi
exit 0
