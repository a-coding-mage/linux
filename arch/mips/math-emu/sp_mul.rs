// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * single precision
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

pub unsafe fn ieee754sp_mul(x: ieee754sp, y: ieee754sp) -> ieee754sp {
    let mut re: i32;
    let mut rs: i32;
    let mut rm: u32;
    let lxm: u16;
    let hxm: u16;
    let lym: u16;
    let hym: u16;
    let mut lrm: u32;
    let mut hrm: u32;
    let mut t: u32;
    let mut at: u32;

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
        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_INF) => {
            ieee754_setcx(IEEE754_INVALID_OPERATION);
            return ieee754sp_indef();
        }

        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_INF)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_DNORM)
        | CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_INF) => return ieee754sp_inf(xs ^ ys),

        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_NORM)
        | CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_DNORM)
        | CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_ZERO)
        | CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_ZERO) => return ieee754sp_zero(xs ^ ys),

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
        _ => unreachable!(),
    }

    /* rm = xm * ym, re = xe+ye basically */
    assert!(xm & SP_HIDDEN_BIT != 0);
    assert!(ym & SP_HIDDEN_BIT != 0);

    /* shunt to top of word */
    xm <<= 32 - (SP_FBITS + 1);
    ym <<= 32 - (SP_FBITS + 1);

    /*
     * Multiply 32 bits xm, ym to give high 32 bits rm with stickness.
     */
    lxm = (xm & 0xffff) as u16;
    hxm = (xm >> 16) as u16;
    lym = (ym & 0xffff) as u16;
    hym = (ym >> 16) as u16;

    lrm = lxm as u32 * lym as u32; /* 16 * 16 => 32 */
    hrm = hxm as u32 * hym as u32; /* 16 * 16 => 32 */

    t = lxm as u32 * hym as u32; /* 16 * 16 => 32 */
    at = lrm.wrapping_add(t << 16);
    hrm += (at < lrm) as u32;
    lrm = at;
    hrm = hrm.wrapping_add(t >> 16);

    t = hxm as u32 * lym as u32; /* 16 * 16 => 32 */
    at = lrm.wrapping_add(t << 16);
    hrm += (at < lrm) as u32;
    lrm = at;
    hrm = hrm.wrapping_add(t >> 16);

    rm = hrm | (lrm != 0) as u32;

    /*
     * Sticky shift down to normal rounding precision.
     */
    if (rm as i32) < 0 {
        rm = (rm >> (32 - (SP_FBITS + 1 + 3)))
            | (((rm << (SP_FBITS + 1 + 3)) != 0) as u32);
        re += 1;
    } else {
        rm = (rm >> (32 - (SP_FBITS + 1 + 3 + 1)))
            | (((rm << (SP_FBITS + 1 + 3 + 1)) != 0) as u32);
    }
    assert!(rm & (SP_HIDDEN_BIT << 3) != 0);

    ieee754sp_format(rs, re, rm)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
