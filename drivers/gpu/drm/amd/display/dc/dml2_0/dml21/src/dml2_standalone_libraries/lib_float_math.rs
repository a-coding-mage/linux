// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Translated from lib_float_math.c.

// NOTE:
//   This file is gcc-parseable HW gospel, coming straight from HW engineers.
//
//   It doesn't adhere to Linux kernel style and sometimes will do things in odd
//   ways. Unless there is something clearly wrong with it the code should
//   remain as-is as it provides us with a guarantee from HW that it is correct.

#[inline]
fn is_nan(number: f64) -> bool {
    number != number
}

pub fn math_mod(arg1: f64, arg2: f64) -> f64 {
    if is_nan(arg1) {
        return arg2;
    }
    if is_nan(arg2) {
        return arg1;
    }
    arg1 - arg2 * ((arg1 / arg2) as i32) as f64
}

pub fn math_min2(arg1: f64, arg2: f64) -> f64 {
    if is_nan(arg1) {
        return arg2;
    }
    if is_nan(arg2) {
        return arg1;
    }
    if arg1 < arg2 { arg1 } else { arg2 }
}

pub fn math_max2(arg1: f64, arg2: f64) -> f64 {
    if is_nan(arg1) {
        return arg2;
    }
    if is_nan(arg2) {
        return arg1;
    }
    if arg1 > arg2 { arg1 } else { arg2 }
}

pub fn math_floor2(arg: f64, significance: f64) -> f64 {
    // ASSERT(significance != 0);
    ((arg / significance) as i32) as f64 * significance
}

pub fn math_floor(arg: f64) -> f64 {
    (arg as i32) as f64
}

pub fn math_ceil(arg: f64) -> f64 {
    (arg + 0.99999) as i32 as f64
}

pub fn math_ceil2(arg: f64, significance: f64) -> f64 {
    ((arg / significance + 0.99999) as i32) as f64 * significance
}

pub fn math_max3(v1: f64, v2: f64, v3: f64) -> f64 {
    if v3 > math_max2(v1, v2) { v3 } else { math_max2(v1, v2) }
}

pub fn math_max4(v1: f64, v2: f64, v3: f64, v4: f64) -> f64 {
    if v4 > math_max3(v1, v2, v3) { v4 } else { math_max3(v1, v2, v3) }
}

pub fn math_max5(v1: f64, v2: f64, v3: f64, v4: f64, v5: f64) -> f64 {
    if math_max3(v1, v2, v3) > math_max2(v4, v5) {
        math_max3(v1, v2, v3)
    } else {
        math_max2(v4, v5)
    }
}

pub fn math_pow(a: f32, exp: f32) -> f32 {
    let temp: f64;
    if exp as i32 == 0 {
        return 1.0;
    }
    temp = math_pow(a, (exp / 2.0) as i32 as f32) as f64;
    if (exp as i32) % 2 == 0 {
        (temp * temp) as f32
    } else if exp as i32 > 0 {
        (a as f64 * temp * temp) as f32
    } else {
        ((temp * temp) / a as f64) as f32
    }
}

pub fn math_fabs(a: f64) -> f64 {
    if a > 0.0 { a } else { -a }
}

pub fn math_log(mut a: f32, b: f32) -> f32 {
    let mut x = a.to_bits() as i32;
    let log_2 = ((x >> 23) & 255) - 128;
    x &= !(255 << 23);
    x += 127 << 23;
    a = f32::from_bits(x as u32);

    a = ((-1.0f32 / 3.0) * a + 2.0) * a - 2.0 / 3.0;

    if b > 2.00001 || b < 1.99999 {
        (a + log_2 as f32) / math_log(b, 2.0)
    } else {
        a + log_2 as f32
    }
}

pub fn math_log2(a: f32) -> f32 {
    math_log(a, 2.0)
}

// approximate log2 value of a input
//  - precise if the input pwr of 2, else the approximation will be an integer = floor(actual_log2)
pub fn math_log2_approx(mut a: u32) -> u32 {
    let mut log2_val = 0;
    while a > 1 {
        a >>= 1;
        log2_val += 1;
    }
    log2_val
}

pub fn math_round(a: f64) -> f64 {
    let round_pt = 0.5;
    math_floor(a + round_pt)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
