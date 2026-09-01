// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2015, Anton Blanchard, IBM Corp.
 */

use std::os::raw::{c_char, c_double, c_int, c_long};
use std::ptr;

// C dependencies:
// #include <sys/time.h>
// #include <stdio.h>
// #include "utils.h"

#[repr(C)]
struct timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

unsafe extern "C" {
    fn gettimeofday(tv: *mut timeval, tz: *mut core::ffi::c_void) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn test_harness(test_function: Option<unsafe extern "C" fn() -> c_int>, name: *const c_char) -> c_int;
}

unsafe fn timersub(a: *const timeval, b: *const timeval, result: *mut timeval) {
    (*result).tv_sec = (*a).tv_sec - (*b).tv_sec;
    (*result).tv_usec = (*a).tv_usec - (*b).tv_usec;
    if (*result).tv_usec < 0 {
        (*result).tv_sec -= 1;
        (*result).tv_usec += 1000000;
    }
}

unsafe extern "C" fn test_gettimeofday() -> c_int {
    let mut i: c_int;

    let mut tv_start: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut tv_end: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut tv_diff: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    unsafe {
        gettimeofday(&mut tv_start, ptr::null_mut());
    }

    i = 0;
    while i < 100000000 {
        unsafe {
            gettimeofday(&mut tv_end, ptr::null_mut());
        }
        i += 1;
    }

    unsafe {
        timersub(&tv_end, &tv_start, &mut tv_diff);

        printf(
            c"time = %.6f\n".as_ptr(),
            tv_diff.tv_sec as c_double + (tv_diff.tv_usec as c_double) * 1e-6f64,
        );
    }

    0
}

fn main() -> c_int {
    unsafe { test_harness(Some(test_gettimeofday), c"gettimeofday".as_ptr()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
