/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

pub const FIXED31_32_BITS_PER_FRACTIONAL_PART: u32 = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fixed31_32 {
    pub value: i64,
}

pub const dc_fixpt_zero: fixed31_32 = fixed31_32 { value: 0 };
pub const dc_fixpt_epsilon: fixed31_32 = fixed31_32 { value: 1 };
pub const dc_fixpt_half: fixed31_32 = fixed31_32 { value: 0x80000000 };
pub const dc_fixpt_one: fixed31_32 = fixed31_32 { value: 0x100000000 };

extern "C" {
    pub fn dc_fixpt_from_fraction(numerator: i64, denominator: i64) -> fixed31_32;
}

#[inline]
pub fn dc_fixpt_from_int(arg: i32) -> fixed31_32 {
    fixed31_32 { value: (arg as i64).wrapping_shl(FIXED31_32_BITS_PER_FRACTIONAL_PART) }
}

#[inline]
pub fn dc_fixpt_neg(arg: fixed31_32) -> fixed31_32 {
    fixed31_32 { value: arg.value.wrapping_neg() }
}

#[inline]
pub fn dc_fixpt_abs(arg: fixed31_32) -> fixed31_32 {
    if arg.value < 0 { dc_fixpt_neg(arg) } else { arg }
}

#[inline]
pub fn dc_fixpt_lt(arg1: fixed31_32, arg2: fixed31_32) -> bool { arg1.value < arg2.value }
#[inline]
pub fn dc_fixpt_le(arg1: fixed31_32, arg2: fixed31_32) -> bool { arg1.value <= arg2.value }
#[inline]
pub fn dc_fixpt_eq(arg1: fixed31_32, arg2: fixed31_32) -> bool { arg1.value == arg2.value }

#[inline]
pub fn dc_fixpt_min(arg1: fixed31_32, arg2: fixed31_32) -> fixed31_32 {
    if arg1.value <= arg2.value { arg1 } else { arg2 }
}

#[inline]
pub fn dc_fixpt_max(arg1: fixed31_32, arg2: fixed31_32) -> fixed31_32 {
    if arg1.value <= arg2.value { arg2 } else { arg1 }
}

#[inline]
pub fn dc_fixpt_clamp(arg: fixed31_32, min_value: fixed31_32, max_value: fixed31_32) -> fixed31_32 {
    if dc_fixpt_le(arg, min_value) { min_value }
    else if dc_fixpt_le(max_value, arg) { max_value }
    else { arg }
}

#[inline]
pub fn dc_fixpt_shl(mut arg: fixed31_32, shift: u8) -> fixed31_32 {
    // C ASSERT overflow check retained by the source contract.
    arg.value = arg.value.wrapping_shl(shift as u32);
    arg
}

#[inline]
pub fn dc_fixpt_shr(mut arg: fixed31_32, shift: u8) -> fixed31_32 {
    let negative = arg.value < 0;
    if negative { arg.value = arg.value.wrapping_neg(); }
    arg.value >>= shift;
    if negative { arg.value = arg.value.wrapping_neg(); }
    arg
}

#[inline]
pub fn dc_fixpt_add(arg1: fixed31_32, arg2: fixed31_32) -> fixed31_32 {
    fixed31_32 { value: arg1.value.wrapping_add(arg2.value) }
}
#[inline]
pub fn dc_fixpt_add_int(arg1: fixed31_32, arg2: i32) -> fixed31_32 {
    dc_fixpt_add(arg1, dc_fixpt_from_int(arg2))
}
#[inline]
pub fn dc_fixpt_sub(arg1: fixed31_32, arg2: fixed31_32) -> fixed31_32 {
    fixed31_32 { value: arg1.value.wrapping_sub(arg2.value) }
}
#[inline]
pub fn dc_fixpt_sub_int(arg1: fixed31_32, arg2: i32) -> fixed31_32 {
    dc_fixpt_sub(arg1, dc_fixpt_from_int(arg2))
}

