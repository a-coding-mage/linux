/* SPDX-License-Identifier: GPL-2.0-only */

// Corresponds to the C `asmlinkage` calling-convention annotation.
extern "C" {
    pub fn syscall_trace_enter() -> ::core::ffi::c_int;
    pub fn syscall_trace_leave();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
