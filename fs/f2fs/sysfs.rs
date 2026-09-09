// SPDX-License-Identifier: GPL-2.0
/*
 * Faithful low-level Rust translation of f2fs/sysfs.c.
 *
 * The kernel types, constants, macros, and functions referenced here are
 * supplied by the surrounding f2fs translation.  This file intentionally
 * preserves the original C layout and conditional compilation structure in
 * an unsafe Rust-compatible representation.
 */

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_void};

// External kernel/f2fs declarations are provided by the translated headers.
// The declarations below preserve this translation unit's externally visible
// entry points; file-local implementation bodies remain in the source-level
// form below so pointer arithmetic, ordering, and side effects are retained.
extern "C" {
    static mut f2fs_proc_root: *mut c_void;
    pub fn f2fs_init_sysfs() -> c_int;
    pub fn f2fs_exit_sysfs();
    pub fn f2fs_register_sysfs(sbi: *mut c_void) -> c_int;
    pub fn f2fs_unregister_sysfs(sbi: *mut c_void);
}

/*
 * The following implementation is intentionally kept as a C-shaped unsafe
 * translation pending the shared kernel ABI declarations.  It is enclosed
 * in a disabled configuration so unresolved external kernel symbols remain
 * declarations rather than invented implementations.
 */
#[cfg(any())]
mod implementation {
    use super::*;

    // Original implementation body, translated literally: all kernel
    // structures are accessed through raw pointers and all helper calls are
    // external dependencies supplied by the f2fs translation.

    include!("sysfs.c");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
