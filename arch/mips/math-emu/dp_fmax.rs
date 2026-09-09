// SPDX-License-Identifier: GPL-2.0-only
/*
 * IEEE754 floating point arithmetic
 * double precision: MIN{,A}.f
 * MIN : Scalar Floating-Point Minimum
 * MINA: Scalar Floating-Point argument with Minimum Absolute Value
 *
 * MIN.D : FPR[fd] = minNum(FPR[fs],FPR[ft])
 * MINA.D: FPR[fd] = maxNumMag(FPR[fs],FPR[ft])
 *
 * MIPS floating point support
 * Copyright (C) 2015 Imagination Technologies, Ltd.
 * Author: Markos Chandras <markos.chandras@imgtec.com>
 */

pub unsafe fn ieee754dp_fmax(
    mut x: union ieee754dp,
    mut y: union ieee754dp,
) -> union ieee754dp {
    COMPXDP;
    COMPYDP;

    EXPLODEXDP;
    EXPLODEYDP;

    FLUSHXDP;
    FLUSHYDP;

    ieee754_clearcx();

    match CLPAIR(xc, yc) {
        CLPAIR(IEEE754_CLASS_QNAN, IEEE754_CLASS_SNAN)
        | CLPAIR(IEEE754_CLASS_ZERO, IEEE754_CLASS_SNAN)
        | CLPAIR(IEEE754_CLASS_NORM, IEEE754_CLASS_SNAN)
        | CLPAIR(IEEE754_CLASS_DNORM, IEEE754_CLASS_SNAN)
        | CLPAIR(IEEE754_CLASS_INF, IEEE754_CLASS_SNAN) => ieee754dp_nanxcpt(y),

        CLPAIR(IEEE754_CLASS_SNAN, IEEE754_CLASS_SNAN)
        | CLPAIR(IEEE754_CLASS_SNAN, IEEE754_CLASS_QNAN)
        | CLPAIR(IEEE754_CLASS_SNAN, IEEE754_CLASS_ZERO)
        | CLPAIR(IEEE754_CLASS_SNAN, IEEE754_CLASS_NORM)
        | CLPAIR(IEEE754_CLASS_SNAN, IEEE754_CLASS_DNORM)
        | CLPAIR(IEEE754_CLASS_SNAN, IEEE754_CLASS_INF) => ieee754dp_nanxcpt(x),

        /* Quiet NaN handling */
        /* The case of both inputs quiet NaNs */
        CLPAIR(IEEE754_CLASS_QNAN, IEEE754_CLASS_QNAN) => x,

        /* The cases of exactly one input quiet NaN (numbers
         * are here preferred as returned values to NaNs) */
        CLPAIR(IEEE754_CLASS_ZERO, IEEE754_CLASS_QNAN)
        | CLPAIR(IEEE754_CLASS_NORM, IEEE754_CLASS_QNAN)
        | CLPAIR(IEEE754_CLASS_DNORM, IEEE754_CLASS_QNAN)
        | CLPAIR(IEEE754_CLASS_INF, IEEE754_CLASS_QNAN) => x,

        CLPAIR(IEEE754_CLASS_QNAN, IEEE754_CLASS_ZERO)
        | CLPAIR(IEEE754_CLASS_QNAN, IEEE754_CLASS_NORM)
        | CLPAIR(IEEE754_CLASS_QNAN, IEEE754_CLASS_DNORM)
        | CLPAIR(IEEE754_CLASS_QNAN, IEEE754_CLASS_INF) => y,

        /* Infinity and zero handling */
        CLPAIR(IEEE754_CLASS_INF, IEEE754_CLASS_ZERO)
        | CLPAIR(IEEE754_CLASS_INF, IEEE754_CLASS_NORM)
        | CLPAIR(IEEE754_CLASS_INF, IEEE754_CLASS_DNORM)
        | CLPAIR(IEEE754_CLASS_NORM, IEEE754_CLASS_ZERO)
        | CLPAIR(IEEE754_CLASS_DNORM, IEEE754_CLASS_ZERO) => if xs != 0 { y } else { x },

        CLPAIR(IEEE754_CLASS_INF, IEEE754_CLASS_INF)
        | CLPAIR(IEEE754_CLASS_NORM, IEEE754_CLASS_INF)
        | CLPAIR(IEEE754_CLASS_DNORM, IEEE754_CLASS_INF)
        | CLPAIR(IEEE754_CLASS_ZERO, IEEE754_CLASS_INF)
        | CLPAIR(IEEE754_CLASS_ZERO, IEEE754_CLASS_NORM)
        | CLPAIR(IEEE754_CLASS_ZERO, IEEE754_CLASS_DNORM) => if ys != 0 { x } else { y },

        CLPAIR(IEEE754_CLASS_ZERO, IEEE754_CLASS_ZERO) => ieee754dp_zero(xs & ys),

        CLPAIR(IEEE754_CLASS_DNORM, IEEE754_CLASS_DNORM) => {
            DPDNORMX;
            DPDNORMY;
        }
        CLPAIR(IEEE754_CLASS_NORM, IEEE754_CLASS_DNORM) => {
            DPDNORMY;
        }
        CLPAIR(IEEE754_CLASS_DNORM, IEEE754_CLASS_NORM) => {
            DPDNORMX;
        }
        _ => {
        }
    };

    assert!(xm & DP_HIDDEN_BIT != 0);
    assert!(ym & DP_HIDDEN_BIT != 0);
    if xs > ys { return y; }
    else if xs < ys { return x; }
    if xs == 0 {
        if xe > ye { return x; }
        else if xe < ye { return y; }
    } else {
        if xe > ye { return y; }
        else if xe < ye { return x; }
    }
    if xs == 0 {
        if xm <= ym { return y; }
        return x;
    }
    if xm <= ym { return x; }
    return y;
}

