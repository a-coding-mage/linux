// SPDX-License-Identifier: MIT
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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
 *
 */

// Dependencies supplied by dm_services.h and bw_fixed.h.

const MAX_I64: i64 = ((1u64 << 63) - 1) as i64;
const MIN_I64: i64 = -MAX_I64 - 1;

const FRACTIONAL_PART_MASK: u64 = (1u64 << BW_FIXED_BITS_PER_FRACTIONAL_PART) - 1;

#[inline]
fn get_fractional_part(x: u64) -> u64 {
    FRACTIONAL_PART_MASK & x
}

#[inline]
fn abs_i64(arg: i64) -> u64 {
    if arg >= 0 { arg as u64 } else { (-arg) as u64 }
}

pub fn bw_int_to_fixed_nonconst(value: i64) -> bw_fixed {
    assert!(value < BW_FIXED_MAX_I32 && value > BW_FIXED_MIN_I32);
    bw_fixed { value: value << BW_FIXED_BITS_PER_FRACTIONAL_PART }
}

pub fn bw_frc_to_fixed(numerator: i64, denominator: i64) -> bw_fixed {
    let arg1_negative = numerator < 0;
    let arg2_negative = denominator < 0;
    let arg1_value: u64;
    let arg2_value: u64;
    let mut remainder: u64;

    // determine integer part
    let mut res_value: u64;

    assert!(denominator != 0);

    arg1_value = abs_i64(numerator);
    arg2_value = abs_i64(denominator);
    res_value = unsafe { div64_u64_rem(arg1_value, arg2_value, &mut remainder) };

    assert!(res_value <= BW_FIXED_MAX_I32);

    // determine fractional part
    {
        let mut i = BW_FIXED_BITS_PER_FRACTIONAL_PART;

        loop {
            remainder <<= 1;
            res_value <<= 1;

            if remainder >= arg2_value {
                res_value |= 1;
                remainder -= arg2_value;
            }
            i -= 1;
            if i == 0 { break; }
        }
    }

    // round up LSB
    {
        let summand: u64 = ((remainder << 1) >= arg2_value) as u64;

        assert!(res_value <= (MAX_I64 as u64) - summand);
        res_value += summand;
    }

    let mut res = bw_fixed { value: res_value as i64 };

    if arg1_negative ^ arg2_negative {
        res.value = -res.value;
    }
    res
}

pub fn bw_floor2(arg: bw_fixed, significance: bw_fixed) -> bw_fixed {
    let multiplicand = unsafe { div64_s64(arg.value, abs_i64(significance.value) as i64) };
    let result = bw_fixed { value: (abs_i64(significance.value) as i64) * multiplicand };
    assert!(abs_i64(result.value) <= abs_i64(arg.value));
    result
}

pub fn bw_ceil2(arg: bw_fixed, significance: bw_fixed) -> bw_fixed {
    let mut result = bw_fixed { value: 0 };
    let multiplicand = unsafe { div64_s64(arg.value, abs_i64(significance.value) as i64) };
    result.value = (abs_i64(significance.value) as i64) * multiplicand;
    if abs_i64(result.value) < abs_i64(arg.value) {
        if arg.value < 0 {
            result.value -= abs_i64(significance.value) as i64;
        } else {
            result.value += abs_i64(significance.value) as i64;
        }
    }
    result
}

pub fn bw_mul(arg1: bw_fixed, arg2: bw_fixed) -> bw_fixed {
    let arg1_negative = arg1.value < 0;
    let arg2_negative = arg2.value < 0;

    let arg1_value = abs_i64(arg1.value);
    let arg2_value = abs_i64(arg2.value);

    let arg1_int = BW_FIXED_GET_INTEGER_PART(arg1_value);
    let arg2_int = BW_FIXED_GET_INTEGER_PART(arg2_value);

    let arg1_fra = get_fractional_part(arg1_value);
    let arg2_fra = get_fractional_part(arg2_value);

    let mut tmp: u64;
    let mut res = bw_fixed { value: (arg1_int * arg2_int) as i64 };

    assert!(res.value <= BW_FIXED_MAX_I32);
    res.value <<= BW_FIXED_BITS_PER_FRACTIONAL_PART;

    tmp = arg1_int * arg2_fra;
    assert!(tmp <= (MAX_I64 - res.value) as u64);
    res.value += tmp as i64;

    tmp = arg2_int * arg1_fra;
    assert!(tmp <= (MAX_I64 - res.value) as u64);
    res.value += tmp as i64;

    tmp = arg1_fra * arg2_fra;
    tmp = (tmp >> BW_FIXED_BITS_PER_FRACTIONAL_PART)
        + (tmp >= bw_frc_to_fixed(1, 2).value as u64) as u64;

    assert!(tmp <= (MAX_I64 - res.value) as u64);
    res.value += tmp as i64;

    if arg1_negative ^ arg2_negative {
        res.value = -res.value;
    }
    res
}

extern "C" {
    fn div64_u64_rem(numerator: u64, denominator: u64, remainder: *mut u64) -> u64;
    fn div64_s64(numerator: i64, denominator: i64) -> i64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
