// SPDX-License-Identifier: GPL-2.0
/*
 * Artificial memory access program for testing DAMON.
 *
 * Receives number of regions and size of each region from user.  Allocate the
 * regions and repeatedly access even numbered (starting from zero) regions.
 */

use core::ffi::{c_char, c_int, c_void};

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn atoi(nptr: *const c_char) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut regions: *mut *mut c_char;
    let nr_regions: c_int;
    let sz_region: c_int;
    let mut i: c_int;

    if argc != 3 {
        printf(
            b"Usage: %s <number> <size (bytes)>\n\0".as_ptr() as *const c_char,
            *argv.offset(0),
        );
        return -1;
    }

    nr_regions = atoi(*argv.offset(1));
    sz_region = atoi(*argv.offset(2));

    regions = malloc(core::mem::size_of::<*mut c_char>() * nr_regions as usize) as *mut *mut c_char;
    i = 0;
    while i < nr_regions {
        *regions.offset(i as isize) = malloc(sz_region as usize) as *mut c_char;
        i += 1;
    }

    loop {
        i = 0;
        while i < nr_regions {
            if i % 2 == 0 {
                memset(
                    *regions.offset(i as isize) as *mut c_void,
                    i,
                    sz_region as usize,
                );
            }
            i += 1;
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
