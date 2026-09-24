/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2003 Bernardo Innocenti <bernie@develer.com>
 * Based on former asm-ppc/div64.h and asm-m68knommu/div64.h
 *
 * Optimization for constant divisors on 32-bit machines:
 * Copyright (C) 2006-2015 Nicolas Pitre
 */

//! Safe generic division and reciprocal-product helpers.
//!
//! Division by zero returns `None`. The mutation-based [`do_div`] leaves its
//! input unchanged in that case. Ordinary functions replace the C macros, so
//! arguments are evaluated once and no crate-global macros are introduced.

#[allow(dead_code, unreachable_pub)]
#[path = "../../lib/math/div64.rs"]
pub(crate) mod implementation;

pub use implementation::do_div;

/// Divide a mutable 64-bit value by a 32-bit divisor, returning the remainder.
///
/// This is the safe generic counterpart of the C pointer helper. Zero leaves
/// the dividend unchanged and returns `None`.
pub const fn __div64_32(dividend: &mut u64, divisor: u32) -> Option<u32> {
    do_div(dividend, divisor)
}

/// Divide by a 32-bit divisor, including when both operands are constant.
///
/// The C macro's reciprocal optimization does not affect its quotient. Rust
/// uses the same canonical division core for runtime and constant evaluation,
/// rather than trying to reproduce `__builtin_constant_p` dispatch.
pub const fn __div64_const32(dividend: u64, divisor: u32) -> Option<u64> {
    implementation::div_u64(dividend, divisor)
}

/// Return the high word of `m * n + if bias { m } else { 0 }`.
///
/// The full unsigned product and optional bias fit in 128 bits. Limb
/// arithmetic does not require a compiler-provided wide multiplication helper.
pub const fn __arch_xprod_64(m: u64, n: u64, bias: bool) -> u64 {
    let (low, high) = implementation::mul_u64_wide(m, n);
    if bias {
        let (_, carry) = low.overflowing_add(m);
        high.wrapping_add(carry as u64)
    } else {
        high
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
