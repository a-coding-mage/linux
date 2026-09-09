/*
 * Copyright 2012-2026 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding translation unit: transform.h, cursor_reg_cache.h

#[repr(C)]
#[derive(Copy, Clone)]
pub struct defer_reg_writes_bits {
    pub disable_blnd_lut: bool,
    pub disable_3dlut: bool,
    pub disable_shaper: bool,
    pub disable_gamcor: bool,
    pub disable_dscl: bool,
}

#[repr(C)]
pub union defer_reg_writes {
    pub bits: defer_reg_writes_bits,
    pub raw: u32,
}

#[repr(C)]
pub struct dpp {
    pub funcs: *const dpp_funcs,
    pub ctx: *mut dc_context,
    /// inst stands for "instance," and it is an id number that references a specific DPP.
    pub inst: ::core::ffi::c_int,
    pub caps: *mut dpp_caps,
    pub regamma_params: pwl_params,
    pub degamma_params: pwl_params,
    pub cur_attr: dpp_cursor_attributes,
    pub deferred_reg_writes: defer_reg_writes,
    pub shaper_params: pwl_params,
    pub cm_bypass_mode: bool,
    pub cursor_offload: bool,
    pub pos: cursor_position_cache_dpp,
    pub att: cursor_attribute_cache_dpp,
}

#[repr(C)]
pub struct dpp_input_csc_matrix {
    pub color_space: dc_color_space,
    pub regval: [u16; 12],
}

pub static dpp_input_csc_matrix_table: [dpp_input_csc_matrix; 9] = [
    dpp_input_csc_matrix { color_space: COLOR_SPACE_SRGB, regval: [0x2000,0,0,0, 0,0x2000,0,0, 0,0,0x2000,0] },
    dpp_input_csc_matrix { color_space: COLOR_SPACE_SRGB_LIMITED, regval: [0x2000,0,0,0, 0,0x2000,0,0, 0,0,0x2000,0] },
    dpp_input_csc_matrix { color_space: COLOR_SPACE_YCBCR601, regval: [0x2cdd,0x2000,0,0xe991, 0xe926,0x2000,0xf4fd,0x10ef, 0,0x2000,0x38b4,0xe3a6] },
    dpp_input_csc_matrix { color_space: COLOR_SPACE_YCBCR601_LIMITED, regval: [0x3353,0x2568,0,0xe400, 0xe5dc,0x2568,0xf367,0x1108, 0,0x2568,0x40de,0xdd3a] },
    dpp_input_csc_matrix { color_space: COLOR_SPACE_YCBCR709, regval: [0x3265,0x2000,0,0xe6ce, 0xf105,0x2000,0xfa01,0xa7d, 0,0x2000,0x3b61,0xe24f] },
    dpp_input_csc_matrix { color_space: COLOR_SPACE_YCBCR709_LIMITED, regval: [0x39a6,0x2568,0,0xe0d6, 0xeedd,0x2568,0xf925,0x9a8, 0,0x2568,0x43ee,0xdbb2] },
    dpp_input_csc_matrix { color_space: COLOR_SPACE_2020_YCBCR_FULL, regval: [0x2f30,0x2000,0,0xe869, 0xedb7,0x2000,0xfabc,0xbc6, 0,0x2000,0x3c34,0xe1e6] },
    dpp_input_csc_matrix { color_space: COLOR_SPACE_2020_YCBCR_LIMITED, regval: [0x35b9,0x2543,0,0xe2b2, 0xeb2f,0x2543,0xfa01,0x0b1f, 0,0x2543,0x4489,0xdb42] },
    dpp_input_csc_matrix { color_space: COLOR_SPACE_2020_RGB_LIMITEDRANGE, regval: [0x35e0,0x255f,0,0xe2b3, 0xeb20,0x255f,0xf9fd,0xb1e, 0,0x255f,0x44bd,0xdb43] },
];

#[repr(C)]
pub struct dpp_grph_csc_adjustment {
    pub temperature_matrix: [fixed31_32; CSC_TEMPERATURE_MATRIX_SIZE],
    pub gamut_adjust_type: graphics_gamut_adjust_type,
}

#[repr(C)]
pub struct cnv_color_keyer_params {
    pub color_keyer_en: ::core::ffi::c_int, pub color_keyer_mode: ::core::ffi::c_int,
    pub color_keyer_alpha_low: ::core::ffi::c_int, pub color_keyer_alpha_high: ::core::ffi::c_int,
    pub color_keyer_red_low: ::core::ffi::c_int, pub color_keyer_red_high: ::core::ffi::c_int,
    pub color_keyer_green_low: ::core::ffi::c_int, pub color_keyer_green_high: ::core::ffi::c_int,
    pub color_keyer_blue_low: ::core::ffi::c_int, pub color_keyer_blue_high: ::core::ffi::c_int,
}

/// Set the 8bit alpha values based on the 2 bit alpha.
#[repr(C)]
pub struct cnv_alpha_2bit_lut { pub lut0: ::core::ffi::c_int, pub lut1: ::core::ffi::c_int, pub lut2: ::core::ffi::c_int, pub lut3: ::core::ffi::c_int }

#[repr(C)]
pub struct dcn_dpp_state {
    pub is_enabled: u32, pub igam_lut_mode: u32, pub igam_input_format: u32, pub dgam_lut_mode: u32, pub rgam_lut_mode: u32,
    // gamut_remap data for dcn10_get_cm_states()
    pub gamut_remap_mode: u32, pub gamut_remap_c11_c12: u32, pub gamut_remap_c13_c14: u32, pub gamut_remap_c21_c22: u32,
    pub gamut_remap_c23_c24: u32, pub gamut_remap_c31_c32: u32, pub gamut_remap_c33_c34: u32,
    // gamut_remap data for dcn*_log_color_state()
    pub gamut_remap: dpp_grph_csc_adjustment, pub shaper_lut_mode: u32, pub lut3d_mode: u32, pub lut3d_bit_depth: u32,
    pub lut3d_size: u32, pub blnd_lut_mode: u32, pub pre_dgam_mode: u32, pub pre_dgam_select: u32, pub gamcor_mode: u32,
}

#[repr(C)]
pub struct dcn_dpp_reg_state { pub recout_start: u32, pub recout_size: u32, pub scl_horz_filter_scale_ratio: u32, pub scl_vert_filter_scale_ratio: u32, pub scl_mode: u32, pub cm_control: u32, pub dpp_control: u32, pub dscl_control: u32, pub obuf_control: u32, pub mpc_size: u32 }

#[repr(C)]
pub struct CM_bias_params { pub cm_bias_cr_r: u32, pub cm_bias_y_g: u32, pub cm_bias_cb_b: u32, pub cm_bias_format: u32 }

// Function-pointer declarations from the C vtable. Exact external definitions are supplied by dependencies.
#[repr(C)]
pub struct dpp_funcs {
    pub dpp_program_gamcor_lut: Option<unsafe extern "C" fn(*mut dpp, *const pwl_params) -> bool>,
    pub dpp_set_pre_degam: Option<unsafe extern "C" fn(*mut dpp, dc_transfer_func_predefined)>,
    pub dpp_program_cm_dealpha: Option<unsafe extern "C" fn(*mut dpp, u32, u32)>,
    pub dpp_program_cm_bias: Option<unsafe extern "C" fn(*mut dpp, *mut CM_bias_params)>,
    pub dpp_read_state: Option<unsafe extern "C" fn(*mut dpp, *mut dcn_dpp_state)>,
    pub dpp_read_reg_state: Option<unsafe extern "C" fn(*mut dpp, *mut dcn_dpp_reg_state)>,
    pub dpp_reset: Option<unsafe extern "C" fn(*mut dpp)>,
    pub dpp_set_scaler: Option<unsafe extern "C" fn(*mut dpp, *const scaler_data)>,
    pub dpp_set_pixel_storage_depth: Option<unsafe extern "C" fn(*mut dpp, lb_pixel_depth, *const bit_depth_reduction_params)>,
    pub dpp_get_optimal_number_of_taps: Option<unsafe extern "C" fn(*mut dpp, *mut scaler_data, *const scaling_taps) -> bool>,
    pub dpp_set_gamut_remap: Option<unsafe extern "C" fn(*mut dpp, *const dpp_grph_csc_adjustment)>,
    pub dpp_set_csc_default: Option<unsafe extern "C" fn(*mut dpp, dc_color_space)>,
    pub dpp_set_csc_adjustment: Option<unsafe extern "C" fn(*mut dpp, *const u16)>,
    pub dpp_power_on_regamma_lut: Option<unsafe extern "C" fn(*mut dpp, bool)>,
    pub dpp_program_regamma_lut: Option<unsafe extern "C" fn(*mut dpp, *const pwl_result_data, u32)>,
    pub dpp_configure_regamma_lut: Option<unsafe extern "C" fn(*mut dpp, bool)>,
    pub dpp_program_regamma_lutb_settings: Option<unsafe extern "C" fn(*mut dpp, *const pwl_params)>,
    pub dpp_program_regamma_luta_settings: Option<unsafe extern "C" fn(*mut dpp, *const pwl_params)>,
    pub dpp_program_regamma_pwl: Option<unsafe extern "C" fn(*mut dpp, *const pwl_params, opp_regamma)>,
    pub dpp_program_bias_and_scale: Option<unsafe extern "C" fn(*mut dpp, *mut dc_bias_and_scale)>,
    pub dpp_set_degamma: Option<unsafe extern "C" fn(*mut dpp, ipp_degamma_mode)>,
    pub dpp_program_input_lut: Option<unsafe extern "C" fn(*mut dpp, *const dc_gamma)>,
    pub dpp_program_degamma_pwl: Option<unsafe extern "C" fn(*mut dpp, *const pwl_params)>,
    pub dpp_setup: Option<unsafe extern "C" fn(*mut dpp, surface_pixel_format, expansion_mode, dc_csc_transform, dc_color_space, *mut cnv_alpha_2bit_lut)>,
    pub dpp_full_bypass: Option<unsafe extern "C" fn(*mut dpp)>,
    pub set_cursor_attributes: Option<unsafe extern "C" fn(*mut dpp, *mut dc_cursor_attributes)>,
    pub set_cursor_position: Option<unsafe extern "C" fn(*mut dpp, *const dc_cursor_position, *const dc_cursor_mi_param, u32, u32)>,
    pub dpp_set_hdr_multiplier: Option<unsafe extern "C" fn(*mut dpp, u32)>,
    pub set_optional_cursor_attributes: Option<unsafe extern "C" fn(*mut dpp, *mut dpp_cursor_attributes)>,
    pub dpp_dppclk_control: Option<unsafe extern "C" fn(*mut dpp, bool, bool)>,
    pub dpp_deferred_update: Option<unsafe extern "C" fn(*mut dpp)>,
    pub dpp_program_blnd_lut: Option<unsafe extern "C" fn(*mut dpp, *const pwl_params) -> bool>,
    pub dpp_program_shaper_lut: Option<unsafe extern "C" fn(*mut dpp, *const pwl_params) -> bool>,
    pub dpp_program_3dlut: Option<unsafe extern "C" fn(*mut dpp, *const tetrahedral_params) -> bool>,
    pub dpp_cnv_set_alpha_keyer: Option<unsafe extern "C" fn(*mut dpp, *mut cnv_color_keyer_params)>,
    pub dpp_get_gamut_remap: Option<unsafe extern "C" fn(*mut dpp, *mut dpp_grph_csc_adjustment)>,
    pub set_cursor_matrix: Option<unsafe extern "C" fn(*mut dpp, dc_color_space, dc_csc_transform)>,
    pub dpp_force_disable_cursor: Option<unsafe extern "C" fn(*mut dpp)>,
    pub dpp_cm_hist_control: Option<unsafe extern "C" fn(*mut dpp, cm_hist_control, dc_color_space)>,
    pub dpp_cm_hist_read: Option<unsafe extern "C" fn(*mut dpp, *mut cm_hist) -> bool>,
    pub dpp_set_pregam_state: Option<unsafe extern "C" fn(*mut dpp, dc_transfer_func_predefined, dc_scaling_linearity)>,
    pub dpp_program_upsp: Option<unsafe extern "C" fn(*mut dpp, *const dscl_prog_data)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