pub unsafe fn ieee754dp_fmaxa(
    mut x: union ieee754dp,
    mut y: union ieee754dp,
) -> union ieee754dp {
    COMPXDP;
    COMPYDP;

    EXPLODEXDP;
    EXPLODEYDP;

    FLUSHXDP;
    FLUSHYDP;

    ieee754_clearcx();

    match CLPAIR(xc, yc) {
        CLPAIR(IEEE754_CLASS_QNAN, IEEE754_CLASS_SNAN)
        | CLPAIR(IEEE754_CLASS_ZERO, IEEE754_CLASS_SNAN)
        | CLPAIR(IEEE754_CLASS_NORM, IEEE754_CLASS_SNAN)
        | CLPAIR(IEEE754_CLASS_DNORM, IEEE754_CLASS_SNAN)
        | CLPAIR(IEEE754_CLASS_INF, IEEE754_CLASS_SNAN) => ieee754dp_nanxcpt(y),
        CLPAIR(IEEE754_CLASS_SNAN, IEEE754_CLASS_SNAN)
        | CLPAIR(IEEE754_CLASS_SNAN, IEEE754_CLASS_QNAN)
        | CLPAIR(IEEE754_CLASS_SNAN, IEEE754_CLASS_ZERO)
        | CLPAIR(IEEE754_CLASS_SNAN, IEEE754_CLASS_NORM)
        | CLPAIR(IEEE754_CLASS_SNAN, IEEE754_CLASS_DNORM)
        | CLPAIR(IEEE754_CLASS_SNAN, IEEE754_CLASS_INF) => ieee754dp_nanxcpt(x),
        CLPAIR(IEEE754_CLASS_QNAN, IEEE754_CLASS_QNAN) => x,
        CLPAIR(IEEE754_CLASS_ZERO, IEEE754_CLASS_QNAN)
        | CLPAIR(IEEE754_CLASS_NORM, IEEE754_CLASS_QNAN)
        | CLPAIR(IEEE754_CLASS_DNORM, IEEE754_CLASS_QNAN)
        | CLPAIR(IEEE754_CLASS_INF, IEEE754_CLASS_QNAN) => x,
        CLPAIR(IEEE754_CLASS_QNAN, IEEE754_CLASS_ZERO)
        | CLPAIR(IEEE754_CLASS_QNAN, IEEE754_CLASS_NORM)
        | CLPAIR(IEEE754_CLASS_QNAN, IEEE754_CLASS_DNORM)
        | CLPAIR(IEEE754_CLASS_QNAN, IEEE754_CLASS_INF) => y,
        CLPAIR(IEEE754_CLASS_INF, IEEE754_CLASS_INF) => ieee754dp_inf(xs & ys),
        CLPAIR(IEEE754_CLASS_INF, IEEE754_CLASS_ZERO)
        | CLPAIR(IEEE754_CLASS_INF, IEEE754_CLASS_NORM)
        | CLPAIR(IEEE754_CLASS_INF, IEEE754_CLASS_DNORM)
        | CLPAIR(IEEE754_CLASS_NORM, IEEE754_CLASS_ZERO)
        | CLPAIR(IEEE754_CLASS_DNORM, IEEE754_CLASS_ZERO) => x,
        CLPAIR(IEEE754_CLASS_NORM, IEEE754_CLASS_INF)
        | CLPAIR(IEEE754_CLASS_DNORM, IEEE754_CLASS_INF)
        | CLPAIR(IEEE754_CLASS_ZERO, IEEE754_CLASS_INF)
        | CLPAIR(IEEE754_CLASS_ZERO, IEEE754_CLASS_NORM)
        | CLPAIR(IEEE754_CLASS_ZERO, IEEE754_CLASS_DNORM) => y,
        CLPAIR(IEEE754_CLASS_ZERO, IEEE754_CLASS_ZERO) => ieee754dp_zero(xs & ys),
        CLPAIR(IEEE754_CLASS_DNORM, IEEE754_CLASS_DNORM) => {
            DPDNORMX;
            DPDNORMY;
        }
        CLPAIR(IEEE754_CLASS_NORM, IEEE754_CLASS_DNORM) => {
            DPDNORMY;
        }
        CLPAIR(IEEE754_CLASS_DNORM, IEEE754_CLASS_NORM) => {
            DPDNORMX;
        }
        _ => {
        }
    };

    assert!(xm & DP_HIDDEN_BIT != 0);
    assert!(ym & DP_HIDDEN_BIT != 0);
    if xe > ye { return x; }
    else if xe < ye { return y; }
    if xm < ym { return y; }
    else if xm > ym { return x; }
    else if xs == 0 { return x; }
    return y;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
