// SPDX-License-Identifier: GPL-2.0
//
// C dependencies from the original source:
// #include <stdlib.h>
// #include <linux/kernel.h>
// #include "hist.h"
// #include "c2c.h"

use core::ffi::c_void;

#[repr(C)]
pub struct perf_hpp_fmt {
    _private: [u8; 0],
}

/*
 * The complete definition is supplied by c2c.h in the original repository.
 * This file uses only the embedded perf_hpp_fmt field named `fmt` and the
 * comparable `dim` field.
 */
#[repr(C)]
pub struct c2c_fmt {
    pub fmt: perf_hpp_fmt,
    pub dim: *mut c_void,
}

extern "C" {
    fn free(ptr: *mut c_void);
}

#[inline]
unsafe fn c2c_fmt_from_fmt(fmt: *mut perf_hpp_fmt) -> *mut c2c_fmt {
    /*
     * Original C:
     * container_of(fmt, struct c2c_fmt, fmt)
     *
     * The member used for container_of is the first field represented in this
     * translation, so the containing pointer has the same address.
     */
    fmt as *mut c2c_fmt
}

#[no_mangle]
pub unsafe extern "C" fn c2c_fmt_free(fmt: *mut perf_hpp_fmt) {
    let c2c_fmt: *mut c2c_fmt;

    c2c_fmt = c2c_fmt_from_fmt(fmt);
    free(c2c_fmt as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn c2c_fmt_equal(a: *mut perf_hpp_fmt, b: *mut perf_hpp_fmt) -> bool {
    let c2c_a: *mut c2c_fmt = c2c_fmt_from_fmt(a);
    let c2c_b: *mut c2c_fmt = c2c_fmt_from_fmt(b);

    (*c2c_a).dim == (*c2c_b).dim
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
