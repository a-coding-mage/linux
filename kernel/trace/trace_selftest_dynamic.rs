// SPDX-License-Identifier: GPL-2.0
//
// C dependencies: <linux/compiler.h> and "trace.h" provide the surrounding
// build configuration and any macro-based symbol naming used by this file.

#[inline(never)]
pub extern "C" fn DYN_FTRACE_TEST_NAME() -> ::core::ffi::c_int {
    /* used to call mcount */
    0
}

#[inline(never)]
pub extern "C" fn DYN_FTRACE_TEST_NAME2() -> ::core::ffi::c_int {
    /* used to call mcount */
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
