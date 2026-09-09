// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependencies supplied by spl_debug.h and spl_custom_float.h remain external.

use core::ffi::c_void;

#[repr(C)]
pub struct spl_fixed31_32 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spl_custom_float_format {
    pub sign: bool,
    pub mantissa_bits: u32,
    pub exponenta_bits: u32,
}

extern "C" {
    static spl_fixpt_zero: spl_fixed31_32;
    static spl_fixpt_one: spl_fixed31_32;

    fn spl_fixpt_from_fraction(numerator: i64, denominator: i64) -> spl_fixed31_32;
    fn spl_fixpt_eq(a: spl_fixed31_32, b: spl_fixed31_32) -> bool;
    fn spl_fixpt_lt(a: spl_fixed31_32, b: spl_fixed31_32) -> bool;
    fn spl_fixpt_le(a: spl_fixed31_32, b: spl_fixed31_32) -> bool;
    fn spl_fixpt_neg(value: spl_fixed31_32) -> spl_fixed31_32;
    fn spl_fixpt_shl(value: spl_fixed31_32, shift: u32) -> spl_fixed31_32;
    fn spl_fixpt_shr(value: spl_fixed31_32, shift: u32) -> spl_fixed31_32;
    fn spl_fixpt_sub(a: spl_fixed31_32, b: spl_fixed31_32) -> spl_fixed31_32;
    fn spl_fixpt_floor(value: spl_fixed31_32) -> u32;
    fn spl_break_to_debugger();
}

unsafe fn spl_build_custom_float(
    mut value: spl_fixed31_32,
    format: *const spl_custom_float_format,
    negative: *mut bool,
    mantissa: *mut u32,
    exponenta: *mut u32,
) -> bool {
    let exp_offset = (1u32 << ((*format).exponenta_bits - 1)) - 1;

    let mantissa_constant_plus_max_fraction = spl_fixpt_from_fraction(
        ((1i64 << ((*format).mantissa_bits + 1)) - 1),
        1i64 << (*format).mantissa_bits,
    );

    let mut mantiss: spl_fixed31_32;

    if spl_fixpt_eq(value, spl_fixpt_zero) {
        *negative = false;
        *mantissa = 0;
        *exponenta = 0;
        return true;
    }

    if spl_fixpt_lt(value, spl_fixpt_zero) {
        *negative = (*format).sign;
        value = spl_fixpt_neg(value);
    } else {
        *negative = false;
    }

    if spl_fixpt_lt(value, spl_fixpt_one) {
        let mut i: u32 = 1;

        loop {
            value = spl_fixpt_shl(value, 1);
            i += 1;
            if !spl_fixpt_lt(value, spl_fixpt_one) {
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
    } else if spl_fixpt_le(mantissa_constant_plus_max_fraction, value) {
        let mut i: u32 = 1;

        loop {
            value = spl_fixpt_shr(value, 1);
            i += 1;
            if !spl_fixpt_lt(mantissa_constant_plus_max_fraction, value) {
                break;
            }
        }

        *exponenta = exp_offset + i - 1;
    } else {
        *exponenta = exp_offset;
    }

    mantiss = spl_fixpt_sub(value, spl_fixpt_one);

    if spl_fixpt_lt(mantiss, spl_fixpt_zero) || spl_fixpt_lt(spl_fixpt_one, mantiss) {
        mantiss = spl_fixpt_zero;
    } else {
        mantiss = spl_fixpt_shl(mantiss, (*format).mantissa_bits);
    }

    *mantissa = spl_fixpt_floor(mantiss);
    true
}

unsafe fn spl_setup_custom_float(
    format: *const spl_custom_float_format,
    negative: bool,
    mut mantissa: u32,
    mut exponenta: u32,
    result: *mut u32,
) -> bool {
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let mut value: u32 = 0;

    /* verification code:
     * once calculation is ok we can remove it
     */
    let mantissa_mask = (1u32 << ((*format).mantissa_bits + 1)) - 1;
    let exponenta_mask = (1u32 << ((*format).exponenta_bits + 1)) - 1;

    if mantissa & !mantissa_mask != 0 {
        spl_break_to_debugger();
        mantissa = mantissa_mask;
    }

    if exponenta & !exponenta_mask != 0 {
        spl_break_to_debugger();
        exponenta = exponenta_mask;
    }

    /* end of verification code */
    while i < (*format).mantissa_bits {
        let mask = 1u32 << i;
        if mantissa & mask != 0 {
            value |= mask;
        }
        i += 1;
    }

    while j < (*format).exponenta_bits {
        let mask = 1u32 << j;
        if exponenta & mask != 0 {
            value |= mask << i;
        }
        j += 1;
    }

    if negative && (*format).sign {
        value |= 1u32 << (i + j);
    }

    *result = value;
    true
}

pub unsafe fn spl_convert_to_custom_float_format(
    value: spl_fixed31_32,
    format: *const spl_custom_float_format,
    result: *mut u32,
) -> bool {
    let mut mantissa: u32 = 0;
    let mut exponenta: u32 = 0;
    let mut negative = false;

    spl_build_custom_float(
        value,
        format,
        &mut negative,
        &mut mantissa,
        &mut exponenta,
    ) && spl_setup_custom_float(format, negative, mantissa, exponenta, result)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
