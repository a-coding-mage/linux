// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * single precision
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 * Copyright (C) 2017 Imagination Technologies, Ltd.
 * Author: Aleksandar Markovic <aleksandar.markovic@imgtec.com>
 */

// Dependency declarations and preprocessor definitions are supplied by ieee754sp.h.

pub unsafe fn ieee754sp_rint(mut x: ieee754sp) -> ieee754sp
{
    let mut ret: ieee754sp;
    let mut residue: u32;
    let mut sticky: i32;
    let mut round: i32;
    let mut odd: i32;

    // COMPXDP; /* <-- DP needed for 64-bit mantissa tmp */
    ieee754_clearcx();

    // EXPLODEXSP;
    // FLUSHXSP;

    if xc == IEEE754_CLASS_SNAN {
        return ieee754sp_nanxcpt(x);
    }

    if (xc == IEEE754_CLASS_QNAN) ||
       (xc == IEEE754_CLASS_INF) ||
       (xc == IEEE754_CLASS_ZERO)
    {
        return x;
    }

    if xe >= SP_FBITS {
        return x;
    }

    if xe < -1 {
        residue = xm as u32;
        round = 0;
        sticky = (residue != 0) as i32;
        xm = 0;
    } else {
        residue = (xm << (xe + 1)) as u32;
        residue <<= 31 - SP_FBITS;
        round = ((residue >> 31) != 0) as i32;
        sticky = ((residue << 1) != 0) as i32;
        xm >>= SP_FBITS - xe;
    }

    odd = ((xm & 0x1) != 0x0) as i32;

    match ieee754_csr.rm {
        FPU_CSR_RN => { // toward nearest
            if round != 0 && (sticky != 0 || odd != 0) {
                xm += 1;
            }
        }
        FPU_CSR_RZ => { // toward zero
        }
        FPU_CSR_RU => { // toward +infinity
            if (round != 0 || sticky != 0) && !xs {
                xm += 1;
            }
        }
        FPU_CSR_RD => { // toward -infinity
            if (round != 0 || sticky != 0) && xs {
                xm += 1;
            }
        }
        _ => {}
    }

    if round != 0 || sticky != 0 {
        ieee754_setcx(IEEE754_INEXACT);
    }

    ret = ieee754sp_flong(xm);
    // SPSIGN(ret) = xs;
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
