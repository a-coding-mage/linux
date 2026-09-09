// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * double precision: common utilities
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 * Copyright (C) 2017 Imagination Technologies, Ltd.
 * Author: Aleksandar Markovic <aleksandar.markovic@imgtec.com>
 */

// Declarations and macros supplied by ieee754dp.h are external dependencies.

pub unsafe fn ieee754dp_rint(mut x: ieee754dp) -> ieee754dp {
    let mut ret: ieee754dp;
    let mut residue: u64;
    let mut sticky: i32;
    let mut round: i32;
    let mut odd: i32;

    COMPXDP!();

    ieee754_clearcx();

    EXPLODEXDP!();
    FLUSHXDP!();

    if xc == IEEE754_CLASS_SNAN {
        return ieee754dp_nanxcpt(x);
    }

    if (xc == IEEE754_CLASS_QNAN) ||
       (xc == IEEE754_CLASS_INF) ||
       (xc == IEEE754_CLASS_ZERO) {
        return x;
    }

    if xe >= DP_FBITS {
        return x;
    }

    if xe < -1 {
        residue = xm;
        round = 0;
        sticky = (residue != 0) as i32;
        xm = 0;
    } else {
        residue = xm << (64 - DP_FBITS + xe) as u32;
        round = ((residue >> 63) != 0) as i32;
        sticky = ((residue << 1) != 0) as i32;
        xm >>= (DP_FBITS - xe) as u32;
    }

    odd = ((xm & 0x1) != 0x0) as i32;

    match ieee754_csr.rm {
        FPU_CSR_RN => { // toward nearest
            if round != 0 && (sticky != 0 || odd != 0) {
                xm = xm.wrapping_add(1);
            }
        }
        FPU_CSR_RZ => { /* toward zero */ }
        FPU_CSR_RU => { // toward +infinity
            if (round != 0 || sticky != 0) && !xs {
                xm = xm.wrapping_add(1);
            }
        }
        FPU_CSR_RD => { // toward -infinity
            if (round != 0 || sticky != 0) && xs {
                xm = xm.wrapping_add(1);
            }
        }
        _ => {}
    }

    if round != 0 || sticky != 0 {
        ieee754_setcx(IEEE754_INEXACT);
    }

    ret = ieee754dp_flong(xm);
    // C lvalue macro equivalent: DPSIGN(ret) = xs;
    DPSIGN!(ret, xs);

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
