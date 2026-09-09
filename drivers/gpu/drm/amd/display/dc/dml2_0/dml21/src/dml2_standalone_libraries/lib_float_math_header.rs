// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

extern "C" {
    pub fn math_mod(arg1: f64, arg2: f64) -> f64;
    pub fn math_min2(arg1: f64, arg2: f64) -> f64;
    pub fn math_max2(arg1: f64, arg2: f64) -> f64;
    pub fn math_floor2(arg: f64, significance: f64) -> f64;
    pub fn math_floor(arg: f64) -> f64;
    pub fn math_ceil(arg: f64) -> f64;
    pub fn math_ceil2(arg: f64, significance: f64) -> f64;
    pub fn math_max3(v1: f64, v2: f64, v3: f64) -> f64;
    pub fn math_max4(v1: f64, v2: f64, v3: f64, v4: f64) -> f64;
    pub fn math_max5(v1: f64, v2: f64, v3: f64, v4: f64, v5: f64) -> f64;
    pub fn math_pow(a: f32, exp: f32) -> f32;
    pub fn math_fabs(a: f64) -> f64;
    pub fn math_log(a: f32, b: f32) -> f32;
    pub fn math_log2(a: f32) -> f32;
    pub fn math_log2_approx(a: u32) -> u32;
    pub fn math_round(a: f64) -> f64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
