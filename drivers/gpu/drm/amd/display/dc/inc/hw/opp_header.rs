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
 */

/* External declarations from hw_shared.h, dc_hw_types.h, transform.h, and mpc.h
 * are intentionally referenced but not defined here. */

#[repr(C)]
#[derive(Copy, Clone)]
pub enum clamping_range {
    CLAMPING_FULL_RANGE = 0,
    CLAMPING_LIMITED_RANGE_8BPC,
    CLAMPING_LIMITED_RANGE_10BPC,
    CLAMPING_LIMITED_RANGE_12BPC,
    CLAMPING_LIMITED_RANGE_PROGRAMMABLE,
}

#[repr(C)]
pub struct clamping_and_pixel_encoding_params {
    pub pixel_encoding: dc_pixel_encoding,
    pub clamping_level: clamping_range,
    pub c_depth: dc_color_depth,
}

#[repr(C)]
pub struct bit_depth_reduction_params {
    pub flags: bit_depth_reduction_flags,
    pub r_seed_value: u32,
    pub b_seed_value: u32,
    pub g_seed_value: u32,
    pub pixel_encoding: dc_pixel_encoding,
}

#[repr(C)]
pub struct bit_depth_reduction_flags {
    pub TRUNCATE_ENABLED: u32,
    pub TRUNCATE_DEPTH: u32,
    pub TRUNCATE_MODE: u32,
    pub SPATIAL_DITHER_ENABLED: u32,
    pub SPATIAL_DITHER_DEPTH: u32,
    pub SPATIAL_DITHER_MODE: u32,
    pub RGB_RANDOM: u32,
    pub FRAME_RANDOM: u32,
    pub HIGHPASS_RANDOM: u32,
    pub FRAME_MODULATION_ENABLED: u32,
    pub FRAME_MODULATION_DEPTH: u32,
    pub TEMPORAL_LEVEL: u32,
    pub FRC25: u32,
    pub FRC50: u32,
    pub FRC75: u32,
}

#[repr(C)]
pub enum wide_gamut_regamma_mode {
    WIDE_GAMUT_REGAMMA_MODE_GRAPHICS_BYPASS,
    WIDE_GAMUT_REGAMMA_MODE_GRAPHICS_SRGB24,
    WIDE_GAMUT_REGAMMA_MODE_GRAPHICS_XYYCC22,
    WIDE_GAMUT_REGAMMA_MODE_GRAPHICS_MATRIX_A,
    WIDE_GAMUT_REGAMMA_MODE_GRAPHICS_MATRIX_B,
    WIDE_GAMUT_REGAMMA_MODE_OVL_BYPASS,
    WIDE_GAMUT_REGAMMA_MODE_OVL_SRGB24,
    WIDE_GAMUT_REGAMMA_MODE_OVL_XYYCC22,
    WIDE_GAMUT_REGAMMA_MODE_OVL_MATRIX_A,
    WIDE_GAMUT_REGAMMA_MODE_OVL_MATRIX_B,
}

#[repr(C)]
pub struct gamma_pixel { pub r: fixed31_32, pub g: fixed31_32, pub b: fixed31_32 }

#[repr(C)]
pub enum channel_name { CHANNEL_NAME_RED, CHANNEL_NAME_GREEN, CHANNEL_NAME_BLUE }

#[repr(C)]
pub struct custom_float_format { pub mantissa_bits: u32, pub exponenta_bits: u32, pub sign: bool }

#[repr(C)]
pub struct custom_float_value { pub mantissa: u32, pub exponenta: u32, pub value: u32, pub negative: bool }

#[repr(C)]
pub struct hw_x_point {
    pub custom_float_x: u32,
    pub x: fixed31_32,
    pub regamma_y_red: fixed31_32,
    pub regamma_y_green: fixed31_32,
    pub regamma_y_blue: fixed31_32,
}

#[repr(C)]
pub struct pwl_float_data_ex {
    pub r: fixed31_32, pub g: fixed31_32, pub b: fixed31_32,
    pub delta_r: fixed31_32, pub delta_g: fixed31_32, pub delta_b: fixed31_32,
}

#[repr(C)]
pub enum hw_point_position { HW_POINT_POSITION_MIDDLE, HW_POINT_POSITION_LEFT, HW_POINT_POSITION_RIGHT }

#[repr(C)]
pub struct gamma_point { pub left_index: i32, pub right_index: i32, pub pos: hw_point_position, pub coeff: fixed31_32 }

#[repr(C)]
pub struct pixel_gamma_point { pub r: gamma_point, pub g: gamma_point, pub b: gamma_point }

#[repr(C)]
pub struct gamma_coefficients {
    pub a0: [fixed31_32; 3], pub a1: [fixed31_32; 3], pub a2: [fixed31_32; 3], pub a3: [fixed31_32; 3],
    pub user_gamma: [fixed31_32; 3], pub user_contrast: fixed31_32, pub user_brightness: fixed31_32,
}

#[repr(C)]
pub struct pwl_float_data { pub r: fixed31_32, pub g: fixed31_32, pub b: fixed31_32 }

