// SPDX-License-Identifier: GPL-2.0-only
/*
 * Some debug functions
 *
 * MIPS floating point support
 *
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 *
 *  Nov 7, 2000
 *  Modified to build and operate in Linux kernel environment.
 *
 *  Kevin D. Kissell, kevink@mips.com and Carsten Langgaard, carstenl@mips.com
 *  Copyright (C) 2000 MIPS Technologies, Inc. All rights reserved.
 */

// Dependencies supplied by the surrounding translation unit:
// ieee754 types, constants, classification functions, field macros, and printk.

extern "C" {
    fn printk(fmt: *const core::ffi::c_char, ...) -> core::ffi::c_int;
}

pub unsafe fn ieee754dp_dump(mut m: *mut core::ffi::c_char, mut x: union ieee754dp) -> union ieee754dp {
    let mut i: i32;

    printk(b"%s\0".as_ptr() as *const core::ffi::c_char, m);
    printk(
        b"<%08x,%08x>\n\0".as_ptr() as *const core::ffi::c_char,
        (x.bits >> 32) as u32,
        x.bits as u32,
    );
    printk(b"\t=\0".as_ptr() as *const core::ffi::c_char);
    match ieee754dp_class(x) {
        IEEE754_CLASS_QNAN | IEEE754_CLASS_SNAN => {
            printk(b"Nan %c\0".as_ptr() as *const core::ffi::c_char, if DPSIGN!(x) { b'-' } else { b'+' });
            i = DP_FBITS - 1;
            while i >= 0 {
                printk(b"%c\0".as_ptr() as *const core::ffi::c_char, if DPMANT!(x) & DP_MBIT!(i) != 0 { b'1' } else { b'0' });
                i -= 1;
            }
        }
        IEEE754_CLASS_INF => printk(b"%cInfinity\0".as_ptr() as *const core::ffi::c_char, if DPSIGN!(x) { b'-' } else { b'+' }),
        IEEE754_CLASS_ZERO => printk(b"%cZero\0".as_ptr() as *const core::ffi::c_char, if DPSIGN!(x) { b'-' } else { b'+' }),
        IEEE754_CLASS_DNORM | IEEE754_CLASS_NORM => {
            printk(if ieee754dp_class(x) == IEEE754_CLASS_DNORM { b"%c0.\0" } else { b"%c1.\0" }.as_ptr() as *const core::ffi::c_char, if DPSIGN!(x) { b'-' } else { b'+' });
            i = DP_FBITS - 1;
            while i >= 0 {
                printk(b"%c\0".as_ptr() as *const core::ffi::c_char, if DPMANT!(x) & DP_MBIT!(i) != 0 { b'1' } else { b'0' });
                i -= 1;
            }
            printk(b"e%d\0".as_ptr() as *const core::ffi::c_char, DPBEXP!(x) - DP_EBIAS);
        }
        _ => printk(b"Illegal/Unknown IEEE754 value class\0".as_ptr() as *const core::ffi::c_char),
    };
    printk(b"\n\0".as_ptr() as *const core::ffi::c_char);
    x
}

pub unsafe fn ieee754sp_dump(mut m: *mut core::ffi::c_char, mut x: union ieee754sp) -> union ieee754sp {
    let mut i: i32;

    printk(b"%s=\0".as_ptr() as *const core::ffi::c_char, m);
    printk(b"<%08x>\n\0".as_ptr() as *const core::ffi::c_char, x.bits as u32);
    printk(b"\t=\0".as_ptr() as *const core::ffi::c_char);
    match ieee754sp_class(x) {
        IEEE754_CLASS_QNAN | IEEE754_CLASS_SNAN => {
            printk(b"Nan %c\0".as_ptr() as *const core::ffi::c_char, if SPSIGN!(x) { b'-' } else { b'+' });
            i = SP_FBITS - 1;
            while i >= 0 {
                printk(b"%c\0".as_ptr() as *const core::ffi::c_char, if SPMANT!(x) & SP_MBIT!(i) != 0 { b'1' } else { b'0' });
                i -= 1;
            }
        }
        IEEE754_CLASS_INF => printk(b"%cInfinity\0".as_ptr() as *const core::ffi::c_char, if SPSIGN!(x) { b'-' } else { b'+' }),
        IEEE754_CLASS_ZERO => printk(b"%cZero\0".as_ptr() as *const core::ffi::c_char, if SPSIGN!(x) { b'-' } else { b'+' }),
        IEEE754_CLASS_DNORM | IEEE754_CLASS_NORM => {
            printk(if ieee754sp_class(x) == IEEE754_CLASS_DNORM { b"%c0.\0" } else { b"%c1.\0" }.as_ptr() as *const core::ffi::c_char, if SPSIGN!(x) { b'-' } else { b'+' });
            i = SP_FBITS - 1;
            while i >= 0 {
                printk(b"%c\0".as_ptr() as *const core::ffi::c_char, if SPMANT!(x) & SP_MBIT!(i) != 0 { b'1' } else { b'0' });
                i -= 1;
            }
            printk(b"e%d\0".as_ptr() as *const core::ffi::c_char, SPBEXP!(x) - SP_EBIAS);
        }
        _ => printk(b"Illegal/Unknown IEEE754 value class\0".as_ptr() as *const core::ffi::c_char),
    };
    printk(b"\n\0".as_ptr() as *const core::ffi::c_char);
    x
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
