/* SPDX-License-Identifier: MIT */

/* Copyright 2024 Advanced Micro Devices, Inc. */

// Translated from spl_fixpt31_32.h.
// The C SPL_NAMESPACE macro is represented by the corresponding Rust symbol
// names; external definitions are supplied by other translation units.

pub const FIXED31_32_BITS_PER_FRACTIONAL_PART: u32 = 32;
pub const LLONG_MAX: i64 = 9_223_372_036_854_775_807;
pub const LLONG_MIN: i64 = -LLONG_MAX - 1;

/*
 * @brief
 * Arithmetic operations on real numbers
 * represented as fixed-point numbers.
 * There are: 1 bit for sign,
 * 31 bit for integer part,
 * 32 bits for fractional part.
 *
 * @note
 * Currently, overflows and underflows are asserted;
 * no special result returned.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spl_fixed31_32 {
    pub value: i64,
}

/* @brief Useful constants */
pub const spl_fixpt_zero: spl_fixed31_32 = spl_fixed31_32 { value: 0 };
pub const spl_fixpt_epsilon: spl_fixed31_32 = spl_fixed31_32 { value: 1 };
pub const spl_fixpt_half: spl_fixed31_32 = spl_fixed31_32 { value: 0x8000_0000 };
pub const spl_fixpt_one: spl_fixed31_32 = spl_fixed31_32 { value: 0x1_0000_0000 };

extern "C" {
    pub fn spl_fixpt_from_fraction(numerator: i64, denominator: i64) -> spl_fixed31_32;
    pub fn spl_fixpt_mul(arg1: spl_fixed31_32, arg2: spl_fixed31_32) -> spl_fixed31_32;
    pub fn spl_fixpt_sqr(arg: spl_fixed31_32) -> spl_fixed31_32;
    pub fn spl_fixpt_recip(arg: spl_fixed31_32) -> spl_fixed31_32;
    pub fn spl_fixpt_sinc(arg: spl_fixed31_32) -> spl_fixed31_32;
    pub fn spl_fixpt_sin(arg: spl_fixed31_32) -> spl_fixed31_32;
    pub fn spl_fixpt_cos(arg: spl_fixed31_32) -> spl_fixed31_32;
    pub fn spl_fixpt_exp(arg: spl_fixed31_32) -> spl_fixed31_32;
    pub fn spl_fixpt_log(arg: spl_fixed31_32) -> spl_fixed31_32;
    pub fn spl_fixpt_u4d19(arg: spl_fixed31_32) -> u32;
    pub fn spl_fixpt_u3d19(arg: spl_fixed31_32) -> u32;
    pub fn spl_fixpt_u2d19(arg: spl_fixed31_32) -> u32;
    pub fn spl_fixpt_u0d19(arg: spl_fixed31_32) -> u32;
    pub fn spl_fixpt_clamp_u0d14(arg: spl_fixed31_32) -> u32;
    pub fn spl_fixpt_clamp_u0d10(arg: spl_fixed31_32) -> u32;
    pub fn spl_fixpt_s4d19(arg: spl_fixed31_32) -> i32;
    pub fn spl_fixpt_from_ux_dy(value: u32, integer_bits: u32, fractional_bits: u32) -> spl_fixed31_32;
}

#[inline]
pub fn spl_fixpt_from_int(arg: i32) -> spl_fixed31_32 {
    spl_fixed31_32 { value: (arg as i64).wrapping_shl(FIXED31_32_BITS_PER_FRACTIONAL_PART) }
}

#[inline]
pub fn spl_fixpt_neg(arg: spl_fixed31_32) -> spl_fixed31_32 {
    spl_fixed31_32 { value: arg.value.wrapping_neg() }
}

#[inline]
pub fn spl_fixpt_abs(arg: spl_fixed31_32) -> spl_fixed31_32 {
    if arg.value < 0 { spl_fixpt_neg(arg) } else { arg }
}

#[inline] pub fn spl_fixpt_lt(a: spl_fixed31_32, b: spl_fixed31_32) -> bool { a.value < b.value }
#[inline] pub fn spl_fixpt_le(a: spl_fixed31_32, b: spl_fixed31_32) -> bool { a.value <= b.value }
#[inline] pub fn spl_fixpt_eq(a: spl_fixed31_32, b: spl_fixed31_32) -> bool { a.value == b.value }
#[inline] pub fn spl_fixpt_min(a: spl_fixed31_32, b: spl_fixed31_32) -> spl_fixed31_32 { if a.value <= b.value { a } else { b } }
#[inline] pub fn spl_fixpt_max(a: spl_fixed31_32, b: spl_fixed31_32) -> spl_fixed31_32 { if a.value <= b.value { b } else { a } }

#[inline]
pub fn spl_fixpt_clamp(arg: spl_fixed31_32, min_value: spl_fixed31_32, max_value: spl_fixed31_32) -> spl_fixed31_32 {
    if spl_fixpt_le(arg, min_value) { min_value } else if spl_fixpt_le(max_value, arg) { max_value } else { arg }
}

