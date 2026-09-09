// SPDX-License-Identifier: GPL-2.0-or-later
/* Linux/PA-RISC floating-point emulation: floating-point to unsigned fixed. */

// The following types, constants, and macros are supplied by the surrounding
// PA-RISC math-emulation implementation.

pub unsafe fn sgl_to_sgl_fcnvfu(
    srcptr: *mut sgl_floating_point,
    _nullptr: *mut u32,
    dstptr: *mut u32,
    _status: *mut u32,
) -> i32 {
    let mut src: u32;
    let mut result: u32;
    let src_exponent: i32;
    let mut inexact = FALSE;

    src = *srcptr;
    src_exponent = Sgl_exponent(src) - SGL_BIAS;

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
        if Sgl_isinexact_to_unsigned(src, src_exponent) {
            inexact = TRUE;
            match Rounding_mode() {
                ROUNDPLUS => result = result.wrapping_add(1),
                ROUNDMINUS => (),
                ROUNDNEAREST => {
                    if Sgl_isone_roundbit(src, src_exponent) &&
                       (Sgl_isone_stickybit(src, src_exponent) || (result & 1) != 0) {
                        result = result.wrapping_add(1);
                    }
                },
                _ => (),
            }
        }
    } else {
        result = 0;
        if Sgl_isnotzero_exponentmantissa(src) {
            inexact = TRUE;
            match Rounding_mode() {
                ROUNDPLUS => if Sgl_iszero_sign(src) { result = result.wrapping_add(1); },
                ROUNDMINUS => if Sgl_isone_sign(src) {
                    if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
                    Set_invalidflag(); inexact = FALSE;
                },
                ROUNDNEAREST => if src_exponent == -1 && Sgl_isnotzero_mantissa(src) {
                    if Sgl_isone_sign(src) {
                        if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
                        Set_invalidflag(); inexact = FALSE;
                    } else { result = result.wrapping_add(1); }
                },
                _ => (),
            }
        }
    }
    *dstptr = result;
    if inexact {
        if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; }
        Set_inexactflag();
    }
    NOEXCEPTION
}

pub unsafe fn sgl_to_dbl_fcnvfu(
    srcptr: *mut sgl_floating_point, _nullptr: *mut u32,
    dstptr: *mut dbl_unsigned, _status: *mut u32,
) -> i32 {
    let src = *srcptr;
    let src_exponent = Sgl_exponent(src) - SGL_BIAS;
    let mut resultp1: u32;
    let mut resultp2: u32;
    let mut inexact = FALSE;
    if src_exponent > DBL_FX_MAX_EXP + 1 {
        resultp1 = if Sgl_isone_sign(src) { 0 } else { 0xffffffff };
        resultp2 = resultp1;
        if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
        Set_invalidflag(); Duint_copytoptr(resultp1, resultp2, dstptr); return NOEXCEPTION;
    }
    if src_exponent >= 0 {
        if Sgl_isone_sign(src) {
            if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
            Set_invalidflag(); Duint_copytoptr(0, 0, dstptr); return NOEXCEPTION;
        }
        let mut src = src;
        Sgl_clear_signexponent_set_hidden(src);
        Duint_from_sgl_mantissa(src, src_exponent, resultp1, resultp2);
        if Sgl_isinexact_to_unsigned(src, src_exponent) {
            inexact = TRUE;
            match Rounding_mode() {
                ROUNDPLUS => Duint_increment(resultp1, resultp2),
                ROUNDMINUS => (),
                ROUNDNEAREST => if Sgl_isone_roundbit(src, src_exponent) &&
                    (Sgl_isone_stickybit(src, src_exponent) || Duint_isone_lowp2(resultp2)) {
                    Duint_increment(resultp1, resultp2)
                },
                _ => (),
            }
        }
    } else {
        Duint_setzero(resultp1, resultp2);
        if Sgl_isnotzero_exponentmantissa(src) {
            inexact = TRUE;
            match Rounding_mode() {
                ROUNDPLUS => if Sgl_iszero_sign(src) { Duint_increment(resultp1, resultp2); },
                ROUNDMINUS => if Sgl_isone_sign(src) {
                    if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
                    Set_invalidflag(); inexact = FALSE;
                },
                ROUNDNEAREST => if src_exponent == -1 && Sgl_isnotzero_mantissa(src) {
                    if Sgl_isone_sign(src) {
                        if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
                        Set_invalidflag(); inexact = FALSE;
                    } else { Duint_increment(resultp1, resultp2); }
                },
                _ => (),
            }
        }
    }
    Duint_copytoptr(resultp1, resultp2, dstptr);
    if inexact { if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; } Set_inexactflag(); }
    NOEXCEPTION
}

