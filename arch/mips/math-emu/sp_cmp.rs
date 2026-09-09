// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * single precision
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependency declarations supplied by the surrounding IEEE754 implementation.
use crate::ieee754sp::*;

pub unsafe fn ieee754sp_cmp(
    x: ieee754sp,
    y: ieee754sp,
    cmp: i32,
    sig: i32,
) -> i32 {
    let mut vx: i32;
    let mut vy: i32;

    // C macros from ieee754sp.h; these establish the component/class locals.
    COMPXSP!(x);
    COMPYSP!(y);

    EXPLODEXSP!();
    EXPLODEYSP!();
    FLUSHXSP!();
    FLUSHYSP!();
    ieee754_clearcx(); /* Even clear inexact flag here */

    if ieee754_class_nan(xc) || ieee754_class_nan(yc) {
        if sig != 0
            || xc == IEEE754_CLASS_SNAN
            || yc == IEEE754_CLASS_SNAN
        {
            ieee754_setcx(IEEE754_INVALID_OPERATION);
        }
        return if (cmp & IEEE754_CUN) != 0 { 1 } else { 0 };
    } else {
        vx = (*(&x)).bits as i32;
        vy = (*(&y)).bits as i32;

        if vx < 0 {
            vx = (-vx) ^ (SP_SIGN_BIT as i32);
        }
        if vy < 0 {
            vy = (-vy) ^ (SP_SIGN_BIT as i32);
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
