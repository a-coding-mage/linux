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

static const dc_fixpt_two_pi: fixed31_32 = fixed31_32 { value: 26986075409i64 };
static const dc_fixpt_ln2: fixed31_32 = fixed31_32 { value: 2977044471i64 };
static const dc_fixpt_ln2_div_2: fixed31_32 = fixed31_32 { value: 1488522236i64 };

static inline fn abs_i64(arg: i64) -> u64 {
    if arg > 0 { arg as u64 } else { (-arg) as u64 }
}

static inline fn complete_integer_division_u64(dividend: u64, divisor: u64, remainder: *mut u64) -> u64 {
    unsafe { div64_u64_rem(dividend, divisor, remainder) }
}

const FRACTIONAL_PART_MASK: u64 = (1u64 << FIXED31_32_BITS_PER_FRACTIONAL_PART) - 1;

#[inline]
fn GET_INTEGER_PART(x: u64) -> u64 { x >> FIXED31_32_BITS_PER_FRACTIONAL_PART }

#[inline]
fn GET_FRACTIONAL_PART(x: u64) -> u64 { FRACTIONAL_PART_MASK & x }

fn dc_fixpt_from_fraction(numerator: i64, denominator: i64) -> fixed31_32 {
    let arg1_negative = numerator < 0;
    let arg2_negative = denominator < 0;
    let arg1_value = if arg1_negative { (-numerator) as u64 } else { numerator as u64 };
    let arg2_value = if arg2_negative { (-denominator) as u64 } else { denominator as u64 };
    let mut remainder = 0u64;
    let mut res_value = complete_integer_division_u64(arg1_value, arg2_value, &mut remainder);
    ASSERT(res_value <= LONG_MAX);
    let mut i = FIXED31_32_BITS_PER_FRACTIONAL_PART;
    loop {
        remainder <<= 1;
        res_value <<= 1;
        if remainder >= arg2_value { res_value |= 1; remainder -= arg2_value; }
        i -= 1;
        if i == 0 { break; }
    }
    let summand = ((remainder << 1) >= arg2_value) as u64;
    ASSERT(res_value <= LLONG_MAX - summand);
    res_value += summand;
    let mut res = fixed31_32 { value: res_value as i64 };
    if arg1_negative ^ arg2_negative { res.value = -res.value; }
    res
}

fn dc_fixpt_mul(arg1: fixed31_32, arg2: fixed31_32) -> fixed31_32 {
    let n1 = arg1.value < 0; let n2 = arg2.value < 0;
    let v1 = if n1 { (-arg1.value) as u64 } else { arg1.value as u64 };
    let v2 = if n2 { (-arg2.value) as u64 } else { arg2.value as u64 };
    let i1 = GET_INTEGER_PART(v1); let i2 = GET_INTEGER_PART(v2);
    let f1 = GET_FRACTIONAL_PART(v1); let f2 = GET_FRACTIONAL_PART(v2);
    let mut value = (i1 * i2) << FIXED31_32_BITS_PER_FRACTIONAL_PART;
    let mut tmp = i1 * f2; ASSERT(tmp <= (LLONG_MAX - value)); value += tmp;
    tmp = i2 * f1; ASSERT(tmp <= (LLONG_MAX - value)); value += tmp;
    tmp = (f1 * f2 >> FIXED31_32_BITS_PER_FRACTIONAL_PART) + ((f1 * f2 >= dc_fixpt_half.value as u64) as u64);
    ASSERT(tmp <= (LLONG_MAX - value)); value += tmp;
    let mut res = fixed31_32 { value: value as i64 };
    if n1 ^ n2 { res.value = -res.value; } res
}

