/*
 * Copyright 2012-16 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependency: transform.h supplies the referenced external types and helpers.

pub const LB_TOTAL_NUMBER_OF_ENTRIES: u32 = 1712;
pub const LB_BITS_PER_ENTRY: u32 = 144;

#[macro_export]
macro_rules! TO_DCE_TRANSFORM { ($transform:expr) => {
    container_of!($transform, dce_transform, base)
} }

// Register-list and register-field-list macros are retained as token-level
// macros because their SRI/OPP_SF symbols are supplied by the generated
// register definitions used with this header.
#[macro_export]
macro_rules! XFM_SF { ($reg:ident, $field:ident, $post:ident) => {
    $reg ## __ ## $field ## $post
} }

#[repr(C)]
pub struct dce_transform_shift {
    pub OUT_CLAMP_MIN_B_CB: u8, pub OUT_CLAMP_MAX_B_CB: u8,
    pub OUT_CLAMP_MIN_G_Y: u8, pub OUT_CLAMP_MAX_G_Y: u8,
    pub OUT_CLAMP_MIN_R_CR: u8, pub OUT_CLAMP_MAX_R_CR: u8,
    pub OUT_ROUND_TRUNC_MODE: u8, pub DCP_SPATIAL_DITHER_EN: u8,
    pub DCP_SPATIAL_DITHER_MODE: u8, pub DCP_SPATIAL_DITHER_DEPTH: u8,
    pub DCP_FRAME_RANDOM_ENABLE: u8, pub DCP_RGB_RANDOM_ENABLE: u8,
    pub DCP_HIGHPASS_RANDOM_ENABLE: u8, pub DENORM_MODE: u8,
    pub INTERLEAVE_EN: u8, pub PIXEL_DEPTH: u8, pub PIXEL_EXPAN_MODE: u8,
    pub GAMUT_REMAP_C11: u8, pub GAMUT_REMAP_C12: u8, pub GAMUT_REMAP_C13: u8,
    pub GAMUT_REMAP_C14: u8, pub GAMUT_REMAP_C21: u8, pub GAMUT_REMAP_C22: u8,
    pub GAMUT_REMAP_C23: u8, pub GAMUT_REMAP_C24: u8, pub GAMUT_REMAP_C31: u8,
    pub GAMUT_REMAP_C32: u8, pub GAMUT_REMAP_C33: u8, pub GAMUT_REMAP_C34: u8,
    pub GRPH_GAMUT_REMAP_MODE: u8, pub OUTPUT_CSC_C11: u8, pub OUTPUT_CSC_C12: u8,
    pub OUTPUT_CSC_GRPH_MODE: u8, pub DCP_REGAMMA_MEM_PWR_DIS: u8,
    pub DCP_LUT_MEM_PWR_DIS: u8, pub REGAMMA_LUT_LIGHT_SLEEP_DIS: u8,
    pub DCP_LUT_LIGHT_SLEEP_DIS: u8, pub REGAMMA_CNTLA_EXP_REGION_START: u8,
    pub REGAMMA_CNTLA_EXP_REGION_START_SEGMENT: u8,
    pub REGAMMA_CNTLA_EXP_REGION_LINEAR_SLOPE: u8,
    pub REGAMMA_CNTLA_EXP_REGION_END: u8, pub REGAMMA_CNTLA_EXP_REGION_END_BASE: u8,
    pub REGAMMA_CNTLA_EXP_REGION_END_SLOPE: u8,
    pub REGAMMA_CNTLA_EXP_REGION0_LUT_OFFSET: u8,
    pub REGAMMA_CNTLA_EXP_REGION0_NUM_SEGMENTS: u8,
    pub REGAMMA_CNTLA_EXP_REGION1_LUT_OFFSET: u8,
    pub REGAMMA_CNTLA_EXP_REGION1_NUM_SEGMENTS: u8,
    pub DCP_REGAMMA_MEM_PWR_STATE: u8, pub REGAMMA_LUT_MEM_PWR_STATE: u8,
    pub REGAMMA_LUT_WRITE_EN_MASK: u8, pub GRPH_REGAMMA_MODE: u8,
    pub SCL_MODE: u8, pub SCL_BYPASS_MODE: u8, pub SCL_PSCL_EN: u8,
    pub SCL_H_NUM_OF_TAPS: u8, pub SCL_V_NUM_OF_TAPS: u8, pub SCL_BOUNDARY_MODE: u8,
    pub EXT_OVERSCAN_LEFT: u8, pub EXT_OVERSCAN_RIGHT: u8, pub EXT_OVERSCAN_TOP: u8,
    pub EXT_OVERSCAN_BOTTOM: u8, pub SCL_COEFF_MEM_PWR_DIS: u8,
    pub SCL_COEFF_MEM_PWR_STATE: u8, pub SCL_C_RAM_FILTER_TYPE: u8,
    pub SCL_C_RAM_PHASE: u8, pub SCL_C_RAM_TAP_PAIR_IDX: u8,
    pub SCL_C_RAM_EVEN_TAP_COEF_EN: u8, pub SCL_C_RAM_EVEN_TAP_COEF: u8,
    pub SCL_C_RAM_ODD_TAP_COEF_EN: u8, pub SCL_C_RAM_ODD_TAP_COEF: u8,
    pub VIEWPORT_X_START: u8, pub VIEWPORT_Y_START: u8, pub VIEWPORT_HEIGHT: u8,
    pub VIEWPORT_WIDTH: u8, pub SCL_H_SCALE_RATIO: u8, pub SCL_V_SCALE_RATIO: u8,
    pub SCL_H_INIT_INT: u8, pub SCL_H_INIT_FRAC: u8, pub SCL_H_INIT_INT_RGB_Y: u8,
    pub SCL_H_INIT_FRAC_RGB_Y: u8, pub SCL_H_INIT_INT_CBCR: u8,
    pub SCL_H_INIT_FRAC_CBCR: u8, pub SCL_V_INIT_INT: u8, pub SCL_V_INIT_FRAC: u8,
    pub DC_LB_MEMORY_CONFIG: u8, pub DC_LB_MEM_SIZE: u8, pub LB_MEMORY_CONFIG: u8,
    pub LB_MEMORY_SIZE: u8, pub SCL_V_2TAP_HARDCODE_COEF_EN: u8,
    pub SCL_H_2TAP_HARDCODE_COEF_EN: u8, pub SCL_V_FILTER_PICK_NEAREST: u8,
    pub SCL_H_FILTER_PICK_NEAREST: u8, pub SCL_COEF_UPDATE_COMPLETE: u8,
    pub ALPHA_EN: u8,
}

// The mask layout is identical to dce_transform_shift, with 32-bit fields.
// This explicit alias preserves the C declaration's field layout and names.
pub type dce_transform_mask = dce_transform_shift;

#[repr(C)]
pub struct dce_transform_registers {
    pub LB_DATA_FORMAT: u32, pub GAMUT_REMAP_CONTROL: u32,
    pub GAMUT_REMAP_C11_C12: u32, pub GAMUT_REMAP_C13_C14: u32,
    pub GAMUT_REMAP_C21_C22: u32, pub GAMUT_REMAP_C23_C24: u32,
    pub GAMUT_REMAP_C31_C32: u32, pub GAMUT_REMAP_C33_C34: u32,
    pub OUTPUT_CSC_C11_C12: u32, pub OUTPUT_CSC_C13_C14: u32,
    pub OUTPUT_CSC_C21_C22: u32, pub OUTPUT_CSC_C23_C24: u32,
    pub OUTPUT_CSC_C31_C32: u32, pub OUTPUT_CSC_C33_C34: u32,
    pub OUTPUT_CSC_CONTROL: u32, pub DCFE_MEM_LIGHT_SLEEP_CNTL: u32,
    pub REGAMMA_CNTLA_START_CNTL: u32, pub REGAMMA_CNTLA_SLOPE_CNTL: u32,
    pub REGAMMA_CNTLA_END_CNTL1: u32, pub REGAMMA_CNTLA_END_CNTL2: u32,
    pub REGAMMA_CNTLA_REGION_0_1: u32, pub REGAMMA_CNTLA_REGION_2_3: u32,
    pub REGAMMA_CNTLA_REGION_4_5: u32, pub REGAMMA_CNTLA_REGION_6_7: u32,
    pub REGAMMA_CNTLA_REGION_8_9: u32, pub REGAMMA_CNTLA_REGION_10_11: u32,
    pub REGAMMA_CNTLA_REGION_12_13: u32, pub REGAMMA_CNTLA_REGION_14_15: u32,
    pub REGAMMA_LUT_WRITE_EN_MASK: u32, pub REGAMMA_LUT_INDEX: u32,
    pub REGAMMA_LUT_DATA: u32, pub REGAMMA_CONTROL: u32, pub DENORM_CONTROL: u32,
    pub DCP_SPATIAL_DITHER_CNTL: u32, pub OUT_ROUND_CONTROL: u32,
    pub OUT_CLAMP_CONTROL_R_CR: u32, pub OUT_CLAMP_CONTROL_G_Y: u32,
    pub OUT_CLAMP_CONTROL_B_CB: u32, pub SCL_MODE: u32, pub SCL_TAP_CONTROL: u32,
    pub SCL_CONTROL: u32, pub SCL_BYPASS_CONTROL: u32,
    pub EXT_OVERSCAN_LEFT_RIGHT: u32, pub EXT_OVERSCAN_TOP_BOTTOM: u32,
    pub SCL_VERT_FILTER_CONTROL: u32, pub SCL_HORZ_FILTER_CONTROL: u32,
    pub DCFE_MEM_PWR_CTRL: u32, pub DCFE_MEM_PWR_STATUS: u32,
    pub SCL_COEF_RAM_SELECT: u32, pub SCL_COEF_RAM_TAP_DATA: u32,
    pub VIEWPORT_START: u32, pub VIEWPORT_SIZE: u32,
    pub SCL_HORZ_FILTER_SCALE_RATIO: u32, pub SCL_VERT_FILTER_SCALE_RATIO: u32,
    pub SCL_HORZ_FILTER_INIT: u32, pub SCL_VERT_FILTER_INIT: u32,
    pub SCL_AUTOMATIC_MODE_CONTROL: u32, pub LB_MEMORY_CTRL: u32,
    pub SCL_UPDATE: u32, pub SCL_F_SHARP_CONTROL: u32,
}

#[repr(C)] pub struct init_int_and_frac { pub integer: u32, pub fraction: u32 }
#[repr(C)] pub struct scl_ratios_inits { pub h_int_scale_ratio: u32, pub v_int_scale_ratio: u32, pub h_init: init_int_and_frac, pub v_init: init_int_and_frac }
#[cfg(feature = "CONFIG_DRM_AMD_DC_SI")]
#[repr(C)] pub struct sclh_ratios_inits { pub h_int_scale_ratio: u32, pub v_int_scale_ratio: u32, pub h_init_luma: init_int_and_frac, pub h_init_chroma: init_int_and_frac, pub v_init: init_int_and_frac }

#[repr(i32)] pub enum ram_filter_type { FILTER_TYPE_RGB_Y_VERTICAL = 0, FILTER_TYPE_CBCR_VERTICAL = 1, FILTER_TYPE_RGB_Y_HORIZONTAL = 2, FILTER_TYPE_CBCR_HORIZONTAL = 3, FILTER_TYPE_ALPHA_VERTICAL = 4, FILTER_TYPE_ALPHA_HORIZONTAL = 5 }

#[repr(C)] pub struct dce_transform {
    pub base: transform,
    pub regs: *const dce_transform_registers,
    pub xfm_shift: *const dce_transform_shift,
    pub xfm_mask: *const dce_transform_mask,
    pub filter_v: *const u16, pub filter_h: *const u16,
    pub filter_v_c: *const u16, pub filter_h_c: *const u16,
    pub lb_pixel_depth_supported: i32, pub lb_memory_size: i32,
    pub lb_bits_per_entry: i32, pub prescaler_on: bool,
}

extern "C" {
    pub fn dce_transform_construct(xfm_dce: *mut dce_transform, ctx: *mut dc_context, inst: u32, regs: *const dce_transform_registers, xfm_shift: *const dce_transform_shift, xfm_mask: *const dce_transform_mask);
    #[cfg(feature = "CONFIG_DRM_AMD_DC_SI")] pub fn dce60_transform_construct(xfm_dce: *mut dce_transform, ctx: *mut dc_context, inst: u32, regs: *const dce_transform_registers, xfm_shift: *const dce_transform_shift, xfm_mask: *const dce_transform_mask);
    pub fn dce_transform_get_optimal_number_of_taps(xfm: *mut transform, scl_data: *mut scaler_data, in_taps: *const scaling_taps) -> bool;
    pub fn dce110_opp_set_csc_adjustment(xfm: *mut transform, tbl_entry: *const out_csc_color_matrix);
    pub fn dce110_opp_set_csc_default(xfm: *mut transform, default_adjust: *const default_adjustment);
    pub fn dce110_opp_power_on_regamma_lut(xfm: *mut transform, power_on: bool);
    pub fn dce110_opp_program_regamma_pwl(xfm: *mut transform, params: *const pwl_params);
    pub fn dce110_opp_set_regamma_mode(xfm: *mut transform, mode: opp_regamma);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
