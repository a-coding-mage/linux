// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2003 Bernardo Innocenti <bernie@develer.com>
 *
 * Based on former do_div() implementation from asm-parisc/div64.h:
 * Copyright (C) 1999 Hewlett-Packard Co
 * Copyright (C) 1999 David Mosberger-Tang <davidm@hpl.hp.com>
 */
//! Checked, allocation-free division and wide multiply/add/divide.
//!
//! Remainders are returned by value. Zero divisors return `None`; signed public
//! helpers also reject the unrepresentable quotient `i64::MIN / -1`. Native
//! 32-bit owners use the explicitly wrapping signed helpers instead. Wide
//! multiply/add/divide follows the generic C implementation's saturation rule,
//! not the overflow trap of some architecture-specific implementations.

/// Generic shift/subtract division, using only 32-bit division instructions.
#[allow(dead_code)] // Native 32-bit owners and differential tests use this helper.
pub(crate) const fn div_u64_rem_generic(mut n: u64, d: u32) -> Option<(u64, u32)> {
    if d == 0 {
        return None;
    }
    let mut quotient = 0u64;
    let high = (n >> 32) as u32;
    if high >= d {
        let high_quotient = high / d;
        quotient = (high_quotient as u64) << 32;
        n = n.wrapping_sub((high_quotient.wrapping_mul(d) as u64) << 32);
    }
    let mut shifted_divisor = d as u64;
    let mut bit = 1u64;
    while (shifted_divisor as i64) > 0 && shifted_divisor < n {
        shifted_divisor = shifted_divisor.wrapping_add(shifted_divisor);
        bit = bit.wrapping_add(bit);
    }
    loop {
        if n >= shifted_divisor {
            n = n.wrapping_sub(shifted_divisor);
            quotient = quotient.wrapping_add(bit);
        }
        shifted_divisor >>= 1;
        bit >>= 1;
        if bit == 0 {
            break;
        }
    }
    Some((quotient, n as u32))
}

/// Divide a 64-bit dividend by a nonzero 32-bit divisor, returning its remainder.
pub const fn div_u64_rem(n: u64, d: u32) -> Option<(u64, u32)> {
    #[cfg(target_pointer_width = "32")]
    {
        div_u64_rem_generic(n, d)
    }
    #[cfg(not(target_pointer_width = "32"))]
    {
        if d == 0 {
            None
        } else {
            Some((n / d as u64, (n % d as u64) as u32))
        }
    }
}

/// Divide a 64-bit dividend by a 32-bit divisor, or return `None` for zero.
pub const fn div_u64(n: u64, d: u32) -> Option<u64> {
    match div_u64_rem(n, d) {
        Some((quotient, _)) => Some(quotient),
        None => None,
    }
}

/// Replace a dividend with its quotient and return the remainder.
///
/// A zero divisor returns `None` without changing the dividend.
pub const fn do_div(n: &mut u64, d: u32) -> Option<u32> {
    match div_u64_rem(*n, d) {
        Some((quotient, remainder)) => {
            *n = quotient;
            Some(remainder)
        }
        None => None,
    }
}

/// Divide two unsigned 64-bit values and return the quotient and remainder.
pub const fn div64_u64_rem(n: u64, d: u64) -> Option<(u64, u64)> {
    let high = (d >> 32) as u32;
    if high == 0 {
        return match div_u64_rem(n, d as u32) {
            Some((quotient, remainder)) => Some((quotient, remainder as u64)),
            None => None,
        };
    }
    let shift = 32 - high.leading_zeros();
    let mut quotient = match div_u64(n >> shift, (d >> shift) as u32) {
        Some(value) => value,
        None => return None,
    };
    if quotient != 0 {
        quotient -= 1;
    }
    let mut remainder = n.wrapping_sub(quotient.wrapping_mul(d));
    if remainder >= d {
        quotient = quotient.wrapping_add(1);
        remainder = remainder.wrapping_sub(d);
    }
    Some((quotient, remainder))
}

/// Divide two unsigned 64-bit values without calculating an output remainder.
pub const fn div64_u64(n: u64, d: u64) -> Option<u64> {
    let high = (d >> 32) as u32;
    if high == 0 {
        return div_u64(n, d as u32);
    }
    let shift = 32 - high.leading_zeros();
    let mut quotient = match div_u64(n >> shift, (d >> shift) as u32) {
        Some(value) => value,
        None => return None,
    };
    if quotient != 0 {
        quotient -= 1;
    }
    if n.wrapping_sub(quotient.wrapping_mul(d)) >= d {
        quotient = quotient.wrapping_add(1);
    }
    Some(quotient)
}

