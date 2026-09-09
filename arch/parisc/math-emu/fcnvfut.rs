// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Linux/PA-RISC Project (http://www.parisc-linux.org/)
 *
 * Floating-point emulation code
 *  Copyright (C) 2001 Hewlett-Packard (Paul Bame) <bame@debian.org>
 */
/*
 * BEGIN_DESC
 *
 *  File:
 *	@(#) pa/spmath/fcnvfut.c $Revision: 1.1 $
 *
 *  Purpose:
 *	Floating-point to Unsigned Fixed-point Converts with Truncation
 *
 * END_DESC
 */

/* Types, constants, and conversion helpers are supplied by the original
 * float.h, sgl_float.h, dbl_float.h, and cnv_float.h dependencies. */

/*
 *  Convert single floating-point to single fixed-point format
 *  with truncated result
 */
pub unsafe fn sgl_to_sgl_fcnvfut(
    srcptr: *mut sgl_floating_point,
    _nullptr: *mut u32,
    dstptr: *mut u32,
    _status: *mut u32,
) -> i32 {
    let mut src: u32 = *srcptr;
    let src_exponent: i32 = Sgl_exponent(src) - SGL_BIAS;
    let mut result: u32;

    if src_exponent > SGL_FX_MAX_EXP + 1 {
        result = if Sgl_isone_sign(src) { 0 } else { 0xffffffff };
        if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
        Set_invalidflag();
        *dstptr = result;
        return NOEXCEPTION;
    }
    if src_exponent >= 0 {
        if Sgl_isone_sign(src) {
            result = 0;
            if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
            Set_invalidflag();
            *dstptr = result;
            return NOEXCEPTION;
        }
        Sgl_clear_signexponent_set_hidden(src);
        Suint_from_sgl_mantissa(src, src_exponent, result);
        *dstptr = result;
        if Sgl_isinexact_to_unsigned(src, src_exponent) {
            if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; }
            Set_inexactflag();
        }
    } else {
        *dstptr = 0;
        if Sgl_isnotzero_exponentmantissa(src) {
            if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; }
            Set_inexactflag();
        }
    }
    NOEXCEPTION
}

/* Single Floating-point to Double Unsigned Fixed */
pub unsafe fn sgl_to_dbl_fcnvfut(
    srcptr: *mut sgl_floating_point,
    _nullptr: *mut u32,
    dstptr: *mut dbl_unsigned,
    _status: *mut u32,
) -> i32 {
    let mut src: u32 = *srcptr;
    let src_exponent: i32 = Sgl_exponent(src) - SGL_BIAS;
    let (mut resultp1, mut resultp2): (u32, u32);

    if src_exponent > DBL_FX_MAX_EXP + 1 {
        resultp1 = if Sgl_isone_sign(src) { 0 } else { 0xffffffff };
        resultp2 = resultp1;
        if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
        Set_invalidflag();
        Duint_copytoptr(resultp1, resultp2, dstptr);
        return NOEXCEPTION;
    }
    if src_exponent >= 0 {
        if Sgl_isone_sign(src) {
            resultp1 = 0; resultp2 = 0;
            if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
            Set_invalidflag();
            Duint_copytoptr(resultp1, resultp2, dstptr);
            return NOEXCEPTION;
        }
        Sgl_clear_signexponent_set_hidden(src);
        Duint_from_sgl_mantissa(src, src_exponent, resultp1, resultp2);
        Duint_copytoptr(resultp1, resultp2, dstptr);
        if Sgl_isinexact_to_unsigned(src, src_exponent) {
            if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; }
            Set_inexactflag();
        }
    } else {
        Duint_setzero(resultp1, resultp2);
        Duint_copytoptr(resultp1, resultp2, dstptr);
        if Sgl_isnotzero_exponentmantissa(src) {
            if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; }
            Set_inexactflag();
        }
    }
    NOEXCEPTION
}

/* Double Floating-point to Single Unsigned Fixed */
pub unsafe fn dbl_to_sgl_fcnvfut(
    srcptr: *mut dbl_floating_point,
    _nullptr: *mut u32,
    dstptr: *mut u32,
    _status: *mut u32,
) -> i32 {
    let (mut srcp1, mut srcp2): (u32, u32);
    let mut result: u32;
    Dbl_copyfromptr(srcptr, srcp1, srcp2);
    let src_exponent: i32 = Dbl_exponent(srcp1) - DBL_BIAS;

    if src_exponent > SGL_FX_MAX_EXP + 1 {
        result = if Dbl_isone_sign(srcp1) { 0 } else { 0xffffffff };
        if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
        Set_invalidflag();
        *dstptr = result;
        return NOEXCEPTION;
    }
    if src_exponent >= 0 {
        if Dbl_isone_sign(srcp1) {
            result = 0;
            if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
            Set_invalidflag();
            *dstptr = result;
            return NOEXCEPTION;
        }
        Dbl_clear_signexponent_set_hidden(srcp1);
        Suint_from_dbl_mantissa(srcp1, srcp2, src_exponent, result);
        *dstptr = result;
        if Dbl_isinexact_to_unsigned(srcp1, srcp2, src_exponent) {
            if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; }
            Set_inexactflag();
        }
    } else {
        *dstptr = 0;
        if Dbl_isnotzero_exponentmantissa(srcp1, srcp2) {
            if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; }
            Set_inexactflag();
        }
    }
    NOEXCEPTION
}

/* Double Floating-point to Double Unsigned Fixed */
pub unsafe fn dbl_to_dbl_fcnvfut(
    srcptr: *mut dbl_floating_point,
    _nullptr: *mut u32,
    dstptr: *mut dbl_unsigned,
    _status: *mut u32,
) -> i32 {
    let (mut srcp1, mut srcp2): (u32, u32);
    let (mut resultp1, mut resultp2): (u32, u32);
    Dbl_copyfromptr(srcptr, srcp1, srcp2);
    let src_exponent: i32 = Dbl_exponent(srcp1) - DBL_BIAS;

    if src_exponent > DBL_FX_MAX_EXP + 1 {
        resultp1 = if Dbl_isone_sign(srcp1) { 0 } else { 0xffffffff };
        resultp2 = resultp1;
        if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
        Set_invalidflag();
        Duint_copytoptr(resultp1, resultp2, dstptr);
        return NOEXCEPTION;
    }
    if src_exponent >= 0 {
        if Dbl_isone_sign(srcp1) {
            resultp1 = 0; resultp2 = 0;
            if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
            Set_invalidflag();
            Duint_copytoptr(resultp1, resultp2, dstptr);
            return NOEXCEPTION;
        }
        Dbl_clear_signexponent_set_hidden(srcp1);
        Duint_from_dbl_mantissa(srcp1, srcp2, src_exponent, resultp1, resultp2);
        Duint_copytoptr(resultp1, resultp2, dstptr);
        if Dbl_isinexact_to_unsigned(srcp1, srcp2, src_exponent) {
            if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; }
            Set_inexactflag();
        }
    } else {
        Duint_setzero(resultp1, resultp2);
        Duint_copytoptr(resultp1, resultp2, dstptr);
        if Dbl_isnotzero_exponentmantissa(srcp1, srcp2) {
            if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; }
            Set_inexactflag();
        }
    }
    NOEXCEPTION
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
