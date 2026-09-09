// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spl_fixed31_32 {
    pub value: i64,
}

const FIXED31_32_BITS_PER_FRACTIONAL_PART: u32 = 32;

static spl_fixpt_two_pi: spl_fixed31_32 = spl_fixed31_32 { value: 26986075409 };
static spl_fixpt_ln2: spl_fixed31_32 = spl_fixed31_32 { value: 2977044471 };
static spl_fixpt_ln2_div_2: spl_fixed31_32 = spl_fixed31_32 { value: 1488522236 };

extern "C" {
    static spl_fixpt_one: spl_fixed31_32;
    static spl_fixpt_half: spl_fixed31_32;
    static spl_fixpt_zero: spl_fixed31_32;
    fn spl_div64_u64_rem(dividend: u64, divisor: u64, remainder: *mut u64) -> u64;
    fn spl_div64_s64(dividend: i64, divisor: i64) -> i64;
    fn spl_fixpt_le(arg1: spl_fixed31_32, arg2: spl_fixed31_32) -> bool;
    fn spl_fixpt_abs(arg: spl_fixed31_32) -> spl_fixed31_32;
    fn spl_fixpt_sub(arg1: spl_fixed31_32, arg2: spl_fixed31_32) -> spl_fixed31_32;
    fn spl_fixpt_mul_int(arg: spl_fixed31_32, mul: i32) -> spl_fixed31_32;
    fn spl_fixpt_div_int(arg: spl_fixed31_32, div: i64) -> spl_fixed31_32;
    fn spl_fixpt_div(arg1: spl_fixed31_32, arg2: spl_fixed31_32) -> spl_fixed31_32;
    fn spl_fixpt_shl(arg: spl_fixed31_32, shift: u32) -> spl_fixed31_32;
    fn spl_fixpt_lt(arg1: spl_fixed31_32, arg2: spl_fixed31_32) -> bool;
    fn spl_fixpt_add(arg1: spl_fixed31_32, arg2: spl_fixed31_32) -> spl_fixed31_32;
    fn spl_fixpt_neg(arg: spl_fixed31_32) -> spl_fixed31_32;
}

#[inline]
unsafe fn abs_i64(arg: i64) -> u64 {
    if arg > 0 { arg as u64 } else { (-arg) as u64 }
}

#[inline]
unsafe fn spl_complete_integer_division_u64(dividend: u64, divisor: u64, remainder: *mut u64) -> u64 {
    spl_div64_u64_rem(dividend, divisor, remainder)
}

const FRACTIONAL_PART_MASK: u64 = (1u64 << FIXED31_32_BITS_PER_FRACTIONAL_PART) - 1;
#[inline] fn get_integer_part(x: u64) -> u64 { x >> FIXED31_32_BITS_PER_FRACTIONAL_PART }
#[inline] fn get_fractional_part(x: u64) -> u64 { FRACTIONAL_PART_MASK & x }

pub unsafe fn spl_fixpt_from_fraction(numerator: i64, denominator: i64) -> spl_fixed31_32 {
    let arg1_negative = numerator < 0;
    let arg2_negative = denominator < 0;
    let arg1_value = if arg1_negative { (-numerator) as u64 } else { numerator as u64 };
    let arg2_value = if arg2_negative { (-denominator) as u64 } else { denominator as u64 };
    let mut remainder = 0u64;
    let mut res_value = spl_complete_integer_division_u64(arg1_value, arg2_value, &mut remainder);
    debug_assert!(res_value <= i64::MAX as u64);
    let mut i = FIXED31_32_BITS_PER_FRACTIONAL_PART;
    loop {
        remainder <<= 1; res_value <<= 1;
        if remainder >= arg2_value { res_value |= 1; remainder -= arg2_value; }
        i -= 1; if i == 0 { break; }
    }
    let summand = ((remainder << 1) >= arg2_value) as u64;
    debug_assert!(res_value <= i64::MAX as u64 - summand);
    res_value += summand;
    let mut res = spl_fixed31_32 { value: res_value as i64 };
    if arg1_negative ^ arg2_negative { res.value = -res.value; }
    res
}

