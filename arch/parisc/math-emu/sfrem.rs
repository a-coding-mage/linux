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
 *	@(#) pa/spmath/sfrem.c $Revision: 1.1 $
 *
 *  Purpose:
 *	Single Precision Floating-point Remainder
 *
 * END_DESC
 */

pub unsafe fn sgl_frem(
    srcptr1: *mut sgl_floating_point,
    srcptr2: *mut sgl_floating_point,
    dstptr: *mut sgl_floating_point,
    status: *mut u32,
) -> i32 {
    let mut opnd1: u32 = *srcptr1;
    let mut opnd2: u32 = *srcptr2;
    let mut result: u32;
    let mut opnd1_exponent: i32;
    let mut opnd2_exponent: i32;
    let mut dest_exponent: i32;
    let mut stepcount: i32;
    let mut roundup = false;

    let _ = status;
    opnd1_exponent = Sgl_exponent(opnd1);
    if opnd1_exponent == SGL_INFINITY_EXPONENT {
        if Sgl_iszero_mantissa(opnd1) {
            if Sgl_isnotnan(opnd2) {
                if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
                Set_invalidflag();
                Sgl_makequietnan(&mut result);
                *dstptr = result;
                return NOEXCEPTION;
            }
        } else {
            if Sgl_isone_signaling(opnd1) {
                if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
                Set_invalidflag();
                Sgl_set_quiet(&mut opnd1);
            } else if Sgl_is_signalingnan(opnd2) {
                if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
                Set_invalidflag();
                Sgl_set_quiet(&mut opnd2);
                *dstptr = opnd2;
                return NOEXCEPTION;
            }
            *dstptr = opnd1;
            return NOEXCEPTION;
        }
    }

    opnd2_exponent = Sgl_exponent(opnd2);
    if opnd2_exponent == SGL_INFINITY_EXPONENT {
        if Sgl_iszero_mantissa(opnd2) {
            *dstptr = opnd1;
            return NOEXCEPTION;
        }
        if Sgl_isone_signaling(opnd2) {
            if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
            Set_invalidflag();
            Sgl_set_quiet(&mut opnd2);
        }
        *dstptr = opnd2;
        return NOEXCEPTION;
    }
    if Sgl_iszero_exponentmantissa(opnd2) {
        if Is_invalidtrap_enabled() { return INVALIDEXCEPTION; }
        Set_invalidflag();
        Sgl_makequietnan(&mut result);
        *dstptr = result;
        return NOEXCEPTION;
    }

    result = opnd1;
    if opnd1_exponent == 0 {
        if Sgl_iszero_mantissa(opnd1) {
            *dstptr = opnd1;
            return NOEXCEPTION;
        }
        opnd1_exponent = 1;
        Sgl_normalize(&mut opnd1, &mut opnd1_exponent);
    } else {
        Sgl_clear_signexponent_set_hidden(&mut opnd1);
    }
    if opnd2_exponent == 0 {
        opnd2_exponent = 1;
        Sgl_normalize(&mut opnd2, &mut opnd2_exponent);
    } else {
        Sgl_clear_signexponent_set_hidden(&mut opnd2);
    }

    dest_exponent = opnd2_exponent - 1;
    stepcount = opnd1_exponent - opnd2_exponent;
    'calculation: {
        if stepcount < 0 {
            if stepcount == -1 && Sgl_isgreaterthan(opnd1, opnd2) {
                Sgl_invert_sign(&mut result);
                Sgl_leftshiftby1(&mut opnd2);
                Sgl_subtract(opnd2, opnd1, &mut opnd2);
                while Sgl_iszero_hidden(opnd2) {
                    Sgl_leftshiftby1(&mut opnd2);
                    dest_exponent -= 1;
                }
                Sgl_set_exponentmantissa(&mut result, opnd2);
                break 'calculation;
            }
            Sgl_set_exponentmantissa(&mut result, opnd1);
            dest_exponent = opnd1_exponent;
            break 'calculation;
        }
        while stepcount > 0 && Sgl_all(opnd1) {
            stepcount -= 1;
            if Sgl_isnotlessthan(opnd1, opnd2) {
                Sgl_subtract(opnd1, opnd2, &mut opnd1);
            }
            Sgl_leftshiftby1(&mut opnd1);
        }
        if Sgl_isnotlessthan(opnd1, opnd2) {
            Sgl_subtract(opnd1, opnd2, &mut opnd1);
            roundup = true;
        }
        if stepcount > 0 || Sgl_iszero(opnd1) {
            Sgl_setzero_exponentmantissa(&mut result);
            *dstptr = result;
            return NOEXCEPTION;
        }
        Sgl_leftshiftby1(&mut opnd1);
        if Sgl_isgreaterthan(opnd1, opnd2) {
            Sgl_invert_sign(&mut result);
            Sgl_subtract(opnd2 << 1, opnd1, &mut opnd1);
        } else if Sgl_isequal(opnd1, opnd2) && roundup {
            Sgl_invert_sign(&mut result);
        }
        while Sgl_iszero_hidden(opnd1) {
            dest_exponent -= 1;
            Sgl_leftshiftby1(&mut opnd1);
        }
        Sgl_set_exponentmantissa(&mut result, opnd1);
    }

    if dest_exponent <= 0 {
        if Is_underflowtrap_enabled() {
            Sgl_setwrapped_exponent(&mut result, dest_exponent, unfl);
            *dstptr = result;
            return UNDERFLOWEXCEPTION;
        }
        if dest_exponent >= 1 - SGL_P {
            Sgl_rightshift_exponentmantissa(&mut result, 1 - dest_exponent);
        } else {
            Sgl_setzero_exponentmantissa(&mut result);
        }
    } else {
        Sgl_set_exponent(&mut result, dest_exponent);
    }
    *dstptr = result;
    NOEXCEPTION
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