/// Native ILP32 magnitude/sign semantics, including the wrapped MIN/-1 result.
pub(crate) const fn div_s64_rem_wrapping(n: i64, d: i32) -> Option<(i64, i32)> {
    match div_u64_rem(n.unsigned_abs(), d.unsigned_abs()) {
        Some((quotient, remainder)) => {
            let quotient = if (n < 0) != (d < 0) {
                quotient.wrapping_neg() as i64
            } else {
                quotient as i64
            };
            let remainder = if n < 0 {
                (remainder as i32).wrapping_neg()
            } else {
                remainder as i32
            };
            Some((quotient, remainder))
        }
        None => None,
    }
}

/// Divide signed values, rejecting zero and an unrepresentable quotient.
pub const fn div_s64_rem(n: i64, d: i32) -> Option<(i64, i32)> {
    if n == i64::MIN && d == -1 {
        None
    } else {
        div_s64_rem_wrapping(n, d)
    }
}

/// Native ILP32 signed quotient, wrapping only the MIN/-1 overflow.
#[allow(dead_code)] // Native owners and the differential harness use this variant.
pub(crate) const fn div_s64_wrapping(n: i64, d: i32) -> Option<i64> {
    match div_s64_rem_wrapping(n, d) {
        Some((quotient, _)) => Some(quotient),
        None => None,
    }
}

/// Divide a signed 64-bit dividend by a signed 32-bit divisor.
pub const fn div_s64(n: i64, d: i32) -> Option<i64> {
    match div_s64_rem(n, d) {
        Some((quotient, _)) => Some(quotient),
        None => None,
    }
}

/// Native ILP32 signed quotient/remainder, including wrapped MIN/-1.
pub(crate) const fn div64_s64_rem_wrapping(n: i64, d: i64) -> Option<(i64, i64)> {
    match div64_u64_rem(n.unsigned_abs(), d.unsigned_abs()) {
        Some((quotient, remainder)) => {
            let sign = n >> 63;
            let remainder = ((remainder as i64) ^ sign).wrapping_sub(sign);
            let sign = (n ^ d) >> 63;
            Some((((quotient as i64) ^ sign).wrapping_sub(sign), remainder))
        }
        None => None,
    }
}

/// Divide signed 64-bit values, rejecting zero and an unrepresentable quotient.
pub const fn div64_s64_rem(n: i64, d: i64) -> Option<(i64, i64)> {
    if n == i64::MIN && d == -1 {
        None
    } else {
        div64_s64_rem_wrapping(n, d)
    }
}

/// Native ILP32 signed quotient with wrapping sign restoration.
pub(crate) const fn div64_s64_wrapping(n: i64, d: i64) -> Option<i64> {
    match div64_u64(n.unsigned_abs(), d.unsigned_abs()) {
        Some(quotient) => {
            let sign = (n ^ d) >> 63;
            Some(((quotient as i64) ^ sign).wrapping_sub(sign))
        }
        None => None,
    }
}

/// Divide signed 64-bit values without an output remainder.
pub const fn div64_s64(n: i64, d: i64) -> Option<i64> {
    if n == i64::MIN && d == -1 {
        None
    } else {
        div64_s64_wrapping(n, d)
    }
}

/// Iterative division for dividends expected to be close to their divisor.
///
/// Returns `None` for zero. The quotient counter wraps at 32 bits, as in C.
/// The optimization barrier prevents replacement with a wide divide intrinsic.
/// Unlike the other helpers, this function is not constant-evaluable on Rust
/// 1.85 because that compiler's constant `black_box` is unstable.
pub fn iter_div_u64_rem(mut n: u64, d: u32) -> Option<(u32, u64)> {
    if d == 0 {
        return None;
    }
    let mut quotient = 0u32;
    while n >= d as u64 {
        n = core::hint::black_box(n);
        n = n.wrapping_sub(d as u64);
        quotient = quotient.wrapping_add(1);
    }
    Some((quotient, n))
}

// The original product helper truncates both multiplicands and the addend to
// 32 bits before multiplying. Its maximum sum fits in a u64.
const fn mul_add(a: u64, b: u64, c: u64) -> u64 {
    (a as u32 as u64) * (b as u32 as u64) + c as u32 as u64
}

