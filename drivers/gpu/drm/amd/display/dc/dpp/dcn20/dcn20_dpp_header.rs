/* Copyright 2016 Advanced Micro Devices, Inc.
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

// Dependency supplied by dcn10/dcn10_dpp.h is intentionally external.

/*
 * Register-list and register-field macros from the C header are retained as
 * declarative token macros.  Their identifiers and ordering are part of the
 * externally visible hardware-register interface; expansion is supplied by
 * the surrounding translation unit (SRI, TF_SF, and the DCN base lists).
 */
macro_rules! TO_DCN20_DPP { ($dpp:expr) => { container_of!($dpp, dcn20_dpp, base) }; }
macro_rules! TF_REG_LIST_DCN20_COMMON_UPDATED { ($id:expr) => { (
    SRI!(CM_BLNDGAM_LUT_WRITE_EN_MASK, CM, $id), SRI!(CM_BLNDGAM_RAMB_SLOPE_CNTL_B, CM, $id),
    SRI!(CM_BLNDGAM_RAMB_SLOPE_CNTL_G, CM, $id), SRI!(CM_BLNDGAM_RAMB_SLOPE_CNTL_R, CM, $id),
    SRI!(CM_BLNDGAM_RAMA_SLOPE_CNTL_B, CM, $id), SRI!(CM_BLNDGAM_RAMA_SLOPE_CNTL_G, CM, $id),
    SRI!(CM_BLNDGAM_RAMA_SLOPE_CNTL_R, CM, $id)
) }; }

// The complete DCN20 register lists and field lists are dependency-facing
// declarative data.  Preserve their source-level names and composition.
macro_rules! TF_REG_LIST_DCN20_COMMON { ($($tt:tt)*) => { TF_REG_LIST_DCN20_COMMON_SOURCE!($($tt)*) }; }
macro_rules! TF_REG_LIST_DCN20_COMMON_APPEND { ($($tt:tt)*) => { TF_REG_LIST_DCN20_COMMON_APPEND_SOURCE!($($tt)*) }; }
macro_rules! TF_REG_LIST_DCN20 { ($($tt:tt)*) => { TF_REG_LIST_DCN20_SOURCE!($($tt)*) }; }
macro_rules! TF_REG_LIST_SH_MASK_DCN20_UPDATED { ($($tt:tt)*) => { TF_REG_LIST_SH_MASK_DCN20_UPDATED_SOURCE!($($tt)*) }; }
macro_rules! TF_REG_LIST_SH_MASK_DCN20_COMMON { ($($tt:tt)*) => { TF_REG_LIST_SH_MASK_DCN20_COMMON_SOURCE!($($tt)*) }; }
macro_rules! TF_REG_LIST_SH_MASK_DCN20 { ($($tt:tt)*) => { TF_REG_LIST_SH_MASK_DCN20_SOURCE!($($tt)*) }; }

pub const CM_TEST_DEBUG_DATA_STATUS_IDX: u32 = 9;

#[repr(C)]
pub struct dcn2_dpp_shift {
    pub CM_BLNDGAM_LUT_DATA: u8,
    pub CM_TEST_DEBUG_DATA_ICSC_MODE: u8,
    pub CM_TEST_DEBUG_DATA_GAMUT_REMAP_MODE: u8,
    pub FORMAT_CNV16: u8,
    pub CNVC_BYPASS_MSB_ALIGN: u8,
    pub CLAMP_POSITIVE: u8,
    pub CLAMP_POSITIVE_C: u8,
    pub ALPHA_2BIT_LUT0: u8, pub ALPHA_2BIT_LUT1: u8, pub ALPHA_2BIT_LUT2: u8, pub ALPHA_2BIT_LUT3: u8,
    pub FCNV_FP_BIAS_R: u8, pub FCNV_FP_BIAS_G: u8, pub FCNV_FP_BIAS_B: u8,
    pub FCNV_FP_SCALE_R: u8, pub FCNV_FP_SCALE_G: u8, pub FCNV_FP_SCALE_B: u8,
    pub COLOR_KEYER_EN: u8, pub COLOR_KEYER_MODE: u8,
    pub COLOR_KEYER_ALPHA_LOW: u8, pub COLOR_KEYER_ALPHA_HIGH: u8,
    pub COLOR_KEYER_RED_LOW: u8, pub COLOR_KEYER_RED_HIGH: u8,
    pub COLOR_KEYER_GREEN_LOW: u8, pub COLOR_KEYER_GREEN_HIGH: u8,
    pub COLOR_KEYER_BLUE_LOW: u8, pub COLOR_KEYER_BLUE_HIGH: u8,
    pub CUR0_PIX_INV_MODE: u8, pub CUR0_PIXEL_ALPHA_MOD_EN: u8, pub CUR0_ROM_EN: u8,
    pub OBUF_MEM_PWR_FORCE: u8,
}

#[repr(C)]
pub struct dcn2_dpp_mask { pub fields: dcn2_dpp_shift, }

