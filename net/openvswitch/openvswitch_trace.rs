// SPDX-License-Identifier: GPL-2.0
// bug in tracepoint.h, it should include this
//
// The Linux kernel module dependency is supplied externally.
//
// sparse isn't too happy with all macros...
// The C source defines CREATE_TRACE_POINTS and includes openvswitch_trace.h
// unless __CHECKER__ is defined. The corresponding trace-point declarations
// are supplied by the surrounding Rust build.
#[cfg(not(__CHECKER__))]
pub const CREATE_TRACE_POINTS: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
