// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * double precision: common utilities
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependency declarations and preprocessor definitions are supplied by
// ieee754dp.h in the surrounding translation unit.

pub unsafe fn ieee754dp_cmp(
    mut x: ieee754dp,
    mut y: ieee754dp,
    cmp: i32,
    sig: i32,
) -> i32 {
    let mut vx: i64;
    let mut vy: i64;

    // C macros: COMPXDP; COMPYDP;
    COMPXDP!();
    COMPYDP!();

    // C macros: EXPLODEXDP; EXPLODEYDP;
    EXPLODEXDP!();
    EXPLODEYDP!();
    // C macros: FLUSHXDP; FLUSHYDP;
    FLUSHXDP!();
    FLUSHYDP!();
    ieee754_clearcx(); // Even clear inexact flag here

    if ieee754_class_nan(xc) || ieee754_class_nan(yc) {
        if sig != 0 || xc == IEEE754_CLASS_SNAN || yc == IEEE754_CLASS_SNAN {
            ieee754_setcx(IEEE754_INVALID_OPERATION);
        }
        return if (cmp & IEEE754_CUN) != 0 { 1 } else { 0 };
    } else {
        vx = x.bits;
        vy = y.bits;

        if vx < 0 {
            vx = (-vx) ^ DP_SIGN_BIT;
        }
        if vy < 0 {
            vy = (-vy) ^ DP_SIGN_BIT;
        }

        if vx < vy {
            return if (cmp & IEEE754_CLT) != 0 { 1 } else { 0 };
        } else if vx == vy {
            return if (cmp & IEEE754_CEQ) != 0 { 1 } else { 0 };
        } else {
            return if (cmp & IEEE754_CGT) != 0 { 1 } else { 0 };
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