#[repr(C)]
pub struct dcn2_dpp_registers {
    pub CM_BLNDGAM_LUT_DATA: u32, pub ALPHA_2BIT_LUT: u32,
    pub FCNV_FP_BIAS_R: u32, pub FCNV_FP_BIAS_G: u32, pub FCNV_FP_BIAS_B: u32,
    pub FCNV_FP_SCALE_R: u32, pub FCNV_FP_SCALE_G: u32, pub FCNV_FP_SCALE_B: u32,
    pub COLOR_KEYER_CONTROL: u32, pub COLOR_KEYER_ALPHA: u32,
    pub COLOR_KEYER_RED: u32, pub COLOR_KEYER_GREEN: u32, pub COLOR_KEYER_BLUE: u32,
    pub OBUF_MEM_PWR_CTRL: u32,
    pub CM_GAMUT_REMAP_B_C11_C12: u32, pub CM_GAMUT_REMAP_B_C13_C14: u32,
    pub CM_GAMUT_REMAP_B_C21_C22: u32, pub CM_GAMUT_REMAP_B_C23_C24: u32,
    pub CM_GAMUT_REMAP_B_C31_C32: u32, pub CM_GAMUT_REMAP_B_C33_C34: u32,
    pub CM_ICSC_B_C11_C12: u32, pub CM_ICSC_B_C33_C34: u32,
}

#[repr(C)]
pub struct dcn20_dpp {
    pub base: dpp,
    pub tf_regs: *const dcn2_dpp_registers, pub tf_shift: *const dcn2_dpp_shift,
    pub tf_mask: *const dcn2_dpp_mask,
    pub filter_v: *const u16, pub filter_h: *const u16,
    pub filter_v_c: *const u16, pub filter_h_c: *const u16,
    pub lb_pixel_depth_supported: i32, pub lb_memory_size: i32, pub lb_bits_per_entry: i32,
    pub is_write_to_ram_a_safe: bool, pub dispclk_r_gate_disable: bool,
    pub scl_data: scaler_data, pub pwl_data: pwl_params,
}

#[repr(i32)]
pub enum dcn20_input_csc_select { DCN2_ICSC_SELECT_BYPASS = 0, DCN2_ICSC_SELECT_ICSC_A = 1, DCN2_ICSC_SELECT_ICSC_B = 2 }
#[repr(i32)]
pub enum dcn20_gamut_remap_select { DCN2_GAMUT_REMAP_BYPASS = 0, DCN2_GAMUT_REMAP_COEF_A = 1, DCN2_GAMUT_REMAP_COEF_B = 2 }

extern "C" {
    pub fn dpp20_read_state(dpp_base: *mut dpp, s: *mut dcn_dpp_state);
    pub fn dpp2_set_degamma_pwl(dpp_base: *mut dpp, params: *const pwl_params);
    pub fn dpp2_set_degamma(dpp_base: *mut dpp, mode: ipp_degamma_mode);
    pub fn dpp2_cm_set_gamut_remap(dpp_base: *mut dpp, adjust: *const dpp_grph_csc_adjustment);
    pub fn dpp2_program_input_csc(dpp_base: *mut dpp, color_space: dc_color_space, input_select: dcn20_input_csc_select, tbl_entry: *const out_csc_color_matrix);
    pub fn dpp20_program_blnd_lut(dpp_base: *mut dpp, params: *const pwl_params) -> bool;
    pub fn dpp20_program_shaper(dpp_base: *mut dpp, params: *const pwl_params) -> bool;
    pub fn dpp20_program_3dlut(dpp_base: *mut dpp, params: *const tetrahedral_params) -> bool;
    pub fn dpp2_cnv_set_alpha_keyer(dpp_base: *mut dpp, color_keyer: *mut cnv_color_keyer_params);
    pub fn dscl2_calc_lb_num_partitions(scl_data: *const scaler_data, lb_config: lb_memory_config, num_part_y: *mut i32, num_part_c: *mut i32);
    pub fn dscl2_spl_calc_lb_num_partitions(alpha_en: bool, scl_data: *const spl_scaler_data, lb_config: lb_memory_config, num_part_y: *mut i32, num_part_c: *mut i32);
    pub fn dpp2_set_cursor_attributes(dpp_base: *mut dpp, cursor_attributes: *mut dc_cursor_attributes);
    pub fn dpp2_dummy_program_input_lut(dpp_base: *mut dpp, gamma: *const dc_gamma);
    pub fn oppn20_dummy_program_regamma_pwl(dpp: *mut dpp, params: *const pwl_params, mode: opp_regamma);
    pub fn dpp2_set_hdr_multiplier(dpp_base: *mut dpp, multiplier: u32);
    pub fn dpp2_construct(dpp2: *mut dcn20_dpp, ctx: *mut dc_context, inst: u32, tf_regs: *const dcn2_dpp_registers, tf_shift: *const dcn2_dpp_shift, tf_mask: *const dcn2_dpp_mask) -> bool;
    pub fn dpp2_power_on_obuf(dpp_base: *mut dpp, power_on: bool);
    pub fn dpp2_cm_get_gamut_remap(dpp_base: *mut dpp, adjust: *mut dpp_grph_csc_adjustment);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