fn dc_fixpt_sqr(arg: fixed31_32) -> fixed31_32 {
    let v = abs_i64(arg.value); let i = GET_INTEGER_PART(v); let f = GET_FRACTIONAL_PART(v);
    let mut value = (i * i) << FIXED31_32_BITS_PER_FRACTIONAL_PART;
    let mut tmp = i * f; ASSERT(tmp <= (LLONG_MAX - value)); value += tmp;
    ASSERT(tmp <= (LLONG_MAX - value)); value += tmp;
    tmp = (f * f >> FIXED31_32_BITS_PER_FRACTIONAL_PART) + ((f * f >= dc_fixpt_half.value as u64) as u64);
    ASSERT(tmp <= (LLONG_MAX - value)); value += tmp;
    fixed31_32 { value: value as i64 }
}

fn dc_fixpt_recip(arg: fixed31_32) -> fixed31_32 { dc_fixpt_from_fraction(dc_fixpt_one.value, arg.value) }

fn dc_fixpt_sinc(arg: fixed31_32) -> fixed31_32 {
    let mut square; let mut res = dc_fixpt_one; let mut n = 27; let mut arg_norm = arg;
    if dc_fixpt_le(dc_fixpt_two_pi, dc_fixpt_abs(arg)) {
        arg_norm = dc_fixpt_sub(arg_norm, dc_fixpt_mul_int(dc_fixpt_two_pi, unsafe { div64_s64(arg_norm.value, dc_fixpt_two_pi.value) } as i32));
    }
    square = dc_fixpt_sqr(arg_norm);
    loop { res = dc_fixpt_sub(dc_fixpt_one, dc_fixpt_div_int(dc_fixpt_mul(square, res), n * (n - 1))); n -= 2; if n <= 2 { break; } }
    if arg.value != arg_norm.value { res = dc_fixpt_div(dc_fixpt_mul(res, arg_norm), arg); } res
}

fn dc_fixpt_sin(arg: fixed31_32) -> fixed31_32 { dc_fixpt_mul(arg, dc_fixpt_sinc(arg)) }

fn dc_fixpt_cos(arg: fixed31_32) -> fixed31_32 {
    /* TODO implement argument normalization */
    let square = dc_fixpt_sqr(arg); let mut res = dc_fixpt_one; let mut n = 26;
    loop { res = dc_fixpt_sub(dc_fixpt_one, dc_fixpt_div_int(dc_fixpt_mul(square, res), (n as i64) * ((n - 1) as i64))); n -= 2; if n == 0 { break; } } res
}

/* Taylor-series implementation for exp(arg), where abs(arg) < 1. */
fn fixed31_32_exp_from_taylor_series(arg: fixed31_32) -> fixed31_32 {
    let mut n = 9u32; let mut res = dc_fixpt_from_fraction((n + 2) as i64, (n + 1) as i64);
    /* TODO find correct res */
    ASSERT(dc_fixpt_lt(arg, dc_fixpt_one));
    loop { res = dc_fixpt_add(dc_fixpt_one, dc_fixpt_div_int(dc_fixpt_mul(arg, res), n as i64)); n -= 1; if n == 1 { break; } }
    dc_fixpt_add(dc_fixpt_one, dc_fixpt_mul(arg, res))
}

fn dc_fixpt_exp(arg: fixed31_32) -> fixed31_32 {
    if dc_fixpt_le(dc_fixpt_ln2_div_2, dc_fixpt_abs(arg)) {
        let m = dc_fixpt_round(dc_fixpt_div(arg, dc_fixpt_ln2));
        let r = dc_fixpt_sub(arg, dc_fixpt_mul_int(dc_fixpt_ln2, m));
        ASSERT(m != 0); ASSERT(dc_fixpt_lt(dc_fixpt_abs(r), dc_fixpt_one));
        if m > 0 { dc_fixpt_shl(fixed31_32_exp_from_taylor_series(r), m as u8) }
        else { dc_fixpt_div_int(fixed31_32_exp_from_taylor_series(r), 1i64 << -m) }
    } else if arg.value != 0 { fixed31_32_exp_from_taylor_series(arg) } else { dc_fixpt_one }
}