pub unsafe fn spl_fixpt_mul(arg1: spl_fixed31_32, arg2: spl_fixed31_32) -> spl_fixed31_32 {
    let n = arg1.value < 0; let m = arg2.value < 0;
    let a = if n { (-arg1.value) as u64 } else { arg1.value as u64 };
    let b = if m { (-arg2.value) as u64 } else { arg2.value as u64 };
    let ai = get_integer_part(a); let bi = get_integer_part(b);
    let af = get_fractional_part(a); let bf = get_fractional_part(b);
    let mut v = (ai * bi) as i64; v <<= FIXED31_32_BITS_PER_FRACTIONAL_PART;
    v += (ai * bf) as i64; v += (bi * af) as i64;
    let t = ((af * bf) >> FIXED31_32_BITS_PER_FRACTIONAL_PART) + ((af * bf >= (*spl_fixpt_half).value as u64) as u64);
    v += t as i64; if n ^ m { v = -v; } spl_fixed31_32 { value: v }
}

pub unsafe fn spl_fixpt_sqr(arg: spl_fixed31_32) -> spl_fixed31_32 {
    let arg_value = abs_i64(arg.value);
    let arg_int = get_integer_part(arg_value);
    let arg_fra = get_fractional_part(arg_value);
    let mut value = (arg_int * arg_int) as i64;
    value <<= FIXED31_32_BITS_PER_FRACTIONAL_PART;
    let tmp = (arg_int * arg_fra) as i64;
    value += tmp;
    value += tmp;
    let product = arg_fra * arg_fra;
    value += ((product >> FIXED31_32_BITS_PER_FRACTIONAL_PART)
        + (product >= (*spl_fixpt_half).value as u64) as u64) as i64;
    spl_fixed31_32 { value }
}

pub unsafe fn spl_fixpt_recip(arg: spl_fixed31_32) -> spl_fixed31_32 { spl_fixpt_from_fraction((*spl_fixpt_one).value, arg.value) }

pub unsafe fn spl_fixpt_sinc(arg: spl_fixed31_32) -> spl_fixed31_32 {
    let mut arg_norm = arg;
    if spl_fixpt_le(spl_fixpt_two_pi, spl_fixpt_abs(arg)) { arg_norm = spl_fixpt_sub(arg_norm, spl_fixpt_mul_int(spl_fixpt_two_pi, spl_div64_s64(arg_norm.value, spl_fixpt_two_pi.value) as i32)); }
    let square = spl_fixpt_sqr(arg_norm); let mut res = *spl_fixpt_one; let mut n = 27;
    loop { res = spl_fixpt_sub(*spl_fixpt_one, spl_fixpt_div_int(spl_fixpt_mul(square, res), (n * (n - 1)) as i64)); n -= 2; if n <= 2 { break; } }
    if arg.value != arg_norm.value { res = spl_fixpt_div(spl_fixpt_mul(res, arg_norm), arg); } res
}

pub unsafe fn spl_fixpt_sin(arg: spl_fixed31_32) -> spl_fixed31_32 { spl_fixpt_mul(arg, spl_fixpt_sinc(arg)) }

pub unsafe fn spl_fixpt_cos(arg: spl_fixed31_32) -> spl_fixed31_32 {
    // TODO implement argument normalization
    let square = spl_fixpt_sqr(arg); let mut res = *spl_fixpt_one; let mut n = 26;
    loop { res = spl_fixpt_sub(*spl_fixpt_one, spl_fixpt_div_int(spl_fixpt_mul(square, res), (n * (n - 1)) as i64)); n -= 2; if n == 0 { break; } } res
}

unsafe fn spl_fixed31_32_exp_from_taylor_series(arg: spl_fixed31_32) -> spl_fixed31_32 {
    let mut n = 9u32; let mut res = spl_fixpt_from_fraction((n + 2) as i64, (n + 1) as i64);
    // TODO find correct res
    debug_assert!(spl_fixpt_lt(arg, *spl_fixpt_one));
    loop { res = spl_fixpt_add(*spl_fixpt_one, spl_fixpt_div_int(spl_fixpt_mul(arg, res), n as i64)); n -= 1; if n == 1 { break; } }
    spl_fixpt_add(*spl_fixpt_one, spl_fixpt_mul(arg, res))
}

pub unsafe fn spl_fixpt_exp(arg: spl_fixed31_32) -> spl_fixed31_32 {
    // exp(x) = exp(r + m * ln(2)) = (1 << m) * exp(r), where m = round(x / ln(2)), r = x - m * ln(2)
    if spl_fixpt_le(spl_fixpt_ln2_div_2, spl_fixpt_abs(arg)) { let m = spl_fixpt_round(spl_fixpt_div(arg, spl_fixpt_ln2)); let r = spl_fixpt_sub(arg, spl_fixpt_mul_int(spl_fixpt_ln2, m)); debug_assert!(m != 0); debug_assert!(spl_fixpt_lt(spl_fixpt_abs(r), *spl_fixpt_one)); if m > 0 { spl_fixpt_shl(spl_fixed31_32_exp_from_taylor_series(r), m as u32) } else { spl_fixpt_div_int(spl_fixed31_32_exp_from_taylor_series(r), 1i64 << -m) } } else if arg.value != 0 { spl_fixed31_32_exp_from_taylor_series(arg) } else { *spl_fixpt_one }
}

