// SPDX-License-Identifier: GPL-2.0
/*
 *	arch/alpha/lib/srm_puts.c
 */

use core::ffi::{c_char, c_int, c_long, c_ulong};

extern "C" {
    static callback_init_done: c_int;
    fn callback_puts(index: c_ulong, string: *const c_char, len: c_long) -> c_long;
}

pub unsafe fn srm_puts(mut str_: *const c_char, len: c_long) -> c_long {
    let mut remaining: c_long;
    let mut written: c_long;

    if callback_init_done == 0 {
        return len;
    }

    remaining = len;
    while remaining > 0 {
        written = callback_puts(0, str_, remaining);
        written = ((written as c_ulong) & 0xffff_ffff) as c_long;
        str_ = str_.add(written as usize);
        remaining -= written;
    }
    len
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
