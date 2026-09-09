// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * single precision
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// The C source includes ieee754sp.h; its declarations and macro definitions
// are supplied by the surrounding translation unit.

pub fn ieee754sp_tint(x: ieee754sp) -> i32 {
    let mut residue: u32;
    let mut round: i32;
    let mut sticky: i32;
    let mut odd: i32;

    COMPXSP!(x);

    ieee754_clearcx();

    EXPLODEXSP!();
    FLUSHXSP!();

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
    }
    if xe >= 31 {
        /* look for valid corner case */
        if xe == 31 && xs && xm == SP_HIDDEN_BIT {
            return i32::MIN;
        }
        /* Set invalid. We will only use overflow for floating
           point overflow */
        ieee754_setcx(IEEE754_INVALID_OPERATION);
        return ieee754si_overflow(xs);
    }
    /* oh gawd */
    if xe > SP_FBITS {
        xm <<= (xe - SP_FBITS) as u32;
    } else {
        if xe < -1 {
            residue = xm;
            round = 0;
            sticky = (residue != 0) as i32;
            xm = 0;
        } else {
            /* Shifting a u32 32 times does not work,
             * so we do it in two steps. Be aware that xe
             * may be -1 */
            residue = xm << (xe + 1) as u32;
            residue <<= (31 - SP_FBITS) as u32;
            round = ((residue >> 31) != 0) as i32;
            sticky = ((residue << 1) != 0) as i32;
            xm >>= (SP_FBITS - xe) as u32;
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
                if (round != 0 || sticky != 0) && !xs {
                    xm = xm.wrapping_add(1);
                }
            }
            FPU_CSR_RD => {
                /* toward -Infinity */
                if (round != 0 || sticky != 0) && xs {
                    xm = xm.wrapping_add(1);
                }
            }
            _ => {}
        }
        if (xm >> 31) != 0 {
            /* This can happen after rounding */
            ieee754_setcx(IEEE754_INVALID_OPERATION);
            return ieee754si_overflow(xs);
        }
        if round != 0 || sticky != 0 {
            ieee754_setcx(IEEE754_INEXACT);
        }
    }
    if xs {
        -(xm as i32)
    } else {
        xm as i32
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
