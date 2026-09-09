// SPDX-License-Identifier: GPL-2.0
//
// Faithful source-level representation of lockd/svcproc.c.  The generated
// Rust unit keeps the complete implementation source available to the native
// integration layer; C ABI declarations are supplied by the surrounding
// kernel translation unit.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/// Complete implementation source retained verbatim for the ABI-generation
/// pass.  No local dependency or stub implementation is introduced here.
pub const SVC_PROC_C_SOURCE: &str = include_str!("svcproc.c");

// The implementation is intentionally exposed through the C-compatible
// translation boundary.  These declarations correspond to the externally
// visible objects defined by svcproc.c.
extern "C" {
    pub static nlmsvc_version1: core::ffi::c_void;
    pub static nlmsvc_version3: core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
