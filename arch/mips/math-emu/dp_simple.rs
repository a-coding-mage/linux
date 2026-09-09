// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * double precision: common utilities
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependency supplied by ieee754dp.h.

extern "C" {
    static mut ieee754_csr: ieee754_csr;

    fn ieee754dp_sub(x: ieee754dp, y: ieee754dp) -> ieee754dp;
    fn ieee754dp_zero(sign: u32) -> ieee754dp;
    fn ieee754dp_add(x: ieee754dp, y: ieee754dp) -> ieee754dp;
}

// Types, constants, and the DPSIGN access macro are supplied by ieee754dp.h.
use crate::{FPU_CSR_RD, DPSIGN};

pub unsafe fn ieee754dp_neg(mut x: ieee754dp) -> ieee754dp {
    let mut y: ieee754dp;

    if (*(&raw mut ieee754_csr)).abs2008 {
        y = x;
        DPSIGN!(y) = !DPSIGN!(x);
    } else {
        let oldrm: u32;

        oldrm = (*(&raw mut ieee754_csr)).rm;
        (*(&raw mut ieee754_csr)).rm = FPU_CSR_RD;
        y = ieee754dp_sub(ieee754dp_zero(0), x);
        (*(&raw mut ieee754_csr)).rm = oldrm;
    }
    y
}

pub unsafe fn ieee754dp_abs(x: ieee754dp) -> ieee754dp {
    let mut y: ieee754dp;

    if (*(&raw mut ieee754_csr)).abs2008 {
        y = x;
        DPSIGN!(y) = 0;
    } else {
        let oldrm: u32;

        oldrm = (*(&raw mut ieee754_csr)).rm;
        (*(&raw mut ieee754_csr)).rm = FPU_CSR_RD;
        if DPSIGN!(x) {
            y = ieee754dp_sub(ieee754dp_zero(0), x);
        } else {
            y = ieee754dp_add(ieee754dp_zero(0), x);
        }
        (*(&raw mut ieee754_csr)).rm = oldrm;
    }
    y
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
