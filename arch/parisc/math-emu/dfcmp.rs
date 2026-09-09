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
 *	@(#)	pa/spmath/dfcmp.c		$Revision: 1.1 $
 *
 *  Purpose:
 *	dbl_cmp: compare two values
 *
 *  External Interfaces:
 *	dbl_fcmp(leftptr, rightptr, cond, status)
 *
 * END_DESC
 */

/* Declarations supplied by float.h and dbl_float.h are external dependencies. */

/*
 * dbl_cmp: compare two values
 */
pub unsafe fn dbl_fcmp(
    leftptr: *mut dbl_floating_point,
    rightptr: *mut dbl_floating_point,
    cond: u32,
    status: *mut u32,
) -> i32 {
    let mut leftp1: u32 = 0;
    let mut leftp2: u32 = 0;
    let mut rightp1: u32 = 0;
    let mut rightp2: u32 = 0;
    let mut xorresult: i32 = 0;

    /* Create local copies of the numbers */
    Dbl_copyfromptr(leftptr, leftp1, leftp2);
    Dbl_copyfromptr(rightptr, rightp1, rightp2);
    /*
     * Test for NaN
     */
    if Dbl_exponent(leftp1) == DBL_INFINITY_EXPONENT
        || Dbl_exponent(rightp1) == DBL_INFINITY_EXPONENT
    {
        /* Check if a NaN is involved.  Signal an invalid exception when
         * comparing a signaling NaN or when comparing quiet NaNs and the
         * low bit of the condition is set */
        if (Dbl_exponent(leftp1) == DBL_INFINITY_EXPONENT
            && Dbl_isnotzero_mantissa(leftp1, leftp2)
            && (Exception(cond) || Dbl_isone_signaling(leftp1)))
            || (Dbl_exponent(rightp1) == DBL_INFINITY_EXPONENT
                && Dbl_isnotzero_mantissa(rightp1, rightp2)
                && (Exception(cond) || Dbl_isone_signaling(rightp1)))
        {
            if Is_invalidtrap_enabled() {
                Set_status_cbit(Unordered(cond));
                return INVALIDEXCEPTION;
            } else {
                Set_invalidflag();
            }
            Set_status_cbit(Unordered(cond));
            return NOEXCEPTION;
        } else if (Dbl_exponent(leftp1) == DBL_INFINITY_EXPONENT
            && Dbl_isnotzero_mantissa(leftp1, leftp2))
            || (Dbl_exponent(rightp1) == DBL_INFINITY_EXPONENT
                && Dbl_isnotzero_mantissa(rightp1, rightp2))
        {
            /* NaNs always compare unordered. */
            Set_status_cbit(Unordered(cond));
            return NOEXCEPTION;
        }
        /* infinities will drop down to the normal compare mechanisms */
    }

    /* First compare for unequal signs => less or greater or
     * special equal case */
    Dbl_xortointp1(leftp1, rightp1, xorresult);
    if xorresult < 0 {
        /* left negative => less, left positive => greater.
         * equal is possible if both operands are zeros. */
        if Dbl_iszero_exponentmantissa(leftp1, leftp2)
            && Dbl_iszero_exponentmantissa(rightp1, rightp2)
        {
            Set_status_cbit(Equal(cond));
        } else if Dbl_isone_sign(leftp1) {
            Set_status_cbit(Lessthan(cond));
        } else {
            Set_status_cbit(Greaterthan(cond));
        }
    } else if Dbl_isequal(leftp1, leftp2, rightp1, rightp2) {
        Set_status_cbit(Equal(cond));
    } else if Dbl_iszero_sign(leftp1) {
        /* Positive compare */
        if Dbl_allp1(leftp1) < Dbl_allp1(rightp1) {
            Set_status_cbit(Lessthan(cond));
        } else if Dbl_allp1(leftp1) > Dbl_allp1(rightp1) {
            Set_status_cbit(Greaterthan(cond));
        } else {
            /* Equal first parts.  Now we must use unsigned compares to
             * resolve the two possibilities. */
            if Dbl_allp2(leftp2) < Dbl_allp2(rightp2) {
                Set_status_cbit(Lessthan(cond));
            } else {
                Set_status_cbit(Greaterthan(cond));
            }
        }
    } else {
        /* Negative compare.  Signed or unsigned compares
         * both work the same.  That distinction is only
         * important when the sign bits differ. */
        if Dbl_allp1(leftp1) > Dbl_allp1(rightp1) {
            Set_status_cbit(Lessthan(cond));
        } else if Dbl_allp1(leftp1) < Dbl_allp1(rightp1) {
            Set_status_cbit(Greaterthan(cond));
        } else {
            /* Equal first parts.  Now we must use unsigned compares to
             * resolve the two possibilities. */
            if Dbl_allp2(leftp2) > Dbl_allp2(rightp2) {
                Set_status_cbit(Lessthan(cond));
            } else {
                Set_status_cbit(Greaterthan(cond));
            }
        }
    }
    NOEXCEPTION
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
