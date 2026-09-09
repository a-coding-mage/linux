// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

/*
 * Support for user memory access from kernel.  This will
 * probably be inlined for performance at some point, but
 * for ease of debug, and to a lesser degree for code size,
 * we implement here as subroutines.
 */

use core::ffi::c_void;

// Supplied by the kernel's architecture-independent headers and assembly.
const PAGE_SIZE: usize = 4096;

unsafe extern "C" {
    static empty_zero_page: u8;

    fn raw_copy_to_user(dest: *mut c_void, src: *const c_void, count: usize) -> isize;
}

/*
 * For clear_user(), exploit previously defined copy_to_user function
 * and the fact that we've got a handy zero page defined in kernel/head.S
 *
 * dczero here would be even faster.
 */
pub unsafe fn __clear_user_hexagon(dest: *mut c_void, mut count: usize) -> usize {
    let mut uncleared: isize;
    let mut dest = dest as *mut u8;

    while count > PAGE_SIZE {
        uncleared = raw_copy_to_user(
            dest as *mut c_void,
            (&raw const empty_zero_page) as *const c_void,
            PAGE_SIZE,
        );
        if uncleared != 0 {
            return count - (PAGE_SIZE - uncleared as usize);
        }
        count -= PAGE_SIZE;
        dest = dest.add(PAGE_SIZE);
    }
    if count != 0 {
        count = raw_copy_to_user(
            dest as *mut c_void,
            (&raw const empty_zero_page) as *const c_void,
            count,
        ) as usize;
    }

    count
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
