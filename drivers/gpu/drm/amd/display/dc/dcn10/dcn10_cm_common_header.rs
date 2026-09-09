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
 */

#[repr(C)]
pub struct xfer_func_shift {
    pub exp_region0_lut_offset: u8,
    pub exp_region0_num_segments: u8,
    pub exp_region1_lut_offset: u8,
    pub exp_region1_num_segments: u8,
    pub field_region_end: u8,
    pub field_region_end_slope: u8,
    pub field_region_end_base: u8,
    pub exp_region_start: u8,
    pub exp_resion_start_segment: u8,
    pub field_region_linear_slope: u8,
}

#[repr(C)]
pub struct xfer_func_mask {
    pub exp_region0_lut_offset: u32,
    pub exp_region0_num_segments: u32,
    pub exp_region1_lut_offset: u32,
    pub exp_region1_num_segments: u32,
    pub field_region_end: u32,
    pub field_region_end_slope: u32,
    pub field_region_end_base: u32,
    pub exp_region_start: u32,
    pub exp_resion_start_segment: u32,
    pub field_region_linear_slope: u32,
}

#[repr(C)]
pub struct xfer_func_reg {
    pub shifts: xfer_func_shift,
    pub masks: xfer_func_mask,
    pub start_cntl_b: u32,
    pub start_cntl_g: u32,
    pub start_cntl_r: u32,
    pub start_slope_cntl_b: u32,
    pub start_slope_cntl_g: u32,
    pub start_slope_cntl_r: u32,
    pub start_end_cntl1_b: u32,
    pub start_end_cntl2_b: u32,
    pub start_end_cntl1_g: u32,
    pub start_end_cntl2_g: u32,
    pub start_end_cntl1_r: u32,
    pub start_end_cntl2_r: u32,
    pub region_start: u32,
    pub region_end: u32,
}

#[repr(C)]
pub struct cm_color_matrix_shift {
    pub csc_c11: u8,
    pub csc_c12: u8,
}

#[repr(C)]
pub struct cm_color_matrix_mask {
    pub csc_c11: u32,
    pub csc_c12: u32,
}

#[repr(C)]
pub struct color_matrices_reg {
    pub shifts: cm_color_matrix_shift,
    pub masks: cm_color_matrix_mask,
    pub csc_c11_c12: u32,
    pub csc_c33_c34: u32,
}

extern "C" {
    pub fn cm_helper_program_color_matrices(
        ctx: *mut dc_context,
        regval: *const u16,
        reg: *const color_matrices_reg,
    );

    pub fn cm_helper_program_xfer_func(
        ctx: *mut dc_context,
        params: *const pwl_params,
        reg: *const xfer_func_reg,
    );

    pub fn cm_helper_convert_to_custom_float(
        rgb_resulted: *mut pwl_result_data,
        corner_points: *mut curve_points3,
        hw_points_num: u32,
        fixpoint: bool,
    ) -> bool;

    pub fn cm_helper_translate_curve_to_hw_format(
        ctx: *mut dc_context,
        output_tf: *const dc_transfer_func,
        lut_params: *mut pwl_params,
        fixpoint: bool,
    ) -> bool;

    pub fn cm_helper_translate_curve_to_degamma_hw_format(
        output_tf: *const dc_transfer_func,
        lut_params: *mut pwl_params,
    ) -> bool;

    pub fn cm_helper_read_color_matrices(
        ctx: *mut dc_context,
        regval: *mut u16,
        reg: *const color_matrices_reg,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