#[inline]
pub fn spl_fixpt_shl(mut arg: spl_fixed31_32, shift: u32) -> spl_fixed31_32 {
    debug_assert!((arg.value >= 0 && arg.value <= (LLONG_MAX >> shift)) || (arg.value < 0 && arg.value >= !((LLONG_MAX >> shift))));
    arg.value = arg.value.wrapping_shl(shift);
    arg
}

#[inline]
pub fn spl_fixpt_shr(mut arg: spl_fixed31_32, shift: u32) -> spl_fixed31_32 {
    let negative = arg.value < 0;
    if negative { arg.value = arg.value.wrapping_neg(); }
    arg.value >>= shift;
    if negative { arg.value = arg.value.wrapping_neg(); }
    arg
}

#[inline]
pub fn spl_fixpt_add(a: spl_fixed31_32, b: spl_fixed31_32) -> spl_fixed31_32 {
    debug_assert!((a.value >= 0 && LLONG_MAX - a.value >= b.value) || (a.value < 0 && LLONG_MIN - a.value <= b.value));
    spl_fixed31_32 { value: a.value.wrapping_add(b.value) }
}
#[inline] pub fn spl_fixpt_add_int(a: spl_fixed31_32, b: i32) -> spl_fixed31_32 { spl_fixpt_add(a, spl_fixpt_from_int(b)) }
#[inline]
pub fn spl_fixpt_sub(a: spl_fixed31_32, b: spl_fixed31_32) -> spl_fixed31_32 {
    debug_assert!((b.value >= 0 && LLONG_MIN + b.value <= a.value) || (b.value < 0 && LLONG_MAX + b.value >= a.value));
    spl_fixed31_32 { value: a.value.wrapping_sub(b.value) }
}
#[inline] pub fn spl_fixpt_sub_int(a: spl_fixed31_32, b: i32) -> spl_fixed31_32 { spl_fixpt_sub(a, spl_fixpt_from_int(b)) }
#[inline] pub unsafe fn spl_fixpt_mul_int(a: spl_fixed31_32, b: i32) -> spl_fixed31_32 { spl_fixpt_mul(a, spl_fixpt_from_int(b)) }
#[inline] pub unsafe fn spl_fixpt_div_int(a: spl_fixed31_32, b: i64) -> spl_fixed31_32 { spl_fixpt_from_fraction(a.value, spl_fixpt_from_int(b as i32).value) }
#[inline] pub unsafe fn spl_fixpt_div(a: spl_fixed31_32, b: spl_fixed31_32) -> spl_fixed31_32 { spl_fixpt_from_fraction(a.value, b.value) }

#[inline]
pub unsafe fn spl_fixpt_pow(a: spl_fixed31_32, b: spl_fixed31_32) -> spl_fixed31_32 {
    if a.value == 0 { if b.value == 0 { spl_fixpt_one } else { spl_fixpt_zero } }
    else { spl_fixpt_exp(spl_fixpt_mul(spl_fixpt_log(a), b)) }
}

#[inline]
pub fn spl_fixpt_floor(arg: spl_fixed31_32) -> i32 {
    let v = if arg.value > 0 { arg.value as u64 } else { arg.value.wrapping_neg() as u64 };
    if arg.value >= 0 { (v >> 32) as i32 } else { -((v >> 32) as i32) }
}
#[inline]
pub fn spl_fixpt_round(arg: spl_fixed31_32) -> i32 {
    let mut v = if arg.value > 0 { arg.value as u64 } else { arg.value.wrapping_neg() as u64 };
    debug_assert!(LLONG_MAX - v as i64 >= spl_fixpt_half.value);
    v += spl_fixpt_half.value as u64;
    if arg.value >= 0 { (v >> 32) as i32 } else { -((v >> 32) as i32) }
}
#[inline]
pub fn spl_fixpt_ceil(arg: spl_fixed31_32) -> i32 {
    let mut v = if arg.value > 0 { arg.value as u64 } else { arg.value.wrapping_neg() as u64 };
    let summand = spl_fixpt_one.value - spl_fixpt_epsilon.value;
    debug_assert!(LLONG_MAX - v as i64 >= summand);
    v += summand as u64;
    if arg.value >= 0 { (v >> 32) as i32 } else { -((v >> 32) as i32) }
}

#[inline]
pub fn spl_fixpt_truncate(mut arg: spl_fixed31_32, frac_bits: u32) -> spl_fixed31_32 {
    let negative = arg.value < 0;
    if frac_bits >= FIXED31_32_BITS_PER_FRACTIONAL_PART { debug_assert!(frac_bits == FIXED31_32_BITS_PER_FRACTIONAL_PART); return arg; }
    if negative { arg.value = arg.value.wrapping_neg(); }
    arg.value &= (!0u64 << (FIXED31_32_BITS_PER_FRACTIONAL_PART - frac_bits)) as i64;
    if negative { arg.value = arg.value.wrapping_neg(); }
    arg
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
