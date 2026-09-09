// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * single precision square root
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// The C source includes ieee754sp.h. Its types, macros, constants, globals,
// and external functions are supplied by the surrounding implementation.

pub unsafe fn ieee754sp_sqrt(mut x: ieee754sp) -> ieee754sp
{
    let mut ix: i32;
    let mut s: i32;
    let mut q: i32;
    let mut m: i32;
    let mut t: i32;
    let mut i: i32;
    let mut r: u32;

    // COMPXSP;
    // EXPLODEXSP;
    ieee754_clearcx();
    // FLUSHXSP;

    /* x == INF or NAN? */
    match xc {
        IEEE754_CLASS_SNAN => return ieee754sp_nanxcpt(x),

        IEEE754_CLASS_QNAN => {
            /* sqrt(Nan) = Nan */
            return x;
        }

        IEEE754_CLASS_ZERO => {
            /* sqrt(0) = 0 */
            return x;
        }

        IEEE754_CLASS_INF => {
            if xs {
                /* sqrt(-Inf) = Nan */
                ieee754_setcx(IEEE754_INVALID_OPERATION);
                return ieee754sp_indef();
            }
            /* sqrt(+Inf) = Inf */
            return x;
        }

        IEEE754_CLASS_DNORM | IEEE754_CLASS_NORM => {
            if xs {
                /* sqrt(-x) = Nan */
                ieee754_setcx(IEEE754_INVALID_OPERATION);
                return ieee754sp_indef();
            }
        }

        _ => {}
    }

    ix = x.bits as i32;

    /* normalize x */
    m = ix >> 23;
    if m == 0 {       /* subnormal x */
        i = 0;
        while (ix & 0x00800000) == 0 {
            i += 1;
            ix <<= 1;
        }
        m -= i - 1;
    }
    m -= 127;          /* unbias exponent */
    ix = (ix & 0x007fffff) | 0x00800000;
    if (m & 1) != 0 {  /* odd m, double x to make it even */
        ix += ix;
    }
    m >>= 1;           /* m = [m/2] */

    /* generate sqrt(x) bit by bit */
    ix += ix;
    s = 0;
    q = 0;             /* q = sqrt(x) */
    r = 0x01000000;    /* r = moving bit from right to left */

    while r != 0 {
        t = s + r as i32;
        if t <= ix {
            s = t + r as i32;
            ix -= t;
            q += r as i32;
        }
        ix += ix;
        r >>= 1;
    }

    if ix != 0 {
        ieee754_setcx(IEEE754_INEXACT);
        match ieee754_csr.rm {
            FPU_CSR_RU => {
                q += 2;
            }
            FPU_CSR_RN => {
                q += q & 1;
            }
            _ => {}
        }
    }
    ix = (q >> 1) + 0x3f000000;
    ix += m << 23;
    x.bits = ix as _;
    x
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
