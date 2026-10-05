#!/bin/sh
# SPDX-License-Identifier: GPL-2.0
# Decode configured header metadata only after bindgen's normal prerequisite.
# No compiler, preprocessor or run-time probing is performed by this script.
set -eu
bindings=$1
vmstat_enabled=${2:-}
sed -n 's/^pub const RUST_VMSTAT_CFG_\([A-Z0-9_]*\): bool[_]* = true;/--cfg RUST_VMSTAT_\1/p' "$bindings"
line=$(sed -n '/^pub const RUST_VMSTAT_NATIVE_READ_MOSTLY:/p' "$bindings")
# An unconfigured vmstat translation unit has no data to annotate.
if [ -z "$line" ]; then
    if [ "$vmstat_enabled" = y ]; then
        echo "Missing configured __read_mostly metadata for vmstat" >&2
        exit 1
    fi
    exit 0
fi
case "$line" in
    *'= b"\0";'|*'= b"";') ;;
    *'= b"__attribute__((__section__(\".data..read_mostly\")))\0";')
        printf '%s\n' '--cfg RUST_VMSTAT_READ_MOSTLY' ;;
    *) echo 'Unsupported native __read_mostly expansion in vmstat bindings' >&2; exit 1 ;;
esac
