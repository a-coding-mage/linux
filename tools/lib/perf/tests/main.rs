// SPDX-License-Identifier: GPL-2.0
// Translated from C source using external declarations corresponding to:
// #include <internal/tests.h>
// #include "tests.h"

use core::ffi::{c_char, c_int};

#[no_mangle]
pub static mut tests_failed: c_int = 0;

#[no_mangle]
pub static mut tests_verbose: c_int = 0;

unsafe extern "C" {
    fn __T(name: *const c_char, ok: c_int);
    fn test_cpumap(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn test_threadmap(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn test_evlist(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn test_evsel(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

#[inline]
fn c_not(value: c_int) -> c_int {
    if value == 0 { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        __T(c"test cpumap".as_ptr(), c_not(test_cpumap(argc, argv)));
        __T(
            c"test threadmap".as_ptr(),
            c_not(test_threadmap(argc, argv)),
        );
        __T(c"test evlist".as_ptr(), c_not(test_evlist(argc, argv)));
        __T(c"test evsel".as_ptr(), c_not(test_evsel(argc, argv)));
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
