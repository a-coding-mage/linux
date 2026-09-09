// SPDX-License-Identifier: GPL-2.0
/*
 * rational fractions
 *
 * Copyright (C) 2009 emlix GmbH, Oskar Schirmer <oskar@scara.com>
 * Copyright (C) 2019 Trent Piepho <tpiepho@gmail.com>
 *
 * helper functions when coping with rational numbers
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/rational.h, linux/compiler.h, linux/export.h, linux/minmax.h,
// linux/limits.h, and linux/module.h.

/*
 * calculate best rational approximation for a given fraction
 * taking into account restricted register size, e.g. to find
 * appropriate values for a pll with 5 bit denominator and
 * 8 bit numerator register fields, trying to set up with a
 * frequency ratio of 3.1415, one would say:
 *
 * rational_best_approximation(31415, 10000,
 *		(1 << 8) - 1, (1 << 5) - 1, &n, &d);
 *
 * you may look at given_numerator as a fixed point number,
 * with the fractional part size described in given_denominator.
 *
 * for theoretical background, see:
 * https://en.wikipedia.org/wiki/Continued_fraction
 */
pub unsafe fn rational_best_approximation(
    given_numerator: ::core::ffi::c_ulong,
    given_denominator: ::core::ffi::c_ulong,
    max_numerator: ::core::ffi::c_ulong,
    max_denominator: ::core::ffi::c_ulong,
    best_numerator: *mut ::core::ffi::c_ulong,
    best_denominator: *mut ::core::ffi::c_ulong,
) {
    /* n/d is the starting rational, which is continually
     * decreased each iteration using the Euclidean algorithm.
     *
     * dp is the value of d from the prior iteration.
     *
     * n2/d2, n1/d1, and n0/d0 are our successively more accurate
     * approximations of the rational.  They are, respectively,
     * the current, previous, and two prior iterations of it.
     *
     * a is current term of the continued fraction.
     */
    let mut n: ::core::ffi::c_ulong = given_numerator;
    let mut d: ::core::ffi::c_ulong = given_denominator;
    let mut n0: ::core::ffi::c_ulong;
    let mut d0: ::core::ffi::c_ulong;
    let mut n1: ::core::ffi::c_ulong;
    let mut d1: ::core::ffi::c_ulong;
    let mut n2: ::core::ffi::c_ulong = 0;
    let mut d2: ::core::ffi::c_ulong = 0;
    n0 = 0;
    d1 = 0;
    n1 = 1;
    d0 = 1;

    loop {
        let dp: ::core::ffi::c_ulong;
        let a: ::core::ffi::c_ulong;

        if d == 0 {
            break;
        }
        /* Find next term in continued fraction, 'a', via
         * Euclidean algorithm.
         */
        dp = d;
        a = n / d;
        d = n % d;
        n = dp;

        /* Calculate the current rational approximation (aka
         * convergent), n2/d2, using the term just found and
         * the two prior approximations.
         */
        n2 = n0 + a * n1;
        d2 = d0 + a * d1;

        /* If the current convergent exceeds the maxes, then
         * return either the previous convergent or the
         * largest semi-convergent, the final term of which is
         * found below as 't'.
         */
        if n2 > max_numerator || d2 > max_denominator {
            let mut t: ::core::ffi::c_ulong = ::core::ffi::c_ulong::MAX;

            if d1 != 0 {
                t = (max_denominator - d0) / d1;
            }
            if n1 != 0 {
                t = core::cmp::min(t, (max_numerator - n0) / n1);
            }

            /* This tests if the semi-convergent is closer than the previous
             * convergent.  If d1 is zero there is no previous convergent as this
             * is the 1st iteration, so always choose the semi-convergent.
             */
            if d1 == 0 || 2u64.wrapping_mul(t) > a ||
                (2u64.wrapping_mul(t) == a && d0 * dp > d1 * d)
            {
                n1 = n0 + t * n1;
                d1 = d0 + t * d1;
            }
            break;
        }
        n0 = n1;
        n1 = n2;
        d0 = d1;
        d1 = d2;
    }
    *best_numerator = n1;
    *best_denominator = d1;
}

// EXPORT_SYMBOL(rational_best_approximation);
// MODULE_DESCRIPTION("Rational fraction support library");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
