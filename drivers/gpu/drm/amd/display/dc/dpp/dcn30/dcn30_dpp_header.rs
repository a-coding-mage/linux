/* Copyright 2020 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND.
 */

// Dependency: dcn20/dcn20_dpp.h supplies the base types and register helpers.

#[macro_export]
macro_rules! TO_DCN30_DPP { ($dpp:expr) => { container_of!($dpp, dcn3_dpp, base) }; }

// Register and field-list macros retain the source register vocabulary.  The
// SRI/TF_SF/TF2_SF and inherited DPP/TF macros are supplied by dependencies.
#[macro_export]
macro_rules! DPP_REG_LIST_DCN30_COMMON { ($id:expr) => {
    SRI!(CM_DEALPHA, CM, $id), SRI!(CM_MEM_PWR_STATUS, CM, $id),
    SRI!(CM_BIAS_CR_R, CM, $id), SRI!(CM_BIAS_Y_G_CB_B, CM, $id),
    SRI!(PRE_DEGAM, CNVC_CFG, $id), SRI!(CM_GAMCOR_CONTROL, CM, $id),
    SRI!(CM_GAMCOR_LUT_CONTROL, CM, $id), SRI!(CM_GAMCOR_LUT_INDEX, CM, $id),
    SRI!(CM_GAMCOR_LUT_INDEX, CM, $id), SRI!(CM_GAMCOR_LUT_DATA, CM, $id),
    SRI!(CM_GAMCOR_RAMB_START_CNTL_B, CM, $id), SRI!(CM_GAMCOR_RAMB_START_CNTL_G, CM, $id),
    SRI!(CM_GAMCOR_RAMB_START_CNTL_R, CM, $id), SRI!(CM_GAMCOR_RAMB_START_SLOPE_CNTL_B, CM, $id),
    SRI!(CM_GAMCOR_RAMB_START_SLOPE_CNTL_G, CM, $id), SRI!(CM_GAMCOR_RAMB_START_SLOPE_CNTL_R, CM, $id),
    SRI!(CM_GAMCOR_RAMA_START_CNTL_B, CM, $id), SRI!(CM_GAMCOR_RAMA_START_CNTL_G, CM, $id),
    SRI!(CM_GAMCOR_RAMA_START_CNTL_R, CM, $id), SRI!(CM_GAMUT_REMAP_CONTROL, CM, $id),
    SRI!(DSCL_EXT_OVERSCAN_LEFT_RIGHT, DSCL, $id), SRI!(DSCL_EXT_OVERSCAN_TOP_BOTTOM, DSCL, $id),
    SRI!(OTG_H_BLANK, DSCL, $id), SRI!(OTG_V_BLANK, DSCL, $id), SRI!(SCL_MODE, DSCL, $id),
    SRI!(LB_DATA_FORMAT, DSCL, $id), SRI!(LB_MEMORY_CTRL, DSCL, $id), SRI!(DSCL_AUTOCAL, DSCL, $id),
    SRI!(DSCL_CONTROL, DSCL, $id), SRI!(SCL_TAP_CONTROL, DSCL, $id), SRI!(SCL_COEF_RAM_TAP_SELECT, DSCL, $id),
    SRI!(SCL_COEF_RAM_TAP_DATA, DSCL, $id), SRI!(RECOUT_START, DSCL, $id), SRI!(RECOUT_SIZE, DSCL, $id),
    SRI!(PRE_DEALPHA, CNVC_CFG, $id), SRI!(PRE_REALPHA, CNVC_CFG, $id), SRI!(PRE_CSC_MODE, CNVC_CFG, $id),
    SRI!(PRE_CSC_C11_C12, CNVC_CFG, $id), SRI!(PRE_CSC_C33_C34, CNVC_CFG, $id),
    SRI!(CM_POST_CSC_CONTROL, CM, $id), SRI!(CM_POST_CSC_C11_C12, CM, $id),
    SRI!(CM_POST_CSC_C33_C34, CM, $id), SRI!(CM_MEM_PWR_CTRL, CM, $id), SRI!(CM_CONTROL, CM, $id),
    SRI!(FORMAT_CONTROL, CNVC_CFG, $id), SRI!(CNVC_SURFACE_PIXEL_FORMAT, CNVC_CFG, $id),
    SRI!(CURSOR0_CONTROL, CNVC_CUR, $id), SRI!(DPP_CONTROL, DPP_TOP, $id), SRI!(CM_HDR_MULT_COEF, CM, $id),
    SRI!(CURSOR_CONTROL, CURSOR0_, $id), SRI!(ALPHA_2BIT_LUT, CNVC_CFG, $id),
    SRI!(COLOR_KEYER_CONTROL, CNVC_CFG, $id), SRI!(COLOR_KEYER_ALPHA, CNVC_CFG, $id),
    SRI!(COLOR_KEYER_RED, CNVC_CFG, $id), SRI!(COLOR_KEYER_GREEN, CNVC_CFG, $id),
    SRI!(COLOR_KEYER_BLUE, CNVC_CFG, $id), SRI!(OBUF_MEM_PWR_CTRL, DSCL, $id),
    SRI!(DSCL_MEM_PWR_STATUS, DSCL, $id), SRI!(DSCL_MEM_PWR_CTRL, DSCL, $id)
}; }

