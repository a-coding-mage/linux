// SPDX-License-Identifier: GPL-2.0-or-later
/* Linux/PA-RISC floating-point emulation code. */

/*
 * Single_subtract: subtract two single precision values.
 *
 * The Sgl_* operations, exception constants, and status helpers are supplied
 * by the surrounding math-emulation implementation.
 */
pub unsafe fn sgl_fsub(
    leftptr: *mut sgl_floating_point,
    rightptr: *mut sgl_floating_point,
    dstptr: *mut sgl_floating_point,
    status: *mut u32,
) -> i32 {
    let mut left: u32 = *leftptr;
    let mut right: u32 = *rightptr;
    let mut result: u32 = 0;
    let mut extent: u32 = 0;
    let mut signless_upper_left: u32 = 0;
    let mut signless_upper_right: u32 = 0;
    let mut save: u32 = 0;
    let mut result_exponent: i32;
    let mut right_exponent: i32;
    let mut diff_exponent: i32;
    let mut sign_save: i32;
    let mut jumpsize: i32;
    let mut inexact = false;
    let mut underflowtrap: bool;

    Sgl_xortointp1(left, right, &mut save);
    result_exponent = Sgl_exponent(left);
    if result_exponent == SGL_INFINITY_EXPONENT {
        if Sgl_iszero_mantissa(left) {
            if Sgl_isnotnan(right) {
                if Sgl_isinfinity(right) && save == 0 {
                    if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
                    Set_invalidflag();
                    Sgl_makequietnan(&mut result);
                    *dstptr = result;
                    return NOEXCEPTION;
                }
                *dstptr = left;
                return NOEXCEPTION;
            }
        } else {
            if Sgl_isone_signaling(left) {
                if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
                Set_invalidflag(); Sgl_set_quiet(&mut left);
            } else if Sgl_is_signalingnan(right) {
                if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
                Set_invalidflag(); Sgl_set_quiet(&mut right);
                *dstptr = right;
                return NOEXCEPTION;
            }
            *dstptr = left;
            return NOEXCEPTION;
        }
    }
    if Sgl_isinfinity_exponent(right) {
        if Sgl_iszero_mantissa(right) {
            Sgl_invert_sign(&mut right); *dstptr = right; return NOEXCEPTION;
        }
        if Sgl_isone_signaling(right) {
            if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
            Set_invalidflag(); Sgl_set_quiet(&mut right);
        }
        *dstptr = right; return NOEXCEPTION;
    }

    Sgl_copytoint_exponentmantissa(left, &mut signless_upper_left);
    Sgl_copytoint_exponentmantissa(right, &mut signless_upper_right);
    if Sgl_ismagnitudeless(signless_upper_left, signless_upper_right) {
        Sgl_xorfromintp1(save, right, &mut right);
        Sgl_xorfromintp1(save, left, &mut left);
        result_exponent = Sgl_exponent(left);
        Sgl_invert_sign(&mut left);
    }

    right_exponent = Sgl_exponent(right);
    if right_exponent == 0 {
        if Sgl_iszero_mantissa(right) {
            if Sgl_iszero_exponentmantissa(left) {
                Sgl_invert_sign(&mut right);
                if Is_rounding_mode(ROUNDMINUS) { Sgl_or_signs(&mut left, right); }
                else { Sgl_and_signs(&mut left, right); }
            } else if result_exponent == 0 && Is_underflowtrap_enabled() {
                sign_save = Sgl_signextendedsign(left); Sgl_leftshiftby1(&mut left);
                Sgl_normalize(&mut left, &mut result_exponent);
                Sgl_set_sign(&mut left, sign_save);
                Sgl_setwrapped_exponent(&mut left, result_exponent, unfl);
                *dstptr = left; return UNDERFLOWEXCEPTION;
            }
            *dstptr = left; return NOEXCEPTION;
        }
        Sgl_clear_sign(&mut right);
        if result_exponent == 0 {
            if (save as i32) >= 0 {
                Sgl_subtract(left, right, &mut result);
                if Sgl_iszero_mantissa(result) {
                    if Is_rounding_mode(ROUNDMINUS) { Sgl_setone_sign(&mut result); }
                    else { Sgl_setzero_sign(&mut result); }
                    *dstptr = result; return NOEXCEPTION;
                }
            } else {
                Sgl_addition(left, right, &mut result);
                if Sgl_isone_hidden(result) { *dstptr = result; return NOEXCEPTION; }
            }
            if Is_underflowtrap_enabled() {
                sign_save = Sgl_signextendedsign(result); Sgl_leftshiftby1(&mut result);
                Sgl_normalize(&mut result, &mut result_exponent);
                Sgl_set_sign(&mut result, sign_save);
                Sgl_setwrapped_exponent(&mut result, result_exponent, unfl);
                *dstptr = result; return UNDERFLOWEXCEPTION;
            }
            *dstptr = result; return NOEXCEPTION;
        }
        right_exponent = 1;
    } else { Sgl_clear_signexponent_set_hidden(&mut right); }
    Sgl_clear_exponent_set_hidden(&mut left);
    diff_exponent = result_exponent - right_exponent;
    if diff_exponent > SGL_THRESHOLD { diff_exponent = SGL_THRESHOLD; }
    Sgl_right_align(right, diff_exponent, &mut extent);

    if (save as i32) >= 0 {
        Sgl_subtract_withextension(left, right, extent, &mut result);
        if Sgl_iszero_hidden(result) {
            sign_save = Sgl_signextendedsign(result);
            Sgl_leftshiftby1_withextent(&mut result, &mut extent, result);
            if Sgl_iszero(result) {
                if Is_rounding_mode(ROUNDMINUS) { Sgl_setone_sign(&mut result); }
                *dstptr = result; return NOEXCEPTION;
            }
            result_exponent -= 1;
            if Sgl_isone_hidden(result) {
                if result_exponent == 0 { goto_underflow!(); }
                Sgl_set_sign(&mut result, sign_save); Ext_leftshiftby1(&mut extent);
                goto_round!();
            }
            underflowtrap = Is_underflowtrap_enabled();
            if !underflowtrap && result_exponent == 0 { goto_underflow!(); }
            Ext_leftshiftby1(&mut extent);
            while Sgl_iszero_hiddenhigh7mantissa(result) {
                Sgl_leftshiftby8(&mut result); result_exponent -= 8;
                if result_exponent <= 0 && !underflowtrap { goto_underflow!(); }
            }
            if Sgl_iszero_hiddenhigh3mantissa(result) {
                Sgl_leftshiftby4(&mut result); result_exponent -= 4;
                if result_exponent <= 0 && !underflowtrap { goto_underflow!(); }
            }
            jumpsize = Sgl_hiddenhigh3mantissa(result);
            if jumpsize > 7 {
                if result_exponent <= 0 { goto_underflow!(); }
                Sgl_set_sign(&mut result, sign_save); Sgl_set_exponent(&mut result, result_exponent);
                *dstptr = result; return NOEXCEPTION;
            }
            Sgl_sethigh4bits(&mut result, sign_save);
            match jumpsize { 1 => { Sgl_leftshiftby3(&mut result); result_exponent -= 3; },
                2 | 3 => { Sgl_leftshiftby2(&mut result); result_exponent -= 2; },
                4..=7 => { Sgl_leftshiftby1(&mut result); result_exponent -= 1; }, _ => {} }
            if result_exponent > 0 { Sgl_set_exponent(&mut result, result_exponent); *dstptr = result; return NOEXCEPTION; }
            goto_underflow!();
        }
    } else {
        Sgl_addition(left, right, &mut result);
        if Sgl_isone_hiddenoverflow(result) {
            Sgl_rightshiftby1_withextent(&mut result, extent, &mut extent);
            Sgl_arithrightshiftby1(&mut result); result_exponent += 1;
        }
    }

    if Ext_isnotzero(extent) {
        inexact = true;
        match Rounding_mode() {
            ROUNDNEAREST => if Ext_isone_sign(extent) && (Ext_isnotzero_lower(extent) || Sgl_isone_lowmantissa(result)) { Sgl_increment(&mut result); },
            ROUNDPLUS => if Sgl_iszero_sign(result) { Sgl_increment(&mut result); },
            ROUNDMINUS => if Sgl_isone_sign(result) { Sgl_increment(&mut result); },
            ROUNDZERO => {}
            _ => {}
        }
        if Sgl_isone_hiddenoverflow(result) { result_exponent += 1; }
    }
    if result_exponent == SGL_INFINITY_EXPONENT {
        if Is_overflowtrap_enabled() {
            Sgl_setwrapped_exponent(&mut result, result_exponent, ovfl); *dstptr = result;
            if inexact { if Is_inexacttrap_enabled() { return OVERFLOWEXCEPTION | INEXACTEXCEPTION; } else { Set_inexactflag(); } }
            return OVERFLOWEXCEPTION;
        }
        Set_overflowflag(); inexact = true; Sgl_setoverflow(&mut result);
    } else { Sgl_set_exponent(&mut result, result_exponent); }
    *dstptr = result;
    if inexact { if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; } else { Set_inexactflag(); } }
    return NOEXCEPTION;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
