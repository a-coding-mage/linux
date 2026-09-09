// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * double precision: common utilities
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependency declarations and the XDPSRSX1 macro are supplied by ieee754dp.h.

pub unsafe fn ieee754dp_flong(x: i64) -> ieee754dp {
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
        if x == i64::MIN {
            xm = 1u64 << 63; // max neg can't be safely negated
        } else {
            xm = (-(x as i128)) as u64;
        }
    } else {
        xm = x as u64;
    }

    // normalize
    xe = DP_FBITS + 3;
    if (xm >> (DP_FBITS + 1 + 3)) != 0 {
        // shunt out overflow bits
        while (xm >> (DP_FBITS + 1 + 3)) != 0 {
            XDPSRSX1!();
        }
    } else {
        // normalize in grs extended double precision
        while (xm >> (DP_FBITS + 3) == 0) {
            xm <<= 1;
            xe -= 1;
        }
    }

    ieee754dp_format(xs, xe, xm)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
