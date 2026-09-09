// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * single precision
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependency intent: declarations and macros are supplied by ieee754sp.h.

pub unsafe fn ieee754sp_div(
    mut x: ieee754sp,
    mut y: ieee754sp,
) -> ieee754sp {
    let mut rm: u32;
    let mut re: i32;
    let mut bm: u32;

    COMPXSP!();
    COMPYSP!();

    EXPLODEXSP!();
    EXPLODEYSP!();

    ieee754_clearcx();

    FLUSHXSP!();
    FLUSHYSP!();

    match CLPAIR!(xc, yc) {
        CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_SNAN)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_SNAN)
        | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_SNAN)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_SNAN)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_SNAN) => return ieee754sp_nanxcpt(y),

        CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_SNAN)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_DNORM)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_INF) => return ieee754sp_nanxcpt(x),

        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_QNAN) => return y,

        CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_DNORM)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_INF) => return x,

        /* Infinity handling */
        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_INF) => {
            ieee754_setcx(IEEE754_INVALID_OPERATION);
            return ieee754sp_indef();
        }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_INF) => return ieee754sp_zero(xs ^ ys),
        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_DNORM) => return ieee754sp_inf(xs ^ ys),

        /* Zero handling */
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_ZERO) => {
            ieee754_setcx(IEEE754_INVALID_OPERATION);
            return ieee754sp_indef();
        }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_ZERO) => {
            ieee754_setcx(IEEE754_ZERO_DIVIDE);
            return ieee754sp_inf(xs ^ ys);
        }
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_DNORM) => {
            return ieee754sp_zero(if xs == ys { 0 } else { 1 });
        }
        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_DNORM) => {
            SPDNORMX!();
            SPDNORMY!();
        }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_DNORM) => {
            SPDNORMY!();
        }
        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_NORM) => {
            SPDNORMX!();
        }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_NORM) => {}
    }

    assert!(xm & SP_HIDDEN_BIT != 0);
    assert!(ym & SP_HIDDEN_BIT != 0);

    /* provide rounding space */
    xm <<= 3;
    ym <<= 3;

    /* now the dirty work */
    rm = 0;
    re = xe - ye;

    bm = SP_MBIT(SP_FBITS + 2);
    while bm != 0 {
        if xm >= ym {
            xm -= ym;
            rm |= bm;
            if xm == 0 {
                break;
            }
        }
        xm <<= 1;
        bm >>= 1;
    }

    rm <<= 1;
    if xm != 0 {
        rm |= 1; /* have remainder, set sticky */
    }

    assert!(rm != 0);

    /* normalise rm to rounding precision ? */
    while (rm >> (SP_FBITS + 3)) == 0 {
        rm <<= 1;
        re -= 1;
    }

    ieee754sp_format(if xs == ys { 0 } else { 1 }, re, rm)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
