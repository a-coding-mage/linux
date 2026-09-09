/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (c) 1994, 95, 96, 97, 98, 2000, 01 Ralf Baechle
 * Copyright (c) 2000 by Silicon Graphics, Inc.
 * Copyright (c) 2001 MIPS Technologies, Inc.
 */

// __HAVE_ARCH_MEMSET
pub const __HAVE_ARCH_MEMSET: bool = true;
unsafe extern "C" {
    pub fn memset(__s: *mut core::ffi::c_void, __c: core::ffi::c_int, __count: usize)
        -> *mut core::ffi::c_void;
}

// __HAVE_ARCH_MEMCPY
pub const __HAVE_ARCH_MEMCPY: bool = true;
unsafe extern "C" {
    pub fn memcpy(
        __to: *mut core::ffi::c_void,
        __from: *const core::ffi::c_void,
        __n: usize,
    ) -> *mut core::ffi::c_void;
}

// __HAVE_ARCH_MEMMOVE
pub const __HAVE_ARCH_MEMMOVE: bool = true;
unsafe extern "C" {
    pub fn memmove(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: usize,
    ) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
