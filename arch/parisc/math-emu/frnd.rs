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
 *  Purpose:
 *\tSingle Floating-point Round to Integer
 *\tDouble Floating-point Round to Integer
 *\tQuad Floating-point Round to Integer (returns unimplemented)
 *
 *  External Interfaces:
 *\tdbl_frnd(srcptr,_nullptr,dstptr,status)
 *\tsgl_frnd(srcptr,_nullptr,dstptr,status)
 *
 * END_DESC
 */

// Dependencies supplied by the surrounding floating-point emulation sources.

pub unsafe fn sgl_frnd(
    srcptr: *mut sgl_floating_point,
    _nullptr: *mut u32,
    dstptr: *mut sgl_floating_point,
    status: *mut u32,
) -> i32 {
    let _ = (status, _nullptr);
    let mut src: u32;
    let mut result: u32;
    let mut src_exponent: i32;
    let mut inexact = false;

    src = *srcptr;
    if (src_exponent = Sgl_exponent(src)) == SGL_INFINITY_EXPONENT {
        if Sgl_isone_signaling(src) {
            if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
            Set_invalidflag();
            Sgl_set_quiet(&mut src);
        }
        *dstptr = src;
        return NOEXCEPTION;
    }
    src_exponent -= SGL_BIAS;
    if src_exponent >= SGL_P - 1 {
        *dstptr = src;
        return NOEXCEPTION;
    }
    if src_exponent >= 0 {
        Sgl_clear_exponent_set_hidden(&mut src);
        result = src;
        Sgl_rightshift(&mut result, (SGL_P - 1) - src_exponent);
        if Sgl_isinexact_to_fix(src, src_exponent) {
            inexact = true;
            match Rounding_mode() {
                ROUNDPLUS => if Sgl_iszero_sign(src) { Sgl_increment(&mut result); },
                ROUNDMINUS => if Sgl_isone_sign(src) { Sgl_increment(&mut result); },
                ROUNDNEAREST => if Sgl_isone_roundbit(src, src_exponent)
                    && (Sgl_isone_stickybit(src, src_exponent) || Sgl_isone_lowmantissa(result))
                { Sgl_increment(&mut result); },
                _ => {}
            }
        }
        Sgl_leftshift(&mut result, (SGL_P - 1) - src_exponent);
        if Sgl_isone_hiddenoverflow(result) {
            Sgl_set_exponent(&mut result, src_exponent + (SGL_BIAS + 1));
        } else { Sgl_set_exponent(&mut result, src_exponent + SGL_BIAS); }
    } else {
        result = src;
        Sgl_setzero_exponentmantissa(&mut result);
        if Sgl_isnotzero_exponentmantissa(src) {
            inexact = true;
            match Rounding_mode() {
                ROUNDPLUS => if Sgl_iszero_sign(src) { Sgl_set_exponent(&mut result, SGL_BIAS); },
                ROUNDMINUS => if Sgl_isone_sign(src) { Sgl_set_exponent(&mut result, SGL_BIAS); },
                ROUNDNEAREST => if src_exponent == -1 && Sgl_isnotzero_mantissa(src) {
                    Sgl_set_exponent(&mut result, SGL_BIAS);
                },
                _ => {}
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

pub unsafe fn dbl_frnd(
    srcptr: *mut dbl_floating_point,
    _nullptr: *mut u32,
    dstptr: *mut dbl_floating_point,
    status: *mut u32,
) -> i32 {
    let _ = (status, _nullptr);
    let (mut srcp1, mut srcp2): (u32, u32);
    let (mut resultp1, mut resultp2): (u32, u32);
    let mut src_exponent: i32;
    let mut inexact = false;

    Dbl_copyfromptr(srcptr, &mut srcp1, &mut srcp2);
    if (src_exponent = Dbl_exponent(srcp1)) == DBL_INFINITY_EXPONENT {
        if Dbl_isone_signaling(srcp1) {
            if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
            Set_invalidflag();
            Dbl_set_quiet(&mut srcp1);
        }
        Dbl_copytoptr(srcp1, srcp2, dstptr);
        return NOEXCEPTION;
    }
    src_exponent -= DBL_BIAS;
    if src_exponent >= DBL_P - 1 {
        Dbl_copytoptr(srcp1, srcp2, dstptr);
        return NOEXCEPTION;
    }
    if src_exponent >= 0 {
        Dbl_clear_exponent_set_hidden(&mut srcp1);
        resultp1 = srcp1; resultp2 = srcp2;
        Dbl_rightshift(&mut resultp1, &mut resultp2, (DBL_P - 1) - src_exponent);
        if Dbl_isinexact_to_fix(srcp1, srcp2, src_exponent) {
            inexact = true;
            match Rounding_mode() {
                ROUNDPLUS => if Dbl_iszero_sign(srcp1) { Dbl_increment(&mut resultp1, &mut resultp2); },
                ROUNDMINUS => if Dbl_isone_sign(srcp1) { Dbl_increment(&mut resultp1, &mut resultp2); },
                ROUNDNEAREST => if Dbl_isone_roundbit(srcp1, srcp2, src_exponent)
                    && (Dbl_isone_stickybit(srcp1, srcp2, src_exponent) || Dbl_isone_lowmantissap2(resultp2))
                { Dbl_increment(&mut resultp1, &mut resultp2); },
                _ => {}
            }
        }
        Dbl_leftshift(&mut resultp1, &mut resultp2, (DBL_P - 1) - src_exponent);
        if Dbl_isone_hiddenoverflow(resultp1) {
            Dbl_set_exponent(&mut resultp1, src_exponent + (DBL_BIAS + 1));
        } else { Dbl_set_exponent(&mut resultp1, src_exponent + DBL_BIAS); }
    } else {
        resultp1 = srcp1;
        Dbl_setzero_exponentmantissa(&mut resultp1, &mut resultp2);
        if Dbl_isnotzero_exponentmantissa(srcp1, srcp2) {
            inexact = true;
            match Rounding_mode() {
                ROUNDPLUS => if Dbl_iszero_sign(srcp1) { Dbl_set_exponent(&mut resultp1, DBL_BIAS); },
                ROUNDMINUS => if Dbl_isone_sign(srcp1) { Dbl_set_exponent(&mut resultp1, DBL_BIAS); },
                ROUNDNEAREST => if src_exponent == -1 && Dbl_isnotzero_mantissa(srcp1, srcp2) {
                    Dbl_set_exponent(&mut resultp1, DBL_BIAS);
                },
                _ => {}
            }
        }
    }
    Dbl_copytoptr(resultp1, resultp2, dstptr);
    if inexact {
        if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; }
        Set_inexactflag();
    }
    NOEXCEPTION
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