#[macro_export]
macro_rules! DPP_REG_LIST_DCN30 { ($id:expr) => { (DPP_REG_LIST_DCN30_COMMON!($id), TF_REG_LIST_DCN20_COMMON!($id), SRI!(CM_BLNDGAM_CONTROL, CM, $id), SRI!(CM_SHAPER_LUT_DATA, CM, $id), SRI!(CM_MEM_PWR_CTRL2, CM, $id), SRI!(CM_MEM_PWR_STATUS2, CM, $id), SRI!(CM_BLNDGAM_LUT_CONTROL, CM, $id)) }; }
#[macro_export]
macro_rules! DPP_REG_LIST_SH_MASK_DCN30_COMMON { ($mask_sh:expr) => { (TF_REG_LIST_SH_MASK_DCN20_COMMON!($mask_sh)) }; }
#[macro_export]
macro_rules! DPP_REG_LIST_SH_MASK_DCN30_UPDATED { ($mask_sh:expr) => { (TF_SF!(CM0_CM_MEM_PWR_STATUS, BLNDGAM_MEM_PWR_STATE, $mask_sh), TF_SF!(CM0_CM_MEM_PWR_CTRL2, HDR3DLUT_MEM_PWR_FORCE, $mask_sh), TF_SF!(CM0_CM_MEM_PWR_CTRL2, SHAPER_MEM_PWR_FORCE, $mask_sh), TF_SF!(CM0_CM_BLNDGAM_CONTROL, CM_BLNDGAM_MODE, $mask_sh), TF_SF!(CM0_CM_BLNDGAM_LUT_CONTROL, CM_BLNDGAM_LUT_CONFIG_MODE, $mask_sh)) }; }
#[macro_export]
macro_rules! DPP_REG_LIST_SH_MASK_DCN30 { ($mask_sh:expr) => { (DPP_REG_LIST_SH_MASK_DCN30_COMMON!($mask_sh), DPP_REG_LIST_SH_MASK_DCN30_UPDATED!($mask_sh)) }; }

#[repr(C)]
pub struct dcn3_dpp_shift { pub base: dcn2_dpp_shift, pub FORMAT_CROSSBAR_R: u8, pub FORMAT_CROSSBAR_G: u8, pub FORMAT_CROSSBAR_B: u8, pub CM_DEALPHA_EN: u8, pub CM_DEALPHA_ABLND: u8, pub CM_BIAS_Y_G: u8, pub CM_BIAS_CB_B: u8, pub CM_BIAS_CR_R: u8, pub GAMCOR_MEM_PWR_DIS: u8, pub GAMCOR_MEM_PWR_FORCE: u8, pub HDR3DLUT_MEM_PWR_FORCE: u8, pub SHAPER_MEM_PWR_FORCE: u8, pub PRE_DEGAM_MODE: u8, pub PRE_DEGAM_SELECT: u8, pub CNVC_ALPHA_PLANE_ENABLE: u8, pub CM_BLNDGAM_MODE: u8, pub CM_BLNDGAM_MODE_CURRENT: u8, pub GAMCOR_MEM_PWR_STATE: u8, pub BLNDGAM_MEM_PWR_STATE: u8, pub HDR3DLUT_MEM_PWR_STATE: u8, pub SHAPER_MEM_PWR_STATE: u8 }
#[repr(C)]
pub struct dcn3_dpp_mask { pub base: dcn2_dpp_mask, pub FORMAT_CROSSBAR_R: u32, pub FORMAT_CROSSBAR_G: u32, pub FORMAT_CROSSBAR_B: u32, pub CM_DEALPHA_EN: u32, pub CM_DEALPHA_ABLND: u32, pub CM_BIAS_Y_G: u32, pub CM_BIAS_CB_B: u32, pub CM_BIAS_CR_R: u32, pub GAMCOR_MEM_PWR_DIS: u32, pub GAMCOR_MEM_PWR_FORCE: u32, pub HDR3DLUT_MEM_PWR_FORCE: u32, pub SHAPER_MEM_PWR_FORCE: u32, pub PRE_DEGAM_MODE: u32, pub PRE_DEGAM_SELECT: u32, pub CNVC_ALPHA_PLANE_ENABLE: u32, pub CM_BLNDGAM_MODE: u32, pub CM_BLNDGAM_MODE_CURRENT: u32, pub GAMCOR_MEM_PWR_STATE: u32, pub BLNDGAM_MEM_PWR_STATE: u32, pub HDR3DLUT_MEM_PWR_STATE: u32, pub SHAPER_MEM_PWR_STATE: u32 }

