/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2005 Paolo 'Blaisorblade' Giarrusso <blaisorblade@yahoo.it>
 */

// Dependency intent: generated/asm-offsets.h is included by the original C
// header and may provide build-specific declarations or constants.

use core::ffi::c_void;

extern "C" {
    pub fn uml_kmalloc(size: i32, flags: i32) -> *mut c_void;
    pub fn kfree(ptr: *const c_void);

    pub fn vmalloc_noprof(size: u64) -> *mut c_void;
    pub fn vfree(ptr: *const c_void);
}

// C macro: vmalloc(...) expands directly to vmalloc_noprof(__VA_ARGS__).
#[inline]
pub unsafe fn vmalloc(size: u64) -> *mut c_void {
    vmalloc_noprof(size)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
