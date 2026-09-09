// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * double precision: common utilities
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependency declarations and build-time macro definitions are supplied by
// the surrounding IEEE754/MIPS implementation.

pub unsafe fn ieee754dp_add(
    mut x: ieee754dp,
    mut y: ieee754dp,
) -> ieee754dp {
    let mut s: i32;
    let mut xc: i32 = 0;
    let mut yc: i32 = 0;
    let mut xs: i32 = 0;
    let mut ys: i32 = 0;
    let mut xe: i32 = 0;
    let mut ye: i32 = 0;
    let mut xm: u64 = 0;
    let mut ym: u64 = 0;

    compxdp!(x);
    compydp!(y);

    explodedp!(x);
    explodeyp!(y);

    ieee754_clearcx();

    flushxdp!();
    flushydp!();

    match clpair!(xc, yc) {
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
            if xs == ys { x } else {
                ieee754_setcx(IEEE754_INVALID_OPERATION);
                ieee754dp_indef()
            }
        }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_INF) => y,
        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_DNORM) => x,

        /* Zero handling */
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_ZERO) => {
            if xs == ys { x } else { ieee754dp_zero(ieee754_csr.rm == FPU_CSR_RD) }
        }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_ZERO) => x,
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_DNORM) => y,

        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_DNORM) => {
            dpdnormx!();
            dpdnormy!();
        }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_DNORM) => { dpdnormy!(); }
        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_NORM) => { dpdnormx!(); }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_NORM) => {}
        _ => unreachable!(),
    }

    assert!(xm & DP_HIDDEN_BIT != 0);
    assert!(ym & DP_HIDDEN_BIT != 0);

    xm <<= 3;
    ym <<= 3;

    if xe > ye {
        s = xe - ye;
        ym = xdpsrs!(ym, s);
        ye += s;
    } else if ye > xe {
        s = ye - xe;
        xm = xdpsrs!(xm, s);
        xe += s;
    }
    assert!(xe == ye);
    assert!(xe <= DP_EMAX);

    if xs == ys {
        xm = xm.wrapping_add(ym);
        if xm >> (DP_FBITS + 1 + 3) != 0 {
            xm = xdpsrs1!(xm);
            xe += 1;
        }
    } else {
        if xm >= ym { xm -= ym; } else { xm = ym - xm; xs = ys; }
        if xm == 0 { return ieee754dp_zero(ieee754_csr.rm == FPU_CSR_RD); }
        while xm >> (DP_FBITS + 3) == 0 {
            xm <<= 1;
            xe -= 1;
        }
    }

    ieee754dp_format(xs, xe, xm)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
