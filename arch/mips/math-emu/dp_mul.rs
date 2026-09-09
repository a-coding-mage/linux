// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * double precision: common utilities
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependency declarations and macro definitions are supplied by ieee754dp.h.

pub unsafe fn ieee754dp_mul(
    mut x: ieee754dp,
    mut y: ieee754dp,
) -> ieee754dp {
    let mut re: i32;
    let mut rs: i32;
    let mut rm: u64;
    let mut lxm: u32;
    let mut hxm: u32;
    let mut lym: u32;
    let mut hym: u32;
    let mut lrm: u64;
    let mut hrm: u64;
    let mut t: u64;
    let mut at: u64;

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
        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_INF) => {
            ieee754_setcx(IEEE754_INVALID_OPERATION);
            return ieee754dp_indef();
        }

        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_DNORM)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_INF) => return ieee754dp_inf(xs ^ ys),

        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_DNORM)
        | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_ZERO) => return ieee754dp_zero(xs ^ ys),

        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_DNORM) => {
            DPDNORMX!();
            DPDNORMY!();
        }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_DNORM) => DPDNORMY!(),
        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_NORM) => DPDNORMX!(),
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_NORM) => {}
    }

    /* rm = xm * ym, re = xe+ye basically */
    assert!(xm & DP_HIDDEN_BIT != 0);
    assert!(ym & DP_HIDDEN_BIT != 0);

    /* shunt to top of word */
    xm <<= 64 - (DP_FBITS + 1);
    ym <<= 64 - (DP_FBITS + 1);

    /* Multiply 64 bits xm, ym to give high 64 bits rm with stickness. */
    lxm = xm as u32;
    hxm = (xm >> 32) as u32;
    lym = ym as u32;
    hym = (ym >> 32) as u32;

    lrm = DPXMULT!(lxm, lym);
    hrm = DPXMULT!(hxm, hym);

    t = DPXMULT!(lxm, hym);
    at = lrm.wrapping_add(t << 32);
    hrm = hrm.wrapping_add((at < lrm) as u64);
    lrm = at;
    hrm = hrm.wrapping_add(t >> 32);

    t = DPXMULT!(hxm, lym);
    at = lrm.wrapping_add(t << 32);
    hrm = hrm.wrapping_add((at < lrm) as u64);
    lrm = at;
    hrm = hrm.wrapping_add(t >> 32);

    rm = hrm | (lrm != 0) as u64;

    /* Sticky shift down to normal rounding precision. */
    if (rm as i64) < 0 {
        rm = (rm >> (64 - (DP_FBITS + 1 + 3)))
            | ((rm << (DP_FBITS + 1 + 3) != 0) as u64);
        re += 1;
    } else {
        rm = (rm >> (64 - (DP_FBITS + 1 + 3 + 1)))
            | ((rm << (DP_FBITS + 1 + 3 + 1) != 0) as u64);
    }
    assert!(rm & (DP_HIDDEN_BIT << 3) != 0);

    ieee754dp_format(rs, re, rm)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
