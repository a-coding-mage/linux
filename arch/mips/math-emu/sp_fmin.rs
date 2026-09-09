// SPDX-License-Identifier: GPL-2.0-only
/*
 * IEEE754 floating point arithmetic
 * single precision: MIN{,A}.f
 * MIN : Scalar Floating-Point Minimum
 * MINA: Scalar Floating-Point argument with Minimum Absolute Value
 *
 * MIN.S : FPR[fd] = minNum(FPR[fs],FPR[ft])
 * MINA.S: FPR[fd] = maxNumMag(FPR[fs],FPR[ft])
 *
 * MIPS floating point support
 * Copyright (C) 2015 Imagination Technologies, Ltd.
 * Author: Markos Chandras <markos.chandras@imgtec.com>
 */

pub unsafe fn ieee754sp_fmin(mut x: ieee754sp, mut y: ieee754sp) -> ieee754sp {
    COMPXSP!();
    COMPYSP!();

    EXPLODEXSP!();
    EXPLODEYSP!();

    FLUSHXSP!();
    FLUSHYSP!();

    ieee754_clearcx();

    'classification: loop {
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

        // Quiet NaN handling: both inputs quiet NaNs.
        CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_QNAN) => x,

        // Exactly one quiet NaN; numbers are preferred as returned values.
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_QNAN) => x,

        CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_DNORM)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_INF) => y,

        // Infinity and zero handling.
        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_DNORM)
        | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_ZERO) => if xs != 0 { x } else { y },

        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_DNORM) => if ys != 0 { y } else { x },

        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_ZERO) => ieee754sp_zero(xs | ys),

        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_DNORM) => {
            SPDNORMX!();
            SPDNORMY!();
            break 'classification;
        }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_DNORM) => {
            SPDNORMY!();
            break 'classification;
        }
        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_NORM) => {
            SPDNORMX!();
            break 'classification;
        }
        _ => break 'classification,
        }
    }

    assert!((xm & SP_HIDDEN_BIT) != 0);
    assert!((ym & SP_HIDDEN_BIT) != 0);

    if xs > ys { return x; }
    if xs < ys { return y; }

    if xs == 0 {
        if xe > ye { return y; }
        if xe < ye { return x; }
    } else {
        if xe > ye { return x; }
        if xe < ye { return y; }
    }

    if xs == 0 {
        if xm <= ym { return x; }
        return y;
    }
    if xm <= ym { return y; }
    x
}

pub unsafe fn ieee754sp_fmina(mut x: ieee754sp, mut y: ieee754sp) -> ieee754sp {
    COMPXSP!();
    COMPYSP!();
    EXPLODEXSP!();
    EXPLODEYSP!();
    FLUSHXSP!();
    FLUSHYSP!();
    ieee754_clearcx();

    'classification: loop {
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
        CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_QNAN) => x,
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_QNAN) => x,
        CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_DNORM)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_INF) => y,
        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_INF) => ieee754sp_inf(xs | ys),
        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_DNORM)
        | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_ZERO) => y,
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_DNORM) => x,
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_ZERO) => ieee754sp_zero(xs | ys),
        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_DNORM) => { SPDNORMX!(); SPDNORMY!(); break 'classification; }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_DNORM) => { SPDNORMY!(); break 'classification; }
        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_NORM) => { SPDNORMX!(); break 'classification; }
        _ => break 'classification,
        }
    }

    assert!((xm & SP_HIDDEN_BIT) != 0);
    assert!((ym & SP_HIDDEN_BIT) != 0);
    if xe > ye { return y; }
    if xe < ye { return x; }
    if xm < ym { return x; }
    if xm > ym { return y; }
    if xs == 1 { return x; }
    y
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
