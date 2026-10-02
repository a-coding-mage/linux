#!/bin/sh
# SPDX-License-Identifier: GPL-2.0
# Accept only address-independent metadata BTF from a separate debug oracle.
# This checks the transplant boundary, not Rust/C ABI equivalence. The caller
# must use the same build and base for the oracle, this check and resolve_btfids.
# resolve_btfids must still parse/finalize the graph against that base and the
# real module. No oracle object or runtime section may enter the real link.
set -eu
LC_ALL=C
export LC_ALL

fail()
{
	echo "check-module-btf-oracle: $*" >&2
	exit 1
}

[ "$#" -eq 4 ] || fail "usage: $0 <raw BTF> <module ELF> <oracle ELF> <base ELF>"
raw=$1
module=$2
oracle=$3
base=$4
for input in "$raw" "$module" "$oracle" "$base"; do
	[ -f "$input" ] && [ -r "$input" ] || fail "unreadable input: $input"
done
work=$(mktemp -d "${TMPDIR:-/tmp}/module-btf-oracle.XXXXXXXX")
trap 'rm -rf "$work"' EXIT
trap 'exit 1' HUP INT TERM

# Read only headers, section tables and section names, even for a large vmlinux.
# Reject extended section numbering and offsets outside AWK exact-integer range.
elf_info()
{
	elf=$1
	role=$2
	bytes=$(wc -c < "$elf")
	od -An -v -t u1 -N 64 "$elf" > "$work/header"
	info=$(awk -v total="$bytes" -v role="$role" '
	function die(s) { print "check-module-btf-oracle: " role ": " s > "/dev/stderr"; exit 1 }
	function u(p, z,    i, v) {
		v = 0
		for (i = 0; i < z; i++) v = v * 256 + b[p + (le ? z - i - 1 : i)]
		if (v > 9007199254740991) die("oversized ELF offset")
		return v
	}
	{ for (i = 1; i <= NF; i++) b[n++] = $i }
	END {
		if (n < 52 || b[0] != 127 || b[1] != 69 || b[2] != 76 || b[3] != 70)
			die("invalid ELF header")
		c = b[4]; e = b[5]; le = e == 1
		if ((c != 1 && c != 2) || (e != 1 && e != 2) || b[6] != 1 || u(20, 4) != 1)
			die("unsupported ELF format")
		if (c == 2 && n != 64) die("truncated ELF64 header")
		if ((role != "base" && u(16, 2) != 1) ||
		    (role == "base" && u(16, 2) != 2 && u(16, 2) != 3))
			die("unexpected ELF type")
		if (!u(18, 2) || u(c == 1 ? 40 : 52, 2) != (c == 1 ? 52 : 64))
			die("invalid ELF machine/header size")
		off = u(c == 1 ? 32 : 40, c == 1 ? 4 : 8)
		ent = u(c == 1 ? 46 : 58, 2); count = u(c == 1 ? 48 : 60, 2)
		names = u(c == 1 ? 50 : 62, 2)
		if (ent != (c == 1 ? 40 : 64) || !count || !names || names >= count ||
		    names == 65535 || off < n || off > total || count * ent > total - off)
			die("invalid/unsupported ELF section table")
		printf "%d %d %d %.0f %d %d %d\n", c, e, u(18, 2), off, ent, count, names
	}' "$work/header") || fail "$role ELF header rejected"
	set -- $info
	class=$1 endian=$2 machine=$3 shoff=$4 shent=$5 shnum=$6 shstr=$7
	od -An -v -t u1 -j "$shoff" -N "$((shent * shnum))" "$elf" > "$work/table"
	awk -v c="$class" -v le="$((2 - endian))" -v ent="$shent" -v count="$shnum" -v total="$bytes" '
	function die(s) { print "check-module-btf-oracle: " s > "/dev/stderr"; exit 1 }
	function u(p, z,    i, v) {
		v = 0
		for (i = 0; i < z; i++) v = v * 256 + b[p + (le ? z - i - 1 : i)]
		if (v > 9007199254740991) die("oversized ELF section field")
		return v
	}
	{ for (i = 1; i <= NF; i++) b[n++] = $i }
	END {
		if (n != count * ent) die("truncated ELF section table")
		for (s = 0; s < count; s++) {
			p = s * ent; type = u(p + 4, 4)
			off = u(p + (c == 1 ? 16 : 24), c == 1 ? 4 : 8)
			size = u(p + (c == 1 ? 20 : 32), c == 1 ? 4 : 8)
			if (type != 0 && type != 8 && (off > total || size > total - off))
				die("ELF section outside file")
			printf "%d %.0f %d %.0f %.0f %.0f %.0f\n", s, u(p, 4), type,
				u(p + 8, c == 1 ? 4 : 8), off, size, u(p + (c == 1 ? 28 : 44), 4)
		}
	}' "$work/table" > "$work/sections" || fail "$role section table rejected"
	info=$(awk -v string_index="$shstr" '$1 == string_index { if ($3 != 3 || !$6) exit 1; print $5, $6; found = 1 } END { if (!found) exit 1 }' "$work/sections") || fail "$role has no section-name table"
	set -- $info
	od -An -v -t u1 -j "$1" -N "$2" "$elf" > "$work/names"
	btf=$(awk -v string_length="$2" -v count="$shnum" -v role="$role" '
	function die(s) { print "check-module-btf-oracle: " role ": " s > "/dev/stderr"; exit 1 }
	FNR == NR { off[$1] = $2; type[$1] = $3; flags[$1] = $4; pos[$1] = $5; size[$1] = $6; dest[$1] = $7; next }
	{ for (i = 1; i <= NF; i++) b[n++] = $i }
	END {
		if (n != string_length || b[0] || b[n - 1]) die("invalid ELF section-name table")
		for (s = 0; s < count; s++) {
			if (off[s] >= n) die("invalid ELF section name offset")
			name = ""
			for (j = off[s]; j < n && b[j]; j++) name = name sprintf("%c", b[j])
			if (j == n) die("unterminated ELF section name")
			names[s] = name
			if (name ~ /^\.BTF/ && name != ".BTF" && role != "base")
				die("BTF adjunct is not supported: " name)
			if (name == ".BTF") {
				if (++found != 1 || type[s] != 1 || !size[s]) die("invalid ELF BTF section")
				if (role != "base" && int(flags[s] / 2) % 2) die("allocated module BTF section")
				btfpos = pos[s]; btfsize = size[s]
			}
		}
		for (s = 0; s < count; s++) {
			if (type[s] != 4 && type[s] != 9 && type[s] != 19) continue
			if (dest[s] >= count) die("invalid relocation target section")
			if (names[dest[s]] ~ /^\.BTF/) die("relocations target BTF")
		}
		if (role == "base" && !found) die("base ELF has no BTF")
		printf "%.0f %.0f\n", btfpos, btfsize
	}' "$work/sections" "$work/names") || fail "$role section boundary rejected"
	printf '%s %s %s %s\n' "$class" "$endian" "$machine" "$btf"
}

module_info=$(elf_info "$module" module)
oracle_info=$(elf_info "$oracle" oracle)
base_info=$(elf_info "$base" base)
set -- $module_info
identity="$1 $2 $3"
endian=$2
set -- $oracle_info
[ "$identity" = "$1 $2 $3" ] || fail "module/oracle ELF class, endian or machine mismatch"
set -- $base_info
[ "$identity" = "$1 $2 $3" ] || fail "module/base ELF class, endian or machine mismatch"
base_btf_offset=$4 base_btf_size=$5
od -An -v -t u1 -j "$base_btf_offset" -N 24 "$base" > "$work/base-btf-header"
raw_size=$(wc -c < "$raw")
[ "$raw_size" -le 16777216 ] || fail "oversized metadata BTF"
od -An -v -t u1 "$raw" > "$work/raw"
awk -v le="$((2 - endian))" -v total="$raw_size" -v base_size="$base_btf_size" '
function die(s) { print "check-module-btf-oracle: " s > "/dev/stderr"; exit 1 }
function u(p,    i, v) {
	v = 0
	for (i = 0; i < 4; i++) v = v * 256 + b[p + (le ? 3 - i : i)]
	return v
}
function header(size) {
	if (n < 24 || b[0] != (le ? 159 : 235) || b[1] != (le ? 235 : 159) ||
	    b[2] != 1 || b[3] || u(4) != 24)
		die("unsupported/truncated BTF v1 header or endian mismatch")
	if (u(8) || !u(12) || u(12) % 4 || u(16) != u(12) || 24 + u(12) + u(20) != size)
		die("invalid/noncontiguous BTF section bounds")
}
function name(off) {
	if (off >= base_strings + strings) die("BTF name offset outside strings")
	# Offsets into base strings are resolved by libbpf against the same base.
	if (off < base_strings) return
	for (q = strstart + off - base_strings; q < n && b[q]; q++) {}
	if (q == n) die("unterminated BTF name")
}
function ref(id, nonzero) {
	if ((nonzero && !id) || id > 1048575) die("invalid BTF type reference")
}
FNR == NR { for (i = 1; i <= NF; i++) b[n++] = $i; next }
FNR == 1 {
	header(base_size)
	base_strings = u(20)
	if (!base_strings) die("base BTF has no strings")
	for (i in b) delete b[i]
	n = 0
}
{ for (i = 1; i <= NF; i++) b[n++] = $i }
END {
	if (!base_strings || n != total) die("truncated BTF input")
	header(total)
	end = 24 + u(12); strings = u(20); strstart = end
	if (strings && b[n - 1]) die("unterminated BTF strings")
	for (p = 24; p < end; p += len) {
		if (end - p < 12) die("truncated BTF type header")
		nm = u(p); info = u(p + 4); value = u(p + 8)
		# BTF_INFO_VLEN in this tree (include/uapi/linux/btf.h) uses 24 bits.
		kind = int(info / 16777216) % 128; vlen = info % 16777216; flag = int(info / 2147483648)
		name(nm); len = 12
		if (kind == 1) len += 4
		else if (kind == 3) len += 12
		else if (kind == 4 || kind == 5 || kind == 19) len += vlen * 12
		else if (kind == 6) len += vlen * 8
		else if (kind != 2 && kind != 7 && kind != 8 && kind != 9 && kind != 10 && kind != 11 && kind != 16)
			die("address-bound or unsupported BTF kind " kind)
		if (len > end - p) die("truncated BTF kind payload")
		if (kind != 4 && kind != 5 && kind != 6 && kind != 19 && vlen)
			die("unexpected BTF vlen")
		if (kind != 4 && kind != 5 && kind != 6 && kind != 7 && kind != 19 && flag)
			die("unexpected BTF kind_flag")
		if (kind == 2 || (kind >= 8 && kind <= 11)) ref(value, 0)
		if (kind == 2 && nm) die("named BTF pointer")
		if (kind == 7 && value) die("invalid BTF forward declaration")
		if (kind == 1) {
			enc = u(p + 12); bits = enc % 256; bitoff = int(enc / 256) % 256; encoding = int(enc / 16777216)
			if (!nm || !value || value > 16 || !bits || bits + bitoff > value * 8 ||
			    int(enc / 65536) % 256 || (encoding != 0 && encoding != 1 && encoding != 2 && encoding != 4))
				die("invalid BTF integer")
		}
		if (kind == 3) {
			if (nm || value) die("invalid BTF array header")
			ref(u(p + 12), 1); ref(u(p + 16), 1)
		}
		if (kind == 4 || kind == 5) {
			last = 0
			for (j = 0; j < vlen; j++) {
				at = p + 12 + j * 12; name(u(at)); ref(u(at + 4), 1)
				bits = u(at + 8); width = flag ? int(bits / 16777216) : 0
				if (flag) bits %= 16777216
				if ((kind == 5 && bits) || bits < last || bits + width > value * 8)
					die("invalid BTF member offset")
				last = bits
			}
		}
		if (kind == 6 || kind == 19) {
			if (value != 1 && value != 2 && value != 4 && value != 8) die("invalid BTF enum size")
			for (j = 0; j < vlen; j++) name(u(p + 12 + j * (kind == 6 ? 8 : 12)))
		}
		if (kind == 16 && (!nm || (value != 2 && value != 4 && value != 8 && value != 12 && value != 16)))
			die("invalid BTF float")
		types++
	}
	if (p != end || !types) die("empty or malformed BTF type graph")
}' "$work/base-btf-header" "$work/raw" || fail "metadata BTF graph rejected"
