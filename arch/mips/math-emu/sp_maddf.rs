// SPDX-License-Identifier: GPL-2.0-only
/*
 * IEEE754 floating point arithmetic
 * single precision: MADDF.f (Fused Multiply Add)
 * MADDF.fmt: FPR[fd] = FPR[fd] + (FPR[fs] x FPR[ft])
 *
 * MIPS floating point support
 * Copyright (C) 2015 Imagination Technologies, Ltd.
 * Author: Markos Chandras <markos.chandras@imgtec.com>
 */

unsafe fn _sp_maddf(
    mut z: ieee754sp,
    mut x: ieee754sp,
    mut y: ieee754sp,
    flags: maddf_flags,
) -> ieee754sp {
    let mut re: i32;
    let mut rs: i32;
    let mut rm: u32;
    let mut rm64: u64;
    let mut zm64: u64;
    let mut s: i32;

    COMPXSP;
    COMPYSP;
    COMPZSP;

    EXPLODEXSP;
    EXPLODEYSP;
    EXPLODEZSP;

    FLUSHXSP;
    FLUSHYSP;
    FLUSHZSP;

    ieee754_clearcx();

    rs = xs ^ ys;
    if flags & MADDF_NEGATE_PRODUCT != 0 {
        rs ^= 1;
    }
    if flags & MADDF_NEGATE_ADDITION != 0 {
        zs ^= 1;
    }

    /*
     * Handle the cases when at least one of x, y or z is a NaN.
     * Order of precedence is sNaN, qNaN and z, x, y.
     */
    if zc == IEEE754_CLASS_SNAN {
        return ieee754sp_nanxcpt(z);
    }
    if xc == IEEE754_CLASS_SNAN {
        return ieee754sp_nanxcpt(x);
    }
    if yc == IEEE754_CLASS_SNAN {
        return ieee754sp_nanxcpt(y);
    }
    if zc == IEEE754_CLASS_QNAN {
        return z;
    }
    if xc == IEEE754_CLASS_QNAN {
        return x;
    }
    if yc == IEEE754_CLASS_QNAN {
        return y;
    }

    if zc == IEEE754_CLASS_DNORM {
        SPDNORMZ;
    }

    match CLPAIR(xc, yc) {
        CLPAIR(IEEE754_CLASS_INF, IEEE754_CLASS_ZERO)
        | CLPAIR(IEEE754_CLASS_ZERO, IEEE754_CLASS_INF) => {
            ieee754_setcx(IEEE754_INVALID_OPERATION);
            return ieee754sp_indef();
        }

        CLPAIR(IEEE754_CLASS_NORM, IEEE754_CLASS_INF)
        | CLPAIR(IEEE754_CLASS_DNORM, IEEE754_CLASS_INF)
        | CLPAIR(IEEE754_CLASS_INF, IEEE754_CLASS_NORM)
        | CLPAIR(IEEE754_CLASS_INF, IEEE754_CLASS_DNORM)
        | CLPAIR(IEEE754_CLASS_INF, IEEE754_CLASS_INF) => {
            if zc == IEEE754_CLASS_INF && zs != rs {
                ieee754_setcx(IEEE754_INVALID_OPERATION);
                return ieee754sp_indef();
            }
            return ieee754sp_inf(rs);
        }

        CLPAIR(IEEE754_CLASS_ZERO, IEEE754_CLASS_ZERO)
        | CLPAIR(IEEE754_CLASS_ZERO, IEEE754_CLASS_NORM)
        | CLPAIR(IEEE754_CLASS_ZERO, IEEE754_CLASS_DNORM)
        | CLPAIR(IEEE754_CLASS_NORM, IEEE754_CLASS_ZERO)
        | CLPAIR(IEEE754_CLASS_DNORM, IEEE754_CLASS_ZERO) => {
            if zc == IEEE754_CLASS_INF {
                return ieee754sp_inf(zs);
            }
            if zc == IEEE754_CLASS_ZERO {
                if zs == rs {
                    return z;
                }
                return ieee754sp_zero(ieee754_csr.rm == FPU_CSR_RD);
            }
            return z;
        }

        CLPAIR(IEEE754_CLASS_DNORM, IEEE754_CLASS_DNORM) => {
            SPDNORMX;
            SPDNORMY;
        }
        CLPAIR(IEEE754_CLASS_NORM, IEEE754_CLASS_DNORM) => {
            if zc == IEEE754_CLASS_INF {
                return ieee754sp_inf(zs);
            }
            SPDNORMY;
        }
        CLPAIR(IEEE754_CLASS_DNORM, IEEE754_CLASS_NORM) => {
            if zc == IEEE754_CLASS_INF {
                return ieee754sp_inf(zs);
            }
            SPDNORMX;
        }
        CLPAIR(IEEE754_CLASS_NORM, IEEE754_CLASS_NORM) => {
            if zc == IEEE754_CLASS_INF {
                return ieee754sp_inf(zs);
            }
        }
        _ => {}
    }

    assert!(xm & SP_HIDDEN_BIT != 0);
    assert!(ym & SP_HIDDEN_BIT != 0);

    re = xe + ye;
    rm64 = (xm as u64) * (ym as u64);
    rm64 <<= 16;

    if (rm64 as i64) < 0 {
        rm64 >>= 1;
        re += 1;
    }

    assert!(rm64 & (1u64 << 62) != 0);

    if zc == IEEE754_CLASS_ZERO {
        rm = XSPSRS64(rm64, 62 - 26);
        return ieee754sp_format(rs, re, rm);
    }

    zm64 = (zm as u64) << (62 - 23);
    assert!(zm64 & (1u64 << 62) != 0);

    if ze > re {
        s = ze - re;
        rm64 = XSPSRS64(rm64, s);
        re += s;
    } else if re > ze {
        s = re - ze;
        zm64 = XSPSRS64(zm64, s);
        ze += s;
    }
    assert!(ze == re);
    assert!(ze <= SP_EMAX);

    if zs == rs {
        zm64 += rm64;
        if (zm64 as i64) < 0 {
            zm64 = XSPSRS1(zm64);
            ze += 1;
        }
    } else {
        if zm64 >= rm64 {
            zm64 -= rm64;
        } else {
            zm64 = rm64 - zm64;
            zs = rs;
        }
        if zm64 == 0 {
            return ieee754sp_zero(ieee754_csr.rm == FPU_CSR_RD);
        }
        while (zm64 >> 62) == 0 {
            zm64 <<= 1;
            ze -= 1;
        }
    }

    zm = XSPSRS64(zm64, 62 - 26);
    ieee754sp_format(zs, ze, zm)
}

