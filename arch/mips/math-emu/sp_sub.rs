// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * single precision
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependency declarations and macro definitions are supplied by ieee754sp.h.

pub unsafe fn ieee754sp_sub(
    x: ieee754sp,
    y: ieee754sp,
) -> ieee754sp {
    let mut s: i32;
    let (mut xc, mut xs, mut xe, mut xm) = (0, 0, 0, 0);
    let (mut yc, mut ys, mut ye, mut ym) = (0, 0, 0, 0);

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
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_SNAN) => ieee754sp_nanxcpt(y),

        CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_SNAN)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_DNORM)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_INF) => ieee754sp_nanxcpt(x),

        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_QNAN) => y,

        CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_DNORM)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_INF) => x,

        /* Infinity handling */
        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_INF) => {
            if xs != ys { return x; }
            ieee754_setcx(IEEE754_INVALID_OPERATION);
            ieee754sp_indef()
        }
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_INF) => ieee754sp_inf(ys ^ 1),

        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_DNORM) => x,

        /* Zero handling */
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_ZERO) => {
            if xs != ys { x } else { ieee754sp_zero((ieee754_csr.rm == FPU_CSR_RD) as i32) }
        }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_ZERO) => x,
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_DNORM) => {
            /* quick fix up */
            SPSIGN!(y) ^= 1;
            y
        }
        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_DNORM) => {
            SPDNORMX!();
            SPDNORMY!();
        }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_DNORM) => { SPDNORMY!(); }
        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_NORM) => { SPDNORMX!(); }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_NORM) => {}
    };

    /* flip sign of y and handle as add */
    ys ^= 1;

    assert!((xm & SP_HIDDEN_BIT) != 0);
    assert!((ym & SP_HIDDEN_BIT) != 0);

    /* provide guard,round and stick bit space */
    xm <<= 3;
    ym <<= 3;

    if xe > ye {
        /* have to shift y fraction right to align */
        s = xe - ye;
        ym = XSPSRS!(ym, s);
        ye += s;
    } else if ye > xe {
        /* have to shift x fraction right to align */
        s = ye - xe;
        xm = XSPSRS!(xm, s);
        xe += s;
    }
    assert!(xe == ye);
    assert!(xe <= SP_EMAX);

    if xs == ys {
        /* generate 28 bit result of adding two 27 bit numbers */
        xm = xm + ym;
        if (xm >> (SP_FBITS + 1 + 3)) != 0 {
            SPXSRSX1!(); /* shift preserving sticky */
        }
    } else {
        if xm >= ym { xm = xm - ym; } else { xm = ym - xm; xs = ys; }
        if xm == 0 {
            if ieee754_csr.rm == FPU_CSR_RD {
                return ieee754sp_zero(1); /* round negative inf. => sign = -1 */
            } else {
                return ieee754sp_zero(0); /* other round modes => sign = 1 */
            }
        }
        /* normalize to rounding precision */
        while (xm >> (SP_FBITS + 3)) == 0 {
            xm <<= 1;
            xe -= 1;
        }
    }

    ieee754sp_format(xs, xe, xm)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
