// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * double precision: common utilities
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependency declarations and build-time definitions are supplied by ieee754dp.

pub unsafe fn ieee754dp_tint(x: ieee754dp) -> i32 {
    let mut residue: u64;
    let mut round: i32;
    let mut sticky: i32;
    let mut odd: i32;

    // C macros from ieee754dp.h; they populate xc, xs, xe, and xm.
    COMPXDP!(x);

    ieee754_clearcx();

    EXPLODEXDP!(x);
    FLUSHXDP!();

    match xc {
        IEEE754_CLASS_SNAN | IEEE754_CLASS_QNAN => {
            ieee754_setcx(IEEE754_INVALID_OPERATION);
            return ieee754si_indef();
        }

        IEEE754_CLASS_INF => {
            ieee754_setcx(IEEE754_INVALID_OPERATION);
            return ieee754si_overflow(xs);
        }

        IEEE754_CLASS_ZERO => return 0,

        IEEE754_CLASS_DNORM | IEEE754_CLASS_NORM => {}

        _ => {}
    }
    if xe > 31 {
        /* Set invalid. We will only use overflow for floating
           point overflow */
        ieee754_setcx(IEEE754_INVALID_OPERATION);
        return ieee754si_overflow(xs);
    }
    /* oh gawd */
    if xe > DP_FBITS {
        xm <<= (xe - DP_FBITS) as u32;
    } else if xe < DP_FBITS {
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
        /* Note: At this point upper 32 bits of xm are guaranteed
           to be zero */
        odd = ((xm & 0x1) != 0x0) as i32;
        match ieee754_csr.rm {
            FPU_CSR_RN => {
                if round != 0 && (sticky != 0 || odd != 0) {
                    xm += 1;
                }
            }
            FPU_CSR_RZ => {}
            FPU_CSR_RU => { /* toward +Infinity */
                if (round != 0 || sticky != 0) && xs == 0 {
                    xm += 1;
                }
            }
            FPU_CSR_RD => { /* toward -Infinity */
                if (round != 0 || sticky != 0) && xs != 0 {
                    xm += 1;
                }
            }
            _ => {}
        }
        /* look for valid corner case 0x80000000 */
        if (xm >> 31) != 0 && (xs == 0 || xm != 0x80000000) {
            /* This can happen after rounding */
            ieee754_setcx(IEEE754_INVALID_OPERATION);
            return ieee754si_overflow(xs);
        }
        if round != 0 || sticky != 0 {
            ieee754_setcx(IEEE754_INEXACT);
        }
    }
    if xs != 0 {
        -(xm as i32)
    } else {
        xm as i32
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
