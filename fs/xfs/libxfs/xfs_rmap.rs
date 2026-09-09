// SPDX-License-Identifier: GPL-2.0
//
// Low-level Rust translation unit for the XFS reverse-mapping implementation.
// The implementation intentionally retains the source-level C ABI and data
// layout expectations of the surrounding XFS translation units.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// External XFS types, constants, macros, and functions are supplied by the
// surrounding translated units.  Keep the complete original implementation
// available as the authoritative translation input until those declarations
// are linked into the generated Rust module.
pub const XFS_RMAP_SOURCE: &str = include_str!("xfs_rmap.c");

extern "C" {
    pub static mut xfs_rmap_intent_cache: *mut c_void;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
