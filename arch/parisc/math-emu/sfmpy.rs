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
 *	@(#)	pa/spmath/sfmpy.c		$Revision: 1.1 $
 *
 *  Purpose:
 *	Single Precision Floating-point Multiply
 *
 *  External Interfaces:
 *	sgl_fmpy(srcptr1,srcptr2,dstptr,status)
 *
 * END_DESC
 */

// C dependencies float.h and sgl_float.h provide the types, constants, and
// bit-manipulation/exception macros referenced below.

pub unsafe fn sgl_fmpy(
    srcptr1: *mut sgl_floating_point,
    srcptr2: *mut sgl_floating_point,
    dstptr: *mut sgl_floating_point,
    status: *mut u32,
) -> i32 {
    let mut opnd1: u32 = *srcptr1;
    let mut opnd2: u32 = *srcptr2;
    let mut opnd3: u32;
    let mut result: u32;
    let mut dest_exponent: i32;
    let mut count: i32;
    let mut inexact = false;
    let mut guardbit = false;
    let mut stickybit = false;
    let mut is_tiny: bool;

    if Sgl_sign(opnd1) ^ Sgl_sign(opnd2) != 0 { Sgl_setnegativezero!(result); }
    else { Sgl_setzero!(result); }

    if Sgl_isinfinity_exponent(opnd1) {
        if Sgl_iszero_mantissa(opnd1) {
            if Sgl_isnotnan(opnd2) {
                if Sgl_iszero_exponentmantissa(opnd2) {
                    if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
                    Set_invalidflag!();
                    Sgl_makequietnan!(result);
                    *dstptr = result;
                    return NOEXCEPTION;
                }
                Sgl_setinfinity_exponentmantissa!(result);
                *dstptr = result;
                return NOEXCEPTION;
            }
        } else {
            if Sgl_isone_signaling(opnd1) {
                if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
                Set_invalidflag!();
                Sgl_set_quiet!(opnd1);
            } else if Sgl_is_signalingnan(opnd2) {
                if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
                Set_invalidflag!();
                Sgl_set_quiet!(opnd2);
                *dstptr = opnd2;
                return NOEXCEPTION;
            }
            *dstptr = opnd1;
            return NOEXCEPTION;
        }
    }
    if Sgl_isinfinity_exponent(opnd2) {
        if Sgl_iszero_mantissa(opnd2) {
            if Sgl_iszero_exponentmantissa(opnd1) {
                if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
                Set_invalidflag!();
                Sgl_makequietnan!(opnd2);
                *dstptr = opnd2;
                return NOEXCEPTION;
            }
            Sgl_setinfinity_exponentmantissa!(result);
            *dstptr = result;
            return NOEXCEPTION;
        }
        if Sgl_isone_signaling(opnd2) {
            if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
            Set_invalidflag!();
            Sgl_set_quiet!(opnd2);
        }
        *dstptr = opnd2;
        return NOEXCEPTION;
    }

    dest_exponent = Sgl_exponent(opnd1) + Sgl_exponent(opnd2) - SGL_BIAS;
    if Sgl_isnotzero_exponent(opnd1) { Sgl_clear_signexponent_set_hidden!(opnd1); }
    else {
        if Sgl_iszero_mantissa(opnd1) {
            Sgl_setzero_exponentmantissa!(result);
            *dstptr = result;
            return NOEXCEPTION;
        }
        Sgl_clear_signexponent!(opnd1);
        Sgl_leftshiftby1!(opnd1);
        Sgl_normalize!(opnd1, dest_exponent);
    }
    if Sgl_isnotzero_exponent(opnd2) { Sgl_clear_signexponent_set_hidden!(opnd2); }
    else {
        if Sgl_iszero_mantissa(opnd2) {
            Sgl_setzero_exponentmantissa!(result);
            *dstptr = result;
            return NOEXCEPTION;
        }
        Sgl_clear_signexponent!(opnd2);
        Sgl_leftshiftby1!(opnd2);
        Sgl_normalize!(opnd2, dest_exponent);
    }

    Sgl_leftshiftby4!(opnd2);
    Sgl_setzero!(opnd3);
    count = 1;
    while count < SGL_P {
        stickybit = stickybit || Slow4(opnd3);
        Sgl_rightshiftby4!(opnd3);
        if Sbit28(opnd1) { Sall(opnd3) = Sall(opnd3).wrapping_add(Sall(opnd2) << 3); }
        if Sbit29(opnd1) { Sall(opnd3) = Sall(opnd3).wrapping_add(Sall(opnd2) << 2); }
        if Sbit30(opnd1) { Sall(opnd3) = Sall(opnd3).wrapping_add(Sall(opnd2) << 1); }
        if Sbit31(opnd1) { Sall(opnd3) = Sall(opnd3).wrapping_add(Sall(opnd2)); }
        Sgl_rightshiftby4!(opnd1);
        count += 4;
    }
    if Sgl_iszero_sign(opnd3) { Sgl_leftshiftby1!(opnd3); }
    else { dest_exponent += 1; }
    while Sgl_iszero_sign(opnd3) { Sgl_leftshiftby1!(opnd3); dest_exponent -= 1; }
    stickybit = stickybit || ((Sgl_all(opnd3) << (SGL_BITLENGTH - SGL_EXP_LENGTH + 1)) != 0);
    guardbit = Sbit24(opnd3);
    inexact = guardbit || stickybit;
    Sgl_rightshiftby8!(opnd3);

    if inexact && (dest_exponent > 0 || Is_underflowtrap_enabled()) {
        Sgl_clear_signexponent!(opnd3);
        match Rounding_mode() {
            ROUNDPLUS if Sgl_iszero_sign(result) => Sgl_increment!(opnd3),
            ROUNDMINUS if Sgl_isone_sign(result) => Sgl_increment!(opnd3),
            ROUNDNEAREST if guardbit && (stickybit || Sgl_isone_lowmantissa(opnd3)) => Sgl_increment!(opnd3),
            _ => {}
        }
        if Sgl_isone_hidden(opnd3) { dest_exponent += 1; }
    }
    Sgl_set_mantissa!(result, opnd3);

    if dest_exponent >= SGL_INFINITY_EXPONENT {
        if Is_overflowtrap_enabled() {
            Sgl_setwrapped_exponent!(result, dest_exponent, ovfl);
            *dstptr = result;
            if inexact {
                if Is_inexacttrap_enabled() { return OVERFLOWEXCEPTION | INEXACTEXCEPTION; }
                Set_inexactflag!();
            }
            return OVERFLOWEXCEPTION;
        }
        inexact = true;
        Set_overflowflag!();
        Sgl_setoverflow!(result);
    } else if dest_exponent <= 0 {
        if Is_underflowtrap_enabled() {
            Sgl_setwrapped_exponent!(result, dest_exponent, unfl);
            *dstptr = result;
            if inexact {
                if Is_inexacttrap_enabled() { return UNDERFLOWEXCEPTION | INEXACTEXCEPTION; }
                Set_inexactflag!();
            }
            return UNDERFLOWEXCEPTION;
        }
        is_tiny = true;
        if dest_exponent == 0 && inexact {
            match Rounding_mode() {
                ROUNDPLUS if Sgl_iszero_sign(result) => { Sgl_increment!(opnd3); if Sgl_isone_hiddenoverflow(opnd3) { is_tiny = false; } Sgl_decrement!(opnd3); }
                ROUNDMINUS if Sgl_isone_sign(result) => { Sgl_increment!(opnd3); if Sgl_isone_hiddenoverflow(opnd3) { is_tiny = false; } Sgl_decrement!(opnd3); }
                ROUNDNEAREST if guardbit && (stickybit || Sgl_isone_lowmantissa(opnd3)) => { Sgl_increment!(opnd3); if Sgl_isone_hiddenoverflow(opnd3) { is_tiny = false; } Sgl_decrement!(opnd3); }
                _ => {}
            }
        }
        stickybit = inexact;
        Sgl_denormalize!(opnd3, dest_exponent, guardbit, stickybit, inexact);
        if inexact {
            match Rounding_mode() {
                ROUNDPLUS if Sgl_iszero_sign(result) => Sgl_increment!(opnd3),
                ROUNDMINUS if Sgl_isone_sign(result) => Sgl_increment!(opnd3),
                ROUNDNEAREST if guardbit && (stickybit || Sgl_isone_lowmantissa(opnd3)) => Sgl_increment!(opnd3),
                _ => {}
            }
            if is_tiny { Set_underflowflag!(); }
        }
        Sgl_set_exponentmantissa!(result, opnd3);
    } else { Sgl_set_exponent!(result, dest_exponent); }
    *dstptr = result;
    if inexact {
        if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; }
        Set_inexactflag!();
    }
    NOEXCEPTION
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
