#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-only
# Newly authored for this public draft; not copied from another repository.
set -euo pipefail
repo=$(realpath -- "${1:?repository}")
event=${2:?push event JSON}
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
manifest=.github/pilots/unicode-request.json
die() { printf 'REQUEST_REJECTED: %s\n' "$*" >&2; exit 1; }
[[ ${GITHUB_EVENT_NAME:-} == push ]] || die 'push event required'
[[ ${GITHUB_RUN_ATTEMPT:-} == 1 ]] || die 'reruns require a newly reviewed request'
jq -e -f "$here/gate-event.jq" "$event" >/dev/null || die 'event not allowlisted'
before=$(jq -er .before "$event")
after=$(jq -er .after "$event")
[[ ${GITHUB_SHA:-} == "$after" ]] || die 'event SHA mismatch'
[[ ${GITHUB_REF:-} == refs/heads/feat/rust-translation-lupos ]] || die 'wrong execution ref'
[[ $(git -C "$repo" rev-parse HEAD) == "$after" ]] || die 'wrong checkout'
# Commit topology is authoritative; payload arrays/path filters are insufficient.
read -r -a parents <<< "$(git -C "$repo" show -s --format=%P "$after")"
[[ ${#parents[@]} == 1 && ${parents[0]} == "$before" ]] || die 'not a one-commit fast-forward push'
git -C "$repo" cat-file -e "$before^{commit}" || die 'missing parent'
# Installation must be the first commit directly above this reviewed source.
# Disarm/rearm pushes cannot reuse this request ID on later history. A newer
# feature-branch tip requires a new reviewed setup base, not a silent ref move.
read -r -a setup_parents <<< "$(git -C "$repo" show -s --format=%P "$before")"
[[ ${#setup_parents[@]} == 1 && ${setup_parents[0]} == 0008179a1ee0b082fa3ead118187fc6f3569a9f2 ]] || die 'setup parent not allowlisted'
changes=$(git -C "$repo" diff-tree --no-commit-id --name-status -r --no-renames "$before" "$after")
[[ $changes == $'M\t'"$manifest" ]] || die 'only the existing manifest may change'
[[ $(git -C "$repo" ls-tree "$after" -- "$manifest" | cut -d' ' -f1) == 100644 ]] || die 'manifest must be a regular file'
(( $(stat -c %s "$repo/$manifest") < 1024 )) || die 'oversized manifest'
# Exact bytes reject duplicate JSON keys, extra fields, unreviewed IDs and profiles.
expected=$(cat <<'JSON'
{
  "schema": 1,
  "request_id": "lupos-integrated-phase2-v1",
  "source_sha": "0008179a1ee0b082fa3ead118187fc6f3569a9f2",
  "profile": "lupos-phase2-original-c-x86_64-v1",
  "evidence": "logs-only",
  "armed": true
}
JSON
)
cmp -s "$repo/$manifest" <(printf '%s\n' "$expected") || die 'request bytes not allowlisted'
cmp -s <(git -C "$repo" show "$before:$manifest") <(printf '%s\n' "${expected/\"armed\": true/\"armed\": false}") || die 'prior request must be unarmed'
printf 'REQUEST_ADMITTED source=%s request=%s trigger=%s parent=%s\n' \
  0008179a1ee0b082fa3ead118187fc6f3569a9f2 lupos-integrated-phase2-v1 "$after" "$before"
