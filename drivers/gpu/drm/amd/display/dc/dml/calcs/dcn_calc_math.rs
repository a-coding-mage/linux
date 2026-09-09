/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding translation unit.

#[inline]
fn is_nan(number: f32) -> bool {
    number != number
}

/*
 * NOTE:
 *   This file is gcc-parseable HW gospel, coming straight from HW engineers.
 *
 * It doesn't adhere to Linux kernel style and sometimes will do things in odd
 * ways. Unless there is something clearly wrong with it the code should
 * remain as-is as it provides us with a guarantee from HW that it is correct.
 */

pub fn dcn_bw_mod(arg1: f32, arg2: f32) -> f32 {
    if is_nan(arg1) {
        return arg2;
    }
    if is_nan(arg2) {
        return arg1;
    }
    arg1 - arg1 * ((arg1 / arg2) as i32 as f32)
}

pub fn dcn_bw_min2(arg1: f32, arg2: f32) -> f32 {
    if is_nan(arg1) {
        return arg2;
    }
    if is_nan(arg2) {
        return arg1;
    }
    if arg1 < arg2 { arg1 } else { arg2 }
}

pub fn dcn_bw_max(arg1: u32, arg2: u32) -> u32 {
    if arg1 > arg2 { arg1 } else { arg2 }
}

pub fn dcn_bw_max2(arg1: f32, arg2: f32) -> f32 {
    if is_nan(arg1) {
        return arg2;
    }
    if is_nan(arg2) {
        return arg1;
    }
    if arg1 > arg2 { arg1 } else { arg2 }
}

pub fn dcn_bw_floor2(arg: f32, significance: f32) -> f32 {
    assert!(significance != 0.0);
    (arg / significance) as i32 as f32 * significance
}

pub fn dcn_bw_floor(arg: f32) -> f32 {
    arg as i32 as f32
}

pub fn dcn_bw_ceil(arg: f32) -> f32 {
    (arg + 0.99999f32) as i32 as f32
}

pub fn dcn_bw_ceil2(arg: f32, significance: f32) -> f32 {
    assert!(significance != 0.0);
    (arg / significance + 0.99999f32) as i32 as f32 * significance
}

pub fn dcn_bw_max3(v1: f32, v2: f32, v3: f32) -> f32 {
    if v3 > dcn_bw_max2(v1, v2) { v3 } else { dcn_bw_max2(v1, v2) }
}

pub fn dcn_bw_max5(v1: f32, v2: f32, v3: f32, v4: f32, v5: f32) -> f32 {
    if dcn_bw_max3(v1, v2, v3) > dcn_bw_max2(v4, v5) {
        dcn_bw_max3(v1, v2, v3)
    } else {
        dcn_bw_max2(v4, v5)
    }
}

pub fn dcn_bw_pow(a: f32, exp: f32) -> f32 {
    let temp: f32;
    /* ASSERT(exp == (int)exp); */
    if exp as i32 == 0 {
        return 1.0;
    }
    temp = dcn_bw_pow(a, (exp / 2.0) as i32 as f32);
    if (exp as i32) % 2 == 0 {
        temp * temp
    } else if exp as i32 > 0 {
        a * temp * temp
    } else {
        (temp * temp) / a
    }
}

pub fn dcn_bw_fabs(a: f64) -> f64 {
    if a > 0.0 { a } else { -a }
}

pub fn dcn_bw_log(mut a: f32, b: f32) -> f32 {
    let mut x: i32 = a.to_bits() as i32;
    let log_2: i32 = ((x >> 23) & 255) - 128;
    x &= !(255 << 23);
    x += 127 << 23;
    a = f32::from_bits(x as u32);

    a = ((-1.0f32 / 3.0) * a + 2.0) * a - 2.0 / 3.0;

    if b > 2.00001 || b < 1.99999 {
        (a + log_2 as f32) / dcn_bw_log(b, 2.0)
    } else {
        a + log_2 as f32
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
