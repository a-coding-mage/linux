#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-only
set -uo pipefail
export LC_ALL=C
id=${1:?owned container ID}; logs=${2:?evidence directory}
[[ $id =~ ^[0-9a-f]{64}$ ]] || exit 79
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd) || exit 79
clock_deadline() { perl -MTime::HiRes=clock_gettime,CLOCK_MONOTONIC -e 'printf "%.9f", clock_gettime(CLOCK_MONOTONIC)+$ARGV[0]' -- "$1"; }
deadline=$(clock_deadline 60) || exit 79
output=''; status=0
before_deadline() { perl -MTime::HiRes=clock_gettime,CLOCK_MONOTONIC -e 'exit(clock_gettime(CLOCK_MONOTONIC)<$ARGV[0]?0:79)' "$deadline"; }
call() {
  local cap=$1 operation=$2 result record; shift 2
  perl "$here/cleanup-command.pl" "$cap" "$deadline" "$logs/cleanup.capture" "$logs/cleanup.command-status" docker "$@"; result=$?
  record=$(cat "$logs/cleanup.command-status") || return 79
  printf 'operation=%s %s\n' "$operation" "$record" >> "$logs/cleanup.inspect" || return 79
  output=$(cat "$logs/cleanup.capture") || return 79
  if (( result != 0 )); then
    printf 'operation=%s wrapper_exit=%s\n%s\n' "$operation" "$result" "$output" >> "$logs/cleanup.stderr"
    return 79
  fi
  [[ $record =~ ^command_exit=([0-9]+)\ capture_exit=0\ supervisor_exit=0\ reaped=1\ group_absent=1$ ]] || return 79
  status=${BASH_REMATCH[1]}
  if (( status == 0 )); then
    printf '%s\n' "$output" >> "$logs/cleanup.inspect" || return 79
  else
    printf 'operation=%s exit=%s\n%s\n' "$operation" "$status" "$output" >> "$logs/cleanup.stderr" || return 79
  fi
  before_deadline || { printf 'cleanup_deadline_exhausted\n' >> "$logs/cleanup.stderr"; return 79; }
}
absent() { (( status == 1 )) && { [[ $output == "Error: No such object: $id" ]] || [[ $output == $'\n'"Error: No such object: $id" ]] || [[ $output == "Error response from daemon: No such container: $id" ]]; }; }
removing() { (( status == 1 )) && [[ $output == "Error response from daemon: removal of container $id is already in progress" ]]; }
verify_absence() {
  absent && before_deadline || return 79
  printf 'cleanup_verified_absent=%s\n' "$id" >> "$logs/cleanup.inspect" || return 79
}
probe() { call "$1" "$2" inspect --format '{{.State.Running}}' "$id"; }
probe 20 initial-inspect || exit 79
if absent; then verify_absence; exit $?; fi
(( status == 0 )) && [[ $output == true || $output == false ]] || exit 79
if [[ $output == true ]]; then
  call 25 stop stop --time 10 "$id" || exit 79
  if (( status != 0 )) && ! absent && ! removing; then exit 79; fi
fi
removal_deadline=$(clock_deadline 20) || exit 79
deadline=$(perl -e 'print $ARGV[0]<$ARGV[1]?$ARGV[0]:$ARGV[1]' "$deadline" "$removal_deadline") || exit 79
call 20 remove rm --force "$id" || exit 79
if (( status != 0 )) && ! absent && ! removing; then exit 79; fi
while before_deadline; do
  probe 20 verify-absent || exit 79
  if absent; then verify_absence; exit $?; fi
  (( status == 0 )) && [[ $output == true || $output == false ]] || exit 79
  sleep 1 || exit 79
done
printf 'cleanup_absence_unproved=%s\n' "$id" >> "$logs/cleanup.stderr"
exit 79
