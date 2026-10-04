#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-only
# Every external result is checked explicitly; correctness does not rely on -e.
set -uo pipefail
export LC_ALL=C
work=${1:?work directory}; logs=${2:?host or runtime evidence directory}
fail() { printf 'TELEMETRY_FAILED metric=%s\n' "$1" >&2; exit 79; }
integer() { [[ $1 =~ ^(0|[1-9][0-9]*)$ && ${#1} -le 18 ]]; }
allocated() {
  local raw value status attempt rest line path summaries errors remaining diagnostic_bytes=0
  # Only a complete successful traversal supplies a measurement. GNU du can
  # print a partial total and exit 1 when a build removes an entry mid-walk.
  # Fast transient failures may use more complete attempts, but never extend
  # the same five-second allocation deadline shared by every root below.
  for ((attempt=1; attempt<=32; attempt++)); do
    [[ -d $1 && ! -L $1 && $1 != *$'\n'* && $1 != *$'\t'* ]] || return 79
    remaining=$((allocation_deadline-SECONDS))
    (( remaining > 0 )) || return 79
    # Keep both pipeline statuses and trailing newlines; cap captured output.
    # Timeout/errors/overflow fail closed, without another attempt.
    raw=$(
      timeout --kill-after=.1s "${remaining}s" du -s -B1 -- "$1" 2>&1 |
        perl -e '
          use strict; use warnings;
          binmode STDIN; binmode STDOUT;
          my $bytes = "";
          while (length($bytes) < 16385) {
            my $n = sysread STDIN, my $chunk, 16385-length($bytes);
            defined $n or die "TELEMETRY_DU_BYTE_READ_FAILED: $!\n";
            last unless $n;
            $bytes .= $chunk;
          }
          my $nul = index($bytes, "\0");
          if ($nul >= 0) {
            printf STDERR "TELEMETRY_DU_INVALID_NUL offset=%d captured_bytes=%d prefix64_hex=%s\n",
              $nul, length($bytes), unpack("H*", substr($bytes, 0, 64));
            exit 79;
          }
          print $bytes or exit 79;
          close STDOUT or exit 79;
        '
      status=${PIPESTATUS[*]}
      printf '\n__DU_STATUS__=%s\n' "$status"
    ) || return 79
    status=${raw##*$'\n'__DU_STATUS__=}
    raw=${raw%$'\n'__DU_STATUS__=*}
    if [[ ${#raw} -gt 16384 || ! $status =~ ^[0-9]+\ 0$ ]]; then
      printf '%s\nTELEMETRY_DU_FAILED attempt=%s status=%s capture_invalid=true\n' "$raw" "$attempt" "$status" >&2
      return 79
    fi
    if [[ $status == '0 0' ]]; then
      (( SECONDS < allocation_deadline )) || return 79
      value=${raw%%$'\t'*}
      integer "$value" && [[ $raw == "$value"$'\t'"$1"$'\n' ]] || return 79
      printf '%s\n' "$value" || return 79
      return 0
    fi
    printf '%sTELEMETRY_DU_FAILED attempt=%s status=%s\n' "$raw" "$attempt" "$status" >&2 || return 79
    diagnostic_bytes=$((diagnostic_bytes+${#raw}+256))
    [[ $status == '1 0' ]] || return 79
    rest=$raw; summaries=0; errors=0
    while [[ $rest == *$'\n'* ]]; do
      line=${rest%%$'\n'*}; rest=${rest#*$'\n'}
      value=${line%%$'\t'*}
      if integer "$value" && [[ $line == "$value"$'\t'"$1" ]]; then
        summaries=$((summaries+1))
      else
        [[ $line == "du: cannot access '"*"': No such file or directory" ]] || return 79
        path=${line#"du: cannot access '"}; path=${path%"': No such file or directory"}
        # Deliberately narrow: ordinary Kbuild paths only. Other quoting,
        # diagnostics, unreadable directories, I/O errors and missing roots fail.
        [[ $path =~ ^[a-zA-Z0-9_./+-]+$ && $path == "$1/"?* ]] || return 79
        errors=$((errors+1))
      fi
    done
    [[ -z $rest && $summaries == 1 && $errors -gt 0 ]] || return 79
    (( attempt < 32 )) || return 79
    # Retain every failure; reserve a full worst-case capture before retrying.
    # Preserve three full attempts and leave terminal-report space below 64 KiB.
    (( diagnostic_bytes+16640 <= 65024 )) || {
      printf 'TELEMETRY_FAILED metric=allocated_retry_diagnostics\n' >&2
      return 79
    }
    printf 'TELEMETRY_RETRY metric=allocated attempt=%s reason=vanished_descendant\n' "$attempt" >&2 || return 79
    sleep .1 || return 79
  done
  return 79
}
# One new stricter budget shared by all roots and retries, not a per-scan reset.
allocation_deadline=$((SECONDS+5))
obytes=$(allocated "$work/O") || fail allocated_O
ebytes=$(allocated "$work/evidence") || fail allocated_runtime_evidence
runtime_path=$(realpath -e -- "$work/evidence") || fail runtime_realpath
log_path=$(realpath -e -- "$logs") || fail logs_realpath
if [[ $runtime_path != "$log_path" ]]; then
  extra=$(allocated "$logs") || fail allocated_host_evidence
  ebytes=$((ebytes+extra))
fi
integer "$ebytes" || fail evidence_shape
(( SECONDS < allocation_deadline )) || fail allocation_deadline
# Read live floors after allocation scans so retries cannot stale these readings.
raw=$(df -PB1 -- "$work") || fail df_bytes
free_bytes=$(awk 'NR==2 { if (NF!=6 || $4 !~ /^[0-9]+$/) exit 1; n++; v=$4 } END { if(n!=1)exit 1; print v }' <<< "$raw") || fail df_bytes_parse
integer "$free_bytes" || fail df_bytes_shape
raw=$(df -Pi -- "$work") || fail df_inodes
inodes=$(awk 'NR==2 { if (NF!=6 || $4 !~ /^[0-9]+$/) exit 1; n++; v=$4 } END { if(n!=1)exit 1; print v }' <<< "$raw") || fail df_inodes_parse
integer "$inodes" || fail df_inodes_shape
memory=$(awk '/^MemAvailable:/ { if($2 !~ /^[0-9]+$/ || $3!="kB")exit 1; n++; v=$2*1024 } END { if(n!=1)exit 1; printf "%.0f\n",v }' /proc/meminfo) || fail memory
integer "$memory" || fail memory_shape
printf '%s %s %s %s %s\n' "$free_bytes" "$memory" "$inodes" "$obytes" "$ebytes" || fail output_write
