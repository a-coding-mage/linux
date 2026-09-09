/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2017 Oracle and/or its affiliates. All rights reserved.
 */

// Declarations corresponding to the Linux and SPARC headers included by the
// original C header are supplied by other translation units.

use core::ffi::{c_int, c_void};

pub type size_t = usize;
pub type s64 = i64;

pub type TItype = i128;

unsafe extern "C" {
    pub fn __memscan_zero(ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn __memscan_generic(ptr: *mut c_void, value: c_int, size: size_t) -> *mut c_void;
    pub fn __bzero(ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn VISenter(); /* Dummy prototype to suppress warning */
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    pub fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn __multi3(a: TItype, b: TItype) -> TItype;
    pub fn _mcount();
    pub fn mcount();

    pub fn __ashldi3(value: s64, shift: c_int) -> s64;
    pub fn __lshrdi3(value: s64, shift: c_int) -> s64;
    pub fn __ashrdi3(value: s64, shift: c_int) -> s64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
