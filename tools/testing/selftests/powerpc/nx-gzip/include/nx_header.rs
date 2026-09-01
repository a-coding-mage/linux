/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2020 IBM Corp.
 *
 */

use core::ffi::{c_char, c_int, c_void};

pub const NX_FUNC_COMP_842: c_int = 1;
pub const NX_FUNC_COMP_GZIP: c_int = 2;

/* C __aligned(x) macro maps to Rust repr(align(x)) where used. */

#[repr(C)]
pub struct nx842_func_args {
    pub use_crc: bool,
    pub decompress: bool, /* true decompress; false compress */
    pub move_data: bool,
    pub timeout: c_int, /* seconds */
}

#[repr(C)]
pub struct nxbuf_t {
    pub len: c_int,
    pub buf: *mut c_char,
}

unsafe extern "C" {
    /* @function should be EFT (aka 842), GZIP etc */
    pub fn nx_function_begin(function: c_int, pri: c_int) -> *mut c_void;

    pub fn nx_function(handle: *mut c_void, in_: *mut nxbuf_t, out: *mut nxbuf_t, arg: *mut c_void)
        -> c_int;

    pub fn nx_function_end(handle: *mut c_void) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