pub unsafe fn spl_fixpt_log(arg: spl_fixed31_32) -> spl_fixed31_32 {
    let mut res = spl_fixpt_neg(*spl_fixpt_one); let mut error;
    // TODO improve 1st estimation
    debug_assert!(arg.value > 0);
    // TODO if arg is negative, return NaN
    // TODO if arg is zero, return -INF
    loop { let res1 = spl_fixpt_add(spl_fixpt_sub(res, *spl_fixpt_one), spl_fixpt_div(arg, spl_fixpt_exp(res))); error = spl_fixpt_sub(res, res1); res = res1; if abs_i64(error.value) <= 100 { break; } }
    res
}

#[inline] unsafe fn spl_ux_dy(value: i64, integer_bits: u32, fractional_bits: u32) -> u32 { let mut result = (1u32 << integer_bits) - 1; let mut fractional_part = (FRACTIONAL_PART_MASK & value as u64) as u32; result &= get_integer_part(value as u64) as u32; result <<= fractional_bits; fractional_part >>= FIXED31_32_BITS_PER_FRACTIONAL_PART - fractional_bits; result | fractional_part }
#[inline] unsafe fn spl_clamp_ux_dy(value: i64, integer_bits: u32, fractional_bits: u32, min_clamp: u32) -> u32 { let truncated_val = spl_ux_dy(value, integer_bits, fractional_bits); if value >= (1i64 << (integer_bits + FIXED31_32_BITS_PER_FRACTIONAL_PART)) { (1u32 << (integer_bits + fractional_bits)) - 1 } else if truncated_val > min_clamp { truncated_val } else { min_clamp } }

pub unsafe fn spl_fixpt_u4d19(arg: spl_fixed31_32) -> u32 { spl_ux_dy(arg.value, 4, 19) }
pub unsafe fn spl_fixpt_u3d19(arg: spl_fixed31_32) -> u32 { spl_ux_dy(arg.value, 3, 19) }
pub unsafe fn spl_fixpt_u2d19(arg: spl_fixed31_32) -> u32 { spl_ux_dy(arg.value, 2, 19) }
pub unsafe fn spl_fixpt_u0d19(arg: spl_fixed31_32) -> u32 { spl_ux_dy(arg.value, 0, 19) }
pub unsafe fn spl_fixpt_clamp_u0d14(arg: spl_fixed31_32) -> u32 { spl_clamp_ux_dy(arg.value, 0, 14, 1) }
pub unsafe fn spl_fixpt_clamp_u0d10(arg: spl_fixed31_32) -> u32 { spl_clamp_ux_dy(arg.value, 0, 10, 1) }
pub unsafe fn spl_fixpt_s4d19(arg: spl_fixed31_32) -> i32 { if arg.value < 0 { -(spl_ux_dy(spl_fixpt_abs(arg).value, 4, 19) as i32) } else { spl_ux_dy(arg.value, 4, 19) as i32 } }

pub unsafe fn spl_fixpt_from_ux_dy(value: u32, integer_bits: u32, fractional_bits: u32) -> spl_fixed31_32 { let mut fixpt_value = *spl_fixpt_zero; let mut fixpt_int_value = *spl_fixpt_zero; let mut frac_mask = (1i64 << integer_bits) - 1; fixpt_value.value = (value as i64) << (FIXED31_32_BITS_PER_FRACTIONAL_PART - fractional_bits); frac_mask <<= fractional_bits; fixpt_int_value.value = value as i64 & frac_mask; fixpt_int_value.value <<= FIXED31_32_BITS_PER_FRACTIONAL_PART - fractional_bits; fixpt_value.value |= fixpt_int_value.value; fixpt_value }

pub unsafe fn spl_fixpt_from_int_dy(int_value: u32, frac_value: u32, _integer_bits: u32, fractional_bits: u32) -> spl_fixed31_32 { let mut fixpt_value = spl_fixpt_from_int(int_value); fixpt_value.value |= (frac_value as i64) << (FIXED31_32_BITS_PER_FRACTIONAL_PART - fractional_bits); fixpt_value }

extern "C" { fn spl_fixpt_round(arg: spl_fixed31_32) -> i32; fn spl_fixpt_from_int(arg: u32) -> spl_fixed31_32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