unsafe fn ieee754sp_maddf(z: ieee754sp, x: ieee754sp, y: ieee754sp) -> ieee754sp {
    _sp_maddf(z, x, y, 0)
}

unsafe fn ieee754sp_msubf(z: ieee754sp, x: ieee754sp, y: ieee754sp) -> ieee754sp {
    _sp_maddf(z, x, y, MADDF_NEGATE_PRODUCT)
}

unsafe fn ieee754sp_madd(z: ieee754sp, x: ieee754sp, y: ieee754sp) -> ieee754sp {
    _sp_maddf(z, x, y, 0)
}

unsafe fn ieee754sp_msub(z: ieee754sp, x: ieee754sp, y: ieee754sp) -> ieee754sp {
    _sp_maddf(z, x, y, MADDF_NEGATE_ADDITION)
}

unsafe fn ieee754sp_nmadd(z: ieee754sp, x: ieee754sp, y: ieee754sp) -> ieee754sp {
    _sp_maddf(z, x, y, MADDF_NEGATE_PRODUCT | MADDF_NEGATE_ADDITION)
}

unsafe fn ieee754sp_nmsub(z: ieee754sp, x: ieee754sp, y: ieee754sp) -> ieee754sp {
    _sp_maddf(z, x, y, MADDF_NEGATE_PRODUCT)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
