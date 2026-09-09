// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * single precision
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependency declarations and macro definitions are supplied by ieee754sp.h.

pub unsafe fn ieee754sp_fint(x: i32) -> ieee754sp {
    let mut xm: u32;
    let mut xe: i32;
    let xs: i32;

    ieee754_clearcx();

    if x == 0 {
        return ieee754sp_zero(0);
    }
    if x == 1 || x == -1 {
        return ieee754sp_one((x < 0) as i32);
    }
    if x == 10 || x == -10 {
        return ieee754sp_ten((x < 0) as i32);
    }

    xs = (x < 0) as i32;
    if xs != 0 {
        if x == (1_i32 << 31) {
            // max neg can't be safely negated
            xm = (1_u32 << 31);
        } else {
            xm = (-x) as u32;
        }
    } else {
        xm = x as u32;
    }
    xe = SP_FBITS + 3;

    if (xm >> (SP_FBITS + 1 + 3)) != 0 {
        /* shunt out overflow bits
         */
        while (xm >> (SP_FBITS + 1 + 3) != 0) {
            SPXSRSX1!();
        }
    } else {
        /* normalize in grs extended single precision
         */
        while (xm >> (SP_FBITS + 3) == 0) {
            xm <<= 1;
            xe -= 1;
        }
    }
    ieee754sp_format(xs, xe, xm)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
