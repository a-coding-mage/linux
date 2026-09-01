/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright(c) 2016-20 Intel Corporation.
 */

use core::ffi::{c_char, c_void};

pub type off_t = i64;
pub type size_t = usize;

pub const ENCL_HEAP_SIZE_DEFAULT: usize = 4096;

#[repr(C)]
pub struct encl_segment {
    pub src: *mut c_void,
    pub offset: off_t,
    pub size: size_t,
    pub prot: ::core::ffi::c_uint,
    pub flags: ::core::ffi::c_uint,
    pub measure: bool,
}

#[repr(C)]
pub struct encl {
    pub fd: ::core::ffi::c_int,
    pub bin: *mut c_void,
    pub bin_size: off_t,
    pub src: *mut c_void,
    pub src_size: size_t,
    pub encl_size: size_t,
    pub encl_base: off_t,
    pub nr_segments: ::core::ffi::c_uint,
    pub segment_tbl: *mut encl_segment,
    pub secs: sgx_secs,
    pub sigstruct: sgx_sigstruct,
}

unsafe extern "C" {
    pub static mut sign_key: u8;
    pub static mut sign_key_end: u8;

    pub fn encl_delete(ctx: *mut encl);
    pub fn encl_load(path: *const c_char, encl: *mut encl, heap_size: ::core::ffi::c_ulong) -> bool;
    pub fn encl_measure(encl: *mut encl) -> bool;
    pub fn encl_build(encl: *mut encl) -> bool;
    pub fn encl_get_entry(encl: *mut encl, symbol: *const c_char) -> u64;

    pub fn sgx_enter_enclave(
        rdi: *mut c_void,
        rsi: *mut c_void,
        rdx: ::core::ffi::c_long,
        function: u32,
        r8: *mut c_void,
        r9: *mut c_void,
        run: *mut sgx_enclave_run,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
