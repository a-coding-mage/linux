// SPDX-License-Identifier: GPL-2.0
/*
 * Artificial memory access program for testing DAMON.
 */

use core::ffi::{c_char, c_int, c_long, c_void};

type clock_t = c_long;

const CLOCKS_PER_SEC: clock_t = 1_000_000;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum access_mode {
    ACCESS_MODE_ONCE,
    ACCESS_MODE_REPEAT,
}

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn atoi(nptr: *const c_char) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn clock() -> clock_t;
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let regions: *mut *mut c_char;
    let mut start_clock: clock_t;
    let nr_regions: c_int;
    let sz_region: c_int;
    let access_time_ms: c_int;
    let mut mode: access_mode = access_mode::ACCESS_MODE_ONCE;

    let mut i: c_int;

    if argc < 4 {
        printf(
            c"Usage: %s <number> <size (bytes)> <time (ms)> [mode]\n".as_ptr(),
            *argv.offset(0),
        );
        return -1;
    }

    nr_regions = atoi(*argv.offset(1));
    sz_region = atoi(*argv.offset(2));
    access_time_ms = atoi(*argv.offset(3));

    if argc > 4 && strcmp(*argv.offset(4), c"repeat".as_ptr()) == 0 {
        mode = access_mode::ACCESS_MODE_REPEAT;
    }

    regions = malloc(
        core::mem::size_of::<*mut c_char>().wrapping_mul(nr_regions as usize),
    ) as *mut *mut c_char;
    i = 0;
    while i < nr_regions {
        *regions.offset(i as isize) = malloc(sz_region as usize) as *mut c_char;
        i += 1;
    }

    loop {
        i = 0;
        while i < nr_regions {
            start_clock = clock();
            while (clock() - start_clock) * 1000 / CLOCKS_PER_SEC < access_time_ms as clock_t {
                memset(
                    *regions.offset(i as isize) as *mut c_void,
                    i,
                    sz_region as usize,
                );
            }
            i += 1;
        }

        if mode != access_mode::ACCESS_MODE_REPEAT {
            break;
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
