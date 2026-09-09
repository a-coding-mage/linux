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
 */

// Dependencies: hw_shared.h, dc_hw_types.h, fixed31_32.h, sspl/dc_spl_types.h

pub const CSC_TEMPERATURE_MATRIX_SIZE: usize = 12;

// External types supplied by the included headers are referenced below.
pub struct bit_depth_reduction_params;

#[repr(C)]
pub struct transform {
    pub funcs: *const transform_funcs,
    pub ctx: *mut dc_context,
    pub inst: i32,
    pub caps: *mut dpp_caps,
    pub regamma_params: pwl_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum colorimetry { COLORIMETRY_NO_DATA = 0, COLORIMETRY_ITU601 = 1, COLORIMETRY_ITU709 = 2, COLORIMETRY_EXTENDED = 3 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum colorimetry_ext { COLORIMETRYEX_XVYCC601 = 0, COLORIMETRYEX_XVYCC709 = 1, COLORIMETRYEX_SYCC601 = 2, COLORIMETRYEX_ADOBEYCC601 = 3, COLORIMETRYEX_ADOBERGB = 4, COLORIMETRYEX_BT2020YCC = 5, COLORIMETRYEX_BT2020RGBYCBCR = 6, COLORIMETRYEX_RESERVED = 7 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum active_format_info { ACTIVE_FORMAT_NO_DATA = 0, ACTIVE_FORMAT_VALID = 1 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum active_format_aspect_ratio { ACTIVE_FORMAT_ASPECT_RATIO_SAME_AS_PICTURE = 8, ACTIVE_FORMAT_ASPECT_RATIO_4_3 = 9, ACTIVE_FORMAT_ASPECT_RATIO_16_9 = 0xA, ACTIVE_FORMAT_ASPECT_RATIO_14_9 = 0xB }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum bar_info { BAR_INFO_NOT_VALID = 0, BAR_INFO_VERTICAL_VALID = 1, BAR_INFO_HORIZONTAL_VALID = 2, BAR_INFO_BOTH_VALID = 3 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum picture_scaling { PICTURE_SCALING_UNIFORM = 0, PICTURE_SCALING_HORIZONTAL = 1, PICTURE_SCALING_VERTICAL = 2, PICTURE_SCALING_BOTH = 3 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum rgb_quantization_range { RGB_QUANTIZATION_DEFAULT_RANGE = 0, RGB_QUANTIZATION_LIMITED_RANGE = 1, RGB_QUANTIZATION_FULL_RANGE = 2, RGB_QUANTIZATION_RESERVED = 3 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum yyc_quantization_range { YYC_QUANTIZATION_LIMITED_RANGE = 0, YYC_QUANTIZATION_FULL_RANGE = 1, YYC_QUANTIZATION_RESERVED2 = 2, YYC_QUANTIZATION_RESERVED3 = 3 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum graphics_gamut_adjust_type { GRAPHICS_GAMUT_ADJUST_TYPE_BYPASS = 0, GRAPHICS_GAMUT_ADJUST_TYPE_HW = 1, GRAPHICS_GAMUT_ADJUST_TYPE_SW = 2 }

#[repr(C)]
pub struct xfm_grph_csc_adjustment { pub temperature_matrix: [fixed31_32; CSC_TEMPERATURE_MATRIX_SIZE], pub gamut_adjust_type: graphics_gamut_adjust_type }
#[repr(C)]
pub struct overscan_info { pub left: i32, pub right: i32, pub top: i32, pub bottom: i32 }
#[repr(C)]
pub struct scaling_ratios { pub horz: fixed31_32, pub vert: fixed31_32, pub horz_c: fixed31_32, pub vert_c: fixed31_32 }
#[repr(C)]
pub struct sharpness_adj { pub horz: i32, pub vert: i32 }
#[repr(C)]
pub struct line_buffer_params { pub alpha_en: bool, pub pixel_expan_mode: bool, pub interleave_en: bool, pub dynamic_pixel_depth: i32, pub depth: lb_pixel_depth }
#[repr(C)]
pub struct scl_inits { pub h: fixed31_32, pub h_c: fixed31_32, pub v: fixed31_32, pub v_c: fixed31_32 }
#[repr(C)]
pub struct scaler_data {
    pub h_active: i32, pub v_active: i32, pub taps: scaling_taps, pub viewport: rect, pub viewport_c: rect, pub recout: rect,
    pub ratios: scaling_ratios, pub inits: scl_inits, pub sharpness: sharpness_adj, pub format: dc_pixel_format,
    pub lb_params: line_buffer_params, // Below struct holds the scaler values to program hw registers
    pub dscl_prog_data: dscl_prog_data, pub upsp: upsp_mode,
}

#[repr(C)]
pub struct transform_funcs {
    pub transform_reset: Option<unsafe extern "C" fn(*mut transform)>,
    pub transform_set_scaler: Option<unsafe extern "C" fn(*mut transform, *const scaler_data)>,
    pub transform_set_pixel_storage_depth: Option<unsafe extern "C" fn(*mut transform, lb_pixel_depth, *const bit_depth_reduction_params)>,
    pub transform_get_optimal_number_of_taps: Option<unsafe extern "C" fn(*mut transform, *mut scaler_data, *const scaling_taps) -> bool>,
    pub transform_set_gamut_remap: Option<unsafe extern "C" fn(*mut transform, *const xfm_grph_csc_adjustment)>,
    pub opp_set_csc_default: Option<unsafe extern "C" fn(*mut transform, *const default_adjustment)>,
    pub opp_set_csc_adjustment: Option<unsafe extern "C" fn(*mut transform, *const out_csc_color_matrix)>,
    pub opp_power_on_regamma_lut: Option<unsafe extern "C" fn(*mut transform, bool)>,
    pub opp_program_regamma_lut: Option<unsafe extern "C" fn(*mut transform, *const pwl_result_data, u32)>,
    pub opp_configure_regamma_lut: Option<unsafe extern "C" fn(*mut transform, bool)>,
    pub opp_program_regamma_lutb_settings: Option<unsafe extern "C" fn(*mut transform, *const pwl_params)>,
    pub opp_program_regamma_luta_settings: Option<unsafe extern "C" fn(*mut transform, *const pwl_params)>,
    pub opp_program_regamma_pwl: Option<unsafe extern "C" fn(*mut transform, *const pwl_params)>,
    pub opp_set_regamma_mode: Option<unsafe extern "C" fn(*mut transform, opp_regamma)>,
    pub ipp_set_degamma: Option<unsafe extern "C" fn(*mut transform, ipp_degamma_mode)>,
    pub ipp_program_input_lut: Option<unsafe extern "C" fn(*mut transform, *const dc_gamma)>,
    pub ipp_program_degamma_pwl: Option<unsafe extern "C" fn(*mut transform, *const pwl_params)>,
    pub ipp_setup: Option<unsafe extern "C" fn(*mut transform, surface_pixel_format, expansion_mode, dc_csc_transform, dc_color_space)>,
    pub ipp_full_bypass: Option<unsafe extern "C" fn(*mut transform)>,
    pub set_cursor_attributes: Option<unsafe extern "C" fn(*mut transform, *const dc_cursor_attributes)>,
}

extern "C" {
    pub fn get_filter_2tap_16p() -> *const u16;
    pub fn get_filter_2tap_64p() -> *const u16;
    pub fn get_filter_3tap_16p(ratio: fixed31_32) -> *const u16;
    pub fn get_filter_3tap_64p(ratio: fixed31_32) -> *const u16;
    pub fn get_filter_4tap_16p(ratio: fixed31_32) -> *const u16;
    pub fn get_filter_4tap_64p(ratio: fixed31_32) -> *const u16;
    pub fn get_filter_5tap_64p(ratio: fixed31_32) -> *const u16;
    pub fn get_filter_6tap_64p(ratio: fixed31_32) -> *const u16;
    pub fn get_filter_7tap_64p(ratio: fixed31_32) -> *const u16;
    pub fn get_filter_8tap_64p(ratio: fixed31_32) -> *const u16;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dscl_data_processing_format { DSCL_DATA_PRCESSING_FIXED_FORMAT = 0, DSCL_DATA_PRCESSING_FLOAT_FORMAT = 1 }

#[repr(C)]
pub struct dpp_caps {
    pub dscl_data_proc_format: dscl_data_processing_format,
    pub max_lb_partitions: u32,
    pub dscl_calc_lb_num_partitions: Option<unsafe extern "C" fn(*const scaler_data, lb_memory_config, *mut i32, *mut i32)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
