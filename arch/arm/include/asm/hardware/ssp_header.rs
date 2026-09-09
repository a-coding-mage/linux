/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  ssp.h
 *
 *  Copyright (C) 2003 Russell King, All Rights Reserved.
 */

#[repr(C)]
pub struct ssp_state {
    pub cr0: ::core::ffi::c_uint,
    pub cr1: ::core::ffi::c_uint,
}

unsafe extern "C" {
    pub fn ssp_write_word(data: u16) -> ::core::ffi::c_int;
    pub fn ssp_read_word(data: *mut u16) -> ::core::ffi::c_int;
    pub fn ssp_flush() -> ::core::ffi::c_int;
    pub fn ssp_enable();
    pub fn ssp_disable();
    pub fn ssp_save_state(ssp: *mut ssp_state);
    pub fn ssp_restore_state(ssp: *mut ssp_state);
    pub fn ssp_init() -> ::core::ffi::c_int;
    pub fn ssp_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
