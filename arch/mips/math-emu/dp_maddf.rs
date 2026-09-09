// SPDX-License-Identifier: GPL-2.0-only
/*
 * IEEE754 floating point arithmetic
 * double precision: MADDF.f (Fused Multiply Add)
 * MADDF.fmt: FPR[fd] = FPR[fd] + (FPR[fs] x FPR[ft])
 *
 * MIPS floating point support
 * Copyright (C) 2015 Imagination Technologies, Ltd.
 * Author: Markos Chandras <markos.chandras@imgtec.com>
 */

/* Dependency declarations and macros are supplied by ieee754dp.h. */

/* 128 bits shift right logical with rounding. */
unsafe fn srl128(hptr: *mut u64, lptr: *mut u64, count: i32) {
    let mut low: u64;

    if count >= 128 {
        *lptr = if *hptr != 0 || *lptr != 0 { 1 } else { 0 };
        *hptr = 0;
    } else if count >= 64 {
        if count == 64 {
            *lptr = *hptr | if *lptr != 0 { 1 } else { 0 };
        } else {
            low = *lptr;
            *lptr = *hptr >> (count - 64);
            *lptr |= if (*hptr << (128 - count)) != 0 || low != 0 { 1 } else { 0 };
        }
        *hptr = 0;
    } else {
        low = *lptr;
        *lptr = (low >> count) | (*hptr << (64 - count));
        *lptr |= if (low << (64 - count)) != 0 { 1 } else { 0 };
        *hptr >>= count;
    }
}

