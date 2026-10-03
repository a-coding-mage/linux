#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-only
set -euo pipefail
[[ ${PILOT_RUN_ATTEMPT:-} == 1 ]]
# No tee pipeline can conceal a failed evidence write.
set +e
perl /pilot/observe-original-cases.pl /work/evidence/guest.log > /work/evidence/strict-observer.txt 2> /work/evidence/strict-observer.stderr
observer=$?
PYTHONDONTWRITEBYTECODE=1 python3 /src/tools/testing/kunit/kunit.py parse --json=/work/evidence/upstream-result.json /work/evidence/guest.log > /work/evidence/upstream-parser.txt 2>&1
parser=$?
set -e
[[ -s /work/evidence/strict-observer.txt && -s /work/evidence/upstream-result.json && -s /work/evidence/upstream-parser.txt ]]
printf 'observer_exit=%s parser_exit=%s\n' "$observer" "$parser" > /work/evidence/observer-parser.status
cat /work/evidence/strict-observer.txt /work/evidence/upstream-parser.txt
(( observer == 0 && parser == 0 ))
