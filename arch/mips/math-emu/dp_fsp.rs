// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * double precision: common utilities
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependencies supplied by ieee754sp.h and ieee754dp.h are intentionally
// referenced here as external Rust items/macros.

#[inline]
unsafe fn ieee754dp_nan_fsp(xs: i32, xm: u64) -> ieee754dp {
    builddp(
        xs,
        DP_EMAX + 1 + DP_EBIAS,
        xm << (DP_FBITS - SP_FBITS),
    )
}

unsafe fn ieee754dp_fsp(x: ieee754sp) -> ieee754dp {
    COMPXSP!();

    EXPLODEXSP!();

    ieee754_clearcx();

    FLUSHXSP!();

    match xc {
        IEEE754_CLASS_SNAN => {
            ieee754dp_nanxcpt(ieee754dp_nan_fsp(xs, xm))
        }

        IEEE754_CLASS_QNAN => ieee754dp_nan_fsp(xs, xm),

        IEEE754_CLASS_INF => ieee754dp_inf(xs),

        IEEE754_CLASS_ZERO => ieee754dp_zero(xs),

        IEEE754_CLASS_DNORM => {
            // normalize
            while (xm >> SP_FBITS) == 0 {
                xm <<= 1;
                xe -= 1;
            }
            // Fall through to the common normalized-number conversion.
            xm &= !SP_HIDDEN_BIT;
            return builddp(
                xs,
                xe + DP_EBIAS,
                (xm as u64) << (DP_FBITS - SP_FBITS),
            );
        }

        IEEE754_CLASS_NORM => {}
    }

    /*
     * Can't possibly overflow,underflow, or need rounding
     */

    // drop the hidden bit
    xm &= !SP_HIDDEN_BIT;

    builddp(
        xs,
        xe + DP_EBIAS,
        (xm as u64) << (DP_FBITS - SP_FBITS),
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
