#!/bin/sh
# SPDX-License-Identifier: GPL-2.0
# Derive early-boot settings from the running compiler's freestanding ABI.
set -eu
output=$1
shift
"$@" -Zunstable-options --target=x86_64-unknown-none --print target-spec-json > "$output.native.tmp"
# Fail closed if rustc changes either native setting or the printed format.
grep -q '"code-model": "kernel"' "$output.native.tmp"
grep -q '"stack-probes": {' "$output.native.tmp"
grep -q '"kind": "inline"' "$output.native.tmp"
sed -e 's/"code-model": "kernel"/"code-model": "small"/' \
    -e 's/"kind": "inline"/"kind": "none"/' "$output.native.tmp" > "$output.next.tmp"
grep -q '"code-model": "small"' "$output.next.tmp"
grep -q '"kind": "none"' "$output.next.tmp"
mv "$output.next.tmp" "$output"
