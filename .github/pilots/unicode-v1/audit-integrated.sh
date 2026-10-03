#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-only
# Read-only adapter for finished incremental C/R member outputs. Never builds.
set -Eeuo pipefail
[[ $# == 2 && ( $1 == c || $1 == rust ) && $2 == /work/O ]]
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)
[[ -d /src && -d /work/O && -d /work/evidence ]]
for stage in build noop; do
  grep -Eq "^stage=$stage native_exit=0 supervisor_exit=0 cleanup_exit=0 interrupted=0 telemetry_exit=0 timeout_seconds=[1-9][0-9]*$" "/work/evidence/$stage.status"
done
for role in owners.commands owners.symbols owners.archives owners.sha256 original-tests.commands original-tests.sha256 elf-ownership.txt ownership-summary.txt audit-integrated.receipt; do
  [[ ! -e /work/evidence/$role ]] || { printf 'refusing to replace %s\n' "$role" >&2; exit 1; }
done
perl "$here/audit-integrated-elf.pl" "$1" "$2"
