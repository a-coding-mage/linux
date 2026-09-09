// SPDX-License-Identifier: GPL-2.0-only
/*
 * IEEE754 floating point arithmetic
 * single precision: MAX{,A}.f
 * MAX : Scalar Floating-Point Maximum
 * MAXA: Scalar Floating-Point argument with Maximum Absolute Value
 *
 * MAX.S : FPR[fd] = maxNum(FPR[fs],FPR[ft])
 * MAXA.S: FPR[fd] = maxNumMag(FPR[fs],FPR[ft])
 *
 * MIPS floating point support
 * Copyright (C) 2015 Imagination Technologies, Ltd.
 * Author: Markos Chandras <markos.chandras@imgtec.com>
 */

pub unsafe fn ieee754sp_fmax(
    mut x: ieee754sp,
    mut y: ieee754sp,
) -> ieee754sp {
    COMPXSP!();
    COMPYSP!();

    EXPLODEXSP!();
    EXPLODEYSP!();

    FLUSHXSP!();
    FLUSHYSP!();

    ieee754_clearcx();

    match CLPAIR!(xc, yc) {
        CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_SNAN)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_SNAN)
        | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_SNAN)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_SNAN)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_SNAN) => ieee754sp_nanxcpt(y),

        CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_SNAN)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_DNORM)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_INF) => ieee754sp_nanxcpt(x),

        // Quiet NaN handling
        // The case of both inputs quiet NaNs
        CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_QNAN) => x,

        // The cases of exactly one input quiet NaN (numbers are here preferred as returned values to NaNs)
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_QNAN) => x,

        CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_DNORM)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_INF) => y,

        // Infinity and zero handling
        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_DNORM)
        | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_ZERO) => if xs != 0 { y } else { x },

        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_DNORM) => if ys != 0 { x } else { y },

        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_ZERO) => ieee754sp_zero(xs & ys),

        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_DNORM) => {
            SPDNORMX!();
            SPDNORMY!();
            // fall through
            if xs == 0 {
                if xe > ye { x } else if xe < ye { y } else if xm <= ym { y } else { x }
            } else {
                if xe > ye { y } else if xe < ye { x } else if xm <= ym { x } else { y }
            }
        }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_DNORM) => {
            SPDNORMY!();
            if xs == 0 {
                if xe > ye { x } else if xe < ye { y } else if xm <= ym { y } else { x }
            } else {
                if xe > ye { y } else if xe < ye { x } else if xm <= ym { x } else { y }
            }
        }
        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_NORM) => {
            SPDNORMX!();
            if xs == 0 {
                if xe > ye { x } else if xe < ye { y } else if xm <= ym { y } else { x }
            } else {
                if xe > ye { y } else if xe < ye { x } else if xm <= ym { x } else { y }
            }
        }
        _ => {
            assert!((xm & SP_HIDDEN_BIT) != 0);
            assert!((ym & SP_HIDDEN_BIT) != 0);
            if xs > ys { y } else if xs < ys { x } else if xs == 0 {
                if xe > ye { x } else if xe < ye { y } else if xm <= ym { y } else { x }
            } else if xe > ye { y } else if xe < ye { x } else if xm <= ym { x } else { y }
        }
    }
}

pub unsafe fn ieee754sp_fmaxa(
    mut x: ieee754sp,
    mut y: ieee754sp,
) -> ieee754sp {
    COMPXSP!(); COMPYSP!(); EXPLODEXSP!(); EXPLODEYSP!(); FLUSHXSP!(); FLUSHYSP!();
    ieee754_clearcx();
    match CLPAIR!(xc, yc) {
        CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_SNAN) | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_SNAN) | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_SNAN) | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_SNAN) | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_SNAN) => ieee754sp_nanxcpt(y),
        CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_SNAN) | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_QNAN) | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_ZERO) | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_NORM) | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_DNORM) | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_INF) => ieee754sp_nanxcpt(x),
        CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_QNAN) => x,
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_QNAN) | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_QNAN) | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_QNAN) | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_QNAN) => x,
        CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_ZERO) | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_NORM) | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_DNORM) | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_INF) => y,
        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_INF) => ieee754sp_inf(xs & ys),
        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_ZERO) | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_NORM) | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_DNORM) | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_ZERO) | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_ZERO) => x,
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_INF) | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_INF) | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_INF) | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_NORM) | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_DNORM) => y,
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_ZERO) => ieee754sp_zero(xs & ys),
        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_DNORM) => { SPDNORMX!(); SPDNORMY!(); fmaxa_compare(x, y, xs, xe, ye, xm, ym) },
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_DNORM) => { SPDNORMY!(); fmaxa_compare(x, y, xs, xe, ye, xm, ym) },
        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_NORM) => { SPDNORMX!(); fmaxa_compare(x, y, xs, xe, ye, xm, ym) },
        _ => fmaxa_compare(x, y, xs, xe, ye, xm, ym),
    }
}

unsafe fn fmaxa_compare(x: ieee754sp, y: ieee754sp, xs: i32, xe: i32, ye: i32, xm: u32, ym: u32) -> ieee754sp {
    assert!((xm & SP_HIDDEN_BIT) != 0); assert!((ym & SP_HIDDEN_BIT) != 0);
    if xe > ye { x } else if xe < ye { y } else if xm < ym { y } else if xm > ym { x } else if xs == 0 { x } else { y }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