#[repr(C)] pub struct dcn3_dpp_registers { pub base: dcn2_dpp_registers, pub CM_MEM_PWR_STATUS: u32, pub CM_MEM_PWR_STATUS2: u32, pub CM_MEM_PWR_CTRL2: u32, pub CM_DEALPHA: u32, pub CM_BIAS_CR_R: u32, pub CM_BIAS_Y_G_CB_B: u32, pub PRE_DEGAM: u32, pub PRE_DEALPHA: u32, pub PRE_REALPHA: u32, pub PRE_CSC_MODE: u32, pub PRE_CSC_C11_C12: u32, pub PRE_CSC_C33_C34: u32, pub CM_POST_CSC_CONTROL: u32, pub CM_POST_CSC_C11_C12: u32, pub CM_POST_CSC_C33_C34: u32, pub CM_GAMCOR_CONTROL: u32, pub CM_GAMCOR_LUT_CONTROL: u32, pub CM_GAMCOR_LUT_INDEX: u32, pub CM_GAMCOR_LUT_DATA: u32, pub CM_BLNDGAM_RAMA_START_SLOPE_CNTL_B: u32, pub CM_BLNDGAM_RAMA_START_SLOPE_CNTL_G: u32, pub CM_BLNDGAM_RAMA_START_SLOPE_CNTL_R: u32, pub CM_BLNDGAM_RAMB_START_SLOPE_CNTL_B: u32, pub CM_BLNDGAM_RAMB_START_SLOPE_CNTL_G: u32, pub CM_BLNDGAM_RAMB_START_SLOPE_CNTL_R: u32, pub CM_BLNDGAM_LUT_CONTROL: u32 }

#[repr(C)] pub struct dcn3_dpp { pub base: dpp, pub tf_regs: *const dcn3_dpp_registers, pub tf_shift: *const dcn3_dpp_shift, pub tf_mask: *const dcn3_dpp_mask, pub filter_v: *const u16, pub filter_h: *const u16, pub filter_v_c: *const u16, pub filter_h_c: *const u16, pub lb_pixel_depth_supported: i32, pub lb_memory_size: i32, pub lb_bits_per_entry: i32, pub is_write_to_ram_a_safe: bool, pub dispclk_r_gate_disable: bool, pub scl_data: scaler_data, pub pwl_data: pwl_params }

extern "C" { pub fn dpp3_construct(dpp3: *mut dcn3_dpp, ctx: *mut dc_context, inst: u32, tf_regs: *const dcn3_dpp_registers, tf_shift: *const dcn3_dpp_shift, tf_mask: *const dcn3_dpp_mask) -> bool; pub fn dpp3_program_gamcor_lut(dpp_base: *mut dpp, params: *const pwl_params) -> bool; pub fn dpp3_program_CM_dealpha(dpp_base: *mut dpp, enable: u32, additive_blending: u32); pub fn dpp30_read_state(dpp_base: *mut dpp, s: *mut dcn_dpp_state); pub fn dpp30_read_reg_state(dpp_base: *mut dpp, dpp_reg_state: *mut dcn_dpp_reg_state); pub fn dpp3_get_optimal_number_of_taps(dpp: *mut dpp, scl_data: *mut scaler_data, in_taps: *const scaling_taps) -> bool; pub fn dpp3_cnv_setup(dpp_base: *mut dpp, format: surface_pixel_format, mode: expansion_mode, input_csc_color_matrix: dc_csc_transform, input_color_space: dc_color_space, alpha_2bit_lut: *mut cnv_alpha_2bit_lut); pub fn dpp3_program_CM_bias(dpp_base: *mut dpp, bias_params: *mut CM_bias_params); pub fn dpp3_set_hdr_multiplier(dpp_base: *mut dpp, multiplier: u32); pub fn dpp3_cm_set_gamut_remap(dpp_base: *mut dpp, adjust: *const dpp_grph_csc_adjustment); pub fn dpp3_set_pre_degam(dpp_base: *mut dpp, tr: dc_transfer_func_predefined); pub fn dpp3_set_cursor_attributes(dpp_base: *mut dpp, cursor_attributes: *mut dc_cursor_attributes); pub fn dpp3_program_post_csc(dpp_base: *mut dpp, color_space: dc_color_space, input_select: dcn10_input_csc_select, tbl_entry: *const out_csc_color_matrix); pub fn dpp3_program_cm_bias(dpp_base: *mut dpp, bias_params: *mut CM_bias_params); pub fn dpp3_program_cm_dealpha(dpp_base: *mut dpp, enable: u32, additive_blending: u32); pub fn dpp3_cm_get_gamut_remap(dpp_base: *mut dpp, adjust: *mut dpp_grph_csc_adjustment); pub fn dpp3_should_bypass_post_csc_for_colorspace(dc_color_space: dc_color_space) -> bool; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
