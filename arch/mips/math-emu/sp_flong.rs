// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * single precision
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependency declarations supplied by ieee754sp.h.
#[repr(C)]
pub union ieee754sp {
    pub bits: u32,
}

extern "C" {
    fn ieee754_clearcx();
    fn ieee754sp_zero(sign: i32) -> ieee754sp;
    fn ieee754sp_one(sign: i32) -> ieee754sp;
    fn ieee754sp_ten(sign: i32) -> ieee754sp;
    fn ieee754sp_format(sign: i32, exponent: i32, mantissa: u64) -> ieee754sp;
}

const SP_FBITS: i32 = 23;

pub unsafe fn ieee754sp_flong(x: i64) -> ieee754sp {
    let mut xm: u64; // <--- need 64-bit mantissa temp
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
        if x == i64::MIN {
            xm = 1u64 << 63; // max neg can't be safely negated
        } else {
            xm = (-x) as u64;
        }
    } else {
        xm = x as u64;
    }
    xe = SP_FBITS + 3;

    if (xm >> (SP_FBITS + 1 + 3)) != 0 {
        /* shunt out overflow bits
         */
        while (xm >> (SP_FBITS + 1 + 3)) != 0 {
            // SPXSRSX1(): shift right one place, retaining the sticky bit,
            // and adjust the exponent.
            let sticky = xm & 1;
            xm >>= 1;
            xm |= sticky;
            xe += 1;
        }
    } else {
        /* normalize in grs extended single precision */
        while (xm >> (SP_FBITS + 3)) == 0 {
            xm <<= 1;
            xe -= 1;
        }
    }
    ieee754sp_format(xs, xe, xm)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
