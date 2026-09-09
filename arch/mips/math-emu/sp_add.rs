// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * single precision
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

pub unsafe fn ieee754sp_add(mut x: ieee754sp, mut y: ieee754sp) -> ieee754sp {
    let mut s: i32 = 0;

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
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_SNAN) => {
            return ieee754sp_nanxcpt(y);
        }

        CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_SNAN)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_DNORM)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_INF) => {
            return ieee754sp_nanxcpt(x);
        }

        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_QNAN) => return y,

        CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_DNORM)
        | CLPAIR!(IEEE754_CLASS_QNAN, IEEE754_CLASS_INF) => return x,

        // Infinity handling
        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_INF) => {
            if xs == ys {
                return x;
            }
            ieee754_setcx(IEEE754_INVALID_OPERATION);
            return ieee754sp_indef();
        }

        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_INF) => return y,

        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_DNORM) => return x,

        // Zero handling
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_ZERO) => {
            if xs == ys {
                return x;
            } else {
                return ieee754sp_zero(ieee754_csr.rm == FPU_CSR_RD);
            }
        }

        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_ZERO) => return x,

        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_DNORM) => return y,

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

    // Provide guard, round and stick bit space.
    xm <<= 3;
    ym <<= 3;

    if xe > ye {
        // Have to shift y fraction right to align.
        s = xe - ye;
        ym = XSPSRS!(ym, s);
        ye += s;
    } else if ye > xe {
        // Have to shift x fraction right to align.
        s = ye - xe;
        xm = XSPSRS!(xm, s);
        xe += s;
    }
    assert!(xe == ye);
    assert!(xe <= SP_EMAX);

    if xs == ys {
        // Generate 28 bit result of adding two 27 bit numbers
        // leaving result in xm, xs and xe.
        xm = xm + ym;

        if (xm >> (SP_FBITS + 1 + 3)) != 0 {
            SPXSRSX1!();
        }
    } else {
        if xm >= ym {
            xm = xm - ym;
        } else {
            xm = ym - xm;
            xs = ys;
        }
        if xm == 0 {
            return ieee754sp_zero(ieee754_csr.rm == FPU_CSR_RD);
        }

        // Normalize in extended single precision
        while (xm >> (SP_FBITS + 3)) == 0 {
            xm <<= 1;
            xe -= 1;
        }
    }

    ieee754sp_format(xs, xe, xm)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
