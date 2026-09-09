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
 *	@(#)	pa/spmath/sfadd.c		$Revision: 1.1 $
 *
 *  Purpose:
 *	Single_add: add two single precision values.
 *
 *  External Interfaces:
 *	sgl_fadd(leftptr, rightptr, dstptr, status)
 *
 *  Internal Interfaces:
 *
 *  Theory:
 *	<<please update with a overview of the operation of this file>>
 *
 * END_DESC
 */

// Dependencies supplied by the surrounding PA-RISC math-emulation code.

/* Single_add: add two single precision values. */
pub unsafe fn sgl_fadd(
    leftptr: *mut sgl_floating_point,
    rightptr: *mut sgl_floating_point,
    dstptr: *mut sgl_floating_point,
    status: *mut u32,
) -> i32 {
    let mut left: u32;
    let mut right: u32;
    let mut result: u32 = 0;
    let mut extent: u32 = 0;
    let mut signless_upper_left: u32 = 0;
    let mut signless_upper_right: u32 = 0;
    let mut save: u32 = 0;

    let mut result_exponent: i32;
    let mut right_exponent: i32;
    let mut diff_exponent: i32;
    let mut sign_save: i32 = 0;
    let mut jumpsize: i32;
    let mut inexact = false;
    let mut underflowtrap: bool;

    let _ = status;

    /* Create local copies of the numbers */
    left = *leftptr;
    right = *rightptr;

    /* A zero "save" helps discover equal operands (for later),
     * and is used in swapping operands (if needed). */
    Sgl_xortointp1!(left, right, save);

    /* check first operand for NaN's or infinity */
    result_exponent = Sgl_exponent!(left);
    if result_exponent == SGL_INFINITY_EXPONENT {
        if Sgl_iszero_mantissa!(left) {
            if Sgl_isnotnan!(right) {
                if Sgl_isinfinity!(right) && save != 0 {
                    if Is_invalidtrap_enabled!() { return INVALIDEXCEPTION; }
                    Set_invalidflag!();
                    Sgl_makequietnan!(result);
                    *dstptr = result;
                    return NOEXCEPTION;
                }
                *dstptr = left;
                return NOEXCEPTION;
            }
        } else {
            if Sgl_isone_signaling!(left) {
                if Is_invalidtrap_enabled!() { return INVALIDEXCEPTION; }
                Set_invalidflag!();
                Sgl_set_quiet!(left);
            } else if Sgl_is_signalingnan!(right) {
                if Is_invalidtrap_enabled!() { return INVALIDEXCEPTION; }
                Set_invalidflag!();
                Sgl_set_quiet!(right);
                *dstptr = right;
                return NOEXCEPTION;
            }
            *dstptr = left;
            return NOEXCEPTION;
        }
    }

    /* check second operand for NaN's or infinity */
    if Sgl_isinfinity_exponent!(right) {
        if Sgl_iszero_mantissa!(right) {
            *dstptr = right;
            return NOEXCEPTION;
        }
        if Sgl_isone_signaling!(right) {
            if Is_invalidtrap_enabled!() { return INVALIDEXCEPTION; }
            Set_invalidflag!();
            Sgl_set_quiet!(right);
        }
        *dstptr = right;
        return NOEXCEPTION;
    }

    Sgl_copytoint_exponentmantissa!(left, signless_upper_left);
    Sgl_copytoint_exponentmantissa!(right, signless_upper_right);

    if Sgl_ismagnitudeless!(signless_upper_left, signless_upper_right) {
        Sgl_xorfromintp1!(save, right, right);
        Sgl_xorfromintp1!(save, left, left);
        result_exponent = Sgl_exponent!(left);
    }

    if (right_exponent = Sgl_exponent!(right)) == 0 {
        if Sgl_iszero_mantissa!(right) {
            if Sgl_iszero_exponentmantissa!(left) {
                if Is_rounding_mode!(ROUNDMINUS) { Sgl_or_signs!(left, right); }
                else { Sgl_and_signs!(left, right); }
            } else if result_exponent == 0 && Is_underflowtrap_enabled!() {
                sign_save = Sgl_signextendedsign!(left);
                Sgl_leftshiftby1!(left);
                Sgl_normalize!(left, result_exponent);
                Sgl_set_sign!(left, sign_save);
                Sgl_setwrapped_exponent!(left, result_exponent, unfl);
                *dstptr = left;
                return UNDERFLOWEXCEPTION;
            }
            *dstptr = left;
            return NOEXCEPTION;
        }
        Sgl_clear_sign!(right);
        if result_exponent == 0 {
            if (save as i32) < 0 {
                Sgl_subtract!(left, right, result);
                if Sgl_iszero_mantissa!(result) {
                    if Is_rounding_mode!(ROUNDMINUS) { Sgl_setone_sign!(result); }
                    else { Sgl_setzero_sign!(result); }
                    *dstptr = result;
                    return NOEXCEPTION;
                }
            } else {
                Sgl_addition!(left, right, result);
                if Sgl_isone_hidden!(result) { *dstptr = result; return NOEXCEPTION; }
            }
            if Is_underflowtrap_enabled!() {
                sign_save = Sgl_signextendedsign!(result);
                Sgl_leftshiftby1!(result);
                Sgl_normalize!(result, result_exponent);
                Sgl_set_sign!(result, sign_save);
                Sgl_setwrapped_exponent!(result, result_exponent, unfl);
                *dstptr = result;
                return UNDERFLOWEXCEPTION;
            }
            *dstptr = result;
            return NOEXCEPTION;
        }
        right_exponent = 1;
    } else {
        Sgl_clear_signexponent_set_hidden!(right);
    }
    Sgl_clear_exponent_set_hidden!(left);
    diff_exponent = result_exponent - right_exponent;
    if diff_exponent > SGL_THRESHOLD { diff_exponent = SGL_THRESHOLD; }
    Sgl_right_align!(right, diff_exponent, extent);

    if (save as i32) < 0 {
        Sgl_subtract_withextension!(left, right, extent, result);
        if Sgl_iszero_hidden!(result) {
            sign_save = Sgl_signextendedsign!(result);
            Sgl_leftshiftby1_withextent!(result, extent, result);
            if Sgl_iszero!(result) {
                if Is_rounding_mode!(ROUNDMINUS) { Sgl_setone_sign!(result); }
                *dstptr = result;
                return NOEXCEPTION;
            }
            result_exponent -= 1;
            if Sgl_isone_hidden!(result) {
                if result_exponent == 0 { goto_underflow!(); }
                Sgl_set_sign!(result, sign_save);
                Ext_leftshiftby1!(extent);
                goto_round!();
            }
            underflowtrap = Is_underflowtrap_enabled!();
            if !underflowtrap && result_exponent == 0 { goto_underflow!(); }
            Ext_leftshiftby1!(extent);
            while Sgl_iszero_hiddenhigh7mantissa!(result) {
                Sgl_leftshiftby8!(result);
                result_exponent -= 8;
                if result_exponent <= 0 && !underflowtrap { goto_underflow!(); }
            }
            if Sgl_iszero_hiddenhigh3mantissa!(result) {
                Sgl_leftshiftby4!(result);
                result_exponent -= 4;
                if result_exponent <= 0 && !underflowtrap { goto_underflow!(); }
            }
            jumpsize = Sgl_hiddenhigh3mantissa!(result);
            if jumpsize > 7 {
                if result_exponent <= 0 { goto_underflow!(); }
                Sgl_set_sign!(result, sign_save);
                Sgl_set_exponent!(result, result_exponent);
                *dstptr = result;
                return NOEXCEPTION;
            }
            Sgl_sethigh4bits!(result, sign_save);
            match jumpsize {
                1 => { Sgl_leftshiftby3!(result); result_exponent -= 3; }
                2 | 3 => { Sgl_leftshiftby2!(result); result_exponent -= 2; }
                4 | 5 | 6 | 7 => { Sgl_leftshiftby1!(result); result_exponent -= 1; }
                _ => {}
            }
            if result_exponent > 0 {
                Sgl_set_exponent!(result, result_exponent);
                *dstptr = result;
                return NOEXCEPTION;
            }
            if Is_underflowtrap_enabled!() {
                Sgl_set_sign!(result, sign_save);
                Sgl_setwrapped_exponent!(result, result_exponent, unfl);
                *dstptr = result;
                return UNDERFLOWEXCEPTION;
            }
            Sgl_right_align!(result, 1 - result_exponent, extent);
            Sgl_clear_signexponent!(result);
            Sgl_set_sign!(result, sign_save);
            *dstptr = result;
            return NOEXCEPTION;
        }
    } else {
        Sgl_addition!(left, right, result);
        if Sgl_isone_hiddenoverflow!(result) {
            Sgl_rightshiftby1_withextent!(result, extent, extent);
            Sgl_arithrightshiftby1!(result);
            result_exponent += 1;
        }
    }

    if Ext_isnotzero!(extent) {
        inexact = true;
        match Rounding_mode!() {
            ROUNDNEAREST => {
                if Ext_isone_sign!(extent) && (Ext_isnotzero_lower!(extent) || Sgl_isone_lowmantissa!(result)) { Sgl_increment!(result); }
            }
            ROUNDPLUS => { if Sgl_iszero_sign!(result) { Sgl_increment!(result); } }
            ROUNDMINUS => { if Sgl_isone_sign!(result) { Sgl_increment!(result); } }
            ROUNDZERO => {}
            _ => {}
        }
        if Sgl_isone_hiddenoverflow!(result) { result_exponent += 1; }
    }
    if result_exponent == SGL_INFINITY_EXPONENT {
        if Is_overflowtrap_enabled!() {
            Sgl_setwrapped_exponent!(result, result_exponent, ovfl);
            *dstptr = result;
            if inexact && Is_inexacttrap_enabled!() { return OVERFLOWEXCEPTION | INEXACTEXCEPTION; }
            if inexact { Set_inexactflag!(); }
            return OVERFLOWEXCEPTION;
        }
        Set_overflowflag!();
        inexact = true;
        Sgl_setoverflow!(result);
    } else { Sgl_set_exponent!(result, result_exponent); }
    *dstptr = result;
    if inexact {
        if Is_inexacttrap_enabled!() { return INEXACTEXCEPTION; }
        Set_inexactflag!();
    }
    NOEXCEPTION
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
