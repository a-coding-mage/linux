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

unsafe extern "C" {
    pub fn dcn_bw_mod(arg1: f32, arg2: f32) -> f32;
    pub fn dcn_bw_min2(arg1: f32, arg2: f32) -> f32;
    pub fn dcn_bw_max(arg1: u32, arg2: u32) -> u32;
    pub fn dcn_bw_max2(arg1: f32, arg2: f32) -> f32;
    pub fn dcn_bw_floor2(arg: f32, significance: f32) -> f32;
    pub fn dcn_bw_floor(arg: f32) -> f32;
    pub fn dcn_bw_ceil2(arg: f32, significance: f32) -> f32;
    pub fn dcn_bw_ceil(arg: f32) -> f32;
    pub fn dcn_bw_max3(v1: f32, v2: f32, v3: f32) -> f32;
    pub fn dcn_bw_max5(v1: f32, v2: f32, v3: f32, v4: f32, v5: f32) -> f32;
    pub fn dcn_bw_pow(a: f32, exp: f32) -> f32;
    pub fn dcn_bw_log(a: f32, b: f32) -> f32;
    pub fn dcn_bw_fabs(a: f64) -> f64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
