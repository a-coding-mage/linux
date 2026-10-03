#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-only
set -euo pipefail
[[ ${GITHUB_RUN_ATTEMPT:-} == 1 ]] || { echo 'RERUN_REJECTED'; exit 78; }
[[ ${PILOT_PROVIDER:-} == c || ${PILOT_PROVIDER:-} == rust ]]
[[ ${GITHUB_RUN_ID:-} =~ ^[0-9]+$ && ${GITHUB_SHA:-} =~ ^[0-9a-f]{40}$ ]]
[[ ${PILOT_JOB_STARTED_EPOCH:-} =~ ^[1-9][0-9]{9,11}$ && ${PILOT_JOB_DEADLINE_EPOCH:-} =~ ^[1-9][0-9]{9,11}$ ]]
(( PILOT_JOB_DEADLINE_EPOCH == PILOT_JOB_STARTED_EPOCH+6900 ))
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
export PILOT_ROOT=$GITHUB_WORKSPACE/work PILOT_LOG_ROOT=$GITHUB_WORKSPACE/evidence PILOT_RUN_ATTEMPT=1
mkdir -p "$PILOT_ROOT" "$PILOT_LOG_ROOT"
image=docker.io/library/debian:trixie-slim@sha256:7792b1f7702a86946cd518db72b6a407302c3e9bc1635634368b878189e8221c
container_name=unicode-pilot-$GITHUB_RUN_ID-$PILOT_PROVIDER
cidfile=$PILOT_LOG_ROOT/container.cid
cleanup=0; emitting=0
finish() {
  local result=$? container_id='' state='' probe=0
  trap - EXIT INT TERM; set +e
  : > "$PILOT_LOG_ROOT/cleanup.inspect" || cleanup=79
  : > "$PILOT_LOG_ROOT/cleanup.stderr" || cleanup=79
  if [[ -s $cidfile ]]; then
    container_id=$(cat "$cidfile") || cleanup=79
    if [[ ! $container_id =~ ^[0-9a-f]{64}$ ]]; then cleanup=79
    else
      timeout 20 docker inspect --format '{{.State.Running}}' "$container_id" > "$PILOT_LOG_ROOT/cleanup.inspect" 2> "$PILOT_LOG_ROOT/cleanup.stderr"; probe=$?
      if (( probe == 0 )); then
        state=$(cat "$PILOT_LOG_ROOT/cleanup.inspect") || cleanup=79
        [[ $state == true || $state == false ]] || cleanup=79
        if [[ $state == true ]]; then
          timeout 25 docker stop --time 10 "$container_id" >> "$PILOT_LOG_ROOT/cleanup.inspect" 2>> "$PILOT_LOG_ROOT/cleanup.stderr" || cleanup=79
        fi
        timeout 20 docker rm --force "$container_id" >> "$PILOT_LOG_ROOT/cleanup.inspect" 2>> "$PILOT_LOG_ROOT/cleanup.stderr" || cleanup=79
      elif (( probe != 1 )) || ! grep -Fq 'No such object:' "$PILOT_LOG_ROOT/cleanup.stderr"; then cleanup=79
      fi
    fi
  fi
  printf 'cleanup_exit=%s\n' "$cleanup" > "$PILOT_LOG_ROOT/cleanup.status" || { cleanup=79; result=79; }
  (( cleanup == 0 )) || result=79
  # A final whole-lifecycle sample includes install, audits, parser and emission
  # inputs. Stage wrappers also sample after their leader and group terminate.
  bash "$here/resources.sh" terminal "$GITHUB_WORKSPACE" > "$PILOT_LOG_ROOT/terminal.txt" 2>&1 || result=78
  printf 'native_exit=%s cleanup_exit=%s\n' "$result" "$cleanup" > "$PILOT_LOG_ROOT/result.status" || result=79
  perl "$here/emit-evidence.pl" "$PILOT_ROOT/evidence" "$PILOT_LOG_ROOT" "$PILOT_PROVIDER" "$GITHUB_RUN_ID" "$GITHUB_SHA"; emission=$?
  (( emission == 0 )) || result=78
  bash "$here/resources.sh" terminal "$GITHUB_WORKSPACE" >/dev/null 2>&1; post_resource=$?
  (( post_resource == 0 )) || result=78
  printf 'UNICODE_PILOT_V3_TERMINAL provider=%s exit=%s cleanup=%s emission=%s post_resource=%s\n' "$PILOT_PROVIDER" "$result" "$cleanup" "$emission" "$post_resource" || result=79
  exit "$result"
}
trap finish EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
bash "$here/resources.sh" initial "$GITHUB_WORKSPACE" > "$PILOT_LOG_ROOT/preflight.txt"
command -v perl >/dev/null
# Monitor image pull as part of the lifecycle, before any container exists.
bash "$here/guard.sh" pull 600 docker pull --platform linux/amd64 "$image"
docker image inspect "$image" --format '{{json .RepoDigests}} {{.Id}} {{.Architecture}}' > "$PILOT_LOG_ROOT/image.txt"
container_started=$(date -u +%s) || exit 79
[[ $container_started =~ ^[1-9][0-9]{9,11}$ ]] || exit 79
container_deadline=$((container_started+6000))
(( container_deadline <= PILOT_JOB_DEADLINE_EPOCH-180 )) || container_deadline=$((PILOT_JOB_DEADLINE_EPOCH-180))
container_seconds=$((container_deadline-container_started))
(( container_seconds > 1560 && container_seconds <= 6000 )) || exit 78
bash "$here/guard.sh" container "$container_seconds" docker run --rm --platform linux/amd64 --name "$container_name" --cidfile "$cidfile" \
  --label "unicode_pilot_run=$GITHUB_RUN_ID" \
  --mount "type=bind,src=$GITHUB_WORKSPACE/linux,dst=/src,readonly" \
  --mount "type=bind,src=$here,dst=/pilot,readonly" \
  --mount "type=bind,src=$PILOT_ROOT,dst=/work" \
  --env "PILOT_PROVIDER=$PILOT_PROVIDER" --env PILOT_RUN_ATTEMPT=1 \
  --env "PILOT_RUN_ID=$GITHUB_RUN_ID" --env "PILOT_TRIGGER_SHA=$GITHUB_SHA" \
  --env "PILOT_JOB_STARTED_EPOCH=$PILOT_JOB_STARTED_EPOCH" --env "PILOT_JOB_DEADLINE_EPOCH=$PILOT_JOB_DEADLINE_EPOCH" \
  --env "PILOT_CONTAINER_STARTED_EPOCH=$container_started" --env "PILOT_CONTAINER_DEADLINE_EPOCH=$container_deadline" \
  "$image" bash /pilot/run-container.sh
