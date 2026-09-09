// SPDX-License-Identifier: GPL-2.0
/*
 * Function used during the kprobe self test. This function is in a separate
 * compile unit so it can be compile with CC_FLAGS_FTRACE to ensure that it
 * can be probed by the selftests.
 */
extern "C" {
    pub fn kprobe_trace_selftest_target(
        a1: ::core::ffi::c_int,
        a2: ::core::ffi::c_int,
        a3: ::core::ffi::c_int,
        a4: ::core::ffi::c_int,
        a5: ::core::ffi::c_int,
        a6: ::core::ffi::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
