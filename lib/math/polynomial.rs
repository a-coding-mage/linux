// SPDX-License-Identifier: GPL-2.0-only
/*
 * Generic polynomial calculation using integer coefficients.
 *
 * Copyright (C) 2020 BAIKAL ELECTRONICS, JSC
 *
 * Authors:
 *   Maxim Kaurkin <maxim.kaurkin@baikalelectronics.ru>
 *   Serge Semin <Sergey.Semin@baikalelectronics.ru>
 *
 */
//! Checked, allocation-free polynomial evaluation with native-long arithmetic.
//!
//! Factor redistribution retains the original quotient/remainder operation
//! order. Products and sums wrap at the native word width, as in kernel C built
//! with `-fno-strict-overflow`; this is not widened or Horner evaluation.

/// One factor-redistributed term, with the original unsigned degree.
///
/// This value type has the original term's field layout. A native C boundary
/// should still use the actual generated bindings for its nominal C types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PolynomialTerm {
    /// Number of redistributed multiplications to perform.
    pub deg: u32,
    /// Initial signed coefficient.
    pub coef: isize,
    /// Divisor applied during each multiplication; unused when `deg` is zero.
    pub divider: isize,
    /// Divisor applied to this term after all its multiplications.
    pub divider_leftover: isize,
}

/// Bounded Rust view of a polynomial, not the C flexible-array structure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Polynomial<'a> {
    /// Divisor applied to the wrapping sum; zero means one.
    pub total_divider: isize,
    /// Ordered terms, ending at the first term whose degree is zero.
    pub terms: &'a [PolynomialTerm],
}

/// Apply one original `mult_frac` step without changing its rounding order.
const fn mult_frac(value: isize, data: isize, divider: isize) -> Option<isize> {
    let quotient = match value.checked_div(divider) {
        Some(value) => value,
        None => return None,
    };
    // The checked quotient above also establishes the remainder's domain.
    let remainder = value % divider;
    let fractional = match remainder.wrapping_mul(data).checked_div(divider) {
        Some(value) => value,
        None => return None,
    };
    Some(quotient.wrapping_mul(data).wrapping_add(fractional))
}

/// Calculate a single term, including its leftover divisor.
///
/// Returns `None` only for a division by zero or signed `MIN / -1` actually
/// reached by the original evaluation. A degree-zero term does not inspect
/// its per-step divider, but still uses its leftover divider.
///
/// All `u32` degrees are accepted. The original signed loop counter wraps
/// under kernel C flags, and its unsigned comparison gives this same count.
/// Work is linear in the degree: callers handling untrusted descriptors must
/// bound that work themselves. No coefficient shortcut skips later errors.
pub const fn polynomial_term_value(term: &PolynomialTerm, data: isize) -> Option<isize> {
    let mut value = term.coef;
    let mut degree = 0u32;
    while degree < term.deg {
        value = match mult_frac(value, data, term.divider) {
            Some(value) => value,
            None => return None,
        };
        // degree < term.deg <= u32::MAX, so this cannot wrap.
        degree += 1;
    }
    value.checked_div(term.divider_leftover)
}

/// Normalize an already wrapping term sum, treating a zero divisor as one.
///
/// Returns `None` for signed `MIN / -1`. This and
/// [`polynomial_term_value`] let a native owner walk actual C flexible-array
/// bindings incrementally, without manufacturing an unbounded Rust slice.
pub const fn polynomial_finalize(sum: isize, total_divider: isize) -> Option<isize> {
    sum.checked_div(if total_divider == 0 { 1 } else { total_divider })
}

/// Evaluate ordered terms up to and including the first degree-zero term.
///
/// Later terms are ignored, including invalid ones. Returns `None` if the
/// supplied slice has no terminating constant or an evaluated division is
/// undefined. Products and accumulation wrap at the native C `long` width;
/// signed division truncates toward zero.
pub const fn polynomial_calc(poly: &Polynomial<'_>, data: isize) -> Option<isize> {
    let mut terms = poly.terms;
    let mut sum = 0isize;
    while let Some((term, remaining)) = terms.split_first() {
        let value = match polynomial_term_value(term, data) {
            Some(value) => value,
            None => return None,
        };
        sum = sum.wrapping_add(value);
        if term.deg == 0 {
            return polynomial_finalize(sum, poly.total_divider);
        }
        terms = remaining;
    }
    None
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
