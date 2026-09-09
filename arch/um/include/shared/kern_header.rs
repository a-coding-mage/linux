/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2000 Jeff Dike (jdike@karaya.com)
 */

/* These are all user-mode things which are convenient to call directly
 * from kernel code and for which writing a wrapper is too much of a pain.
 * The regular include files can't be included because this file is included
 * only into kernel code, and user-space includes conflict with kernel
 * includes.
 */

use ::core::ffi::{c_char, c_int, c_void};

unsafe extern "C" {
    pub fn printf(fmt: *const c_char, ...) -> c_int;
    pub fn sbrk(increment: c_int) -> *mut c_void;
    pub fn pause() -> c_int;
    pub fn exit(status: c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
