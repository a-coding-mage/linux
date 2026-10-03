#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-only
# Existing observer/upstream parser boundary, applied to one original suite.
set -euo pipefail
[[ ${PILOT_RUN_ATTEMPT:-} == 1 ]]
suite=${1:?suite}
case $suite in crc) cases=/pilot/crc-cases.txt;; chacha20poly1305) cases=/pilot/chacha-cases.txt;; iov_iter) cases=/pilot/iov-cases.txt;; *) exit 78;; esac
stage=${2:?stage}
[[ $stage == crc && $suite == crc || $stage == chacha && $suite == chacha20poly1305 || $stage == iov && $suite == iov_iter ]]
prefix=/work/evidence/$stage
set +e
perl /pilot/observe-integrated-cases.pl "$suite" "$cases" "$prefix.log" "$prefix.qemu-exit" > "$prefix.observer.txt" 2> "$prefix.observer.stderr"
observer=$?
PYTHONDONTWRITEBYTECODE=1 python3 -B /src/tools/testing/kunit/kunit.py parse --json="$prefix.upstream.json" "$prefix.log" > "$prefix.upstream.txt" 2>&1
parser=$?
set -e
[[ -s $prefix.observer.txt && -s $prefix.upstream.json && -s $prefix.upstream.txt ]]
printf 'observer_exit=%s parser_exit=%s\n' "$observer" "$parser" > "$prefix.parser.status"
cat "$prefix.observer.txt" "$prefix.upstream.txt"
(( observer == 0 && parser == 0 ))
