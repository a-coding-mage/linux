#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-only
set -euo pipefail
[[ ${PILOT_RUN_ATTEMPT:-} == 1 ]]
for key in PILOT_JOB_STARTED_EPOCH PILOT_JOB_DEADLINE_EPOCH PILOT_CONTAINER_STARTED_EPOCH PILOT_CONTAINER_DEADLINE_EPOCH; do
 [[ ${!key:-} =~ ^[1-9][0-9]{9,11}$ ]] || exit 78
done
now=$(date -u +%s) || exit 79
[[ $now =~ ^[1-9][0-9]{9,11}$ ]] || exit 79
job_remaining=$((PILOT_JOB_DEADLINE_EPOCH-now))
container_remaining=$((PILOT_CONTAINER_DEADLINE_EPOCH-now))
# Preserve each original native timeout and parser tail in the shared budget.
profile=${1:?profile}
case $profile in
 boot) native=240; allowance=360; parser=120;;
 crc|chacha|iov) native=900; allowance=1020; parser=300;;
 *) exit 78;;
esac
required=$((allowance+parser+60+180))
(( PILOT_JOB_DEADLINE_EPOCH == PILOT_JOB_STARTED_EPOCH+6900 && PILOT_CONTAINER_STARTED_EPOCH >= PILOT_JOB_STARTED_EPOCH && PILOT_CONTAINER_DEADLINE_EPOCH <= PILOT_CONTAINER_STARTED_EPOCH+6000 && PILOT_CONTAINER_DEADLINE_EPOCH <= PILOT_JOB_DEADLINE_EPOCH-180 && now >= PILOT_CONTAINER_STARTED_EPOCH && container_remaining >= required && job_remaining >= required )) || exit 78
printf '{"now":%s,"job_remaining":%s,"container_remaining":%s,"required_seconds":%s,"guest_timeout":%s,"guest_kill_grace":10,"guest_stage_allowance":%s,"parser_seconds":%s,"cleanup_seconds":60,"evidence_seconds":180}\n' "$now" "$job_remaining" "$container_remaining" "$required" "$native" "$allowance" "$parser" || exit 79