fn dc_fixpt_log(arg: fixed31_32) -> fixed31_32 {
    let mut res = dc_fixpt_neg(dc_fixpt_one); let mut error;
    /* TODO improve 1st estimation */
    ASSERT(arg.value > 0);
    /* TODO if arg is negative, return NaN */
    /* TODO if arg is zero, return -INF */
    loop {
        let res1 = dc_fixpt_add(dc_fixpt_sub(res, dc_fixpt_one), dc_fixpt_div(arg, dc_fixpt_exp(res)));
        error = dc_fixpt_sub(res, res1); res = res1;
        /* TODO determine max_allowed_error based on quality of exp() */
        if abs_i64(error.value) <= 100 { break; }
    } res
}

static inline fn ux_dy(value: i64, integer_bits: u32, fractional_bits: u32) -> u32 {
    let mut result = (1u32 << integer_bits) - 1;
    let mut fractional_part = FRACTIONAL_PART_MASK & value as u64;
    result &= GET_INTEGER_PART(value as u64) as u32;
    result <<= fractional_bits;
    fractional_part >>= FIXED31_32_BITS_PER_FRACTIONAL_PART - fractional_bits;
    result | fractional_part as u32
}

static inline fn clamp_ux_dy(value: i64, integer_bits: u32, fractional_bits: u32, min_clamp: u32) -> u32 {
    let truncated_val = ux_dy(value, integer_bits, fractional_bits);
    if value >= (1i64 << (integer_bits + FIXED31_32_BITS_PER_FRACTIONAL_PART)) { (1u32 << (integer_bits + fractional_bits)) - 1 }
    else if truncated_val > min_clamp { truncated_val } else { min_clamp }
}

fn dc_fixpt_u4d19(arg: fixed31_32) -> u32 { ux_dy(arg.value, 4, 19) }
fn dc_fixpt_u3d19(arg: fixed31_32) -> u32 { ux_dy(arg.value, 3, 19) }
fn dc_fixpt_u2d19(arg: fixed31_32) -> u32 { ux_dy(arg.value, 2, 19) }
fn dc_fixpt_u0d19(arg: fixed31_32) -> u32 { ux_dy(arg.value, 0, 19) }
fn dc_fixpt_clamp_u0d14(arg: fixed31_32) -> u32 { clamp_ux_dy(arg.value, 0, 14, 1) }
fn dc_fixpt_clamp_u0d10(arg: fixed31_32) -> u32 { clamp_ux_dy(arg.value, 0, 10, 1) }
fn dc_fixpt_s4d19(arg: fixed31_32) -> i32 { if arg.value < 0 { -(ux_dy(dc_fixpt_abs(arg).value, 4, 19) as i32) } else { ux_dy(arg.value, 4, 19) as i32 } }

fn dc_fixpt_from_ux_dy(value: u32, integer_bits: u32, fractional_bits: u32) -> fixed31_32 {
    let mut fixpt_value = dc_fixpt_zero; let mut fixpt_int_value = dc_fixpt_zero;
    let mut frac_mask = ((1i64 << integer_bits) - 1) << fractional_bits;
    fixpt_value.value = (value as i64) << (FIXED31_32_BITS_PER_FRACTIONAL_PART - fractional_bits);
    fixpt_int_value.value = (value as i64) & frac_mask;
    fixpt_int_value.value <<= FIXED31_32_BITS_PER_FRACTIONAL_PART - fractional_bits;
    fixpt_value.value |= fixpt_int_value.value; fixpt_value
}

fn dc_fixpt_from_int_dy(int_value: u32, frac_value: u32, integer_bits: u32, fractional_bits: u32) -> fixed31_32 {
    let _ = integer_bits;
    let mut fixpt_value = dc_fixpt_from_int(int_value);
    fixpt_value.value |= (frac_value as i64) << (FIXED31_32_BITS_PER_FRACTIONAL_PART - fractional_bits);
    fixpt_value
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
