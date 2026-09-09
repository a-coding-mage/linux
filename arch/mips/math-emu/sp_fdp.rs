// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * single precision
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependencies supplied by the corresponding IEEE754 single- and double-
// precision implementation are intentionally left external.

#[inline]
fn ieee754sp_nan_fdp(xs: i32, xm: u64) -> ieee754sp {
    buildsp(xs, SP_EMAX + 1 + SP_EBIAS, xm >> (DP_FBITS - SP_FBITS))
}

pub fn ieee754sp_fdp(mut x: ieee754dp) -> ieee754sp {
    let mut y: ieee754sp;
    let mut rm: u32;

    COMPXDP!();
    COMPYSP!();

    EXPLODEXDP!();

    ieee754_clearcx();

    FLUSHXDP!();

    match xc {
        IEEE754_CLASS_SNAN => {
            x = ieee754dp_nanxcpt(x);
            EXPLODEXDP!();
            // C fallthrough: continue with the quiet-NaN handling below.
            y = ieee754sp_nan_fdp(xs, xm);
            if !ieee754_csr.nan2008 {
                EXPLODEYSP!();
                if !ieee754_class_nan(yc) {
                    y = ieee754sp_indef();
                }
            }
            y
        }

        IEEE754_CLASS_QNAN => {
            y = ieee754sp_nan_fdp(xs, xm);
            if !ieee754_csr.nan2008 {
                EXPLODEYSP!();
                if !ieee754_class_nan(yc) {
                    y = ieee754sp_indef();
                }
            }
            y
        }

        IEEE754_CLASS_INF => ieee754sp_inf(xs),

        IEEE754_CLASS_ZERO => ieee754sp_zero(xs),

        IEEE754_CLASS_DNORM => {
            /* can't possibly be sp representable */
            ieee754_setcx(IEEE754_UNDERFLOW);
            ieee754_setcx(IEEE754_INEXACT);
            if (ieee754_csr.rm == FPU_CSR_RU && xs == 0)
                || (ieee754_csr.rm == FPU_CSR_RD && xs != 0)
            {
                ieee754sp_mind(xs)
            } else {
                ieee754sp_zero(xs)
            }
        }

        IEEE754_CLASS_NORM => {
            /* Convert from DP_FBITS to SP_FBITS+3 with sticky right shift. */
            rm = (xm >> (DP_FBITS - (SP_FBITS + 3))) as u32
                | u32::from((xm << (64 - (DP_FBITS - (SP_FBITS + 3)))) != 0);

            ieee754sp_format(xs, xe, rm)
        }

        _ => unreachable!(),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