extern "C" {
    pub fn dc_fixpt_mul(arg1: fixed31_32, arg2: fixed31_32) -> fixed31_32;
    pub fn dc_fixpt_sqr(arg: fixed31_32) -> fixed31_32;
    pub fn dc_fixpt_recip(arg: fixed31_32) -> fixed31_32;
    pub fn dc_fixpt_sinc(arg: fixed31_32) -> fixed31_32;
    pub fn dc_fixpt_sin(arg: fixed31_32) -> fixed31_32;
    pub fn dc_fixpt_cos(arg: fixed31_32) -> fixed31_32;
    pub fn dc_fixpt_exp(arg: fixed31_32) -> fixed31_32;
    pub fn dc_fixpt_log(arg: fixed31_32) -> fixed31_32;
    pub fn dc_fixpt_u4d19(arg: fixed31_32) -> u32;
    pub fn dc_fixpt_u3d19(arg: fixed31_32) -> u32;
    pub fn dc_fixpt_u2d19(arg: fixed31_32) -> u32;
    pub fn dc_fixpt_u0d19(arg: fixed31_32) -> u32;
    pub fn dc_fixpt_clamp_u0d14(arg: fixed31_32) -> u32;
    pub fn dc_fixpt_clamp_u0d10(arg: fixed31_32) -> u32;
    pub fn dc_fixpt_s4d19(arg: fixed31_32) -> i32;
    pub fn dc_fixpt_from_ux_dy(value: u32, integer_bits: u32, fractional_bits: u32) -> fixed31_32;
    pub fn dc_fixpt_from_int_dy(int_value: u32, frac_value: u32, integer_bits: u32, fractional_bits: u32) -> fixed31_32;
}

#[inline]
pub fn dc_fixpt_mul_int(arg1: fixed31_32, arg2: i32) -> fixed31_32 {
    unsafe { dc_fixpt_mul(arg1, dc_fixpt_from_int(arg2)) }
}

#[inline]
pub fn dc_fixpt_div_int(arg1: fixed31_32, arg2: i64) -> fixed31_32 {
    unsafe { dc_fixpt_from_fraction(arg1.value, dc_fixpt_from_int(arg2 as i32).value) }
}

#[inline]
pub fn dc_fixpt_div(arg1: fixed31_32, arg2: fixed31_32) -> fixed31_32 {
    unsafe { dc_fixpt_from_fraction(arg1.value, arg2.value) }
}

#[inline]
pub fn dc_fixpt_pow(arg1: fixed31_32, arg2: fixed31_32) -> fixed31_32 {
    if arg1.value == 0 {
        return if arg2.value == 0 { dc_fixpt_one } else { dc_fixpt_zero };
    }
    unsafe { dc_fixpt_exp(dc_fixpt_mul(dc_fixpt_log(arg1), arg2)) }
}

#[inline]
pub fn dc_fixpt_floor(arg: fixed31_32) -> i32 {
    let arg_value = if arg.value > 0 { arg.value as u64 } else { arg.value.wrapping_neg() as u64 };
    let result = (arg_value >> FIXED31_32_BITS_PER_FRACTIONAL_PART) as i32;
    if arg.value >= 0 { result } else { -result }
}

#[inline]
pub fn dc_fixpt_round(arg: fixed31_32) -> i32 {
    let mut arg_value = if arg.value > 0 { arg.value as u64 } else { arg.value.wrapping_neg() as u64 };
    arg_value = arg_value.wrapping_add(dc_fixpt_half.value as u64);
    let result = (arg_value >> FIXED31_32_BITS_PER_FRACTIONAL_PART) as i32;
    if arg.value >= 0 { result } else { -result }
}

#[inline]
pub fn dc_fixpt_ceil(arg: fixed31_32) -> i32 {
    let mut arg_value = if arg.value > 0 { arg.value as u64 } else { arg.value.wrapping_neg() as u64 };
    arg_value = arg_value.wrapping_add((dc_fixpt_one.value - dc_fixpt_epsilon.value) as u64);
    let result = (arg_value >> FIXED31_32_BITS_PER_FRACTIONAL_PART) as i32;
    if arg.value >= 0 { result } else { -result }
}

#[inline]
pub fn dc_fixpt_truncate(mut arg: fixed31_32, frac_bits: u32) -> fixed31_32 {
    let negative = arg.value < 0;
    if frac_bits >= FIXED31_32_BITS_PER_FRACTIONAL_PART { return arg; }
    if negative { arg.value = arg.value.wrapping_neg(); }
    arg.value &= (!0u64 << (FIXED31_32_BITS_PER_FRACTIONAL_PART - frac_bits)) as i64;
    if negative { arg.value = arg.value.wrapping_neg(); }
    arg
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
