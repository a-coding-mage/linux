// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2009 emlix GmbH, Oskar Schirmer <oskar@scara.com>
// Copyright (C) 2019 Trent Piepho <tpiepho@gmail.com>

//! Native-word rational approximation using continued fractions.

/// Approximates a fraction within the supplied numerator and denominator limits.
///
/// Returns the numerator and denominator of the selected convergent or
/// semi-convergent. For example, approximating `31415 / 10000` with an 8-bit
/// numerator and 5-bit denominator returns `(22, 7)`.
///
/// All arithmetic preserves the kernel's unsigned-long wrapping, including
/// comparisons of products. Zero bounds retain the original algorithm's
/// behavior rather than being rejected; a zero input denominator returns
/// `(1, 0)`. These edge cases need not describe a fraction within the bounds.
#[inline]
pub const fn rational_best_approximation(
    given_numerator: usize,
    given_denominator: usize,
    max_numerator: usize,
    max_denominator: usize,
) -> (usize, usize) {
    let (mut n, mut d) = (given_numerator, given_denominator);
    let (mut n0, mut d0) = (0usize, 1usize);
    let (mut n1, mut d1) = (1usize, 0usize);

    loop {
        // A zero denominator ends the Euclidean iteration. Checked division
        // and remainder avoid panic dependencies even in unoptimized builds.
        let (a, remainder) = match (n.checked_div(d), n.checked_rem(d)) {
            (Some(a), Some(remainder)) => (a, remainder),
            _ => break,
        };
        let dp = d;
        d = remainder;
        n = dp;

        let n2 = n0.wrapping_add(a.wrapping_mul(n1));
        let d2 = d0.wrapping_add(a.wrapping_mul(d1));
        if n2 > max_numerator || d2 > max_denominator {
            // A zero previous numerator/denominator imposes no bound on t.
            let mut t = match max_denominator.wrapping_sub(d0).checked_div(d1) {
                Some(bound) => bound,
                None => usize::MAX,
            };
            if let Some(bound) = max_numerator.wrapping_sub(n0).checked_div(n1) {
                if bound < t {
                    t = bound;
                }
            }

            // With no previous convergent, always choose the semi-convergent.
            // Otherwise preserve both C's tie-break and native-word overflow.
            let twice_t = t.wrapping_mul(2);
            if d1 == 0 || twice_t > a || (twice_t == a && d0.wrapping_mul(dp) > d1.wrapping_mul(d))
            {
                n1 = n0.wrapping_add(t.wrapping_mul(n1));
                d1 = d0.wrapping_add(t.wrapping_mul(d1));
            }
            break;
        }

        n0 = n1;
        n1 = n2;
        d0 = d1;
        d1 = d2;
    }
    (n1, d1)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
