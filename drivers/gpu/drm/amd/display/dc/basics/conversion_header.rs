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

// Dependency supplied by the surrounding translation unit:
// include/fixed31_32.h

pub const DIVIDER: i32 = 10000;

pub const S2D13_MIN: i32 = -4 * DIVIDER;
pub const S2D13_MAX: i32 = 39999;

pub const S3D12_MIN: i32 = -8 * DIVIDER;
pub const S3D12_MAX: i32 = 79998;

extern "C" {
    pub fn fixed_point_to_int_frac(
        arg: fixed31_32,
        integer_bits: u8,
        fractional_bits: u8,
    ) -> u16;

    pub fn convert_float_matrix(
        matrix: *mut u16,
        flt: *const fixed31_32,
        format: cm_gamut_coef_format,
        buffer_size: u32,
    );

    pub fn reduce_fraction(
        num: u32,
        den: u32,
        out_num: *mut u32,
        out_den: *mut u32,
    );

    pub fn convert_hw_matrix(
        matrix: *mut fixed31_32,
        reg: *mut u16,
        format: cm_gamut_coef_format,
        buffer_size: u32,
    );

    // Supplied by the included integer-logarithm dependency.
    pub fn ilog2(num: u32) -> u32;
}

#[inline]
pub unsafe fn log_2(num: u32) -> u32 {
    ilog2(num)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
