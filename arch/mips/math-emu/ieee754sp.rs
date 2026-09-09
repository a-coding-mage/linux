// SPDX-License-Identifier: GPL-2.0-only
/* IEEE754 floating point arithmetic
 * single precision
 */
/*
 * MIPS floating point support
 * Copyright (C) 1994-2000 Algorithmics Ltd.
 */

// linux/compiler.h and ieee754sp.h provide the macros, constants, types,
// globals, and helper functions referenced below.

pub fn ieee754sp_class(x: ieee754sp) -> i32 {
    COMPXSP!();
    EXPLODEXSP!();
    xc
}

fn ieee754sp_isnan(x: ieee754sp) -> i32 {
    ieee754_class_nan(ieee754sp_class(x))
}

fn ieee754sp_issnan(x: ieee754sp) -> i32 {
    let qbit: i32;

    assert!(ieee754sp_isnan(x) != 0);
    qbit = ((SPMANT!(x) & SP_MBIT!(SP_FBITS - 1)) == SP_MBIT!(SP_FBITS - 1)) as i32;
    ieee754_csr.nan2008 ^ qbit
}

/*
 * Raise the Invalid Operation IEEE 754 exception
 * and convert the signaling NaN supplied to a quiet NaN.
 */
pub fn ieee754sp_nanxcpt(mut r: ieee754sp) -> ieee754sp {
    assert!(ieee754sp_issnan(r) != 0);

    ieee754_setcx(IEEE754_INVALID_OPERATION);
    if ieee754_csr.nan2008 != 0 {
        SPMANT!(r) |= SP_MBIT!(SP_FBITS - 1);
    } else {
        SPMANT!(r) &= !SP_MBIT!(SP_FBITS - 1);
        if ieee754sp_isnan(r) == 0 {
            SPMANT!(r) |= SP_MBIT!(SP_FBITS - 2);
        }
    }

    r
}

fn ieee754sp_get_rounding(sn: i32, mut xm: u32) -> u32 {
    /* inexact must round of 3 bits
     */
    if xm & (SP_MBIT!(3) - 1) != 0 {
        match ieee754_csr.rm {
            FPU_CSR_RZ => {}
            FPU_CSR_RN => {
                xm += 0x3 + ((xm >> 3) & 1);
                /* xm += (xm&0x8)?0x4:0x3 */
            }
            FPU_CSR_RU => {
                /* toward +Infinity */
                if sn == 0 {
                    xm += 0x8;
                }
            }
            FPU_CSR_RD => {
                /* toward -Infinity */
                if sn != 0 {
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
pub fn ieee754sp_format(sn: i32, mut xe: i32, mut xm: u32) -> ieee754sp {
    assert!(xm != 0); /* we don't gen exact zeros (probably should) */

    assert!((xm >> (SP_FBITS + 1 + 3)) == 0); /* no excess */
    assert!(xm & (SP_HIDDEN_BIT << 3) != 0);

    if xe < SP_EMIN {
        /* strip lower bits */
        let es = SP_EMIN - xe;

        if ieee754_csr.nod != 0 {
            ieee754_setcx(IEEE754_UNDERFLOW);
            ieee754_setcx(IEEE754_INEXACT);

            match ieee754_csr.rm {
                FPU_CSR_RN | FPU_CSR_RZ => return ieee754sp_zero(sn),
                FPU_CSR_RU => {
                    /* toward +Infinity */
                    if sn == 0 {
                        return ieee754sp_min(0);
                    } else {
                        return ieee754sp_zero(1);
                    }
                }
                FPU_CSR_RD => {
                    /* toward -Infinity */
                    if sn == 0 {
                        return ieee754sp_zero(0);
                    } else {
                        return ieee754sp_min(1);
                    }
                }
                _ => {}
            }
        }

        if xe == SP_EMIN - 1
            && ieee754sp_get_rounding(sn, xm) >> (SP_FBITS + 1 + 3) != 0
        {
            /* Not tiny after rounding */
            ieee754_setcx(IEEE754_INEXACT);
            xm = ieee754sp_get_rounding(sn, xm);
            xm >>= 1;
            /* Clear grs bits */
            xm &= !(SP_MBIT!(3) - 1);
            xe += 1;
        } else {
            /* sticky right shift es bits
             */
            xm = XSPSRS!(xm, es);
            xe += es;
            assert!(xm & (SP_HIDDEN_BIT << 3) == 0);
            assert!(xe == SP_EMIN);
        }
    }
    if xm & (SP_MBIT!(3) - 1) != 0 {
        ieee754_setcx(IEEE754_INEXACT);
        if xm & (SP_HIDDEN_BIT << 3) == 0 {
            ieee754_setcx(IEEE754_UNDERFLOW);
        }

        /* inexact must round of 3 bits
         */
        xm = ieee754sp_get_rounding(sn, xm);
        /* adjust exponent for rounding add overflowing
         */
        if xm >> (SP_FBITS + 1 + 3) != 0 {
            /* add causes mantissa overflow */
            xm >>= 1;
            xe += 1;
        }
    }
    /* strip grs bits */
    xm >>= 3;

    assert!((xm >> (SP_FBITS + 1)) == 0); /* no excess */
    assert!(xe >= SP_EMIN);

    if xe > SP_EMAX {
        ieee754_setcx(IEEE754_OVERFLOW);
        ieee754_setcx(IEEE754_INEXACT);
        /* -O can be table indexed by (rm,sn) */
        match ieee754_csr.rm {
            FPU_CSR_RN => return ieee754sp_inf(sn),
            FPU_CSR_RZ => return ieee754sp_max(sn),
            FPU_CSR_RU => {
                /* toward +Infinity */
                if sn == 0 {
                    return ieee754sp_inf(0);
                } else {
                    return ieee754sp_max(1);
                }
            }
            FPU_CSR_RD => {
                /* toward -Infinity */
                if sn == 0 {
                    return ieee754sp_max(0);
                } else {
                    return ieee754sp_inf(1);
                }
            }
            _ => {}
        }
    }
    /* gen norm/denorm/zero */

    if xm & SP_HIDDEN_BIT == 0 {
        /* we underflow (tiny/zero) */
        assert!(xe == SP_EMIN);
        if ieee754_csr.mx & IEEE754_UNDERFLOW != 0 {
            ieee754_setcx(IEEE754_UNDERFLOW);
        }
        buildsp(sn, SP_EMIN - 1 + SP_EBIAS, xm)
    } else {
        assert!((xm >> (SP_FBITS + 1)) == 0); /* no excess */
        assert!(xm & SP_HIDDEN_BIT != 0);

        buildsp(sn, xe + SP_EBIAS, xm & !SP_HIDDEN_BIT)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
