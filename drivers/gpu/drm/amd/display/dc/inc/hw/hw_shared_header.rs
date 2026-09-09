/*
 * Copyright 2015 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding translation unit:
// os_types.h, fixed31_32.h, dc_hw_types.h

pub const MAX_AUDIOS: usize = 7;
pub const MAX_PIPES: usize = 6;
pub const MAX_PHANTOM_PIPES: usize = MAX_PIPES / 2;
pub const MAX_DPIA: usize = 6;
pub const MAX_CONNECTOR: usize = 6;
pub const MAX_VIRTUAL_LINKS: usize = 4;
pub const MAX_LINKS: usize = MAX_DPIA + MAX_CONNECTOR + MAX_VIRTUAL_LINKS;
pub const MAX_DIG_LINK_ENCODERS: usize = 7;
pub const MAX_DAC_LINK_ENCODERS: usize = 2;
pub const MAX_LINK_ENCODERS: usize = MAX_DIG_LINK_ENCODERS + MAX_DAC_LINK_ENCODERS;
pub const MAX_DWB_PIPES: usize = 1;
pub const MAX_HDMI_FRL_ENCODERS: usize = 2;
pub const MAX_HPO_DP2_ENCODERS: usize = 4;
pub const MAX_HPO_DP2_LINK_ENCODERS: usize = 4;
pub const MAX_TOPOLOGY_SNAPSHOTS: usize = 4;

#[repr(C)] pub struct pipe_topology_line { pub is_phantom_pipe: bool, pub plane_idx: i32, pub slice_idx: i32, pub stream_idx: i32, pub dpp_inst: i32, pub opp_inst: i32, pub tg_inst: i32 }
#[repr(C)] pub struct pipe_topology_snapshot { pub pipe_log_lines: [pipe_topology_line; MAX_PIPES], pub line_count: i32, pub timestamp_us: u64, pub stream_count: i32, pub phantom_stream_count: i32 }
#[repr(C)] pub struct pipe_topology_history { pub snapshots: [pipe_topology_snapshot; MAX_TOPOLOGY_SNAPSHOTS], pub current_snapshot_index: i32 }
#[repr(C)] pub struct gamma_curve { pub offset: u32, pub segments_num: u32 }

#[repr(C)] pub struct curve_points { pub x: fixed31_32, pub y: fixed31_32, pub offset: fixed31_32, pub slope: fixed31_32, pub custom_float_x: u32, pub custom_float_y: u32, pub custom_float_offset: u32, pub custom_float_slope: u32 }
#[repr(C)] pub struct curve_points3 { pub red: curve_points, pub green: curve_points, pub blue: curve_points }
#[repr(C)] pub struct pwl_result_data { pub red: fixed31_32, pub green: fixed31_32, pub blue: fixed31_32, pub delta_red: fixed31_32, pub delta_green: fixed31_32, pub delta_blue: fixed31_32, pub red_reg: u32, pub green_reg: u32, pub blue_reg: u32, pub delta_red_reg: u32, pub delta_green_reg: u32, pub delta_blue_reg: u32 }
#[repr(C)] pub struct dc_rgb { pub red: u32, pub green: u32, pub blue: u32 }
#[repr(C)] pub struct tetrahedral_33x33x33 { pub lut0: [dc_rgb; 8985], pub lut1: [dc_rgb; 8984], pub lut2: [dc_rgb; 8984], pub lut3: [dc_rgb; 8984] }
#[repr(C)] pub struct tetrahedral_17x17x17 { pub lut0: [dc_rgb; 1229], pub lut1: [dc_rgb; 1228], pub lut2: [dc_rgb; 1228], pub lut3: [dc_rgb; 1228] }
#[repr(C)] pub struct tetrahedral_9x9x9 { pub lut0: [dc_rgb; 183], pub lut1: [dc_rgb; 182], pub lut2: [dc_rgb; 182], pub lut3: [dc_rgb; 182] }

#[repr(i32)] pub enum lut_dimension { LUT_DIM_INVALID = 0, LUT_DIM_9 = 9, LUT_DIM_17 = 17, LUT_DIM_33 = 33 }
#[repr(C)] pub union tetrahedral_params_data { pub tetrahedral_17: tetrahedral_17x17x17, pub tetrahedral_9: tetrahedral_9x9x9 }
#[repr(C)] pub struct tetrahedral_params { pub data: tetrahedral_params_data, pub use_tetrahedral_9: bool, pub use_12bits: bool, pub lut_dim: lut_dimension }

#[repr(C)] pub union pwl_params_points { pub arr_points: [curve_points; 2], pub corner_points: [curve_points3; 2] }
#[repr(C)] pub struct pwl_params { pub arr_curve_points: [gamma_curve; 34], pub points: pwl_params_points, pub rgb_resulted: [pwl_result_data; 259], pub hw_points_num: u32 }

#[repr(i32)] pub enum lb_pixel_depth { LB_PIXEL_DEPTH_18BPP = 1, LB_PIXEL_DEPTH_24BPP = 2, LB_PIXEL_DEPTH_30BPP = 4, LB_PIXEL_DEPTH_36BPP = 8 }
#[repr(i32)] pub enum graphics_csc_adjust_type { GRAPHICS_CSC_ADJUST_TYPE_BYPASS = 0, GRAPHICS_CSC_ADJUST_TYPE_HW, GRAPHICS_CSC_ADJUST_TYPE_SW }
#[repr(i32)] pub enum ipp_degamma_mode { IPP_DEGAMMA_MODE_BYPASS, IPP_DEGAMMA_MODE_HW_sRGB, IPP_DEGAMMA_MODE_HW_xvYCC, IPP_DEGAMMA_MODE_USER_PWL }
#[repr(i32)] pub enum gamcor_mode { GAMCOR_MODE_BYPASS, GAMCOR_MODE_RESERVED_1, GAMCOR_MODE_USER_PWL, GAMCOR_MODE_RESERVED_3 }
#[repr(i32)] pub enum ipp_output_format { IPP_OUTPUT_FORMAT_12_BIT_FIX, IPP_OUTPUT_FORMAT_16_BIT_BYPASS, IPP_OUTPUT_FORMAT_FLOAT }
#[repr(i32)] pub enum expansion_mode { EXPANSION_MODE_DYNAMIC, EXPANSION_MODE_ZERO }

#[repr(C)] pub struct default_adjustment { pub lb_color_depth: lb_pixel_depth, pub out_color_space: dc_color_space, pub in_color_space: dc_color_space, pub color_depth: dc_color_depth, pub surface_pixel_format: dc_pixel_format, pub csc_adjust_type: graphics_csc_adjust_type, pub force_hw_default: bool }
#[repr(C)] pub struct out_csc_color_matrix { pub color_space: dc_color_space, pub regval: [u16; 12] }
#[repr(i32)] pub enum gamut_remap_select { GAMUT_REMAP_BYPASS = 0, GAMUT_REMAP_COEFF, GAMUT_REMAP_COMA_COEFF, GAMUT_REMAP_COMB_COEFF }
#[repr(i32)] pub enum opp_regamma { OPP_REGAMMA_BYPASS = 0, OPP_REGAMMA_SRGB, OPP_REGAMMA_XVYCC, OPP_REGAMMA_USER }
#[repr(i32)] pub enum optc_dsc_mode { OPTC_DSC_DISABLED = 0, OPTC_DSC_ENABLED_444 = 1, OPTC_DSC_ENABLED_NATIVE_SUBSAMPLED = 2 }
#[repr(C)] pub struct dc_bias_and_scale { pub scale_red: u32, pub bias_red: u32, pub scale_green: u32, pub bias_green: u32, pub scale_blue: u32, pub bias_blue: u32, pub bias_and_scale_valid: bool }
#[repr(i32)] pub enum test_pattern_dyn_range { TEST_PATTERN_DYN_RANGE_VESA = 0, TEST_PATTERN_DYN_RANGE_CEA }
#[repr(i32)] pub enum test_pattern_mode { TEST_PATTERN_MODE_COLORSQUARES_RGB = 0, TEST_PATTERN_MODE_COLORSQUARES_YCBCR601, TEST_PATTERN_MODE_COLORSQUARES_YCBCR709, TEST_PATTERN_MODE_VERTICALBARS, TEST_PATTERN_MODE_HORIZONTALBARS, TEST_PATTERN_MODE_SINGLERAMP_RGB, TEST_PATTERN_MODE_DUALRAMP_RGB, TEST_PATTERN_MODE_XR_BIAS_RGB }
#[repr(i32)] pub enum test_pattern_color_format { TEST_PATTERN_COLOR_FORMAT_BPC_6 = 0, TEST_PATTERN_COLOR_FORMAT_BPC_8, TEST_PATTERN_COLOR_FORMAT_BPC_10, TEST_PATTERN_COLOR_FORMAT_BPC_12 }
#[repr(i32)] pub enum controller_dp_test_pattern { CONTROLLER_DP_TEST_PATTERN_D102 = 0, CONTROLLER_DP_TEST_PATTERN_SYMBOLERROR, CONTROLLER_DP_TEST_PATTERN_PRBS7, CONTROLLER_DP_TEST_PATTERN_COLORSQUARES, CONTROLLER_DP_TEST_PATTERN_VERTICALBARS, CONTROLLER_DP_TEST_PATTERN_HORIZONTALBARS, CONTROLLER_DP_TEST_PATTERN_COLORRAMP, CONTROLLER_DP_TEST_PATTERN_VIDEOMODE, CONTROLLER_DP_TEST_PATTERN_RESERVED_8, CONTROLLER_DP_TEST_PATTERN_RESERVED_9, CONTROLLER_DP_TEST_PATTERN_RESERVED_A, CONTROLLER_DP_TEST_PATTERN_COLORSQUARES_CEA, CONTROLLER_DP_TEST_PATTERN_SOLID_COLOR }
#[repr(i32)] pub enum controller_dp_color_space { CONTROLLER_DP_COLOR_SPACE_RGB, CONTROLLER_DP_COLOR_SPACE_YCBCR601, CONTROLLER_DP_COLOR_SPACE_YCBCR709, CONTROLLER_DP_COLOR_SPACE_UDEFINED }
#[repr(i32)] pub enum dc_lut_mode { LUT_BYPASS, LUT_RAM_A, LUT_RAM_B }

#[repr(C)] pub struct audio_cea_channels_bits { pub FL: u32, pub FR: u32, pub LFE: u32, pub FC: u32, pub RL_RC: u32, pub RR: u32, pub RC_RLC_FLC: u32, pub RRC_FRC: u32 }
#[repr(C)] pub union audio_cea_channels { pub all: u8, pub channels: audio_cea_channels_bits }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
