#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-only
set -euo pipefail
export GIT_OPTIONAL_LOCKS=0
source_tree=$(realpath -- "${1:?source directory}")
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
[[ $(git -C "$source_tree" rev-parse HEAD) == 0008179a1ee0b082fa3ead118187fc6f3569a9f2 ]]
[[ -z $(git -C "$source_tree" status --porcelain=v1 --untracked-files=all) ]]
(cd "$source_tree" && sha256sum -c "$here/public-source.sha256")
printf 'SOURCE_VERIFIED=0008179a1ee0b082fa3ead118187fc6f3569a9f2\n'
