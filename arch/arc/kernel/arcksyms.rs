// SPDX-License-Identifier: GPL-2.0-only
/*
 * arcksyms.c - Exporting symbols not exportable from their own sources
 *
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// The C source includes <linux/module.h>, which supplies EXPORT_SYMBOL.
// The exported symbols are retained below as Rust foreign declarations and
// export markers in comments; their definitions are supplied externally.

/* libgcc functions, not part of kernel sources */
unsafe extern "C" {
    pub fn __ashldi3();
    pub fn __ashrdi3();
    pub fn __divsi3();
    pub fn __divsf3();
    pub fn __lshrdi3();
    pub fn __modsi3();
    pub fn __muldi3();
    pub fn __ucmpdi2();
    pub fn __udivsi3();
    pub fn __umodsi3();
    pub fn __cmpdi2();
    pub fn __fixunsdfsi();
    pub fn __muldf3();
    pub fn __divdf3();
    pub fn __floatunsidf();
    pub fn __floatunsisf();
    pub fn __udivdi3();
}

// EXPORT_SYMBOL(__ashldi3);
// EXPORT_SYMBOL(__ashrdi3);
// EXPORT_SYMBOL(__divsi3);
// EXPORT_SYMBOL(__divsf3);
// EXPORT_SYMBOL(__lshrdi3);
// EXPORT_SYMBOL(__modsi3);
// EXPORT_SYMBOL(__muldi3);
// EXPORT_SYMBOL(__ucmpdi2);
// EXPORT_SYMBOL(__udivsi3);
// EXPORT_SYMBOL(__umodsi3);
// EXPORT_SYMBOL(__cmpdi2);
// EXPORT_SYMBOL(__fixunsdfsi);
// EXPORT_SYMBOL(__muldf3);
// EXPORT_SYMBOL(__divdf3);
// EXPORT_SYMBOL(__floatunsidf);
// EXPORT_SYMBOL(__floatunsisf);
// EXPORT_SYMBOL(__udivdi3);

/* ARC optimised assembler routines */
unsafe extern "C" {
    pub fn memset(dest: *mut core::ffi::c_void, value: i32, count: usize) -> *mut core::ffi::c_void;
    pub fn memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        count: usize,
    ) -> *mut core::ffi::c_void;
    pub fn memcmp(
        lhs: *const core::ffi::c_void,
        rhs: *const core::ffi::c_void,
        count: usize,
    ) -> i32;
    pub fn strchr(string: *const core::ffi::c_char, character: i32) -> *mut core::ffi::c_char;
    pub fn strcpy(dest: *mut core::ffi::c_char, src: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn strcmp(lhs: *const core::ffi::c_char, rhs: *const core::ffi::c_char) -> i32;
    pub fn strlen(string: *const core::ffi::c_char) -> usize;
}

// EXPORT_SYMBOL(memset);
// EXPORT_SYMBOL(memcpy);
// EXPORT_SYMBOL(memcmp);
// EXPORT_SYMBOL(strchr);
// EXPORT_SYMBOL(strcpy);
// EXPORT_SYMBOL(strcmp);
// EXPORT_SYMBOL(strlen);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
