// SPDX-License-Identifier: GPL-2.0

// Dependency declarations from "trace_kprobe_selftest.h" are supplied by the
// surrounding build; no local implementation is required here.

/*
 * Function used during the kprobe self test. This function is in a separate
 * compile unit so it can be compile with CC_FLAGS_FTRACE to ensure that it
 * can be probed by the selftests.
 */
#[no_mangle]
pub extern "C" fn kprobe_trace_selftest_target(
    a1: i32,
    a2: i32,
    a3: i32,
    a4: i32,
    a5: i32,
    a6: i32,
) -> i32 {
    a1.wrapping_add(a2)
        .wrapping_add(a3)
        .wrapping_add(a4)
        .wrapping_add(a5)
        .wrapping_add(a6)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
