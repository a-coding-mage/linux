// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Linux/PA-RISC Project (http://www.parisc-linux.org/)
 *
 * Floating-point emulation code
 *  Copyright (C) 2001 Hewlett-Packard (Paul Bame) <bame@debian.org>
 */
/*
 * Single Fixed-point to Single Floating-point
 * Single Fixed-point to Double Floating-point
 * Double Fixed-point to Single Floating-point
 * Double Fixed-point to Double Floating-point
 */

pub unsafe fn sgl_to_sgl_fcnvxf(
    srcptr: *mut i32,
    _nullptr: *mut u32,
    dstptr: *mut sgl_floating_point,
    _status: *mut u32,
) -> i32 {
    let mut src: i32 = *srcptr;
    let mut dst_exponent: i32;
    let mut result: u32 = 0;

    if src < 0 {
        Sgl_setone_sign!(result);
        Int_negate!(src);
    } else {
        Sgl_setzero_sign!(result);
        if src == 0 {
            Sgl_setzero!(result);
            *dstptr = result;
            return NOEXCEPTION;
        }
    }

    dst_exponent = 16;
    Find_ms_one_bit!(src, dst_exponent);
    if dst_exponent >= 0 {
        src <<= dst_exponent;
    } else {
        src = 1 << 30;
    }
    Sgl_set_mantissa!(result, src >> (SGL_EXP_LENGTH - 1));
    Sgl_set_exponent!(result, 30 + SGL_BIAS - dst_exponent);

    if Int_isinexact_to_sgl!(src) {
        match Rounding_mode!() {
            ROUNDPLUS => {
                if Sgl_iszero_sign!(result) { Sgl_increment!(result); }
            }
            ROUNDMINUS => {
                if Sgl_isone_sign!(result) { Sgl_increment!(result); }
            }
            ROUNDNEAREST => Sgl_roundnearest_from_int!(src, result),
            _ => {}
        }
        if Is_inexacttrap_enabled!() {
            *dstptr = result;
            return INEXACTEXCEPTION;
        } else { Set_inexactflag!(); }
    }
    *dstptr = result;
    NOEXCEPTION
}

pub unsafe fn sgl_to_dbl_fcnvxf(
    srcptr: *mut i32,
    _nullptr: *mut u32,
    dstptr: *mut dbl_floating_point,
    _status: *mut u32,
) -> i32 {
    let mut src: i32 = *srcptr;
    let mut dst_exponent: i32;
    let mut resultp1: u32 = 0;
    let mut resultp2: u32 = 0;

    if src < 0 {
        Dbl_setone_sign!(resultp1);
        Int_negate!(src);
    } else {
        Dbl_setzero_sign!(resultp1);
        if src == 0 {
            Dbl_setzero!(resultp1, resultp2);
            Dbl_copytoptr!(resultp1, resultp2, dstptr);
            return NOEXCEPTION;
        }
    }
    dst_exponent = 16;
    Find_ms_one_bit!(src, dst_exponent);
    if dst_exponent >= 0 { src <<= dst_exponent; } else { src = 1 << 30; }
    Dbl_set_mantissap1!(resultp1, src >> (DBL_EXP_LENGTH - 1));
    Dbl_set_mantissap2!(resultp2, src << (33 - DBL_EXP_LENGTH));
    Dbl_set_exponent!(resultp1, 30 + DBL_BIAS - dst_exponent);
    Dbl_copytoptr!(resultp1, resultp2, dstptr);
    NOEXCEPTION
}

