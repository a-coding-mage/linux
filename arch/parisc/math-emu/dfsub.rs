// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Linux/PA-RISC Project (http://www.parisc-linux.org/)
 *
 * Floating-point emulation code
 * Copyright (C) 2001 Hewlett-Packard (Paul Bame) <bame@debian.org>
 */

// Dependencies supplied by float.h and dbl_float.h remain external to this translation.

pub unsafe fn dbl_fsub(
    leftptr: *mut dbl_floating_point,
    rightptr: *mut dbl_floating_point,
    dstptr: *mut dbl_floating_point,
    status: *mut u32,
) -> i32 {
    let mut signless_upper_left: u32;
    let mut signless_upper_right: u32;
    let mut save: u32;
    let mut leftp1: u32;
    let mut leftp2: u32;
    let mut rightp1: u32;
    let mut rightp2: u32;
    let mut extent: u32;
    let mut resultp1: u32 = 0;
    let mut resultp2: u32 = 0;
    let mut result_exponent: i32;
    let mut right_exponent: i32;
    let mut diff_exponent: i32;
    let mut sign_save: i32;
    let mut jumpsize: i32;
    let mut inexact: boolean = FALSE;
    let mut underflowtrap: boolean;

    Dbl_copyfromptr!(leftptr, leftp1, leftp2);
    Dbl_copyfromptr!(rightptr, rightp1, rightp2);
    Dbl_xortointp1!(leftp1, rightp1, save);

    if { result_exponent = Dbl_exponent!(leftp1); result_exponent } == DBL_INFINITY_EXPONENT {
        if Dbl_iszero_mantissa!(leftp1, leftp2) {
            if Dbl_isnotnan!(rightp1, rightp2) {
                if Dbl_isinfinity!(rightp1, rightp2) && save == 0 {
                    if Is_invalidtrap_enabled!() { return INVALIDEXCEPTION; }
                    Set_invalidflag!();
                    Dbl_makequietnan!(resultp1, resultp2);
                    Dbl_copytoptr!(resultp1, resultp2, dstptr);
                    return NOEXCEPTION;
                }
                Dbl_copytoptr!(leftp1, leftp2, dstptr);
                return NOEXCEPTION;
            }
        } else {
            if Dbl_isone_signaling!(leftp1) {
                if Is_invalidtrap_enabled!() { return INVALIDEXCEPTION; }
                Set_invalidflag!(); Dbl_set_quiet!(leftp1);
            } else if Dbl_is_signalingnan!(rightp1) {
                if Is_invalidtrap_enabled!() { return INVALIDEXCEPTION; }
                Set_invalidflag!(); Dbl_set_quiet!(rightp1);
                Dbl_copytoptr!(rightp1, rightp2, dstptr);
                return NOEXCEPTION;
            }
            Dbl_copytoptr!(leftp1, leftp2, dstptr);
            return NOEXCEPTION;
        }
    }
    if Dbl_isinfinity_exponent!(rightp1) {
        if Dbl_iszero_mantissa!(rightp1, rightp2) {
            Dbl_invert_sign!(rightp1); Dbl_copytoptr!(rightp1, rightp2, dstptr);
            return NOEXCEPTION;
        }
        if Dbl_isone_signaling!(rightp1) {
            if Is_invalidtrap_enabled!() { return INVALIDEXCEPTION; }
            Set_invalidflag!(); Dbl_set_quiet!(rightp1);
        }
        Dbl_copytoptr!(rightp1, rightp2, dstptr); return NOEXCEPTION;
    }

    Dbl_copytoint_exponentmantissap1!(leftp1, signless_upper_left);
    Dbl_copytoint_exponentmantissap1!(rightp1, signless_upper_right);
    if Dbl_ismagnitudeless!(leftp2, rightp2, signless_upper_left, signless_upper_right) {
        Dbl_xorfromintp1!(save, rightp1, rightp1);
        Dbl_xorfromintp1!(save, leftp1, leftp1);
        Dbl_swap_lower!(leftp2, rightp2);
        result_exponent = Dbl_exponent!(leftp1); Dbl_invert_sign!(leftp1);
    }

    if { right_exponent = Dbl_exponent!(rightp1); right_exponent } == 0 {
        if Dbl_iszero_mantissa!(rightp1, rightp2) {
            if Dbl_iszero_exponentmantissa!(leftp1, leftp2) {
                Dbl_invert_sign!(rightp1);
                if Is_rounding_mode!(ROUNDMINUS) { Dbl_or_signs!(leftp1, rightp1); }
                else { Dbl_and_signs!(leftp1, rightp1); }
            } else {
                if result_exponent == 0 && Is_underflowtrap_enabled!() {
                    sign_save = Dbl_signextendedsign!(leftp1); Dbl_leftshiftby1!(leftp1, leftp2);
                    Dbl_normalize!(leftp1, leftp2, result_exponent); Dbl_set_sign!(leftp1, sign_save);
                    Dbl_setwrapped_exponent!(leftp1, result_exponent, unfl);
                    Dbl_copytoptr!(leftp1, leftp2, dstptr); return UNDERFLOWEXCEPTION;
                }
            }
            Dbl_copytoptr!(leftp1, leftp2, dstptr); return NOEXCEPTION;
        }
        Dbl_clear_sign!(rightp1);
        if result_exponent == 0 {
            if (save as i32) >= 0 {
                Dbl_subtract!(leftp1, leftp2, rightp1, rightp2, resultp1, resultp2);
                if Dbl_iszero_mantissa!(resultp1, resultp2) {
                    if Is_rounding_mode!(ROUNDMINUS) { Dbl_setone_sign!(resultp1); }
                    else { Dbl_setzero_sign!(resultp1); }
                    Dbl_copytoptr!(resultp1, resultp2, dstptr); return NOEXCEPTION;
                }
            } else {
                Dbl_addition!(leftp1, leftp2, rightp1, rightp2, resultp1, resultp2);
                if Dbl_isone_hidden!(resultp1) { Dbl_copytoptr!(resultp1, resultp2, dstptr); return NOEXCEPTION; }
            }
            if Is_underflowtrap_enabled!() {
                sign_save = Dbl_signextendedsign!(resultp1); Dbl_leftshiftby1!(resultp1, resultp2);
                Dbl_normalize!(resultp1, resultp2, result_exponent); Dbl_set_sign!(resultp1, sign_save);
                Dbl_setwrapped_exponent!(resultp1, result_exponent, unfl);
                Dbl_copytoptr!(resultp1, resultp2, dstptr); return UNDERFLOWEXCEPTION;
            }
            Dbl_copytoptr!(resultp1, resultp2, dstptr); return NOEXCEPTION;
        }
        right_exponent = 1;
    } else { Dbl_clear_signexponent_set_hidden!(rightp1); }
    Dbl_clear_exponent_set_hidden!(leftp1);
    diff_exponent = result_exponent - right_exponent;
    if diff_exponent > DBL_THRESHOLD { diff_exponent = DBL_THRESHOLD; }
    Dbl_right_align!(rightp1, rightp2, diff_exponent, extent);

    if (save as i32) >= 0 {
        Dbl_subtract_withextension!(leftp1, leftp2, rightp1, rightp2, extent, resultp1, resultp2);
        if Dbl_iszero_hidden!(resultp1) {
            sign_save = Dbl_signextendedsign!(resultp1);
            Dbl_leftshiftby1_withextent!(resultp1, resultp2, extent, resultp1, resultp2);
            if Dbl_iszero!(resultp1, resultp2) { if Is_rounding_mode!(ROUNDMINUS) { Dbl_setone_sign!(resultp1); } Dbl_copytoptr!(resultp1, resultp2, dstptr); return NOEXCEPTION; }
            result_exponent -= 1;
            if Dbl_isone_hidden!(resultp1) {
                if result_exponent == 0 { goto_underflow!(underflow); }
                Dbl_set_sign!(resultp1, sign_save); Ext_leftshiftby1!(extent); goto_round!(round);
            }
            underflowtrap = Is_underflowtrap_enabled!();
            if !underflowtrap && result_exponent == 0 { goto_underflow!(underflow); }
            Ext_leftshiftby1!(extent);
            while Dbl_iszero_hiddenhigh7mantissa!(resultp1) { Dbl_leftshiftby8!(resultp1, resultp2); result_exponent -= 8; if result_exponent <= 0 && !underflowtrap { goto_underflow!(underflow); } }
            if Dbl_iszero_hiddenhigh3mantissa!(resultp1) { Dbl_leftshiftby4!(resultp1, resultp2); result_exponent -= 4; if result_exponent <= 0 && !underflowtrap { goto_underflow!(underflow); } }
            jumpsize = Dbl_hiddenhigh3mantissa!(resultp1);
            if jumpsize > 7 { if result_exponent <= 0 { goto_underflow!(underflow); } Dbl_set_sign!(resultp1, sign_save); Dbl_set_exponent!(resultp1, result_exponent); Dbl_copytoptr!(resultp1, resultp2, dstptr); return NOEXCEPTION; }
            Dbl_sethigh4bits!(resultp1, sign_save);
            match jumpsize { 1 => { Dbl_leftshiftby3!(resultp1, resultp2); result_exponent -= 3; }, 2 | 3 => { Dbl_leftshiftby2!(resultp1, resultp2); result_exponent -= 2; }, 4..=7 => { Dbl_leftshiftby1!(resultp1, resultp2); result_exponent -= 1; }, _ => {} }
            if result_exponent > 0 { Dbl_set_exponent!(resultp1, result_exponent); Dbl_copytoptr!(resultp1, resultp2, dstptr); return NOEXCEPTION; }
        }
    } else {
        Dbl_addition!(leftp1, leftp2, rightp1, rightp2, resultp1, resultp2);
        if Dbl_isone_hiddenoverflow!(resultp1) { Dbl_rightshiftby1_withextent!(resultp2, extent, extent); Dbl_arithrightshiftby1!(resultp1, resultp2); result_exponent += 1; }
    }

    round: {
        if Ext_isnotzero!(extent) {
            inexact = TRUE;
            match Rounding_mode!() {
                ROUNDNEAREST => if Ext_isone_sign!(extent) && (Ext_isnotzero_lower!(extent) || Dbl_isone_lowmantissap2!(resultp2)) { Dbl_increment!(resultp1, resultp2); },
                ROUNDPLUS => if Dbl_iszero_sign!(resultp1) { Dbl_increment!(resultp1, resultp2); },
                ROUNDMINUS => if Dbl_isone_sign!(resultp1) { Dbl_increment!(resultp1, resultp2); },
                ROUNDZERO => {},
                _ => {},
            }
            if Dbl_isone_hiddenoverflow!(resultp1) { result_exponent += 1; }
        }
        if result_exponent == DBL_INFINITY_EXPONENT {
            if Is_overflowtrap_enabled!() { Dbl_setwrapped_exponent!(resultp1, result_exponent, ovfl); Dbl_copytoptr!(resultp1, resultp2, dstptr); if inexact && Is_inexacttrap_enabled!() { return OVERFLOWEXCEPTION | INEXACTEXCEPTION; } if inexact { Set_inexactflag!(); } return OVERFLOWEXCEPTION; }
            inexact = TRUE; Set_overflowflag!(); Dbl_setoverflow!(resultp1, resultp2);
        } else { Dbl_set_exponent!(resultp1, result_exponent); }
        Dbl_copytoptr!(resultp1, resultp2, dstptr); if inexact { if Is_inexacttrap_enabled!() { return INEXACTEXCEPTION; } Set_inexactflag!(); } return NOEXCEPTION;
    }

    underflow: {
        if Is_underflowtrap_enabled!() { Dbl_set_sign!(resultp1, sign_save); Dbl_setwrapped_exponent!(resultp1, result_exponent, unfl); Dbl_copytoptr!(resultp1, resultp2, dstptr); return UNDERFLOWEXCEPTION; }
        Dbl_fix_overshift!(resultp1, resultp2, 1 - result_exponent, extent); Dbl_clear_signexponent!(resultp1); Dbl_set_sign!(resultp1, sign_save); Dbl_copytoptr!(resultp1, resultp2, dstptr); return NOEXCEPTION;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