const fn mul_u64_add_wide(a: u64, b: u64, c: u64) -> (u64, u64) {
    let x = mul_add(a, b, c);
    let y = mul_add(a, b >> 32, c >> 32).wrapping_add(x >> 32);
    let z = mul_add(a >> 32, b >> 32, y >> 32);
    let y = mul_add(a >> 32, b, y);
    (
        (y << 32).wrapping_add(x as u32 as u64),
        z.wrapping_add(y >> 32),
    )
}

/// Full-width unsigned product as `(low, high)`, without a 128-bit runtime.
#[allow(dead_code)] // Shared math64 headers also use this core primitive.
pub(crate) const fn mul_u64_wide(a: u64, b: u64) -> (u64, u64) {
    mul_u64_add_wide(a, b, 0)
}

// A 64x32 multiply plus 64-bit add, retaining its carry above bit 63.
const fn mul_u64_u32_add_wide(a: u64, b: u32, c: u64) -> (u64, u32) {
    let low = mul_add(a, b as u64, c);
    let medium = mul_add(a >> 32, b as u64, c >> 32).wrapping_add(low >> 32);
    ((medium << 32) | low as u32 as u64, (medium >> 32) as u32)
}

// Two instantiations preserve the original native-word quotient divisions.
// On ILP32 the guess is u32/u32, never a u64 divide intrinsic.
macro_rules! wide_division {
    ($name:ident, $digit:ty, $bits:expr, $multiply:expr) => {
        #[allow(dead_code)] // Both digit variants are retained for validation.
        pub(crate) const fn $name(a: u64, b: u64, c: u64, mut d: u64) -> Option<u64> {
            if d == 0 {
                return None;
            }
            let (mut low, mut high) = mul_u64_add_wide(a, b, c);
            if high == 0 {
                return div64_u64(low, d);
            }
            if high >= d {
                return Some(u64::MAX);
            }
            let shift = d.leading_zeros();
            if shift != 0 {
                d <<= shift;
                high = (high << shift) | (low >> (64 - shift));
                low <<= shift;
            }
            let mut repetitions = 64 / $bits;
            if (high >> 32) as u32 == 0 {
                repetitions -= 32 / $bits;
                high = (high << 32) | (low >> 32);
                low <<= 32;
            }
            if $bits == 16 && (high >> 48) as u32 == 0 {
                repetitions -= 1;
                high = (high << 16).wrapping_add(low >> 48);
                low <<= 16;
            }
            low = !low;
            high = !high;
            let divisor_high = ((d >> (64 - $bits)) + 1) as $digit;
            let mut quotient = 0u64;
            while repetitions != 0 {
                repetitions -= 1;
                let mut digit = ((!high >> (64 - 2 * $bits)) as $digit) / divisor_high;
                let mut overflow = (high >> (64 - $bits)) as u32;
                high = (high << $bits).wrapping_add(low >> (64 - $bits));
                low <<= $bits;
                let (next, carry) = $multiply(d, digit, high);
                high = next;
                overflow = overflow.wrapping_add(carry as u32);
                while overflow < (u32::MAX >> (32 - $bits)) {
                    digit = digit.wrapping_add(1);
                    high = high.wrapping_add(d);
                    overflow = overflow.wrapping_add((high < d) as u32);
                }
                quotient = (quotient << $bits).wrapping_add(digit as u64);
            }
            if high.wrapping_add(d) > high {
                quotient = quotient.wrapping_add(1);
            }
            Some(quotient)
        }
    };
}

wide_division!(mul_u64_add_u64_div_u64_32, u32, 16, mul_u64_u32_add_wide);
#[cfg(not(target_pointer_width = "32"))]
wide_division!(mul_u64_add_u64_div_u64_64, u64, 32, mul_u64_add_wide);

/// Multiply two 64-bit values, add a third, and divide their full-width sum.
///
/// Zero returns `None`. A quotient exceeding `u64::MAX` saturates, matching
/// generic C rather than the overflow trap of some architecture overrides.
pub const fn mul_u64_add_u64_div_u64(a: u64, b: u64, c: u64, d: u64) -> Option<u64> {
    #[cfg(target_pointer_width = "32")]
    {
        mul_u64_add_u64_div_u64_32(a, b, c, d)
    }
    #[cfg(not(target_pointer_width = "32"))]
    {
        mul_u64_add_u64_div_u64_64(a, b, c, d)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