pub unsafe fn dbl_to_sgl_fcnvxf(
    srcptr: *mut dbl_integer,
    _nullptr: *mut u32,
    dstptr: *mut sgl_floating_point,
    _status: *mut u32,
) -> i32 {
    let mut dst_exponent: i32;
    let mut srcp1: i32;
    let mut srcp2: u32;
    let mut result: u32 = 0;

    Dint_copyfromptr!(srcptr, srcp1, srcp2);
    if srcp1 < 0 {
        Sgl_setone_sign!(result);
        Dint_negate!(srcp1, srcp2);
    } else {
        Sgl_setzero_sign!(result);
        if srcp1 == 0 && srcp2 == 0 {
            Sgl_setzero!(result);
            *dstptr = result;
            return NOEXCEPTION;
        }
    }
    dst_exponent = 16;
    if srcp1 == 0 {
        Find_ms_one_bit!(srcp2, dst_exponent);
        if dst_exponent >= 0 { srcp1 = (srcp2 as i32) << dst_exponent; srcp2 = 0; }
        else { srcp1 = (srcp2 >> 1) as i32; srcp2 <<= 31; }
        dst_exponent += 32;
    } else {
        Find_ms_one_bit!(srcp1, dst_exponent);
        if dst_exponent > 0 {
            Variable_shift_double!(srcp1, srcp2, 32 - dst_exponent, srcp1);
            srcp2 <<= dst_exponent;
        } else { srcp1 >>= -dst_exponent; }
    }
    Sgl_set_mantissa!(result, srcp1 >> (SGL_EXP_LENGTH - 1));
    Sgl_set_exponent!(result, 62 + SGL_BIAS - dst_exponent);
    if Dint_isinexact_to_sgl!(srcp1, srcp2) {
        match Rounding_mode!() {
            ROUNDPLUS => { if Sgl_iszero_sign!(result) { Sgl_increment!(result); } }
            ROUNDMINUS => { if Sgl_isone_sign!(result) { Sgl_increment!(result); } }
            ROUNDNEAREST => Sgl_roundnearest_from_dint!(srcp1, srcp2, result),
            _ => {}
        }
        if Is_inexacttrap_enabled!() { *dstptr = result; return INEXACTEXCEPTION; }
        else { Set_inexactflag!(); }
    }
    *dstptr = result;
    NOEXCEPTION
}

pub unsafe fn dbl_to_dbl_fcnvxf(
    srcptr: *mut dbl_integer,
    _nullptr: *mut u32,
    dstptr: *mut dbl_floating_point,
    _status: *mut u32,
) -> i32 {
    let mut srcp1: i32;
    let mut dst_exponent: i32;
    let mut srcp2: u32;
    let mut resultp1: u32 = 0;
    let mut resultp2: u32 = 0;

    Dint_copyfromptr!(srcptr, srcp1, srcp2);
    if srcp1 < 0 { Dbl_setone_sign!(resultp1); Dint_negate!(srcp1, srcp2); }
    else {
        Dbl_setzero_sign!(resultp1);
        if srcp1 == 0 && srcp2 == 0 { Dbl_setzero!(resultp1, resultp2); Dbl_copytoptr!(resultp1, resultp2, dstptr); return NOEXCEPTION; }
    }
    dst_exponent = 16;
    if srcp1 == 0 {
        Find_ms_one_bit!(srcp2, dst_exponent);
        if dst_exponent >= 0 { srcp1 = (srcp2 as i32) << dst_exponent; srcp2 = 0; }
        else { srcp1 = (srcp2 >> 1) as i32; srcp2 <<= 31; }
        dst_exponent += 32;
    } else {
        Find_ms_one_bit!(srcp1, dst_exponent);
        if dst_exponent > 0 { Variable_shift_double!(srcp1, srcp2, 32 - dst_exponent, srcp1); srcp2 <<= dst_exponent; }
        else { srcp1 >>= -dst_exponent; }
    }
    Dbl_set_mantissap1!(resultp1, srcp1 >> (DBL_EXP_LENGTH - 1));
    Shiftdouble!(srcp1, srcp2, DBL_EXP_LENGTH - 1, resultp2);
    Dbl_set_exponent!(resultp1, 62 + DBL_BIAS - dst_exponent);
    if Dint_isinexact_to_dbl!(srcp2) {
        match Rounding_mode!() {
            ROUNDPLUS => { if Dbl_iszero_sign!(resultp1) { Dbl_increment!(resultp1, resultp2); } }
            ROUNDMINUS => { if Dbl_isone_sign!(resultp1) { Dbl_increment!(resultp1, resultp2); } }
            ROUNDNEAREST => Dbl_roundnearest_from_dint!(srcp2, resultp1, resultp2),
            _ => {}
        }
        if Is_inexacttrap_enabled!() { Dbl_copytoptr!(resultp1, resultp2, dstptr); return INEXACTEXCEPTION; }
        else { Set_inexactflag!(); }
    }
    Dbl_copytoptr!(resultp1, resultp2, dstptr);
    NOEXCEPTION
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