unsafe fn _dp_maddf(mut z: ieee754dp, mut x: ieee754dp, mut y: ieee754dp,
                     flags: maddf_flags) -> ieee754dp {
    let mut re: i32;
    let mut rs: i32;
    let mut lxm: u32;
    let mut hxm: u32;
    let mut lym: u32;
    let mut hym: u32;
    let mut lrm: u64;
    let mut hrm: u64;
    let mut lzm: u64;
    let mut hzm: u64;
    let mut t: u64;
    let mut at: u64;
    let mut s: i32;

    COMPXDP!();
    COMPYDP!();
    COMPZDP!();
    EXPLODEXDP!();
    EXPLODEYDP!();
    EXPLODEZDP!();
    FLUSHXDP!();
    FLUSHYDP!();
    FLUSHZDP!();

    ieee754_clearcx();

    rs = xs ^ ys;
    if (flags & MADDF_NEGATE_PRODUCT) != 0 { rs ^= 1; }
    if (flags & MADDF_NEGATE_ADDITION) != 0 { zs ^= 1; }

    if zc == IEEE754_CLASS_SNAN { return ieee754dp_nanxcpt(z); }
    if xc == IEEE754_CLASS_SNAN { return ieee754dp_nanxcpt(x); }
    if yc == IEEE754_CLASS_SNAN { return ieee754dp_nanxcpt(y); }
    if zc == IEEE754_CLASS_QNAN { return z; }
    if xc == IEEE754_CLASS_QNAN { return x; }
    if yc == IEEE754_CLASS_QNAN { return y; }

    if zc == IEEE754_CLASS_DNORM { DPDNORMZ!(); }

    match CLPAIR!(xc, yc) {
        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_ZERO) |
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_INF) => {
            ieee754_setcx(IEEE754_INVALID_OPERATION); return ieee754dp_indef();
        }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_INF) |
        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_INF) |
        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_NORM) |
        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_DNORM) |
        CLPAIR!(IEEE754_CLASS_INF, IEEE754_CLASS_INF) => {
            if zc == IEEE754_CLASS_INF && zs != rs {
                ieee754_setcx(IEEE754_INVALID_OPERATION); return ieee754dp_indef();
            }
            return ieee754dp_inf(rs);
        }
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_ZERO) |
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_NORM) |
        CLPAIR!(IEEE754_CLASS_ZERO, IEEE754_CLASS_DNORM) |
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_ZERO) |
        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_ZERO) => {
            if zc == IEEE754_CLASS_INF { return ieee754dp_inf(zs); }
            if zc == IEEE754_CLASS_ZERO {
                if zs == rs { return z; }
                return ieee754dp_zero(ieee754_csr.rm == FPU_CSR_RD);
            }
            return z;
        }
        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_DNORM) => {
            DPDNORMX!();
            /* fall through */
            DPDNORMY!();
        }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_DNORM) => {
            if zc == IEEE754_CLASS_INF { return ieee754dp_inf(zs); }
            DPDNORMY!();
        }
        CLPAIR!(IEEE754_CLASS_DNORM, IEEE754_CLASS_NORM) => {
            if zc == IEEE754_CLASS_INF { return ieee754dp_inf(zs); }
            DPDNORMX!();
        }
        CLPAIR!(IEEE754_CLASS_NORM, IEEE754_CLASS_NORM) => {
            if zc == IEEE754_CLASS_INF { return ieee754dp_inf(zs); }
        }
        _ => {}
    }

    assert!((xm & DP_HIDDEN_BIT) != 0);
    assert!((ym & DP_HIDDEN_BIT) != 0);
    re = xe + ye;
    xm <<= 64 - (DP_FBITS + 1);
    ym <<= 64 - (DP_FBITS + 1);
    lxm = xm; hxm = xm >> 32; lym = ym; hym = ym >> 32;
    lrm = DPXMULT!(lxm, lym); hrm = DPXMULT!(hxm, hym);
    t = DPXMULT!(lxm, hym); at = lrm + (t << 32); hrm += if at < lrm { 1 } else { 0 }; lrm = at; hrm += t >> 32;
    t = DPXMULT!(hxm, lym); at = lrm + (t << 32); hrm += if at < lrm { 1 } else { 0 }; lrm = at; hrm += t >> 32;
    if (hrm as i64) < 0 { lrm = (hrm << 63) | (lrm >> 1); hrm >>= 1; re += 1; }
    assert!((hrm & (1u64 << 62)) != 0);
    if zc == IEEE754_CLASS_ZERO {
        srl128(&mut hrm, &mut lrm, 126 - 55);
        return ieee754dp_format(rs, re, lrm);
    }
    lzm = 0; hzm = zm << 10;
    assert!((hzm & (1u64 << 62)) != 0);
    if ze > re { s = ze - re; srl128(&mut hrm, &mut lrm, s); re += s; }
    else if re > ze { s = re - ze; srl128(&mut hzm, &mut lzm, s); ze += s; }
    assert!(ze == re); assert!(ze <= DP_EMAX);
    if zs == rs {
        hzm = hzm + hrm + if lzm > lzm.wrapping_add(lrm) { 1 } else { 0 };
        lzm = lzm.wrapping_add(lrm);
        if (hzm as i64) < 0 { srl128(&mut hzm, &mut lzm, 1); ze += 1; }
    } else {
        if hzm > hrm || (hzm == hrm && lzm >= lrm) { hzm = hzm - hrm - if lzm < lrm { 1 } else { 0 }; lzm = lzm - lrm; }
        else { hzm = hrm - hzm - if lrm < lzm { 1 } else { 0 }; lzm = lrm - lzm; zs = rs; }
        if lzm == 0 && hzm == 0 { return ieee754dp_zero(ieee754_csr.rm == FPU_CSR_RD); }
        if hzm == 0 {
            if (lzm as i64) < 0 { hzm = lzm >> 1; lzm <<= 63; ze -= 63; }
            else { hzm = lzm; lzm = 0; ze -= 64; }
        }
        t = 0; while (hzm >> (62 - t)) == 0 { t += 1; }
        assert!(t <= 62);
        if t != 0 { hzm = (hzm << t) | (lzm >> (64 - t)); lzm <<= t; ze -= t; }
    }
    srl128(&mut hzm, &mut lzm, 126 - 55);
    ieee754dp_format(zs, ze, lzm)
}

pub unsafe fn ieee754dp_maddf(z: ieee754dp, x: ieee754dp, y: ieee754dp) -> ieee754dp { _dp_maddf(z, x, y, 0) }
pub unsafe fn ieee754dp_msubf(z: ieee754dp, x: ieee754dp, y: ieee754dp) -> ieee754dp { _dp_maddf(z, x, y, MADDF_NEGATE_PRODUCT) }
pub unsafe fn ieee754dp_madd(z: ieee754dp, x: ieee754dp, y: ieee754dp) -> ieee754dp { _dp_maddf(z, x, y, 0) }
pub unsafe fn ieee754dp_msub(z: ieee754dp, x: ieee754dp, y: ieee754dp) -> ieee754dp { _dp_maddf(z, x, y, MADDF_NEGATE_ADDITION) }
pub unsafe fn ieee754dp_nmadd(z: ieee754dp, x: ieee754dp, y: ieee754dp) -> ieee754dp { _dp_maddf(z, x, y, MADDF_NEGATE_PRODUCT | MADDF_NEGATE_ADDITION) }
pub unsafe fn ieee754dp_nmsub(z: ieee754dp, x: ieee754dp, y: ieee754dp) -> ieee754dp { _dp_maddf(z, x, y, MADDF_NEGATE_PRODUCT) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
