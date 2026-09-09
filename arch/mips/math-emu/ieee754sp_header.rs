/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * IEEE754 floating point
 * double precision internal header file
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// #include <linux/compiler.h>
// #include "ieee754int.h"

#[allow(non_camel_case_types)]
pub type u32 = core::primitive::u32;

pub const SP_EBIAS: i32 = 127;
pub const SP_EMIN: i32 = -126;
pub const SP_EMAX: i32 = 127;
pub const SP_FBITS: i32 = 23;
pub const SP_MBITS: i32 = 23;

#[inline]
pub const fn sp_mbit(x: u32) -> u32 {
    1u32 << x
}

pub const SP_HIDDEN_BIT: u32 = sp_mbit(SP_FBITS as u32);
pub const SP_SIGN_BIT: u32 = sp_mbit(31);

// C macros SPSIGN, SPBEXP, and SPMANT are field accesses on ieee754sp.

#[inline]
pub fn ieee754sp_finite(x: ieee754sp) -> bool {
    x.bexp != (SP_EMAX + 1 + SP_EBIAS)
}

/* 64 bit right shift with rounding */
#[inline]
pub fn xspsrs64(v: u64, rs: u32) -> u64 {
    if rs >= 64 {
        if v != 0 { 1 } else { 0 }
    } else {
        (v >> rs) | (((v << (64 - rs)) != 0) as u64)
    }
}

/* 3bit extended single precision sticky right shift */
#[inline]
pub fn xspsrs(v: u32, rs: u32) -> u32 {
    if rs > (SP_FBITS as u32 + 3) {
        1
    } else {
        (v >> rs) | (((v << (32 - rs)) != 0) as u32)
    }
}

#[inline]
pub fn xspsrs1(m: u32) -> u32 {
    (m >> 1) | (m & 1)
}

/*
 * The following macros preserve the original C expression semantics and
 * operate on caller-provided mutable exponent and mantissa variables.
 */
#[macro_export]
macro_rules! SPXSRSX1 {
    ($xe:expr, $xm:expr) => {{
        $xe += 1;
        $xm = $crate::xspsrs1($xm);
    }};
}

#[macro_export]
macro_rules! SPXSRSY1 {
    ($ye:expr, $ym:expr) => {{
        $ye += 1;
        $ym = $crate::xspsrs1($ym);
    }};
}

/* convert denormal to normalized with extended exponent */
#[macro_export]
macro_rules! SPDNORMx {
    ($m:expr, $e:expr) => {{
        while (($m >> SP_FBITS) == 0) {
            $m <<= 1;
            $e -= 1;
        }
    }};
}

#[macro_export]
macro_rules! SPDNORMX {
    ($xm:expr, $xe:expr) => { $crate::SPDNORMx!($xm, $xe) };
}

#[macro_export]
macro_rules! SPDNORMY {
    ($ym:expr, $ye:expr) => { $crate::SPDNORMx!($ym, $ye) };
}

#[macro_export]
macro_rules! SPDNORMZ {
    ($zm:expr, $ze:expr) => { $crate::SPDNORMx!($zm, $ze) };
}

#[inline]
pub fn buildsp(s: i32, bx: i32, m: u32) -> ieee754sp {
    // assert((s) == 0 || (s) == 1);
    // assert((bx) >= SP_EMIN - 1 + SP_EBIAS
    //        && (bx) <= SP_EMAX + 1 + SP_EBIAS);
    // assert(((m) >> SP_FBITS) == 0);
    let mut r: ieee754sp = unsafe { core::mem::zeroed() };
    r.sign = s;
    r.bexp = bx;
    r.mant = m;
    r
}

extern "C" {
    pub fn ieee754sp_nanxcpt(x: ieee754sp) -> ieee754sp;
    pub fn ieee754sp_format(a: i32, b: i32, c: u32) -> ieee754sp;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
