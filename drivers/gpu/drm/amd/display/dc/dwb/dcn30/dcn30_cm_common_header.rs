/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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

// Dependency: dcn10/dcn10_cm_common.h
// The C macro TF_HELPER_REG_FIELD_LIST_DCN3(type) expands the external
// TF_HELPER_REG_FIELD_LIST(type) fields followed by these two fields.

#[repr(C)]
pub struct DCN3_xfer_func_shift {
    // TF_HELPER_REG_FIELD_LIST(u8) expansion supplied by the dependency.
    pub field_region_start_base: u8,
    pub field_offset: u8,
}

#[repr(C)]
pub struct DCN3_xfer_func_mask {
    // TF_HELPER_REG_FIELD_LIST(u32) expansion supplied by the dependency.
    pub field_region_start_base: u32,
    pub field_offset: u32,
}

#[repr(C)]
pub struct dcn3_xfer_func_reg {
    pub shifts: DCN3_xfer_func_shift,
    pub masks: DCN3_xfer_func_mask,

    // TF_HELPER_REG_LIST expansion supplied by the dependency.
    pub offset_b: u32,
    pub offset_g: u32,
    pub offset_r: u32,
    pub start_base_cntl_b: u32,
    pub start_base_cntl_g: u32,
    pub start_base_cntl_r: u32,
}

extern "C" {
    pub fn cm_helper_program_gamcor_xfer_func(
        ctx: *mut dc_context,
        params: *const pwl_params,
        reg: *const dcn3_xfer_func_reg,
    );

    pub fn cm3_helper_translate_curve_to_hw_format(
        ctx: *mut dc_context,
        output_tf: *const dc_transfer_func,
        lut_params: *mut pwl_params,
        fixpoint: bool,
    ) -> bool;

    pub fn cm3_helper_translate_curve_to_degamma_hw_format(
        output_tf: *const dc_transfer_func,
        lut_params: *mut pwl_params,
    ) -> bool;

    pub fn cm3_helper_convert_to_custom_float(
        rgb_resulted: *mut pwl_result_data,
        corner_points: *mut curve_points3,
        hw_points_num: u32,
        fixpoint: bool,
    ) -> bool;

    pub fn is_rgb_equal(rgb: *const pwl_result_data, num: u32) -> bool;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
