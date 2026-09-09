// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * double precision: common utilities
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependency declarations and preprocessor definitions are supplied by ieee754dp.h.

pub unsafe fn ieee754dp_sub(mut x: ieee754dp, mut y: ieee754dp) -> ieee754dp {
    let mut s: i32;

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
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_SNAN) => ieee754dp_nanxcpt(y),

        CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_SNAN)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_QNAN)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_DNORM)
        | CLPAIR!(IEEE754_CLASS_SNAN, IEEE754_CLASS_INF) => ieee754dp_nanxcpt(x),

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
            if xs != ys {
                x
            } else {
                ieee754_setcx(IEEE754_INVALID_OPERATION);
                ieee754dp_indef()
            }
        }

        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_INF) => ieee754dp_inf(ys ^ 1),

        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_DNORM) => x,

        /* Zero handling */
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_ZERO) => {
            if xs != ys {
                x
            } else {
                ieee754dp_zero(ieee754_csr.rm == FPU_CSR_RD)
            }
        }

        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_ZERO) => x,

        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_DNORM) => {
            /* quick fix up */
            DPSIGN!(y) ^= 1;
            y
        }

        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_DNORM) => {
            DPDNORMX!();
            // fallthrough
            DPDNORMY!();
            ()
        }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_DNORM) => {
            /* normalize ym,ye */
            DPDNORMY!();
            ()
        }

        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_NORM) => {
            /* normalize xm,xe */
            DPDNORMX!();
            ()
        }

        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_NORM) => (),
    }

    /* flip sign of y and handle as add */
    ys ^= 1;

    assert!(xm & DP_HIDDEN_BIT != 0);
    assert!(ym & DP_HIDDEN_BIT != 0);

    /* provide guard,round and stick bit dpace */
    xm <<= 3;
    ym <<= 3;

    if xe > ye {
        /* Have to shift y fraction right to align */
        s = xe - ye;
        ym = XDPSRS!(ym, s);
        ye += s;
    } else if ye > xe {
        /* Have to shift x fraction right to align */
        s = ye - xe;
        xm = XDPSRS!(xm, s);
        xe += s;
    }
    assert!(xe == ye);
    assert!(xe <= DP_EMAX);

    if xs == ys {
        /* generate 28 bit result of adding two 27 bit numbers */
        xm = xm + ym;

        if xm >> (DP_FBITS + 1 + 3) != 0 {
            xm = XDPSRS1!(xm); /* shift preserving sticky */
            xe += 1;
        }
    } else {
        if xm >= ym {
            xm = xm - ym;
        } else {
            xm = ym - xm;
            xs = ys;
        }
        if xm == 0 {
            if ieee754_csr.rm == FPU_CSR_RD {
                return ieee754dp_zero(1); /* round negative inf. => sign = -1 */
            } else {
                return ieee754dp_zero(0); /* other round modes => sign = 1 */
            }
        }

        /* normalize to rounding precision */
        while xm >> (DP_FBITS + 3) == 0 {
            xm <<= 1;
            xe -= 1;
        }
    }

    ieee754dp_format(xs, xe, xm)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
