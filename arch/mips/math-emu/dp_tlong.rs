// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * double precision: common utilities
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependency declarations and the COMPXDP, EXPLODEXDP, and FLUSHXDP
// operations are supplied by ieee754dp.h in the C implementation.

pub unsafe fn ieee754dp_tlong(mut x: union_ieee754dp) -> i64 {
    let mut residue: u64;
    let mut round: i32;
    let mut sticky: i32;
    let mut odd: i32;

    let mut xs: i32 = 0;
    let mut xm: u64 = 0;
    let mut xe: i32 = 0;
    let mut xc: i32 = 0;

    // COMPXDP;
    ieee754_clearcx();

    // EXPLODEXDP;
    // FLUSHXDP;

    match xc {
        IEEE754_CLASS_SNAN | IEEE754_CLASS_QNAN => {
            ieee754_setcx(IEEE754_INVALID_OPERATION);
            return ieee754di_indef();
        }

        IEEE754_CLASS_INF => {
            ieee754_setcx(IEEE754_INVALID_OPERATION);
            return ieee754di_overflow(xs);
        }

        IEEE754_CLASS_ZERO => return 0,

        IEEE754_CLASS_DNORM | IEEE754_CLASS_NORM => {}
        _ => {}
    }

    if xe >= 63 {
        /* look for valid corner case */
        if xe == 63 && xs != 0 && xm == DP_HIDDEN_BIT {
            return -0x8000000000000000_i64;
        }
        /* Set invalid. We will only use overflow for floating
           point overflow */
        ieee754_setcx(IEEE754_INVALID_OPERATION);
        return ieee754di_overflow(xs);
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
            /* Shifting a u64 64 times does not work,
             * so we do it in two steps. Be aware that xe
             * may be -1 */
            residue = xm << (xe + 1) as u32;
            residue <<= (63 - DP_FBITS) as u32;
            round = ((residue >> 63) != 0) as i32;
            sticky = ((residue << 1) != 0) as i32;
            xm >>= (DP_FBITS - xe) as u32;
        }
        odd = ((xm & 0x1) != 0x0) as i32;
        match ieee754_csr.rm {
            FPU_CSR_RN => {
                if round != 0 && (sticky != 0 || odd != 0) {
                    xm = xm.wrapping_add(1);
                }
            }
            FPU_CSR_RZ => {}
            FPU_CSR_RU => {
                /* toward +Infinity */
                if (round != 0 || sticky != 0) && xs == 0 {
                    xm = xm.wrapping_add(1);
                }
            }
            FPU_CSR_RD => {
                /* toward -Infinity */
                if (round != 0 || sticky != 0) && xs != 0 {
                    xm = xm.wrapping_add(1);
                }
            }
            _ => {}
        }
        if (xm >> 63) != 0 {
            /* This can happen after rounding */
            ieee754_setcx(IEEE754_INVALID_OPERATION);
            return ieee754di_overflow(xs);
        }
        if round != 0 || sticky != 0 {
            ieee754_setcx(IEEE754_INEXACT);
        }
    }
    if xs != 0 {
        -(xm as i64)
    } else {
        xm as i64
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
