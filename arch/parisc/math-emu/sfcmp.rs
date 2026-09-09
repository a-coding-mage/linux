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
 *	@(#)	pa/spmath/sfcmp.c		$Revision: 1.1 $
 *
 *  Purpose:
 *	sgl_cmp: compare two values
 *
 *  External Interfaces:
 *	sgl_fcmp(leftptr, rightptr, cond, status)
 *
 *  Internal Interfaces:
 *
 *  Theory:
 *	<<please update with a overview of the operation of this file>>
 *
 * END_DESC
 */

/* Dependencies are supplied by the floating-point emulation environment. */

/* sgl_cmp: compare two values */
pub unsafe fn sgl_fcmp(
    leftptr: *const u32,
    rightptr: *const u32,
    cond: u32,
    status: *mut u32,
) -> i32 {
    let left: u32;
    let right: u32;
    let xorresult: i32;

    /* Create local copies of the numbers */
    left = *leftptr;
    right = *rightptr;

    /*
     * Test for NaN
     */
    if (Sgl_exponent(left) == SGL_INFINITY_EXPONENT
        || Sgl_exponent(right) == SGL_INFINITY_EXPONENT)
    {
        /* Check if a NaN is involved.  Signal an invalid exception when
         * comparing a signaling NaN or when comparing quiet NaNs and the
         * low bit of the condition is set */
        if ((Sgl_exponent(left) == SGL_INFINITY_EXPONENT
            && Sgl_isnotzero_mantissa(left)
            && (Exception(cond) || Sgl_isone_signaling(left)))
            || (Sgl_exponent(right) == SGL_INFINITY_EXPONENT
                && Sgl_isnotzero_mantissa(right)
                && (Exception(cond) || Sgl_isone_signaling(right))))
        {
            if (Is_invalidtrap_enabled()) {
                Set_status_cbit(Unordered(cond));
                return INVALIDEXCEPTION;
            } else {
                Set_invalidflag();
            }
            Set_status_cbit(Unordered(cond));
            return NOEXCEPTION;
        }
        /* All the exceptional conditions are handled, now special case
         * NaN compares */
        else if ((Sgl_exponent(left) == SGL_INFINITY_EXPONENT
            && Sgl_isnotzero_mantissa(left))
            || (Sgl_exponent(right) == SGL_INFINITY_EXPONENT
                && Sgl_isnotzero_mantissa(right)))
        {
            /* NaNs always compare unordered. */
            Set_status_cbit(Unordered(cond));
            return NOEXCEPTION;
        }
        /* infinities will drop down to the normal compare mechanisms */
    }

    /* First compare for unequal signs => less or greater or
     * special equal case */
    xorresult = (left ^ right) as i32;
    if xorresult < 0 {
        /* left negative => less, left positive => greater.
         * equal is possible if both operands are zeros. */
        if Sgl_iszero_exponentmantissa(left) && Sgl_iszero_exponentmantissa(right) {
            Set_status_cbit(Equal(cond));
        } else if Sgl_isone_sign(left) {
            Set_status_cbit(Lessthan(cond));
        } else {
            Set_status_cbit(Greaterthan(cond));
        }
    }
    /* Signs are the same.  Treat negative numbers separately
     * from the positives because of the reversed sense. */
    else if Sgl_all(left) == Sgl_all(right) {
        Set_status_cbit(Equal(cond));
    } else if Sgl_iszero_sign(left) {
        /* Positive compare */
        if Sgl_all(left) < Sgl_all(right) {
            Set_status_cbit(Lessthan(cond));
        } else {
            Set_status_cbit(Greaterthan(cond));
        }
    } else {
        /* Negative compare.  Signed or unsigned compares
         * both work the same.  That distinction is only
         * important when the sign bits differ. */
        if Sgl_all(left) > Sgl_all(right) {
            Set_status_cbit(Lessthan(cond));
        } else {
            Set_status_cbit(Greaterthan(cond));
        }
    }
    NOEXCEPTION
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
