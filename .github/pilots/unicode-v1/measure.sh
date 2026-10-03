#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-only
# Every external result is checked explicitly; correctness does not rely on -e.
set -uo pipefail
export LC_ALL=C
work=${1:?work directory}; logs=${2:?host or runtime evidence directory}
fail() { printf 'TELEMETRY_FAILED metric=%s\n' "$1" >&2; exit 79; }
integer() { [[ $1 =~ ^(0|[1-9][0-9]*)$ && ${#1} -le 18 ]]; }
raw=$(df -PB1 -- "$work") || fail df_bytes
free_bytes=$(awk 'NR==2 { if (NF!=6 || $4 !~ /^[0-9]+$/) exit 1; n++; v=$4 } END { if(n!=1)exit 1; print v }' <<< "$raw") || fail df_bytes_parse
integer "$free_bytes" || fail df_bytes_shape
raw=$(df -Pi -- "$work") || fail df_inodes
inodes=$(awk 'NR==2 { if (NF!=6 || $4 !~ /^[0-9]+$/) exit 1; n++; v=$4 } END { if(n!=1)exit 1; print v }' <<< "$raw") || fail df_inodes_parse
integer "$inodes" || fail df_inodes_shape
memory=$(awk '/^MemAvailable:/ { if($2 !~ /^[0-9]+$/ || $3!="kB")exit 1; n++; v=$2*1024 } END { if(n!=1)exit 1; printf "%.0f\n",v }' /proc/meminfo) || fail memory
integer "$memory" || fail memory_shape
allocated() {
  local raw value
  [[ -d $1 && ! -L $1 ]] || return 79
  raw=$(du -s -B1 -- "$1") || return 79
  [[ $raw != *$'\n'* && $raw == *$'\t'* ]] || return 79
  value=${raw%%$'\t'*}
  integer "$value" || return 79
  printf '%s\n' "$value" || return 79
}
obytes=$(allocated "$work/O") || fail allocated_O
ebytes=$(allocated "$work/evidence") || fail allocated_runtime_evidence
runtime_path=$(realpath -e -- "$work/evidence") || fail runtime_realpath
log_path=$(realpath -e -- "$logs") || fail logs_realpath
if [[ $runtime_path != "$log_path" ]]; then
  extra=$(allocated "$logs") || fail allocated_host_evidence
  ebytes=$((ebytes+extra))
fi
integer "$ebytes" || fail evidence_shape
printf '%s %s %s %s %s\n' "$free_bytes" "$memory" "$inodes" "$obytes" "$ebytes" || fail output_write
