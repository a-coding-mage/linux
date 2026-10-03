#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-only
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
cat "$here/launch.json"
# These values may be changed only in the exact owner-reviewed setup revision.
# This is a recorded approval gate, not a way for software to grant permission.
if ! jq -e '.schema==1 and .status=="reviewed-pilot" and .public_source_review=="approved" and .owner_pilot_approval=="one-logs-only-execution-approved" and .evidence_mode=="logs-only"' "$here/launch.json" >/dev/null; then
  printf '%s\n' 'LAUNCH_BLOCKED: exact public draft and one logs-only execution lack owner approval.' >&2
  exit 78
fi
# All experiment inputs are frozen in the same reviewed setup revision.
# This extends the existing launch boundary; it is not a second activation.
[[ $(sed -n 's/.*source_sha.*"\([0-9a-f]\{40\}\)".*/\1/p' "$here/../unicode-request.json") =~ ^[0-9a-f]{40}$ ]]
jq -e '.schema==1 and (.c_sha256|test("^[0-9a-f]{64}$")) and (.rust_sha256|test("^[0-9a-f]{64}$"))' "$here/config-lock.json" >/dev/null
[[ -f $here/integrated-rust.config.gz && ! -L $here/integrated-rust.config.gz ]]
[[ $(gzip -dc "$here/integrated-rust.config.gz" | sha256sum | cut -d' ' -f1) == "$(jq -er .rust_sha256 "$here/config-lock.json")" ]]
for file in boot-packages.tsv boot-init prepare-boot.sh verify-boot.pl audit-integrated.sh audit-integrated-elf.pl integrated-owners.tsv integrated-c-owners.tsv crc-cases.txt chacha-cases.txt iov-cases.txt observe-integrated-cases.pl public-source.sha256; do
  [[ -s $here/$file && ! -L $here/$file ]]
done
