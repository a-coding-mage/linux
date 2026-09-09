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
 *
 */

// Dependencies supplied by the surrounding translation unit.

pub unsafe fn fixed_point_to_int_frac(
    arg: fixed31_32,
    integer_bits: u8,
    fractional_bits: u8,
) -> u16 {
    let mut numerator: i32;
    let divisor: i32 = 1i32 << fractional_bits;

    let d: u16 = dc_fixpt_floor(dc_fixpt_abs(arg)) as u16;

    if d <= ((1u16 << integer_bits) - (1u16 / divisor as u16)) {
        numerator = dc_fixpt_round(dc_fixpt_mul_int(arg, divisor)) as u16 as i32;
    } else {
        numerator = dc_fixpt_floor(dc_fixpt_sub(
            dc_fixpt_from_int(1i64 << integer_bits),
            dc_fixpt_recip(dc_fixpt_from_int(divisor as i64)),
        ));
    }

    let mut result: u16;
    if numerator >= 0 {
        result = numerator as u16;
    } else {
        result = ((1u16 << (integer_bits + fractional_bits + 1)) as i32 + numerator) as u16;
    }

    if result != 0 && dc_fixpt_lt(arg, dc_fixpt_zero()) {
        result |= 1u16 << (integer_bits + fractional_bits);
    }

    result
}

/*
 * convert_float_matrix - This converts a double into HW register spec defined format S2D13 / S3D12.
 */
pub unsafe fn convert_float_matrix(
    matrix: *mut u16,
    flt: *const fixed31_32,
    format: cm_gamut_coef_format,
    buffer_size: u32,
) {
    let min: fixed31_32;
    let max: fixed31_32;
    let num_int_bits: u8;
    let num_dec_bits: u8;

    if format == CM_GAMUT_REMAP_COEF_FORMAT_S2_13 {
        min = dc_fixpt_from_fraction(S2D13_MIN, DIVIDER);
        max = dc_fixpt_from_fraction(S2D13_MAX, DIVIDER);
        num_int_bits = 2;
        num_dec_bits = 13;
    } else if format == CM_GAMUT_REMAP_COEF_FORMAT_S3_12 {
        min = dc_fixpt_from_fraction(S3D12_MIN, DIVIDER);
        max = dc_fixpt_from_fraction(S3D12_MAX, DIVIDER);
        num_int_bits = 3;
        num_dec_bits = 12;
    } else {
        ASSERT(false);
        return;
    }

    for i in 0..buffer_size {
        let reg_value = fixed_point_to_int_frac(
            dc_fixpt_clamp(*flt.add(i as usize), min, max),
            num_int_bits,
            num_dec_bits,
        );
        *matrix.add(i as usize) = reg_value;
    }
}

unsafe fn int_frac_to_fixed_point(arg: u16, integer_bits: u8, fractional_bits: u8) -> fixed31_32 {
    let mut result: fixed31_32;
    let sign_mask: u16 = 1u16 << (fractional_bits + integer_bits);
    let value_mask: u16 = sign_mask - 1;

    result.value = ((arg & value_mask) as i64)
        << (FIXED31_32_BITS_PER_FRACTIONAL_PART - fractional_bits);

    if arg & sign_mask != 0 {
        result = dc_fixpt_neg(result);
    }

    result
}

/**
 * convert_hw_matrix - converts HW values into fixed31_32 matrix.
 * @matrix: fixed point 31.32 matrix
 * @reg: array of register values
 * @buffer_size: size of the array of register values
 *
 * Converts HW register spec defined format S2D13 into a fixed-point 31.32
 * matrix.
 */
pub unsafe fn convert_hw_matrix(
    matrix: *mut fixed31_32,
    reg: *mut u16,
    format: cm_gamut_coef_format,
    buffer_size: u32,
) {
    let num_int_bits: u8;
    let num_dec_bits: u8;

    if format == CM_GAMUT_REMAP_COEF_FORMAT_S2_13 {
        num_int_bits = 2;
        num_dec_bits = 13;
    } else if format == CM_GAMUT_REMAP_COEF_FORMAT_S3_12 {
        num_int_bits = 3;
        num_dec_bits = 12;
    } else {
        ASSERT(false);
        return;
    }

    for i in 0..buffer_size {
        *matrix.add(i as usize) = int_frac_to_fixed_point(
            *reg.add(i as usize),
            num_int_bits,
            num_dec_bits,
        );
    }
}

unsafe fn find_gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

pub unsafe fn reduce_fraction(num: u32, den: u32, out_num: *mut u32, out_den: *mut u32) {
    let gcd = find_gcd(num, den);
    *out_num = num / gcd;
    *out_den = den / gcd;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
