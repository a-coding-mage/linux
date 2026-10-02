# SPDX-License-Identifier: GPL-2.0
# Decode one marked, configured C __read_mostly expansion. Deliberately accept
# only the forms present in the authoritative cache.h headers: empty or one
# section attribute. Do not drop unknown tokens or invent architecture rules.
function reject(message) {
	print "power/process state generation: " message > "/dev/stderr"
	bad = 1
}
$0 == "LUPOS_PROCESS_STATE_BEGIN" {
	if (state != 0)
		reject("duplicate or misplaced begin marker")
	state = 1
	begins++
	next
}
$0 == "LUPOS_PROCESS_STATE_END" {
	if (state != 1)
		reject("duplicate or misplaced end marker")
	state = 2
	ends++
	next
}
state == 1 {
	annotation = annotation " " $0
}
END {
	if (bad || begins != 1 || ends != 1 || state != 2) {
		reject("expected exactly one complete annotation record")
		exit 1
	}
	if (annotation !~ /^[[:space:]]*$/) {
		parts = split(annotation, quoted, "\"")
		if (parts != 3 || quoted[2] !~ /^[A-Za-z0-9_.]+$/ ||
		    quoted[1] !~ /^[[:space:]]*__attribute__[[:space:]]*\([[:space:]]*\([[:space:]]*__section__[[:space:]]*\([[:space:]]*$/ ||
		    quoted[3] !~ /^[[:space:]]*\)[[:space:]]*\)[[:space:]]*\)[[:space:]]*$/) {
			reject("unsupported __read_mostly annotation: " annotation)
			exit 1
		}
		section = quoted[2]
	}
	print "// SPDX-License-Identifier: GPL-2.0"
	print "// Generated from configured __read_mostly; do not edit."
	if (section != "") {
		print "#[link_section = \"" section "\"]"
	}
	print "#[no_mangle]"
	print "pub static mut freeze_timeout_msecs: c_uint = 20 * LUPOS_PROCESS_MSEC_PER_SEC as c_uint;"
}
