// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * double precision: common utilities
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// The declarations and constants below are supplied by ieee754dp.h.

extern "C" {
    fn ieee754_clearcx();
    fn ieee754dp_zero(sign: i32) -> ieee754dp;
    fn ieee754dp_one(sign: i32) -> ieee754dp;
    fn ieee754dp_ten(sign: i32) -> ieee754dp;
    fn builddp(sign: i32, exponent: i32, fraction: u64) -> ieee754dp;
}

#[repr(C)]
pub union ieee754dp {
    pub bits: u64,
}

// Values defined by ieee754dp.h.
extern "C" {
    static DP_FBITS: i32;
    static DP_EBIAS: i32;
    static DP_HIDDEN_BIT: u64;
}

pub unsafe fn ieee754dp_fint(x: i32) -> ieee754dp {
    let mut xm: u64;
    let mut xe: i32;
    let xs: i32;

    ieee754_clearcx();

    if x == 0 {
        return ieee754dp_zero(0);
    }
    if x == 1 || x == -1 {
        return ieee754dp_one((x < 0) as i32);
    }
    if x == 10 || x == -10 {
        return ieee754dp_ten((x < 0) as i32);
    }

    xs = (x < 0) as i32;
    if xs != 0 {
        if x == i32::MIN {
            // The maximum negative integer cannot be safely negated.
            xm = (1u64 << 31);
        } else {
            xm = (-(x as i64)) as u64;
        }
    } else {
        xm = x as u64;
    }

    /* normalize - result can never be inexact or overflow */
    xe = DP_FBITS;
    while (xm >> DP_FBITS) == 0 {
        xm <<= 1;
        xe -= 1;
    }
    builddp(xs, xe + DP_EBIAS, xm & !DP_HIDDEN_BIT)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