#[repr(C)]
pub struct mpc_tree_cfg { pub num_pipes: i32, pub dpp: [i32; MAX_PIPES], pub mpcc: [i32; MAX_PIPES] }

#[repr(C)]
pub struct output_pixel_processor {
    pub ctx: *mut dc_context,
    pub inst: u32,
    pub regamma_params: pwl_params,
    pub mpc_tree_params: mpc_tree,
    pub mpcc_disconnect_pending: [bool; MAX_PIPES],
    pub funcs: *const opp_funcs,
    pub dyn_expansion: u32,
}

#[repr(C)]
pub enum fmt_stereo_action { FMT_STEREO_ACTION_ENABLE = 0, FMT_STEREO_ACTION_DISABLE, FMT_STEREO_ACTION_UPDATE_POLARITY }

#[repr(C)]
pub struct opp_grph_csc_adjustment {
    pub c_space: dc_color_space, pub color_depth: dc_color_depth, pub csc_adjust_type: graphics_csc_adjust_type,
    pub adjust_divider: i32, pub grph_cont: i32, pub grph_sat: i32, pub grph_bright: i32, pub grph_hue: i32,
}

#[repr(C)]
pub struct hw_adjustment_range { pub hw_default: i32, pub min: i32, pub max: i32, pub step: i32, pub divider: u32 }

#[repr(C)]
pub enum ovl_csc_adjust_item { OVERLAY_BRIGHTNESS = 0, OVERLAY_GAMMA, OVERLAY_CONTRAST, OVERLAY_SATURATION, OVERLAY_HUE, OVERLAY_ALPHA, OVERLAY_ALPHA_PER_PIX, OVERLAY_COLOR_TEMPERATURE }

#[repr(C)]
pub enum oppbuf_display_segmentation { OPPBUF_DISPLAY_SEGMENTATION_1_SEGMENT = 0, OPPBUF_DISPLAY_SEGMENTATION_2_SEGMENT = 1, OPPBUF_DISPLAY_SEGMENTATION_4_SEGMENT = 2, OPPBUF_DISPLAY_SEGMENTATION_4_SEGMENT_SPLIT_LEFT = 3, OPPBUF_DISPLAY_SEGMENTATION_4_SEGMENT_SPLIT_RIGHT = 4 }

#[repr(C)]
pub struct oppbuf_params { pub active_width: u32, pub mso_segmentation: oppbuf_display_segmentation, pub mso_overlap_pixel_num: u32, pub pixel_repetition: u32, pub num_segment_padded_pixels: u32 }

#[repr(C)]
pub struct dcn_opp_reg_state { pub dpg_control: u32, pub fmt_control: u32, pub oppbuf_control: u32, pub opp_pipe_control: u32, pub opp_pipe_crc_control: u32, pub opp_abm_control: u32, pub dscrm_dsc_forward_config: u32 }

#[repr(C)]
pub struct opp_funcs {
    pub opp_program_fmt: Option<unsafe extern "C" fn(*mut output_pixel_processor, *mut bit_depth_reduction_params, *mut clamping_and_pixel_encoding_params)>,
    pub opp_set_dyn_expansion: Option<unsafe extern "C" fn(*mut output_pixel_processor, dc_color_space, dc_color_depth, signal_type)>,
    pub opp_program_bit_depth_reduction: Option<unsafe extern "C" fn(*mut output_pixel_processor, *const bit_depth_reduction_params)>,
    pub opp_get_underlay_adjustment_range: Option<unsafe extern "C" fn(*mut output_pixel_processor, ovl_csc_adjust_item, *mut hw_adjustment_range)>,
    pub opp_destroy: Option<unsafe extern "C" fn(*mut *mut output_pixel_processor)>,
    pub opp_program_stereo: Option<unsafe extern "C" fn(*mut output_pixel_processor, bool, *const dc_crtc_timing)>,
    pub opp_pipe_clock_control: Option<unsafe extern "C" fn(*mut output_pixel_processor, bool)>,
    pub opp_set_disp_pattern_generator: Option<unsafe extern "C" fn(*mut output_pixel_processor, controller_dp_test_pattern, controller_dp_color_space, dc_color_depth, *const tg_color, i32, i32, i32)>,
    pub opp_program_dpg_dimensions: Option<unsafe extern "C" fn(*mut output_pixel_processor, u32, u32)>,
    pub dpg_is_blanked: Option<unsafe extern "C" fn(*mut output_pixel_processor) -> bool>,
    pub dpg_is_pending: Option<unsafe extern "C" fn(*mut output_pixel_processor) -> bool>,
    pub opp_dpg_set_blank_color: Option<unsafe extern "C" fn(*mut output_pixel_processor, *const tg_color)>,
    pub opp_program_left_edge_extra_pixel: Option<unsafe extern "C" fn(*mut output_pixel_processor, dc_pixel_encoding, bool)>,
    pub opp_get_left_edge_extra_pixel_count: Option<unsafe extern "C" fn(*mut output_pixel_processor, dc_pixel_encoding, bool) -> u32>,
    pub opp_read_reg_state: Option<unsafe extern "C" fn(*mut output_pixel_processor, *mut dcn_opp_reg_state)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
