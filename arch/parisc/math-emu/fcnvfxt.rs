// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Linux/PA-RISC Project (http://www.parisc-linux.org/)
 *
 * Floating-point emulation code
 *  Copyright (C) 2001 Hewlett-Packard (Paul Bame) <bame@debian.org>
 */
/*
 * Single Floating-point to Single/Double Fixed-point, and
 * Double Floating-point to Single/Double Fixed-point, with truncated result.
 *
 * C headers supplied by the surrounding translation unit provide the types,
 * constants, and helper macros/functions referenced below.
 */

pub unsafe fn sgl_to_sgl_fcnvfxt(
    srcptr: *mut sgl_floating_point,
    _nullptr: *mut u32,
    dstptr: *mut i32,
    _status: *mut u32,
) -> i32 {
    let src: u32 = *srcptr;
    let src_exponent: i32 = Sgl_exponent(src) - SGL_BIAS;
    let mut result: i32;

    if src_exponent > SGL_FX_MAX_EXP {
        if src_exponent > SGL_FX_MAX_EXP + 1
            || Sgl_isnotzero_mantissa(src)
            || Sgl_iszero_sign(src)
        {
            if Sgl_iszero_sign(src) { result = 0x7fffffff; }
            else { result = 0x80000000u32 as i32; }
            if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
            Set_invalidflag();
            *dstptr = result;
            return NOEXCEPTION;
        }
    }
    if src_exponent >= 0 {
        let mut temp = src;
        Sgl_clear_signexponent_set_hidden(temp);
        Int_from_sgl_mantissa(temp, src_exponent);
        if Sgl_isone_sign(src) { result = -Sgl_all(temp); }
        else { result = Sgl_all(temp); }
        *dstptr = result;
        if Sgl_isinexact_to_fix(src, src_exponent) {
            if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; }
            else { Set_inexactflag(); }
        }
    } else {
        *dstptr = 0;
        if Sgl_isnotzero_exponentmantissa(src) {
            if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; }
            else { Set_inexactflag(); }
        }
    }
    NOEXCEPTION
}

pub unsafe fn sgl_to_dbl_fcnvfxt(
    srcptr: *mut sgl_floating_point, _nullptr: *mut u32,
    dstptr: *mut dbl_integer, _status: *mut u32,
) -> i32 {
    let src = *srcptr;
    let src_exponent = Sgl_exponent(src) - SGL_BIAS;
    let mut resultp1: i32;
    let mut resultp2: u32;
    if src_exponent > DBL_FX_MAX_EXP {
        if src_exponent > DBL_FX_MAX_EXP + 1 || Sgl_isnotzero_mantissa(src) || Sgl_iszero_sign(src) {
            if Sgl_iszero_sign(src) { resultp1 = 0x7fffffff; resultp2 = 0xffffffff; }
            else { resultp1 = 0x80000000u32 as i32; resultp2 = 0; }
            if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
            Set_invalidflag(); Dint_copytoptr(resultp1, resultp2, dstptr); return NOEXCEPTION;
        }
        Dint_set_minint(resultp1, resultp2); Dint_copytoptr(resultp1, resultp2, dstptr); return NOEXCEPTION;
    }
    if src_exponent >= 0 {
        let mut temp = src; Sgl_clear_signexponent_set_hidden(temp);
        Dint_from_sgl_mantissa(temp, src_exponent, resultp1, resultp2);
        if Sgl_isone_sign(src) { Dint_setone_sign(resultp1, resultp2); }
        Dint_copytoptr(resultp1, resultp2, dstptr);
        if Sgl_isinexact_to_fix(src, src_exponent) { if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; } else { Set_inexactflag(); } }
    } else {
        Dint_setzero(resultp1, resultp2); Dint_copytoptr(resultp1, resultp2, dstptr);
        if Sgl_isnotzero_exponentmantissa(src) { if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; } else { Set_inexactflag(); } }
    }
    NOEXCEPTION
}

pub unsafe fn dbl_to_sgl_fcnvfxt(
    srcptr: *mut dbl_floating_point, _nullptr: *mut u32,
    dstptr: *mut i32, _status: *mut u32,
) -> i32 {
    let (srcp1, srcp2) = Dbl_copyfromptr(srcptr);
    let src_exponent = Dbl_exponent(srcp1) - DBL_BIAS;
    let mut result: i32;
    if src_exponent > SGL_FX_MAX_EXP {
        if Dbl_isoverflow_to_int(src_exponent, srcp1, srcp2) {
            if Dbl_iszero_sign(srcp1) { result = 0x7fffffff; } else { result = 0x80000000u32 as i32; }
            if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
            Set_invalidflag(); *dstptr = result; return NOEXCEPTION;
        }
    }
    if src_exponent >= 0 {
        let (mut tempp1, mut tempp2) = (srcp1, srcp2);
        Dbl_clear_signexponent_set_hidden(tempp1);
        Int_from_dbl_mantissa(tempp1, tempp2, src_exponent);
        if Dbl_isone_sign(srcp1) && src_exponent <= SGL_FX_MAX_EXP { result = -Dbl_allp1(tempp1); } else { result = Dbl_allp1(tempp1); }
        *dstptr = result;
        if Dbl_isinexact_to_fix(srcp1, srcp2, src_exponent) { if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; } else { Set_inexactflag(); } }
    } else {
        *dstptr = 0;
        if Dbl_isnotzero_exponentmantissa(srcp1, srcp2) { if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; } else { Set_inexactflag(); } }
    }
    NOEXCEPTION
}

pub unsafe fn dbl_to_dbl_fcnvfxt(
    srcptr: *mut dbl_floating_point, _nullptr: *mut u32,
    dstptr: *mut dbl_integer, _status: *mut u32,
) -> i32 {
    let (srcp1, srcp2) = Dbl_copyfromptr(srcptr);
    let src_exponent = Dbl_exponent(srcp1) - DBL_BIAS;
    let mut resultp1: i32; let mut resultp2: u32;
    if src_exponent > DBL_FX_MAX_EXP {
        if src_exponent > DBL_FX_MAX_EXP + 1 || Dbl_isnotzero_mantissa(srcp1, srcp2) || Dbl_iszero_sign(srcp1) {
            if Dbl_iszero_sign(srcp1) { resultp1 = 0x7fffffff; resultp2 = 0xffffffff; } else { resultp1 = 0x80000000u32 as i32; resultp2 = 0; }
            if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
            Set_invalidflag(); Dint_copytoptr(resultp1, resultp2, dstptr); return NOEXCEPTION;
        }
    }
    if src_exponent >= 0 {
        let (mut tempp1, mut tempp2) = (srcp1, srcp2);
        Dbl_clear_signexponent_set_hidden(tempp1);
        Dint_from_dbl_mantissa(tempp1, tempp2, src_exponent, resultp1, resultp2);
        if Dbl_isone_sign(srcp1) { Dint_setone_sign(resultp1, resultp2); }
        Dint_copytoptr(resultp1, resultp2, dstptr);
        if Dbl_isinexact_to_fix(srcp1, srcp2, src_exponent) { if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; } else { Set_inexactflag(); } }
    } else {
        Dint_setzero(resultp1, resultp2); Dint_copytoptr(resultp1, resultp2, dstptr);
        if Dbl_isnotzero_exponentmantissa(srcp1, srcp2) { if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; } else { Set_inexactflag(); } }
    }
    NOEXCEPTION
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
