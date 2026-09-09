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
 *	@(#)	pa/spmath/fcnvuf.c		$Revision: 1.1 $
 *
 *  Purpose:
 *	Fixed point to Floating-point Converts
 *
 * END_DESC
 */

pub unsafe fn sgl_to_sgl_fcnvuf(
    srcptr: *mut ::core::ffi::c_uint,
    _nullptr: *mut ::core::ffi::c_uint,
    dstptr: *mut sgl_floating_point,
    _status: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut src: ::core::ffi::c_uint;
    let mut result: ::core::ffi::c_uint = 0;
    let mut dst_exponent: ::core::ffi::c_int;

    src = *srcptr;
    if src == 0 {
        Sgl_setzero!(result);
        *dstptr = result;
        return NOEXCEPTION;
    }
    dst_exponent = 16;
    Find_ms_one_bit!(src, dst_exponent);
    src <<= (dst_exponent + 1) as u32;
    Sgl_set_mantissa!(result, src >> SGL_EXP_LENGTH);
    Sgl_set_exponent!(result, 30 + SGL_BIAS - dst_exponent);
    if Suint_isinexact_to_sgl!(src) {
        match Rounding_mode!() {
            ROUNDPLUS => Sgl_increment!(result),
            ROUNDMINUS => {},
            ROUNDNEAREST => Sgl_roundnearest_from_suint!(src, result),
            _ => {},
        }
        if Is_inexacttrap_enabled!() {
            *dstptr = result;
            return INEXACTEXCEPTION;
        } else {
            Set_inexactflag!();
        }
    }
    *dstptr = result;
    NOEXCEPTION
}

pub unsafe fn sgl_to_dbl_fcnvuf(
    srcptr: *mut ::core::ffi::c_uint,
    _nullptr: *mut ::core::ffi::c_uint,
    dstptr: *mut dbl_floating_point,
    _status: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut dst_exponent: ::core::ffi::c_int;
    let mut src: ::core::ffi::c_uint;
    let mut resultp1: ::core::ffi::c_uint = 0;
    let mut resultp2: ::core::ffi::c_uint = 0;

    src = *srcptr;
    if src == 0 {
        Dbl_setzero!(resultp1, resultp2);
        Dbl_copytoptr!(resultp1, resultp2, dstptr);
        return NOEXCEPTION;
    }
    dst_exponent = 16;
    Find_ms_one_bit!(src, dst_exponent);
    src <<= (dst_exponent + 1) as u32;
    Dbl_set_mantissap1!(resultp1, src >> DBL_EXP_LENGTH);
    Dbl_set_mantissap2!(resultp2, src << (32 - DBL_EXP_LENGTH));
    Dbl_set_exponent!(resultp1, (30 + DBL_BIAS) - dst_exponent);
    Dbl_copytoptr!(resultp1, resultp2, dstptr);
    NOEXCEPTION
}

pub unsafe fn dbl_to_sgl_fcnvuf(
    srcptr: *mut dbl_unsigned,
    _nullptr: *mut ::core::ffi::c_uint,
    dstptr: *mut sgl_floating_point,
    _status: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut dst_exponent: ::core::ffi::c_int;
    let mut srcp1: ::core::ffi::c_uint;
    let mut srcp2: ::core::ffi::c_uint;
    let mut result: ::core::ffi::c_uint = 0;

    Duint_copyfromptr!(srcptr, srcp1, srcp2);
    if srcp1 == 0 && srcp2 == 0 {
        Sgl_setzero!(result);
        *dstptr = result;
        return NOEXCEPTION;
    }
    dst_exponent = 16;
    if srcp1 == 0 {
        Find_ms_one_bit!(srcp2, dst_exponent);
        srcp1 = srcp2 << (dst_exponent + 1) as u32;
        srcp2 = 0;
        dst_exponent += 32;
    } else {
        Find_ms_one_bit!(srcp1, dst_exponent);
        if dst_exponent >= 0 {
            Variable_shift_double!(srcp1, srcp2, 31 - dst_exponent, srcp1);
            srcp2 <<= (dst_exponent + 1) as u32;
        }
    }
    Sgl_set_mantissa!(result, srcp1 >> SGL_EXP_LENGTH);
    Sgl_set_exponent!(result, (62 + SGL_BIAS) - dst_exponent);
    if Duint_isinexact_to_sgl!(srcp1, srcp2) {
        match Rounding_mode!() {
            ROUNDPLUS => Sgl_increment!(result),
            ROUNDMINUS => {},
            ROUNDNEAREST => Sgl_roundnearest_from_duint!(srcp1, srcp2, result),
            _ => {},
        }
        if Is_inexacttrap_enabled!() {
            *dstptr = result;
            return INEXACTEXCEPTION;
        } else {
            Set_inexactflag!();
        }
    }
    *dstptr = result;
    NOEXCEPTION
}

pub unsafe fn dbl_to_dbl_fcnvuf(
    srcptr: *mut dbl_unsigned,
    _nullptr: *mut ::core::ffi::c_uint,
    dstptr: *mut dbl_floating_point,
    _status: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut dst_exponent: ::core::ffi::c_int;
    let mut srcp1: ::core::ffi::c_uint;
    let mut srcp2: ::core::ffi::c_uint;
    let mut resultp1: ::core::ffi::c_uint = 0;
    let mut resultp2: ::core::ffi::c_uint = 0;

    Duint_copyfromptr!(srcptr, srcp1, srcp2);
    if srcp1 == 0 && srcp2 == 0 {
        Dbl_setzero!(resultp1, resultp2);
        Dbl_copytoptr!(resultp1, resultp2, dstptr);
        return NOEXCEPTION;
    }
    dst_exponent = 16;
    if srcp1 == 0 {
        Find_ms_one_bit!(srcp2, dst_exponent);
        srcp1 = srcp2 << (dst_exponent + 1) as u32;
        srcp2 = 0;
        dst_exponent += 32;
    } else {
        Find_ms_one_bit!(srcp1, dst_exponent);
        if dst_exponent >= 0 {
            Variable_shift_double!(srcp1, srcp2, 31 - dst_exponent, srcp1);
            srcp2 <<= (dst_exponent + 1) as u32;
        }
    }
    Dbl_set_mantissap1!(resultp1, srcp1 >> DBL_EXP_LENGTH);
    Shiftdouble!(srcp1, srcp2, DBL_EXP_LENGTH, resultp2);
    Dbl_set_exponent!(resultp1, (62 + DBL_BIAS) - dst_exponent);
    if Duint_isinexact_to_dbl!(srcp2) {
        match Rounding_mode!() {
            ROUNDPLUS => Dbl_increment!(resultp1, resultp2),
            ROUNDMINUS => {},
            ROUNDNEAREST => Dbl_roundnearest_from_duint!(srcp2, resultp1, resultp2),
            _ => {},
        }
        if Is_inexacttrap_enabled!() {
            Dbl_copytoptr!(resultp1, resultp2, dstptr);
            return INEXACTEXCEPTION;
        } else {
            Set_inexactflag!();
        }
    }
    Dbl_copytoptr!(resultp1, resultp2, dstptr);
    NOEXCEPTION
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
