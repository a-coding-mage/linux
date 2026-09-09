// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * single precision
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependency declarations and macros are supplied by ieee754sp.h in the C
// implementation and are expected to be provided by the surrounding Rust
// translation.

pub unsafe fn ieee754sp_neg(mut x: ieee754sp) -> ieee754sp {
    let mut y: ieee754sp;

    if ieee754_csr.abs2008 {
        y = x;
        y.f.sign = (!x.f.sign) as _;
    } else {
        let oldrm: u32;

        oldrm = ieee754_csr.rm;
        ieee754_csr.rm = FPU_CSR_RD;
        y = ieee754sp_sub(ieee754sp_zero(0), x);
        ieee754_csr.rm = oldrm;
    }
    y
}

pub unsafe fn ieee754sp_abs(x: ieee754sp) -> ieee754sp {
    let mut y: ieee754sp;

    if ieee754_csr.abs2008 {
        y = x;
        y.f.sign = 0;
    } else {
        let oldrm: u32;

        oldrm = ieee754_csr.rm;
        ieee754_csr.rm = FPU_CSR_RD;
        if x.f.sign != 0 {
            y = ieee754sp_sub(ieee754sp_zero(0), x);
        } else {
            y = ieee754sp_add(ieee754sp_zero(0), x);
        }
        ieee754_csr.rm = oldrm;
    }
    y
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
