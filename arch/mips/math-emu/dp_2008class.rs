// SPDX-License-Identifier: GPL-2.0-only
/*
 * IEEE754 floating point arithmetic
 * double precision: CLASS.f
 * FPR[fd] = class(FPR[fs])
 *
 * MIPS floating point support
 * Copyright (C) 2015 Imagination Technologies, Ltd.
 * Author: Markos Chandras <markos.chandras@imgtec.com>
 */

// Dependency supplied by the surrounding IEEE754 double-precision implementation.

pub unsafe fn ieee754dp_2008class(x: ieee754dp) -> i32 {
    // COMPXDP;
    COMPXDP!(x);

    // EXPLODEXDP;
    EXPLODEXDP!();

    /*
     * 10 bit mask as follows:
     *
     * bit0 = SNAN
     * bit1 = QNAN
     * bit2 = -INF
     * bit3 = -NORM
     * bit4 = -DNORM
     * bit5 = -ZERO
     * bit6 = INF
     * bit7 = NORM
     * bit8 = DNORM
     * bit9 = ZERO
     */

    match xc {
        IEEE754_CLASS_SNAN => 0x01,
        IEEE754_CLASS_QNAN => 0x02,
        IEEE754_CLASS_INF => 0x04 << (if xs { 0 } else { 4 }),
        IEEE754_CLASS_NORM => 0x08 << (if xs { 0 } else { 4 }),
        IEEE754_CLASS_DNORM => 0x10 << (if xs { 0 } else { 4 }),
        IEEE754_CLASS_ZERO => 0x20 << (if xs { 0 } else { 4 }),
        _ => {
            pr_err!("Unknown class: %d\n", xc);
            0
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
