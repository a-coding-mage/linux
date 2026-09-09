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
 *	@(#)	pa/spmath/dfrem.c		$Revision: 1.1 $
 *
 *  Purpose:
 *	Double Precision Floating-point Remainder
 *
 * END_DESC
 */

// Dependencies supplied by the surrounding floating-point emulation code.

pub unsafe fn dbl_frem(
    srcptr1: *mut dbl_floating_point,
    srcptr2: *mut dbl_floating_point,
    dstptr: *mut dbl_floating_point,
    status: *mut u32,
) -> i32 {
    let mut opnd1p1: u32 = 0;
    let mut opnd1p2: u32 = 0;
    let mut opnd2p1: u32 = 0;
    let mut opnd2p2: u32 = 0;
    let mut resultp1: u32 = 0;
    let mut resultp2: u32 = 0;
    let mut opnd1_exponent: i32;
    let mut opnd2_exponent: i32;
    let mut dest_exponent: i32;
    let mut stepcount: i32;
    let mut roundup = false;

    Dbl_copyfromptr!(srcptr1, opnd1p1, opnd1p2);
    Dbl_copyfromptr!(srcptr2, opnd2p1, opnd2p2);
    if (opnd1_exponent = Dbl_exponent!(opnd1p1)) == DBL_INFINITY_EXPONENT {
        if Dbl_iszero_mantissa!(opnd1p1, opnd1p2) {
            if Dbl_isnotnan!(opnd2p1, opnd2p2) {
                if Is_invalidtrap_enabled!() { return INVALIDEXCEPTION; }
                Set_invalidflag!();
                Dbl_makequietnan!(resultp1, resultp2);
                Dbl_copytoptr!(resultp1, resultp2, dstptr);
                return NOEXCEPTION;
            }
        } else {
            if Dbl_isone_signaling!(opnd1p1) {
                if Is_invalidtrap_enabled!() { return INVALIDEXCEPTION; }
                Set_invalidflag!();
                Dbl_set_quiet!(opnd1p1);
            } else if Dbl_is_signalingnan!(opnd2p1) {
                if Is_invalidtrap_enabled!() { return INVALIDEXCEPTION; }
                Set_invalidflag!();
                Dbl_set_quiet!(opnd2p1);
                Dbl_copytoptr!(opnd2p1, opnd2p2, dstptr);
                return NOEXCEPTION;
            }
            Dbl_copytoptr!(opnd1p1, opnd1p2, dstptr);
            return NOEXCEPTION;
        }
    }
    if (opnd2_exponent = Dbl_exponent!(opnd2p1)) == DBL_INFINITY_EXPONENT {
        if Dbl_iszero_mantissa!(opnd2p1, opnd2p2) {
            Dbl_copytoptr!(opnd1p1, opnd1p2, dstptr);
            return NOEXCEPTION;
        }
        if Dbl_isone_signaling!(opnd2p1) {
            if Is_invalidtrap_enabled!() { return INVALIDEXCEPTION; }
            Set_invalidflag!();
            Dbl_set_quiet!(opnd2p1);
        }
        Dbl_copytoptr!(opnd2p1, opnd2p2, dstptr);
        return NOEXCEPTION;
    }
    if Dbl_iszero_exponentmantissa!(opnd2p1, opnd2p2) {
        if Is_invalidtrap_enabled!() { return INVALIDEXCEPTION; }
        Set_invalidflag!();
        Dbl_makequietnan!(resultp1, resultp2);
        Dbl_copytoptr!(resultp1, resultp2, dstptr);
        return NOEXCEPTION;
    }

    resultp1 = opnd1p1;
    if opnd1_exponent == 0 {
        if Dbl_iszero_mantissa!(opnd1p1, opnd1p2) {
            Dbl_copytoptr!(opnd1p1, opnd1p2, dstptr);
            return NOEXCEPTION;
        }
        opnd1_exponent = 1;
        Dbl_normalize!(opnd1p1, opnd1p2, opnd1_exponent);
    } else { Dbl_clear_signexponent_set_hidden!(opnd1p1); }
    if opnd2_exponent == 0 {
        opnd2_exponent = 1;
        Dbl_normalize!(opnd2p1, opnd2p2, opnd2_exponent);
    } else { Dbl_clear_signexponent_set_hidden!(opnd2p1); }

    dest_exponent = opnd2_exponent - 1;
    stepcount = opnd1_exponent - opnd2_exponent;
    if stepcount < 0 {
        if stepcount == -1 && Dbl_isgreaterthan!(opnd1p1, opnd1p2, opnd2p1, opnd2p2) {
            Dbl_allp1!(resultp1) = !Dbl_allp1!(resultp1);
            Dbl_leftshiftby1!(opnd2p1, opnd2p2);
            Dbl_subtract!(opnd2p1, opnd2p2, opnd1p1, opnd1p2, opnd2p1, opnd2p2);
            while Dbl_iszero_hidden!(opnd2p1) { Dbl_leftshiftby1!(opnd2p1, opnd2p2); dest_exponent -= 1; }
            Dbl_set_exponentmantissa!(resultp1, resultp2, opnd2p1, opnd2p2);
            goto_testforunderflow!(dest_exponent, resultp1, resultp2, dstptr);
        }
        Dbl_set_exponentmantissa!(resultp1, resultp2, opnd1p1, opnd1p2);
        dest_exponent = opnd1_exponent;
        goto_testforunderflow!(dest_exponent, resultp1, resultp2, dstptr);
    }
    while { if stepcount <= 0 { break; } stepcount -= 1; if !(Dbl_allp1!(opnd1p1) != 0 || Dbl_allp2!(opnd1p2) != 0) { break; } if Dbl_isnotlessthan!(opnd1p1, opnd1p2, opnd2p1, opnd2p2) { Dbl_subtract!(opnd1p1, opnd1p2, opnd2p1, opnd2p2, opnd1p1, opnd1p2); } Dbl_leftshiftby1!(opnd1p1, opnd1p2); }
    if Dbl_isnotlessthan!(opnd1p1, opnd1p2, opnd2p1, opnd2p2) { Dbl_subtract!(opnd1p1, opnd1p2, opnd2p1, opnd2p2, opnd1p1, opnd1p2); roundup = true; }
    if stepcount > 0 || Dbl_iszero!(opnd1p1, opnd1p2) { Dbl_setzero_exponentmantissa!(resultp1, resultp2); Dbl_copytoptr!(resultp1, resultp2, dstptr); return NOEXCEPTION; }
    Dbl_leftshiftby1!(opnd1p1, opnd1p2);
    if Dbl_isgreaterthan!(opnd1p1, opnd1p2, opnd2p1, opnd2p2) { Dbl_invert_sign!(resultp1); Dbl_leftshiftby1!(opnd2p1, opnd2p2); Dbl_subtract!(opnd2p1, opnd2p2, opnd1p1, opnd1p2, opnd1p1, opnd1p2); }
    else if Dbl_isequal!(opnd1p1, opnd1p2, opnd2p1, opnd2p2) && roundup { Dbl_invert_sign!(resultp1); }
    while Dbl_iszero_hidden!(opnd1p1) { dest_exponent -= 1; Dbl_leftshiftby1!(opnd1p1, opnd1p2); }
    Dbl_set_exponentmantissa!(resultp1, resultp2, opnd1p1, opnd1p2);
    if dest_exponent <= 0 {
        if Is_underflowtrap_enabled!() { Dbl_setwrapped_exponent!(resultp1, dest_exponent, unfl); Dbl_copytoptr!(resultp1, resultp2, dstptr); return UNDERFLOWEXCEPTION; }
        if dest_exponent >= 1 - DBL_P { Dbl_rightshift_exponentmantissa!(resultp1, resultp2, 1 - dest_exponent); } else { Dbl_setzero_exponentmantissa!(resultp1, resultp2); }
    } else { Dbl_set_exponent!(resultp1, dest_exponent); }
    Dbl_copytoptr!(resultp1, resultp2, dstptr);
    NOEXCEPTION
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
