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
# 17-minute stage covers unchanged QEMU900s+kill10s and110s postcheck margin.
# Parser300s, cleanup60s and evidence180s are reserved in addition to the stage.
required=1560
(( PILOT_JOB_DEADLINE_EPOCH == PILOT_JOB_STARTED_EPOCH+6900 && PILOT_CONTAINER_STARTED_EPOCH >= PILOT_JOB_STARTED_EPOCH && PILOT_CONTAINER_DEADLINE_EPOCH <= PILOT_CONTAINER_STARTED_EPOCH+6000 && PILOT_CONTAINER_DEADLINE_EPOCH <= PILOT_JOB_DEADLINE_EPOCH-180 && now >= PILOT_CONTAINER_STARTED_EPOCH && container_remaining >= required && job_remaining >= required )) || exit 78
printf '{"now":%s,"job_remaining":%s,"container_remaining":%s,"required_seconds":1560,"guest_timeout":900,"guest_kill_grace":10,"guest_stage_allowance":1020,"parser_seconds":300,"cleanup_seconds":60,"evidence_seconds":180}\n' "$now" "$job_remaining" "$container_remaining" || exit 79
