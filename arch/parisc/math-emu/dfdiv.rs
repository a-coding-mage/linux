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
 *	@(#) pa/spmath/dfdiv.c        $Revision: 1.1 $
 *
 *  Purpose:
 *	Double Precision Floating-point Divide
 *
 * END_DESC
 */

// Dependencies supplied by the surrounding floating-point emulation sources.

/* Double Precision Floating-point Divide */
pub unsafe fn dbl_fdiv(
    srcptr1: *mut dbl_floating_point,
    srcptr2: *mut dbl_floating_point,
    dstptr: *mut dbl_floating_point,
    status: *mut u32,
) -> i32 {
    let mut opnd1p1: u32;
    let mut opnd1p2: u32;
    let mut opnd2p1: u32;
    let mut opnd2p2: u32;
    let mut opnd3p1: u32;
    let mut opnd3p2: u32;
    let mut resultp1: u32;
    let mut resultp2: u32;
    let mut dest_exponent: i32;
    let mut count: i32;
    let mut inexact = false;
    let mut guardbit = false;
    let mut stickybit = false;
    let mut is_tiny: bool;

    Dbl_copyfromptr(srcptr1, opnd1p1, opnd1p2);
    Dbl_copyfromptr(srcptr2, opnd2p1, opnd2p2);
    if Dbl_sign(opnd1p1) ^ Dbl_sign(opnd2p1) {
        Dbl_setnegativezerop1(resultp1);
    } else {
        Dbl_setzerop1(resultp1);
    }
    if Dbl_isinfinity_exponent(opnd1p1) {
        if Dbl_iszero_mantissa(opnd1p1, opnd1p2) {
            if Dbl_isnotnan(opnd2p1, opnd2p2) {
                if Dbl_isinfinity(opnd2p1, opnd2p2) {
                    if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
                    Set_invalidflag();
                    Dbl_makequietnan(resultp1, resultp2);
                    Dbl_copytoptr(resultp1, resultp2, dstptr);
                    return NOEXCEPTION;
                }
                Dbl_setinfinity_exponentmantissa(resultp1, resultp2);
                Dbl_copytoptr(resultp1, resultp2, dstptr);
                return NOEXCEPTION;
            }
        } else {
            if Dbl_isone_signaling(opnd1p1) {
                if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
                Set_invalidflag();
                Dbl_set_quiet(opnd1p1);
            } else if Dbl_is_signalingnan(opnd2p1) {
                if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
                Set_invalidflag();
                Dbl_set_quiet(opnd2p1);
                Dbl_copytoptr(opnd2p1, opnd2p2, dstptr);
                return NOEXCEPTION;
            }
            Dbl_copytoptr(opnd1p1, opnd1p2, dstptr);
            return NOEXCEPTION;
        }
    }
    if Dbl_isinfinity_exponent(opnd2p1) {
        if Dbl_iszero_mantissa(opnd2p1, opnd2p2) {
            Dbl_setzero_exponentmantissa(resultp1, resultp2);
            Dbl_copytoptr(resultp1, resultp2, dstptr);
            return NOEXCEPTION;
        }
        if Dbl_isone_signaling(opnd2p1) {
            if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
            Set_invalidflag();
            Dbl_set_quiet(opnd2p1);
        }
        Dbl_copytoptr(opnd2p1, opnd2p2, dstptr);
        return NOEXCEPTION;
    }
    if Dbl_iszero_exponentmantissa(opnd2p1, opnd2p2) {
        if Dbl_iszero_exponentmantissa(opnd1p1, opnd1p2) {
            if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
            Set_invalidflag();
            Dbl_makequietnan(resultp1, resultp2);
            Dbl_copytoptr(resultp1, resultp2, dstptr);
            return NOEXCEPTION;
        }
        if Is_divisionbyzerotrap_enabled() { return DIVISIONBYZEROEXCEPTION; }
        Set_divisionbyzeroflag();
        Dbl_setinfinity_exponentmantissa(resultp1, resultp2);
        Dbl_copytoptr(resultp1, resultp2, dstptr);
        return NOEXCEPTION;
    }
    dest_exponent = Dbl_exponent(opnd1p1) - Dbl_exponent(opnd2p1) + DBL_BIAS;
    if Dbl_isnotzero_exponent(opnd1p1) {
        Dbl_clear_signexponent_set_hidden(opnd1p1);
    } else {
        if Dbl_iszero_mantissa(opnd1p1, opnd1p2) {
            Dbl_setzero_exponentmantissa(resultp1, resultp2);
            Dbl_copytoptr(resultp1, resultp2, dstptr);
            return NOEXCEPTION;
        }
        Dbl_clear_signexponent(opnd1p1);
        Dbl_leftshiftby1(opnd1p1, opnd1p2);
        Dbl_normalize(opnd1p1, opnd1p2, dest_exponent);
    }
    if Dbl_isnotzero_exponent(opnd2p1) {
        Dbl_clear_signexponent_set_hidden(opnd2p1);
    } else {
        Dbl_clear_signexponent(opnd2p1);
        Dbl_leftshiftby1(opnd2p1, opnd2p2);
        while Dbl_iszero_hiddenhigh7mantissa(opnd2p1) {
            dest_exponent += 8;
            Dbl_leftshiftby8(opnd2p1, opnd2p2);
        }
        if Dbl_iszero_hiddenhigh3mantissa(opnd2p1) {
            dest_exponent += 4;
            Dbl_leftshiftby4(opnd2p1, opnd2p2);
        }
        while Dbl_iszero_hidden(opnd2p1) {
            dest_exponent += 1;
            Dbl_leftshiftby1(opnd2p1, opnd2p2);
        }
    }
    Twoword_subtract(opnd1p1, opnd1p2, opnd2p1, opnd2p2);
    Dbl_setzero(opnd3p1, opnd3p2);
    count = 1;
    while count <= DBL_P && (opnd1p1 != 0 || opnd1p2 != 0) {
        Dbl_leftshiftby1(opnd1p1, opnd1p2);
        Dbl_leftshiftby1(opnd3p1, opnd3p2);
        if Dbl_iszero_sign(opnd1p1) {
            Dbl_setone_lowmantissap2(opnd3p2);
            Twoword_subtract(opnd1p1, opnd1p2, opnd2p1, opnd2p2);
        } else {
            Twoword_add(opnd1p1, opnd1p2, opnd2p1, opnd2p2);
        }
        count += 1;
    }
    if count <= DBL_P {
        Dbl_leftshiftby1(opnd3p1, opnd3p2);
        Dbl_setone_lowmantissap2(opnd3p2);
        Dbl_leftshift(opnd3p1, opnd3p2, DBL_P - count);
        if Dbl_iszero_hidden(opnd3p1) {
            Dbl_leftshiftby1(opnd3p1, opnd3p2);
            dest_exponent -= 1;
        }
    } else {
        if Dbl_iszero_hidden(opnd3p1) {
            Dbl_leftshiftby1(opnd1p1, opnd1p2);
            Dbl_leftshiftby1(opnd3p1, opnd3p2);
            if Dbl_iszero_sign(opnd1p1) {
                Dbl_setone_lowmantissap2(opnd3p2);
                Twoword_subtract(opnd1p1, opnd1p2, opnd2p1, opnd2p2);
            } else {
                Twoword_add(opnd1p1, opnd1p2, opnd2p1, opnd2p2);
            }
            dest_exponent -= 1;
        }
        if Dbl_iszero_sign(opnd1p1) { guardbit = true; }
        stickybit = Dbl_allp1(opnd1p1) || Dbl_allp2(opnd1p2);
    }
    inexact = guardbit || stickybit;
    if inexact && (dest_exponent > 0 || Is_underflowtrap_enabled()) {
        Dbl_clear_signexponent(opnd3p1);
        match Rounding_mode() {
            ROUNDPLUS => if Dbl_iszero_sign(resultp1) { Dbl_increment(opnd3p1, opnd3p2); },
            ROUNDMINUS => if Dbl_isone_sign(resultp1) { Dbl_increment(opnd3p1, opnd3p2); },
            ROUNDNEAREST => if guardbit && (stickybit || Dbl_isone_lowmantissap2(opnd3p2)) { Dbl_increment(opnd3p1, opnd3p2); },
            _ => (),
        }
        if Dbl_isone_hidden(opnd3p1) { dest_exponent += 1; }
    }
    Dbl_set_mantissa(resultp1, resultp2, opnd3p1, opnd3p2);
    if dest_exponent >= DBL_INFINITY_EXPONENT {
        if Is_overflowtrap_enabled() {
            Dbl_setwrapped_exponent(resultp1, dest_exponent, ovfl);
            Dbl_copytoptr(resultp1, resultp2, dstptr);
            if inexact {
                if Is_inexacttrap_enabled() { return OVERFLOWEXCEPTION | INEXACTEXCEPTION; }
                Set_inexactflag();
            }
            return OVERFLOWEXCEPTION;
        }
        Set_overflowflag();
        Dbl_setoverflow(resultp1, resultp2);
        inexact = true;
    } else if dest_exponent <= 0 {
        if Is_underflowtrap_enabled() {
            Dbl_setwrapped_exponent(resultp1, dest_exponent, unfl);
            Dbl_copytoptr(resultp1, resultp2, dstptr);
            if inexact {
                if Is_inexacttrap_enabled() { return UNDERFLOWEXCEPTION | INEXACTEXCEPTION; }
                Set_inexactflag();
            }
            return UNDERFLOWEXCEPTION;
        }
        is_tiny = true;
        if dest_exponent == 0 && inexact {
            match Rounding_mode() {
                ROUNDPLUS => if Dbl_iszero_sign(resultp1) { Dbl_increment(opnd3p1, opnd3p2); if Dbl_isone_hiddenoverflow(opnd3p1) { is_tiny = false; } Dbl_decrement(opnd3p1, opnd3p2); },
                ROUNDMINUS => if Dbl_isone_sign(resultp1) { Dbl_increment(opnd3p1, opnd3p2); if Dbl_isone_hiddenoverflow(opnd3p1) { is_tiny = false; } Dbl_decrement(opnd3p1, opnd3p2); },
                ROUNDNEAREST => if guardbit && (stickybit || Dbl_isone_lowmantissap2(opnd3p2)) { Dbl_increment(opnd3p1, opnd3p2); if Dbl_isone_hiddenoverflow(opnd3p1) { is_tiny = false; } Dbl_decrement(opnd3p1, opnd3p2); },
                _ => (),
            }
        }
        stickybit = inexact;
        Dbl_denormalize(opnd3p1, opnd3p2, dest_exponent, guardbit, stickybit, inexact);
        if inexact {
            match Rounding_mode() {
                ROUNDPLUS => if Dbl_iszero_sign(resultp1) { Dbl_increment(opnd3p1, opnd3p2); },
                ROUNDMINUS => if Dbl_isone_sign(resultp1) { Dbl_increment(opnd3p1, opnd3p2); },
                ROUNDNEAREST => if guardbit && (stickybit || Dbl_isone_lowmantissap2(opnd3p2)) { Dbl_increment(opnd3p1, opnd3p2); },
                _ => (),
            }
            if is_tiny { Set_underflowflag(); }
        }
        Dbl_set_exponentmantissa(resultp1, resultp2, opnd3p1, opnd3p2);
    } else {
        Dbl_set_exponent(resultp1, dest_exponent);
    }
    Dbl_copytoptr(resultp1, resultp2, dstptr);
    if inexact {
        if Is_inexacttrap_enabled() { return INEXACTEXCEPTION; }
        Set_inexactflag();
    }
    NOEXCEPTION
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
