#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-only
set -euo pipefail
stage=${1:?stage}; volume=${2:?output volume}
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd) || exit 79
work=${PILOT_ROOT:-$volume/work}; logs=${PILOT_LOG_ROOT:-$volume/evidence}
mkdir -p "$work/O" "$work/evidence" "$logs" || exit 79
metrics=$(bash "$here/measure.sh" "$work" "$logs") || exit 79
[[ $metrics =~ ^[0-9]+\ [0-9]+\ [0-9]+\ [0-9]+\ [0-9]+$ ]] || exit 79
read -r disk mem inodes obytes ebytes <<< "$metrics" || exit 79
timestamp=$(date -u +%FT%TZ) || exit 79
printf 'RESOURCE_STAGE=%s UTC=%s\n' "$stage" "$timestamp" || exit 79
uname -a || exit 79
printf 'RUNNER_OS=%s RUNNER_ARCH=%s ImageOS=%s ImageVersion=%s\n' "${RUNNER_OS:-unknown}" "${RUNNER_ARCH:-unknown}" "${ImageOS:-unknown}" "${ImageVersion:-unknown}" || exit 79
free -b || exit 79
df -PB1 -- "$volume" || exit 79
df -Pi -- "$volume" || exit 79
cpus=$(nproc) || exit 79
[[ $cpus =~ ^[1-9][0-9]*$ ]] || exit 79
printf 'CPUS=%s\n' "$cpus" || exit 79
for item in memory.max memory.current memory.events cpu.max; do
  if [[ -r /sys/fs/cgroup/$item ]]; then printf '%s=' "$item" || exit 79; cat "/sys/fs/cgroup/$item" || exit 79; fi
done
printf 'CAPS O_allocated_bytes=%s evidence_allocated_bytes=%s\n' "$obytes" "$ebytes" || exit 79
(( disk >= 1073741824 && mem >= 536870912 && inodes >= 10000 && obytes <= 1342177280 && ebytes <= 134209536 )) || exit 78
if [[ $stage == initial ]]; then
  [[ ${RUNNER_OS:-} == Linux && ${RUNNER_ARCH:-} == X64 && ${ImageOS:-} == ubuntu24 ]] || exit 78
  (( cpus >= 4 && disk >= 10737418240 && mem >= 6442450944 && inodes >= 200000 )) || exit 78
fi
