// SPDX-License-Identifier: GPL-2.0-only
//! Allocation-free least common multiples with native unsigned wrapping.

#[path = "gcd.rs"]
// GCD's public standalone API is an implementation detail of this module.
#[allow(unreachable_pub)]
mod greatest_common_divisor;

/// Returns the least common multiple, or zero if either argument is zero.
///
/// Division precedes multiplication; a result exceeding `usize` wraps exactly
/// like the kernel's unsigned-long implementation.
#[inline]
pub const fn lcm(a: usize, b: usize) -> usize {
    multiply_quotient(a, b, greatest_common_divisor::gcd(a, b))
}

/// Returns the least common multiple using an explicit GCD strategy.
#[inline]
pub const fn lcm_with_ffs(a: usize, b: usize, efficient_ffs: bool) -> usize {
    multiply_quotient(
        a,
        b,
        greatest_common_divisor::gcd_with_ffs(a, b, efficient_ffs),
    )
}

const fn multiply_quotient(a: usize, b: usize, divisor: usize) -> usize {
    // gcd is zero only when both inputs are zero. checked_div handles that case
    // and avoids a division-by-zero panic dependency even in unoptimized code.
    match a.checked_div(divisor) {
        Some(quotient) => quotient.wrapping_mul(b),
        None => 0,
    }
}

/// Returns the nonzero LCM, falling back to `b`, then `a`, when it is zero.
#[inline]
pub const fn lcm_not_zero(a: usize, b: usize) -> usize {
    nonzero_result(a, b, lcm(a, b))
}

/// Returns the nonzero LCM using an explicit GCD strategy.
#[inline]
pub const fn lcm_not_zero_with_ffs(a: usize, b: usize, efficient_ffs: bool) -> usize {
    nonzero_result(a, b, lcm_with_ffs(a, b, efficient_ffs))
}

const fn nonzero_result(a: usize, b: usize, result: usize) -> usize {
    if result != 0 {
        result
    } else if b != 0 {
        b
    } else {
        a
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
