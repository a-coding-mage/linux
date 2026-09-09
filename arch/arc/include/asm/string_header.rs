/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * vineetg: May 2011
 *  -We had half-optimised memset/memcpy, got better versions of those
 *  -Added memcmp, strchr, strcpy, strcmp, strlen
 *
 * Amit Bhor: Codito Technologies 2004
 */

// Source header guard: _ASM_ARC_STRING_H
// C dependency: <linux/types.h> supplies __kernel_size_t.

pub const __HAVE_ARCH_MEMSET: bool = true;
pub const __HAVE_ARCH_MEMCPY: bool = true;
pub const __HAVE_ARCH_MEMCMP: bool = true;
pub const __HAVE_ARCH_STRCHR: bool = true;
pub const __HAVE_ARCH_STRCPY: bool = true;
pub const __HAVE_ARCH_STRCMP: bool = true;
pub const __HAVE_ARCH_STRLEN: bool = true;

unsafe extern "C" {
    pub fn memset(ptr: *mut core::ffi::c_void, c: core::ffi::c_int, size: __kernel_size_t)
        -> *mut core::ffi::c_void;
    pub fn memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        size: __kernel_size_t,
    ) -> *mut core::ffi::c_void;
    pub fn memzero(ptr: *mut core::ffi::c_void, n: __kernel_size_t);
    pub fn memcmp(
        lhs: *const core::ffi::c_void,
        rhs: *const core::ffi::c_void,
        size: __kernel_size_t,
    ) -> core::ffi::c_int;
    pub fn strchr(s: *const core::ffi::c_char, c: core::ffi::c_int)
        -> *mut core::ffi::c_char;
    pub fn strcpy(
        dest: *mut core::ffi::c_char,
        src: *const core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
    pub fn strcmp(
        cs: *const core::ffi::c_char,
        ct: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn strlen(s: *const core::ffi::c_char) -> __kernel_size_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
