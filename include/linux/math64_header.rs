/* SPDX-License-Identifier: GPL-2.0 */

//! Safe, allocation-free counterparts of the complete generic math64 header.
//!
//! Divisions return `None` on zero, and signed divisions also reject the
//! minimum signed value divided by minus one. Remainder helpers return tuples
//! instead of accepting raw pointers. Unsigned rounding and multiply helpers
//! preserve the C operations' wrapping, not arbitrary-precision rounding.
//! Wide multiply/divide uses the generic saturating policy; architecture C
//! overrides that trap on overflow are not emulated by this portable API.

#[allow(dead_code, unused_imports, unreachable_pub)]
#[path = "../vdso/math64_header.rs"]
mod vdso;

pub use vdso::division::implementation::{
    div64_s64, div64_s64_rem, div64_u64, div64_u64_rem, div_s64, div_s64_rem, div_u64, div_u64_rem,
    iter_div_u64_rem, mul_u64_add_u64_div_u64,
};
pub use vdso::division::{__arch_xprod_64, __div64_32, __div64_const32, do_div};
pub use vdso::{__iter_div_u64_rem, mul_u32_u32, mul_u64_u32_add_u64_shr};

/// Divide a signed 64-bit value by a native signed word.
pub const fn div64_long(dividend: i64, divisor: isize) -> Option<i64> {
    div64_s64(dividend, divisor as i64)
}

/// Divide an unsigned 64-bit value by a native unsigned word.
pub const fn div64_ul(dividend: u64, divisor: usize) -> Option<u64> {
    div64_u64(dividend, divisor as u64)
}

/// Add a 32-bit word, retaining the low 64 bits.
pub const fn add_u64_u32(a: u64, b: u32) -> u64 {
    a.wrapping_add(b as u64)
}

/// Multiply by a 32-bit word and shift with the original C shift domain.
///
/// The 128-bit configuration accepts shifts below 128. Otherwise the shift
/// must be below 64, and at most 32 when `a` has a nonzero high word.
pub const fn mul_u64_u32_shr(a: u64, mul: u32, shift: u32) -> Option<u64> {
    mul_u64_u32_add_u64_shr(a, mul, 0, shift)
}

/// Shift a full unsigned product, retaining its low 64 resulting bits.
///
/// The 128-bit configuration rejects shifts of 128 or more. The generic C
/// fallback explicitly masks the high-word shift: for any shift of 64 or more
/// it returns `product_high >> (shift & 63)`, including shifts above 127.
pub const fn mul_u64_u64_shr(a: u64, b: u64, shift: u32) -> Option<u64> {
    if cfg!(all(
        CONFIG_ARCH_SUPPORTS_INT128,
        target_pointer_width = "64"
    )) && shift >= 128
    {
        return None;
    }
    let (low, high) = vdso::division::implementation::mul_u64_wide(a, b);
    Some(vdso::shift_product(low, high, shift))
}

/// Shift the magnitude of a signed/unsigned product and restore its sign.
///
/// Signed negation preserves the kernel's wrapping semantics, including the
/// minimum signed input. Shift validity is the same as [`mul_u64_u64_shr`].
pub const fn mul_s64_u64_shr(a: i64, b: u64, shift: u32) -> Option<u64> {
    match mul_u64_u64_shr(a.unsigned_abs(), b, shift) {
        Some(value) => Some(if a < 0 { value.wrapping_neg() } else { value }),
        None => None,
    }
}

/// Divide a 96-bit product, retaining the low 64 bits of its quotient.
///
/// This preserves the generic header's truncation on quotient overflow; it
/// deliberately differs from the saturating full-width multiply/divide API.
pub const fn mul_u64_u32_div(a: u64, mul: u32, divisor: u32) -> Option<u64> {
    let low = mul_u32_u32(a as u32, mul);
    let high = mul_u32_u32((a >> 32) as u32, mul).wrapping_add(low >> 32);
    let (high_quotient, remainder) = match div_u64_rem(high, divisor) {
        Some(value) => value,
        None => return None,
    };
    match div_u64((low as u32 as u64) | ((remainder as u64) << 32), divisor) {
        Some(value) => Some((value as u32 as u64) | ((high_quotient as u32 as u64) << 32)),
        None => None,
    }
}

/// Divide a full product, saturating its quotient to `u64::MAX` on overflow.
pub const fn mul_u64_u64_div_u64(a: u64, b: u64, divisor: u64) -> Option<u64> {
    mul_u64_add_u64_div_u64(a, b, 0, divisor)
}

/// Round a full-product quotient up, preserving generic saturation.
pub const fn mul_u64_u64_div_u64_roundup(a: u64, b: u64, divisor: u64) -> Option<u64> {
    mul_u64_add_u64_div_u64(a, b, divisor.wrapping_sub(1), divisor)
}

/// Round division up after the C macro's wrapping `dividend + divisor - 1`.
pub const fn div64_u64_round_up(dividend: u64, divisor: u64) -> Option<u64> {
    div64_u64(dividend.wrapping_add(divisor).wrapping_sub(1), divisor)
}

/// Round division by a 32-bit value up, preserving wrapping before division.
pub const fn div_u64_round_up(dividend: u64, divisor: u32) -> Option<u64> {
    div_u64(
        dividend.wrapping_add(divisor as u64).wrapping_sub(1),
        divisor,
    )
}

/// Round to the nearest integer, preserving wrapping before division.
pub const fn div64_u64_round_closest(dividend: u64, divisor: u64) -> Option<u64> {
    div64_u64(dividend.wrapping_add(divisor >> 1), divisor)
}

/// Round division by a 32-bit value to nearest with the C macro's wrapping.
pub const fn div_u64_round_closest(dividend: u64, divisor: u32) -> Option<u64> {
    div_u64(dividend.wrapping_add((divisor >> 1) as u64), divisor)
}

/// Round signed division to nearest, with ties away from zero.
///
/// The adjustment wraps as in the kernel's signed C arithmetic. A zero
/// divisor or minimum-value/minus-one overflow after adjustment returns `None`.
pub const fn div_s64_round_closest(dividend: i64, divisor: i32) -> Option<i64> {
    let half = (divisor / 2) as i64;
    let adjusted = if (dividend > 0) == (divisor > 0) {
        dividend.wrapping_add(half)
    } else {
        dividend.wrapping_sub(half)
    };
    div_s64(adjusted, divisor)
}

/// Round to a 32-bit multiple, preserving wrapping in both arithmetic steps.
pub const fn roundup_u64(value: u64, multiple: u32) -> Option<u64> {
    match div_u64_round_up(value, multiple) {
        Some(quotient) => Some(quotient.wrapping_mul(multiple as u64)),
        None => None,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
