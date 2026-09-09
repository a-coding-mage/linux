/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * IEEE754 floating point
 * common internal header file
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

/* Dependency: declarations and constants from ieee754.h are supplied externally. */

#[inline]
pub unsafe fn ieee754_clearcx() {
    ieee754_csr.cx = 0;
}

#[inline]
pub unsafe fn ieee754_setcx(flags: u32) {
    ieee754_csr.cx |= flags;
    ieee754_csr.sx |= flags;
}

#[inline]
pub unsafe fn ieee754_setandtestcx(x: u32) -> i32 {
    ieee754_setcx(x);
    (ieee754_csr.mx & x) as i32
}

#[inline]
pub fn ieee754_class_nan(xc: i32) -> bool {
    xc >= IEEE754_CLASS_SNAN
}

pub const CLPAIR: fn(i32, i32) -> i32 = |x, y| x.wrapping_mul(6).wrapping_add(y);

#[repr(C)]
pub enum maddf_flags {
    MADDF_NEGATE_PRODUCT = 1 << 0,
    MADDF_NEGATE_ADDITION = 1 << 1,
}

/* C declaration macros, represented as local Rust declarations where expanded. */
macro_rules! COMPXSP { () => { let mut xm: u32; let mut xe: i32; let mut xs: i32; let mut xc: i32; }; }
macro_rules! COMPYSP { () => { let mut ym: u32; let mut ye: i32; let mut ys: i32; let mut yc: i32; }; }
macro_rules! COMPZSP { () => { let mut zm: u32; let mut ze: i32; let mut zs: i32; let mut zc: i32; }; }
macro_rules! COMPXDP { () => { let mut xm: u64; let mut xe: i32; let mut xs: i32; let mut xc: i32; }; }
macro_rules! COMPYDP { () => { let mut ym: u64; let mut ye: i32; let mut ys: i32; let mut yc: i32; }; }
macro_rules! COMPZDP { () => { let mut zm: u64; let mut ze: i32; let mut zs: i32; let mut zc: i32; }; }

macro_rules! EXPLODESP {
    ($v:expr, $vc:ident, $vs:ident, $ve:ident, $vm:ident) => {{
        $vs = SPSIGN($v);
        $ve = SPBEXP($v);
        $vm = SPMANT($v);
        if $ve == SP_EMAX + 1 + SP_EBIAS {
            if $vm == 0 { $vc = IEEE754_CLASS_INF; }
            else if ieee754_csr.nan2008 ^ !($vm & SP_MBIT(SP_FBITS - 1)) != 0 { $vc = IEEE754_CLASS_QNAN; }
            else { $vc = IEEE754_CLASS_SNAN; }
        } else if $ve == SP_EMIN - 1 + SP_EBIAS {
            if $vm != 0 { $ve = SP_EMIN; $vc = IEEE754_CLASS_DNORM; }
            else { $vc = IEEE754_CLASS_ZERO; }
        } else { $ve -= SP_EBIAS; $vm |= SP_HIDDEN_BIT; $vc = IEEE754_CLASS_NORM; }
    }};
}
macro_rules! EXPLODEDP {
    ($v:expr, $vc:ident, $vs:ident, $ve:ident, $vm:ident) => {{
        $vm = DPMANT($v); $vs = DPSIGN($v); $ve = DPBEXP($v);
        if $ve == DP_EMAX + 1 + DP_EBIAS {
            if $vm == 0 { $vc = IEEE754_CLASS_INF; }
            else if ieee754_csr.nan2008 ^ !($vm & DP_MBIT(DP_FBITS - 1)) != 0 { $vc = IEEE754_CLASS_QNAN; }
            else { $vc = IEEE754_CLASS_SNAN; }
        } else if $ve == DP_EMIN - 1 + DP_EBIAS {
            if $vm != 0 { $ve = DP_EMIN; $vc = IEEE754_CLASS_DNORM; }
            else { $vc = IEEE754_CLASS_ZERO; }
        } else { $ve -= DP_EBIAS; $vm |= DP_HIDDEN_BIT; $vc = IEEE754_CLASS_NORM; }
    }};
}

macro_rules! EXPLODEXSP { () => { EXPLODESP!(x, xc, xs, xe, xm); }; }
macro_rules! EXPLODEYSP { () => { EXPLODESP!(y, yc, ys, ye, ym); }; }
macro_rules! EXPLODEZSP { () => { EXPLODESP!(z, zc, zs, ze, zm); }; }
macro_rules! EXPLODEXDP { () => { EXPLODEDP!(x, xc, xs, xe, xm); }; }
macro_rules! EXPLODEYDP { () => { EXPLODEDP!(y, yc, ys, ye, ym); }; }
macro_rules! EXPLODEZDP { () => { EXPLODEDP!(z, zc, zs, ze, zm); }; }

macro_rules! FLUSHDP { ($v:ident, $vc:ident, $vs:ident, $ve:ident, $vm:ident) => { if $vc == IEEE754_CLASS_DNORM && ieee754_csr.nod { unsafe { ieee754_setcx(IEEE754_INEXACT); } $vc = IEEE754_CLASS_ZERO; $ve = DP_EMIN - 1 + DP_EBIAS; $vm = 0; $v = unsafe { ieee754dp_zero($vs) }; } }; }
macro_rules! FLUSHSP { ($v:ident, $vc:ident, $vs:ident, $ve:ident, $vm:ident) => { if $vc == IEEE754_CLASS_DNORM && ieee754_csr.nod { unsafe { ieee754_setcx(IEEE754_INEXACT); } $vc = IEEE754_CLASS_ZERO; $ve = SP_EMIN - 1 + SP_EBIAS; $vm = 0; $v = unsafe { ieee754sp_zero($vs) }; } }; }

macro_rules! FLUSHXDP { () => { FLUSHDP!(x, xc, xs, xe, xm); }; }
macro_rules! FLUSHYDP { () => { FLUSHDP!(y, yc, ys, ye, ym); }; }
macro_rules! FLUSHZDP { () => { FLUSHDP!(z, zc, zs, ze, zm); }; }
macro_rules! FLUSHXSP { () => { FLUSHSP!(x, xc, xs, xe, xm); }; }
macro_rules! FLUSHYSP { () => { FLUSHSP!(y, yc, ys, ye, ym); }; }
macro_rules! FLUSHZSP { () => { FLUSHSP!(z, zc, zs, ze, zm); }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
