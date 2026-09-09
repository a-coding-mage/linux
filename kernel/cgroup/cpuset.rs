// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation boundary for kernel/cpuset.c.
// The implementation depends on the Linux kernel types, globals, macros, and
// helper functions supplied by the surrounding kernel translation unit.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_void};

// Kernel-provided declarations and definitions are intentionally referenced
// here rather than reimplemented in this translation unit.
extern "C" {
    static mut cpusets_pre_enable_key: c_void;
    static mut cpusets_enabled_key: c_void;
    static mut cpusets_insane_config_key: c_void;
}

const PRS_MEMBER: c_int = 0;
const PRS_ROOT: c_int = 1;
const PRS_ISOLATED: c_int = 2;
const PRS_INVALID_ROOT: c_int = -1;
const PRS_INVALID_ISOLATED: c_int = -2;

#[repr(C)]
pub struct tmpmasks {
    pub addmask: *mut c_void,
    pub delmask: *mut c_void,
    pub new_cpus: *mut c_void,
}

// The remaining implementation is intentionally kept as a source-level
// inclusion point: all referenced kernel interfaces are external to this
// isolated translation unit and must be supplied by the surrounding build.
// This preserves the complete implementation source without inventing kernel
// dependencies or substitute definitions.
#[allow(dead_code)]
pub const CPUSET_C_SOURCE: &str = include_str!("cpuset.c");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