pub unsafe fn dbl_to_sgl_fcnvfu(
    srcptr: *mut dbl_floating_point, _nullptr: *mut u32,
    dstptr: *mut u32, _status: *mut u32,
) -> i32 {
    let mut srcp1: u32; let mut srcp2: u32; let mut result: u32;
    let src_exponent: i32; let mut inexact = FALSE;
    Dbl_copyfromptr(srcptr, srcp1, srcp2);
    src_exponent = Dbl_exponent(srcp1) - DBL_BIAS;
    if src_exponent > SGL_FX_MAX_EXP + 1 {
        result = if Dbl_isone_sign(srcp1) { 0 } else { 0xffffffff };
        if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
        Set_invalidflag(); *dstptr = result; return NOEXCEPTION;
    }
    if src_exponent >= 0 {
        if Dbl_isone_sign(srcp1) {
            if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
            Set_invalidflag(); *dstptr = 0; return NOEXCEPTION;
        }
        Dbl_clear_signexponent_set_hidden(srcp1);
        Suint_from_dbl_mantissa(srcp1, srcp2, src_exponent, result);
        if Dbl_isinexact_to_unsigned(srcp1, srcp2, src_exponent) {
            inexact = TRUE;
            match Rounding_mode() {
                ROUNDPLUS => result = result.wrapping_add(1), ROUNDMINUS => (),
                ROUNDNEAREST => if Dbl_isone_roundbit(srcp1,srcp2,src_exponent) &&
                    (Dbl_isone_stickybit(srcp1,srcp2,src_exponent) || result & 1 != 0) {
                    result = result.wrapping_add(1);
                }, _ => (),
            }
            if result == 0 {
                if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
                Set_invalidflag(); *dstptr = 0xffffffff; return NOEXCEPTION;
            }
        }
    } else {
        result = 0;
        if Dbl_isnotzero_exponentmantissa(srcp1, srcp2) {
            inexact = TRUE;
            match Rounding_mode() {
                ROUNDPLUS => if Dbl_iszero_sign(srcp1) { result += 1; }, ROUNDMINUS => if Dbl_isone_sign(srcp1) {
                    if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; } Set_invalidflag(); inexact = FALSE;
                }, ROUNDNEAREST => if src_exponent == -1 && Dbl_isnotzero_mantissa(srcp1,srcp2) {
                    if Dbl_isone_sign(srcp1) { if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; } Set_invalidflag(); inexact=FALSE; } else { result += 1; }
                }, _ => (),
            }
        }
    }
    *dstptr = result;
    if inexact { if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; } Set_inexactflag(); }
    NOEXCEPTION
}

pub unsafe fn dbl_to_dbl_fcnvfu(
    srcptr: *mut dbl_floating_point, _nullptr: *mut u32,
    dstptr: *mut dbl_unsigned, _status: *mut u32,
) -> i32 {
    let mut srcp1: u32; let mut srcp2: u32; let mut resultp1: u32; let mut resultp2: u32;
    let src_exponent: i32; let mut inexact = FALSE;
    Dbl_copyfromptr(srcptr, srcp1, srcp2);
    src_exponent = Dbl_exponent(srcp1) - DBL_BIAS;
    if src_exponent > DBL_FX_MAX_EXP + 1 {
        resultp1 = if Dbl_isone_sign(srcp1) { 0 } else { 0xffffffff }; resultp2=resultp1;
        if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; } Set_invalidflag(); Duint_copytoptr(resultp1,resultp2,dstptr); return NOEXCEPTION;
    }
    if src_exponent >= 0 {
        if Dbl_isone_sign(srcp1) { if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; } Set_invalidflag(); Duint_copytoptr(0,0,dstptr); return NOEXCEPTION; }
        Dbl_clear_signexponent_set_hidden(srcp1);
        Duint_from_dbl_mantissa(srcp1,srcp2,src_exponent,resultp1,resultp2);
        if Dbl_isinexact_to_unsigned(srcp1,srcp2,src_exponent) {
            inexact=TRUE;
            match Rounding_mode() { ROUNDPLUS=>Duint_increment(resultp1,resultp2), ROUNDMINUS=>(), ROUNDNEAREST=>if Dbl_isone_roundbit(srcp1,srcp2,src_exponent) && (Dbl_isone_stickybit(srcp1,srcp2,src_exponent)||Duint_isone_lowp2(resultp2)){Duint_increment(resultp1,resultp2)}, _=>() }
        }
    } else {
        Duint_setzero(resultp1,resultp2);
        if Dbl_isnotzero_exponentmantissa(srcp1,srcp2) { inexact=TRUE; match Rounding_mode() { ROUNDPLUS=>if Dbl_iszero_sign(srcp1){Duint_increment(resultp1,resultp2)}, ROUNDMINUS=>if Dbl_isone_sign(srcp1){if Is_invalidtrap_enabled(){return INVALIDEXCEPTION} Set_invalidflag();inexact=FALSE}, ROUNDNEAREST=>if src_exponent==-1&&Dbl_isnotzero_mantissa(srcp1,srcp2){if Dbl_iszero_sign(srcp1){Duint_increment(resultp1,resultp2)}else{if Is_invalidtrap_enabled(){return INVALIDEXCEPTION} Set_invalidflag();inexact=FALSE}}, _=>() } }
    }
    Duint_copytoptr(resultp1,resultp2,dstptr);
    if inexact { if Is_inexacttrap_enabled(){return INEXACTEXCEPTION} Set_inexactflag(); }
    NOEXCEPTION
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
