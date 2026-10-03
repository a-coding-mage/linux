#!/bin/sh
# SPDX-License-Identifier: GPL-2.0
# Compressed boot never applies absolute runtime pointer relocations.
set -eu
readelf=$1
object=$2
relocations=$("$readelf" -rW "$object")
printf '%s\n' "$relocations" | awk '
 /^Relocation section/ { debug = ($0 ~ /\.debug/ || $0 ~ /\.zdebug/) }
 !debug && /R_X86_64_(64|32|32S|GOT[A-Z0-9_]*|GLOB_DAT|JUMP_SLOT|RELATIVE|COPY)[[:space:]]/ {
   print "unsupported Rust boot relocation: " $0 > "/dev/stderr"; bad = 1
 }
 END { exit bad }
'
sections=$("$readelf" -SW "$object")
printf '%s\n' "$sections" | awk '
 /[[:space:]]\.(got(\.plt)?|plt|rela?\.dyn)[[:space:]]/ {
   print "unsupported Rust boot linkage section: " $0 > "/dev/stderr"; bad = 1
 }
 END { exit bad }
'
