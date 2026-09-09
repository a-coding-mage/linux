// SPDX-License-Identifier: GPL-2.0-only
/*
 * IEEE754 floating point arithmetic
 * single precision: CLASS.f
 * FPR[fd] = class(FPR[fs])
 *
 * MIPS floating point support
 * Copyright (C) 2015 Imagination Technologies, Ltd.
 * Author: Markos Chandras <markos.chandras@imgtec.com>
 */

// The following type, component extraction helper, class constants, and
// error logger are supplied by ieee754sp.h and the surrounding implementation.
use core::ffi::{c_char, c_int};

extern "C" {
    fn ieee754sp_components(x: ieee754sp) -> (c_int, bool);
    fn pr_err(fmt: *const c_char, ...);
}

#[repr(C)]
pub union ieee754sp {
    pub bits: u32,
}

extern "C" {
    static IEEE754_CLASS_SNAN: c_int;
    static IEEE754_CLASS_QNAN: c_int;
    static IEEE754_CLASS_INF: c_int;
    static IEEE754_CLASS_NORM: c_int;
    static IEEE754_CLASS_DNORM: c_int;
    static IEEE754_CLASS_ZERO: c_int;
}

#[no_mangle]
pub unsafe extern "C" fn ieee754sp_2008class(x: ieee754sp) -> c_int {
    // COMPXSP;
    // EXPLODEXSP;
    let (xc, xs) = ieee754sp_components(x);

    /*
     * 10 bit mask as follows:
     *
     * bit0 = SNAN
     * bit1 = QNAN
     * bit2 = -INF
     * bit3 = -NORM
     * bit4 = -DNORM
     * bit5 = -ZERO
     * bit6 = INF
     * bit7 = NORM
     * bit8 = DNORM
     * bit9 = ZERO
     */

    if xc == IEEE754_CLASS_SNAN {
        return 0x01;
    }
    if xc == IEEE754_CLASS_QNAN {
        return 0x02;
    }
    if xc == IEEE754_CLASS_INF {
        return 0x04 << if xs { 0 } else { 4 };
    }
    if xc == IEEE754_CLASS_NORM {
        return 0x08 << if xs { 0 } else { 4 };
    }
    if xc == IEEE754_CLASS_DNORM {
        return 0x10 << if xs { 0 } else { 4 };
    }
    if xc == IEEE754_CLASS_ZERO {
        return 0x20 << if xs { 0 } else { 4 };
    }

    pr_err(c"Unknown class: %d\n".as_ptr(), xc);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
