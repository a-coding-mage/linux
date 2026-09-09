/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * IEEE754 floating point
 * double precision internal header file
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependency supplied by the surrounding translation unit:
// #include <linux/compiler.h>
// #include "ieee754int.h"

macro_rules! assert {
    ($expr:expr) => {{ let _ = &$expr; }};
}

pub const DP_EBIAS: i32 = 1023;
pub const DP_EMIN: i32 = -1022;
pub const DP_EMAX: i32 = 1023;
pub const DP_FBITS: u32 = 52;
pub const DP_MBITS: u32 = 52;

macro_rules! DP_MBIT {
    ($x:expr) => { (1u64 << ($x)) };
}

pub const DP_HIDDEN_BIT: u64 = 1u64 << DP_FBITS;
pub const DP_SIGN_BIT: u64 = 1u64 << 63;

macro_rules! DPSIGN {
    ($dp:expr) => { ($dp).sign };
}
macro_rules! DPBEXP {
    ($dp:expr) => { ($dp).bexp };
}
macro_rules! DPMANT {
    ($dp:expr) => { ($dp).mant };
}

pub unsafe fn ieee754dp_finite(x: union ieee754dp) -> bool {
    DPBEXP!(x) != DP_EMAX + 1 + DP_EBIAS
}

/* 3bit extended double precision sticky right shift */
macro_rules! XDPSRS {
    ($v:expr, $rs:expr) => {
        if ($rs) > (DP_FBITS + 3) {
            1u64
        } else {
            (($v) >> ($rs)) | (((($v) << (64 - ($rs))) != 0) as u64)
        }
    };
}

macro_rules! XDPSRSX1 {
    ($xe:ident, $xm:ident) => {{
        $xe += 1;
        $xm = ($xm >> 1) | ($xm & 1);
    }};
}

macro_rules! XDPSRS1 {
    ($v:expr) => {
        (($v) >> 1) | (($v) & 1)
    };
}

/* 32bit * 32bit => 64bit unsigned integer multiplication */
macro_rules! DPXMULT {
    ($x:expr, $y:expr) => { (($x) as u64) * (($y) as u64) };
}

/* convert denormal to normalized with extended exponent */
macro_rules! DPDNORMx {
    ($m:ident, $e:ident) => {
        while (($m >> DP_FBITS) == 0) {
            $m <<= 1;
            $e -= 1;
        }
    };
}
macro_rules! DPDNORMX {
    ($xm:ident, $xe:ident) => { DPDNORMx!($xm, $xe) };
}
macro_rules! DPDNORMY {
    ($ym:ident, $ye:ident) => { DPDNORMx!($ym, $ye) };
}
macro_rules! DPDNORMZ {
    ($zm:ident, $ze:ident) => { DPDNORMx!($zm, $ze) };
}

pub unsafe fn builddp(s: i32, bx: i32, m: u64) -> union ieee754dp {
    let mut r: union ieee754dp = core::mem::zeroed();

    assert!((s) == 0 || (s) == 1);
    assert!((bx) >= DP_EMIN - 1 + DP_EBIAS && (bx) <= DP_EMAX + 1 + DP_EBIAS);
    assert!(((m) >> DP_FBITS) == 0);

    r.sign = s;
    r.bexp = bx;
    r.mant = m;

    r
}

extern "C" {
    pub fn ieee754dp_nanxcpt(x: union ieee754dp) -> union ieee754dp;
    pub fn ieee754dp_format(s: i32, bexp: i32, mant: u64) -> union ieee754dp;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
