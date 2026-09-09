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
 *
 */

// Dependencies supplied by dcn_calc_math.h.
unsafe extern "C" {
    fn dcn_bw_min2(a: f32, b: f32) -> f32;
    fn dcn_bw_max2(a: f32, b: f32) -> f32;
    fn dcn_bw_ceil2(a: f32, granularity: f32) -> f32;
    fn dcn_bw_floor2(a: f32, granularity: f32) -> f32;
    fn dcn_bw_pow(a: f32, exp: f32) -> f32;
    fn dcn_bw_mod(a: f32, val: f32) -> f32;
}

#[inline]
fn dml_min(a: f64, b: f64) -> f64 {
    unsafe { dcn_bw_min2(a as f32, b as f32) as f64 }
}

#[inline]
fn dml_min3(a: f64, b: f64, c: f64) -> f64 {
    dml_min(dml_min(a, b), c)
}

#[inline]
fn dml_min4(a: f64, b: f64, c: f64, d: f64) -> f64 {
    dml_min(dml_min(a, b), dml_min(c, d))
}

#[inline]
fn dml_max(a: f64, b: f64) -> f64 {
    unsafe { dcn_bw_max2(a as f32, b as f32) as f64 }
}

#[inline]
fn dml_max3(a: f64, b: f64, c: f64) -> f64 {
    dml_max(dml_max(a, b), c)
}

#[inline]
fn dml_max4(a: f64, b: f64, c: f64, d: f64) -> f64 {
    dml_max(dml_max(a, b), dml_max(c, d))
}

#[inline]
fn dml_max5(a: f64, b: f64, c: f64, d: f64, e: f64) -> f64 {
    dml_max(dml_max4(a, b, c, d), e)
}

#[inline]
fn dml_ceil(a: f64, granularity: f64) -> f64 {
    if granularity == 0.0 { return 0.0; }
    unsafe { dcn_bw_ceil2(a as f32, granularity as f32) as f64 }
}

#[inline]
fn dml_floor(a: f64, granularity: f64) -> f64 {
    if granularity == 0.0 { return 0.0; }
    unsafe { dcn_bw_floor2(a as f32, granularity as f32) as f64 }
}

#[inline]
fn dml_round(a: f64) -> f64 {
    let round_pt: f64 = 0.5;
    dml_floor(a + round_pt, 1.0)
}

/* float
static inline int dml_log2(float x)
{
	unsigned int ix = *((unsigned int *)&x);

	return (int)((ix >> 23) & 0xff) - 127;
}*/

/* double */
#[inline]
fn dml_log2(x: f64) -> i32 {
    let ix: u64 = x.to_bits();
    ((ix >> 52) & 0x7ff) as i32 - 1023
}

#[inline]
fn dml_pow(a: f64, exp: i32) -> f64 {
    unsafe { dcn_bw_pow(a as f32, exp as f32) as f64 }
}

#[inline]
fn dml_fmod(f: f64, val: i32) -> f64 {
    unsafe { dcn_bw_mod(f as f32, val as f32) as f64 }
}

#[inline]
fn dml_ceil_2(f: f64) -> f64 {
    unsafe { dcn_bw_ceil2(f as f32, 2.0_f32) as f64 }
}

#[inline]
fn dml_ceil_ex(x: f64, granularity: f64) -> f64 {
    if granularity == 0.0 { return 0.0; }
    unsafe { dcn_bw_ceil2(x as f32, granularity as f32) as f64 }
}

#[inline]
fn dml_floor_ex(x: f64, granularity: f64) -> f64 {
    if granularity == 0.0 { return 0.0; }
    unsafe { dcn_bw_floor2(x as f32, granularity as f32) as f64 }
}

#[inline]
fn dml_round_to_multiple(num: u32, multiple: u32, up: u8) -> u32 {
    if multiple == 0 { return num; }

    let remainder = num % multiple;
    if remainder == 0 { return num; }

    if up != 0 {
        num + multiple - remainder
    } else {
        num - remainder
    }
}

#[inline]
fn dml_abs(a: f64) -> f64 {
    if a > 0.0 { a } else { a * (-1.0) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
