// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * double precision: common utilities
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependency declarations and build-time definitions are supplied by ieee754dp.

pub unsafe fn ieee754dp_div(
    mut x: union ieee754dp,
    mut y: union ieee754dp,
) -> union ieee754dp {
    let mut rm: u64;
    let mut re: i32;
    let mut bm: u64;

    COMPXDP!();
    COMPYDP!();

    EXPLODEXDP!();
    EXPLODEYDP!();

    ieee754_clearcx();

    FLUSHXDP!();
    FLUSHYDP!();

    match CLPAIR!(xc, yc) {
        CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_SNAN)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_SNAN)
        | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_SNAN)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_SNAN)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_SNAN) => return ieee754dp_nanxcpt(y),

        CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_SNAN)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_DNORM)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_INF) => return ieee754dp_nanxcpt(x),

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
            return ieee754dp_indef();
        }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_INF) => return ieee754dp_zero(xs ^ ys),

        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_DNORM) => return ieee754dp_inf(xs ^ ys),

        /* Zero handling */
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_ZERO) => {
            ieee754_setcx(IEEE754_INVALID_OPERATION);
            return ieee754dp_indef();
        }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_ZERO) => {
            ieee754_setcx(IEEE754_ZERO_DIVIDE);
            return ieee754dp_inf(xs ^ ys);
        }
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_DNORM) => {
            return ieee754dp_zero(if xs == ys { 0 } else { 1 });
        }

        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_DNORM) => {
            DPDNORMX!();
            DPDNORMY!();
        }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_DNORM) => {
            DPDNORMY!();
        }
        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_NORM) => {
            DPDNORMX!();
        }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_NORM) => {}
        _ => unreachable!(),
    }

    assert!(xm & DP_HIDDEN_BIT != 0);
    assert!(ym & DP_HIDDEN_BIT != 0);

    /* provide rounding space */
    xm <<= 3;
    ym <<= 3;

    /* now the dirty work */
    rm = 0;
    re = xe - ye;

    bm = DP_MBIT!(DP_FBITS + 2);
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

    /* Normalise rm to rounding precision ? */
    while (rm >> (DP_FBITS + 3)) == 0 {
        rm <<= 1;
        re -= 1;
    }

    ieee754dp_format(if xs == ys { 0 } else { 1 }, re, rm)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
