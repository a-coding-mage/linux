// SPDX-License-Identifier: GPL-2.0
//! Checked, allocation-free reciprocal multipliers for unsigned division.
//!
//! These helpers preserve the original fixed-width arithmetic on its defined
//! domain. Invalid divisors, precision values, and shift counts return `None`.
//! Constructing a multiplier is a slow-path operation; [`reciprocal_divide`]
//! applies a basic multiplier without a division instruction.

/// Basic reciprocal multiplier, with the layout of C's `reciprocal_value`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ReciprocalValue {
    /// Low 32 bits of the reciprocal multiplier.
    pub m: u32,
    /// Shift applied to the dividend-minus-product correction.
    pub sh1: u8,
    /// Final quotient shift.
    pub sh2: u8,
}

/// Advanced multiplier, with the layout of C's `reciprocal_value_adv`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ReciprocalValueAdv {
    /// Low 32 bits of the multiplier.
    pub m: u32,
    /// Post-multiplication shift.
    pub sh: u8,
    /// Ceiling of the divisor's base-two logarithm.
    pub exp: u8,
    /// Whether the calculated multiplier exceeds 32 bits.
    pub is_wide_m: bool,
}

// Avoid a compiler-runtime 64-bit divide dependency on 32-bit targets. Divide
// the high word natively, then perform restoring division for the low word.
// The remainder is at most 2*d-1, so its intermediate needs only 33 bits.
const fn divide_u64_u32(numerator: u64, divisor: u32) -> Option<u64> {
    #[cfg(not(target_pointer_width = "32"))]
    {
        numerator.checked_div(divisor as u64)
    }
    #[cfg(target_pointer_width = "32")]
    {
        let high = (numerator >> 32) as u32;
        let (quotient_high, mut remainder) =
            match (high.checked_div(divisor), high.checked_rem(divisor)) {
                (Some(quotient), Some(remainder)) => (quotient, remainder as u64),
                _ => return None,
            };
        let mut low = numerator as u32;
        let mut quotient_low = 0u32;
        let mut remaining = 32u32;
        while remaining != 0 {
            remainder = (remainder << 1) | (low >> 31) as u64;
            low <<= 1;
            quotient_low <<= 1;
            if remainder >= divisor as u64 {
                remainder = remainder.wrapping_sub(divisor as u64);
                quotient_low |= 1;
            }
            remaining = remaining.wrapping_sub(1);
        }
        Some(((quotient_high as u64) << 32) | quotient_low as u64)
    }
}

/// Construct a basic reciprocal for any nonzero 32-bit divisor.
///
/// Returns `None` for zero, for which the original C performs a division by zero.
pub const fn reciprocal_value(divisor: u32) -> Option<ReciprocalValue> {
    if divisor == 0 {
        return None;
    }
    let exponent = 32u32.wrapping_sub(divisor.wrapping_sub(1).leading_zeros());
    let numerator =
        (1u64 << 32).wrapping_mul(1u64.rotate_left(exponent).wrapping_sub(divisor as u64));
    let multiplier = match divide_u64_u32(numerator, divisor) {
        Some(value) => value.wrapping_add(1),
        None => return None,
    };
    Some(ReciprocalValue {
        m: multiplier as u32,
        sh1: if exponent < 1 { exponent as u8 } else { 1 },
        sh2: if exponent == 0 {
            0
        } else {
            exponent.wrapping_sub(1) as u8
        },
    })
}

/// Apply a basic reciprocal to a dividend, preserving unsigned wrapping.
///
/// A multiplier returned by [`reciprocal_value`] produces the integer quotient.
/// Manually constructed multipliers are also accepted when both shifts are
/// below 32; other shift counts, undefined in C, return `None`.
pub const fn reciprocal_divide(dividend: u32, reciprocal: ReciprocalValue) -> Option<u32> {
    if reciprocal.sh1 >= 32 || reciprocal.sh2 >= 32 {
        return None;
    }
    let product = ((dividend as u64).wrapping_mul(reciprocal.m as u64) >> 32) as u32;
    // With a zero high word and a shift below 32, rotated bits land above
    // bit 31 and are discarded by the cast. This is a logical right shift
    // without even an unoptimized checked-shift precondition panic helper.
    let correction =
        (dividend.wrapping_sub(product) as u64).rotate_right(reciprocal.sh1 as u32) as u32;
    Some((product.wrapping_add(correction) as u64).rotate_right(reciprocal.sh2 as u32) as u32)
}

/// Construct the advanced reciprocal used for JIT division code generation.
///
/// The defined C domain is `1 <= divisor <= 2^31` and
/// `precision <= 32 + ceil(log2(divisor))`. Other inputs return `None`.
/// Precision zero is permitted, including the original wrapping 64-bit sum;
/// not every permitted precision describes full-width quotient generation.
pub const fn reciprocal_value_adv(divisor: u32, precision: u8) -> Option<ReciprocalValueAdv> {
    if divisor == 0 || divisor > (1u32 << 31) {
        return None;
    }
    let exponent = 32u32.wrapping_sub(divisor.wrapping_sub(1).leading_zeros());
    let power = 32u32.wrapping_add(exponent);
    if precision as u32 > power {
        return None;
    }
    let numerator = 1u64.rotate_left(power);
    let mut low = match divide_u64_u32(numerator, divisor) {
        Some(value) => value,
        None => return None,
    };
    let mut high = match divide_u64_u32(
        numerator.wrapping_add(1u64.rotate_left(power.wrapping_sub(precision as u32))),
        divisor,
    ) {
        Some(value) => value,
        None => return None,
    };
    let mut post_shift = exponent;
    while post_shift != 0 {
        let next_low = low >> 1;
        let next_high = high >> 1;
        if next_low >= next_high {
            break;
        }
        low = next_low;
        high = next_high;
        post_shift = post_shift.wrapping_sub(1);
    }
    Some(ReciprocalValueAdv {
        m: high as u32,
        sh: post_shift as u8,
        exp: exponent as u8,
        is_wide_m: high > u32::MAX as u64,
    })
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
