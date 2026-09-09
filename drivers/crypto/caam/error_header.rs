/* SPDX-License-Identifier: GPL-2.0 */
/*
 * CAAM Error Reporting code header
 *
 * Copyright 2009-2011 Freescale Semiconductor, Inc.
 */

// Dependency supplied by desc.h in the original header.

pub const CAAM_ERROR_STR_MAX: usize = 302;

// External C types referenced by this header.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scatterlist {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn caam_strstatus(dev: *mut device, status: u32, qi_v2: bool) -> i32;

    pub fn caam_dump_sg(
        prefix_str: *const core::ffi::c_char,
        prefix_type: i32,
        rowsize: i32,
        groupsize: i32,
        sg: *mut scatterlist,
        tlen: usize,
        ascii: bool,
    );
}

#[inline]
pub unsafe fn caam_jr_strstatus(jrdev: *mut device, status: u32) -> i32 {
    unsafe { caam_strstatus(jrdev, status, false) }
}

#[inline]
pub unsafe fn caam_qi2_strstatus(qidev: *mut device, status: u32) -> i32 {
    unsafe { caam_strstatus(qidev, status, true) }
}

#[inline]
pub const fn is_mdha(algtype: u32) -> bool {
    // OP_ALG_* constants are supplied by desc.h in the original header.
    (algtype & OP_ALG_ALGSEL_MASK & !OP_ALG_ALGSEL_SUBMASK) == OP_ALG_CHA_MDHA
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
