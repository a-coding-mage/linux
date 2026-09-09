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
 */

use core::ffi::c_void;

#[repr(C)]
pub struct fixed31_32 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct custom_float_format {
    pub sign: bool,
    pub mantissa_bits: u32,
    pub exponenta_bits: u32,
}

extern "C" {
    fn dc_fixpt_from_fraction(numerator: i64, denominator: i64) -> fixed31_32;
    fn dc_fixpt_eq(a: fixed31_32, b: fixed31_32) -> bool;
    fn dc_fixpt_lt(a: fixed31_32, b: fixed31_32) -> bool;
    fn dc_fixpt_le(a: fixed31_32, b: fixed31_32) -> bool;
    fn dc_fixpt_zero() -> fixed31_32;
    fn dc_fixpt_one() -> fixed31_32;
    fn dc_fixpt_neg(value: fixed31_32) -> fixed31_32;
    fn dc_fixpt_shl(value: fixed31_32, shift: u8) -> fixed31_32;
    fn dc_fixpt_shr(value: fixed31_32, shift: u8) -> fixed31_32;
    fn dc_fixpt_sub(a: fixed31_32, b: fixed31_32) -> fixed31_32;
    fn dc_fixpt_floor(value: fixed31_32) -> u32;
    fn BREAK_TO_DEBUGGER();
}

unsafe fn build_custom_float(
    mut value: fixed31_32,
    format: *const custom_float_format,
    negative: *mut bool,
    mantissa: *mut u32,
    exponenta: *mut u32,
) -> bool {
    let exp_offset = (1u32 << ((*format).exponenta_bits - 1)) - 1;
    let mantissa_constant_plus_max_fraction = dc_fixpt_from_fraction(
        ((1i64 << ((*format).mantissa_bits + 1)) - 1),
        1i64 << (*format).mantissa_bits,
    );

    let mut mantiss: fixed31_32;

    if dc_fixpt_eq(value, dc_fixpt_zero()) {
        *negative = false;
        *mantissa = 0;
        *exponenta = 0;
        return true;
    }

    if dc_fixpt_lt(value, dc_fixpt_zero()) {
        *negative = (*format).sign;
        value = dc_fixpt_neg(value);
    } else {
        *negative = false;
    }

    if dc_fixpt_lt(value, dc_fixpt_one()) {
        let mut i = 1u32;
        loop {
            value = dc_fixpt_shl(value, 1);
            i += 1;
            if !dc_fixpt_lt(value, dc_fixpt_one()) {
                break;
            }
        }
        i -= 1;

        if exp_offset <= i {
            *mantissa = 0;
            *exponenta = 0;
            return true;
        }
        *exponenta = exp_offset - i;
    } else if dc_fixpt_le(mantissa_constant_plus_max_fraction, value) {
        let mut i = 1u32;
        loop {
            value = dc_fixpt_shr(value, 1);
            i += 1;
            if !dc_fixpt_lt(mantissa_constant_plus_max_fraction, value) {
                break;
            }
        }
        *exponenta = exp_offset + i - 1;
    } else {
        *exponenta = exp_offset;
    }

    mantiss = dc_fixpt_sub(value, dc_fixpt_one());
    if dc_fixpt_lt(mantiss, dc_fixpt_zero()) || dc_fixpt_lt(dc_fixpt_one(), mantiss) {
        mantiss = dc_fixpt_zero();
    } else {
        mantiss = dc_fixpt_shl(mantiss, (*format).mantissa_bits as u8);
    }
    *mantissa = dc_fixpt_floor(mantiss);
    true
}

unsafe fn setup_custom_float(
    format: *const custom_float_format,
    negative: bool,
    mut mantissa: u32,
    mut exponenta: u32,
    result: *mut u32,
) -> bool {
    let mut i = 0u32;
    let mut j = 0u32;
    let mut value = 0u32;

    /* verification code: once calculation is ok we can remove it */
    let mantissa_mask = (1u32 << ((*format).mantissa_bits + 1)) - 1;
    let exponenta_mask = (1u32 << ((*format).exponenta_bits + 1)) - 1;

    if mantissa & !mantissa_mask != 0 {
        BREAK_TO_DEBUGGER();
        mantissa = mantissa_mask;
    }
    if exponenta & !exponenta_mask != 0 {
        BREAK_TO_DEBUGGER();
        exponenta = exponenta_mask;
    }

    while i < (*format).mantissa_bits {
        let mask = 1u32 << i;
        if mantissa & mask != 0 { value |= mask; }
        i += 1;
    }
    while j < (*format).exponenta_bits {
        let mask = 1u32 << j;
        if exponenta & mask != 0 { value |= mask << i; }
        j += 1;
    }
    if negative && (*format).sign { value |= 1u32 << (i + j); }
    *result = value;
    true
}

pub unsafe fn convert_to_custom_float_format(
    value: fixed31_32,
    format: *const custom_float_format,
    result: *mut u32,
) -> bool {
    let mut mantissa = 0u32;
    let mut exponenta = 0u32;
    let mut negative = false;
    build_custom_float(value, format, &mut negative, &mut mantissa, &mut exponenta)
        && setup_custom_float(format, negative, mantissa, exponenta, result)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
