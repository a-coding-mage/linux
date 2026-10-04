#!/bin/sh
# SPDX-License-Identifier: GPL-2.0
# Decode configured header metadata only after bindgen's normal prerequisite.
# No compiler, preprocessor or run-time probing is performed by this script.
set -eu
bindings=$1
compaction_enabled=${2:-}
sed -n 's/^pub const RUST_COMPACTION_CFG_\([A-Z0-9_]*\): bool[_]* = true;/--cfg RUST_COMPACTION_\1/p' "$bindings"
line=$(sed -n '/^pub const RUST_COMPACTION_NATIVE_READ_MOSTLY:/p' "$bindings")
# An unconfigured compaction translation unit has no data to annotate.
if [ -z "$line" ]; then
    if [ "$compaction_enabled" = y ]; then
        echo "Missing configured __read_mostly metadata for compaction" >&2
        exit 1
    fi
    exit 0
fi
case "$line" in
    *'= b"\0";'|*'= b"";') ;;
    *'= b"__attribute__((__section__(\".data..read_mostly\")))\0";')
        printf '%s\n' '--cfg RUST_COMPACTION_READ_MOSTLY' ;;
    *) echo 'Unsupported native __read_mostly expansion in compaction bindings' >&2; exit 1 ;;
esac
