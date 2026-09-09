// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * double precision square root
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependency declarations and build-time definitions are supplied by ieee754dp.h.

static TABLE: [u32; 32] = [
    0, 1204, 3062, 5746, 9193, 13348, 18162, 23592,
    29598, 36145, 43202, 50740, 58733, 67158, 75992,
    85215, 83599, 71378, 60428, 50647, 41945, 34246,
    27478, 21581, 16499, 12183, 8588, 5674, 3403,
    1742, 661, 130,
];

pub unsafe fn ieee754dp_sqrt(mut x: ieee754dp) -> ieee754dp {
    let mut oldcsr: _ieee754_csr;
    let mut y: ieee754dp;
    let mut z: ieee754dp;
    let mut t: ieee754dp;
    let mut scalx: u32;
    let mut yh: u32;

    // COMPXDP and EXPLODEXDP provide xc, xs, xe, and xm from the external ABI.
    compxdp!(x);
    explodexdp!(x);
    ieee754_clearcx();
    flushxdp!();

    match xc {
        IEEE754_CLASS_SNAN => return ieee754dp_nanxcpt(x),
        IEEE754_CLASS_QNAN => return x,
        IEEE754_CLASS_ZERO => return x,
        IEEE754_CLASS_INF => {
            if xs != 0 {
                ieee754_setcx(IEEE754_INVALID_OPERATION);
                return ieee754dp_indef();
            }
            return x;
        }
        IEEE754_CLASS_DNORM => {
            dpdnormx!();
        }
        IEEE754_CLASS_NORM => {}
        _ => {}
    }

    if xs != 0 {
        ieee754_setcx(IEEE754_INVALID_OPERATION);
        return ieee754dp_indef();
    }

    oldcsr = ieee754_csr;
    ieee754_csr.mx &= !IEEE754_INEXACT;
    ieee754_csr.sx &= !IEEE754_INEXACT;
    ieee754_csr.rm = FPU_CSR_RN;

    scalx = 0;
    if xe > 512 {
        xe -= 512;
        scalx += 256;
    } else if xe < -512 {
        xe += 512;
        scalx -= 256;
    }

    x = builddp(0, xe + DP_EBIAS, xm & !DP_HIDDEN_BIT);
    y = x;

    yh = y.bits >> 32;
    yh = (yh >> 1) + 0x1ff80000;
    yh -= TABLE[((yh >> 15) & 31) as usize];
    y.bits = ((yh as u64) << 32) | (y.bits & 0xffff_ffff);

    t = ieee754dp_div(x, y);
    y = ieee754dp_add(y, t);
    y.bits -= 0x0010_0006_0000_0000u64;
    y.bits &= 0xffff_ffff_0000_0000u64;

    t = ieee754dp_mul(y, y);
    z = t;
    t.bexp += 0x001;
    t = ieee754dp_add(t, z);
    z = ieee754dp_mul(ieee754dp_sub(x, z), y);

    t = ieee754dp_div(z, ieee754dp_add(t, x));
    t.bexp += 0x001;
    y = ieee754dp_add(y, t);

    ieee754_csr.rm = FPU_CSR_RZ;
    ieee754_csr.sx &= !IEEE754_INEXACT;
    t = ieee754dp_div(x, y);

    if (ieee754_csr.sx & IEEE754_INEXACT != 0 || t.bits != y.bits) {
        if ieee754_csr.sx & IEEE754_INEXACT == 0 {
            t.bits -= 1;
        }
        oldcsr.cx |= IEEE754_INEXACT;
        oldcsr.sx |= IEEE754_INEXACT;

        match oldcsr.rm {
            FPU_CSR_RU => {
                y.bits += 1;
                t.bits += 1;
            }
            FPU_CSR_RN => t.bits += 1,
            _ => {}
        }

        y = ieee754dp_add(y, t);
        scalx -= 1;
    }

    y.bexp += scalx;
    ieee754_csr = oldcsr;
    y
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
