// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * double precision: common utilities
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// Dependency intent preserved from linux/compiler.h and ieee754dp.h.

pub unsafe fn ieee754dp_class(x: ieee754dp) -> i32 {
    COMPXDP!(x);
    EXPLODEXDP!(x);
    xc
}

unsafe fn ieee754dp_isnan(x: ieee754dp) -> i32 {
    ieee754_class_nan(ieee754dp_class(x))
}

unsafe fn ieee754dp_issnan(x: ieee754dp) -> i32 {
    let qbit: i32;

    assert!(ieee754dp_isnan(x) != 0);
    qbit = if (DPMANT!(x) & DP_MBIT!(DP_FBITS - 1)) == DP_MBIT!(DP_FBITS - 1) {
        1
    } else {
        0
    };
    ieee754_csr.nan2008 ^ qbit
}

/*
 * Raise the Invalid Operation IEEE 754 exception
 * and convert the signaling NaN supplied to a quiet NaN.
 */
pub unsafe fn ieee754dp_nanxcpt(mut r: ieee754dp) -> ieee754dp {
    assert!(ieee754dp_issnan(r) != 0);

    ieee754_setcx(IEEE754_INVALID_OPERATION);
    if ieee754_csr.nan2008 != 0 {
        DPMANT!(r) |= DP_MBIT!(DP_FBITS - 1);
    } else {
        DPMANT!(r) &= !DP_MBIT!(DP_FBITS - 1);
        if ieee754dp_isnan(r) == 0 {
            DPMANT!(r) |= DP_MBIT!(DP_FBITS - 2);
        }
    }

    r
}

unsafe fn ieee754dp_get_rounding(sn: i32, mut xm: u64) -> u64 {
    /* inexact must round of 3 bits
     */
    if xm & (DP_MBIT!(3) - 1) != 0 {
        match ieee754_csr.rm {
            FPU_CSR_RZ => {}
            FPU_CSR_RN => {
                xm += 0x3 + ((xm >> 3) & 1);
                /* xm += (xm&0x8)?0x4:0x3 */
            }
            FPU_CSR_RU => { /* toward +Infinity */
                if sn == 0 { /* ?? */
                    xm += 0x8;
                }
            }
            FPU_CSR_RD => { /* toward -Infinity */
                if sn != 0 { /* ?? */
                    xm += 0x8;
                }
            }
            _ => {}
        }
    }
    xm
}

/* generate a normal/denormal number with over,under handling
 * sn is sign
 * xe is an unbiased exponent
 * xm is 3bit extended precision value.
 */
pub unsafe fn ieee754dp_format(sn: i32, mut xe: i32, mut xm: u64) -> ieee754dp {
    assert!(xm != 0); /* we don't gen exact zeros (probably should) */

    assert!((xm >> (DP_FBITS + 1 + 3)) == 0); /* no excess */
    assert!(xm & (DP_HIDDEN_BIT << 3) != 0);

    if xe < DP_EMIN {
        /* strip lower bits */
        let es = DP_EMIN - xe;

        if ieee754_csr.nod != 0 {
            ieee754_setcx(IEEE754_UNDERFLOW);
            ieee754_setcx(IEEE754_INEXACT);

            match ieee754_csr.rm {
                FPU_CSR_RN | FPU_CSR_RZ => return ieee754dp_zero(sn),
                FPU_CSR_RU => { /* toward +Infinity */
                    if sn == 0 {
                        return ieee754dp_min(0);
                    } else {
                        return ieee754dp_zero(1);
                    }
                }
                FPU_CSR_RD => { /* toward -Infinity */
                    if sn == 0 {
                        return ieee754dp_zero(0);
                    } else {
                        return ieee754dp_min(1);
                    }
                }
                _ => {}
            }
        }

        if xe == DP_EMIN - 1
            && (ieee754dp_get_rounding(sn, xm) >> (DP_FBITS + 1 + 3)) != 0
        {
            /* Not tiny after rounding */
            ieee754_setcx(IEEE754_INEXACT);
            xm = ieee754dp_get_rounding(sn, xm);
            xm >>= 1;
            /* Clear grs bits */
            xm &= !(DP_MBIT!(3) - 1);
            xe += 1;
        } else {
            /* sticky right shift es bits
             */
            xm = XDPSRS!(xm, es);
            xe += es;
            assert!(xm & (DP_HIDDEN_BIT << 3) == 0);
            assert!(xe == DP_EMIN);
        }
    }
    if xm & (DP_MBIT!(3) - 1) != 0 {
        ieee754_setcx(IEEE754_INEXACT);
        if xm & (DP_HIDDEN_BIT << 3) == 0 {
            ieee754_setcx(IEEE754_UNDERFLOW);
        }

        /* inexact must round of 3 bits
         */
        xm = ieee754dp_get_rounding(sn, xm);
        /* adjust exponent for rounding add overflowing
         */
        if xm >> (DP_FBITS + 3 + 1) != 0 {
            /* add causes mantissa overflow */
            xm >>= 1;
            xe += 1;
        }
    }
    /* strip grs bits */
    xm >>= 3;

    assert!(xm >> (DP_FBITS + 1) == 0); /* no excess */
    assert!(xe >= DP_EMIN);

    if xe > DP_EMAX {
        ieee754_setcx(IEEE754_OVERFLOW);
        ieee754_setcx(IEEE754_INEXACT);
        /* -O can be table indexed by (rm,sn) */
        match ieee754_csr.rm {
            FPU_CSR_RN => return ieee754dp_inf(sn),
            FPU_CSR_RZ => return ieee754dp_max(sn),
            FPU_CSR_RU => { /* toward +Infinity */
                if sn == 0 {
                    return ieee754dp_inf(0);
                } else {
                    return ieee754dp_max(1);
                }
            }
            FPU_CSR_RD => { /* toward -Infinity */
                if sn == 0 {
                    return ieee754dp_max(0);
                } else {
                    return ieee754dp_inf(1);
                }
            }
            _ => {}
        }
    }
    /* gen norm/denorm/zero */

    if xm & DP_HIDDEN_BIT == 0 {
        /* we underflow (tiny/zero) */
        assert!(xe == DP_EMIN);
        if ieee754_csr.mx & IEEE754_UNDERFLOW != 0 {
            ieee754_setcx(IEEE754_UNDERFLOW);
        }
        builddp(sn, DP_EMIN - 1 + DP_EBIAS, xm)
    } else {
        assert!(xm >> (DP_FBITS + 1) == 0); /* no excess */
        assert!(xm & DP_HIDDEN_BIT != 0);

        builddp(sn, xe + DP_EBIAS, xm & !DP_HIDDEN_BIT)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
