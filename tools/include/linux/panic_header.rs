/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies translated as external declarations:
// <stdarg.h>, <stdio.h>, and <stdlib.h>.
//
// The C source defines a static inline variadic function. A direct Rust
// definition of a C-variadic function requires Rust's c_variadic support in the
// including crate.

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut stderr: *mut FILE;

    pub fn vfprintf(stream: *mut FILE, format: *const c_char, arg: core::ffi::VaList) -> c_int;

    pub fn exit(status: c_int) -> !;
}

pub unsafe extern "C" fn panic(fmt: *const c_char, mut argp: ...) {
    unsafe {
        vfprintf(stderr, fmt, argp.as_va_list());
        exit(-1);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
