/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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

// Dependency supplied by the surrounding translation unit: color_table.h

pub enum dc_transfer_func {}
pub enum dc_gamma {}
pub enum dc_transfer_func_distributed_points {}
pub enum dc_rgb_fixed {}
pub enum dc_color_caps {}
pub enum dal_logger {}
pub enum dc_transfer_func_predefined {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regamma_flags_bits {
    // C bit-fields occupying one unsigned int; use the corresponding masks below.
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union regamma_flags {
    pub raw: u32,
    pub bits: regamma_flags_bits,
}

pub const REGAMMA_FLAG_GAMMA_RAMP_ARRAY: u32 = 1 << 0;
pub const REGAMMA_FLAG_GAMMA_FROM_EDID: u32 = 1 << 1;
pub const REGAMMA_FLAG_GAMMA_FROM_EDID_EX: u32 = 1 << 2;
pub const REGAMMA_FLAG_GAMMA_FROM_USER: u32 = 1 << 3;
pub const REGAMMA_FLAG_COEFF_FROM_USER: u32 = 1 << 4;
pub const REGAMMA_FLAG_COEFF_FROM_EDID: u32 = 1 << 5;
pub const REGAMMA_FLAG_APPLY_DEGAMMA: u32 = 1 << 6;
pub const REGAMMA_FLAG_GAMMA_PREDEFINED_SRGB: u32 = 1 << 7;
pub const REGAMMA_FLAG_GAMMA_PREDEFINED_PQ: u32 = 1 << 8;
pub const REGAMMA_FLAG_GAMMA_PREDEFINED_PQ2084_INTERIM: u32 = 1 << 9;
pub const REGAMMA_FLAG_GAMMA_PREDEFINED_36: u32 = 1 << 10;
pub const REGAMMA_FLAG_GAMMA_PREDEFINED_RESET: u32 = 1 << 11;

#[repr(C)]
pub struct regamma_ramp {
    pub gamma: [u16; 256 * 3], // gamma ramp packed in same way as OS windows, r, g & b
}

#[repr(C)]
pub struct regamma_coeff {
    pub gamma: [i32; 3],
    pub A0: [i32; 3],
    pub A1: [i32; 3],
    pub A2: [i32; 3],
    pub A3: [i32; 3],
}

#[repr(C)]
pub union regamma_lut_data {
    pub ramp: regamma_ramp,
    pub coeff: regamma_coeff,
}

#[repr(C)]
pub struct regamma_lut {
    pub flags: regamma_flags,
    pub data: regamma_lut_data,
}

#[repr(C)]
pub struct hdr_tm_params {
    pub sdr_white_level: u32,
    pub min_content: u32, // luminance in 1/10000 nits
    pub max_content: u32, // luminance in nits
    pub min_display: u32, // luminance in 1/10000 nits
    pub max_display: u32, // luminance in nits
    pub skip_tm: u32, // skip tm
}

// fixed31_32 and NUM_PTS_IN_REGION are supplied by color_table.h.
#[repr(C)]
pub struct calculate_buffer {
    pub buffer_index: i32,
    pub buffer: [fixed31_32; NUM_PTS_IN_REGION],
    pub gamma_of_2: fixed31_32,
}

#[repr(C)]
pub struct translate_from_linear_space_args {
    pub arg: fixed31_32,
    pub a0: fixed31_32,
    pub a1: fixed31_32,
    pub a2: fixed31_32,
    pub a3: fixed31_32,
    pub gamma: fixed31_32,
    pub cal_buffer: *mut calculate_buffer,
}

extern "C" {
    pub fn setup_x_points_distribution();
    pub fn log_x_points_distribution(logger: *mut dal_logger);
    pub fn precompute_pq();
    pub fn precompute_de_pq();

    pub fn mod_color_calculate_regamma_params(
        output_tf: *mut dc_transfer_func,
        ramp: *const dc_gamma,
        mapUserRamp: bool,
        canRomBeUsed: bool,
        fs_params: *const hdr_tm_params,
        cal_buffer: *mut calculate_buffer,
    ) -> bool;

    pub fn mod_color_calculate_degamma_params(
        dc_caps: *mut dc_color_caps,
        output_tf: *mut dc_transfer_func,
        ramp: *const dc_gamma,
        mapUserRamp: bool,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
