/* SPDX-License-Identifier: MIT */
/* Copyright 2023-2026 Advanced Micro Devices, Inc. */

// Rust translation of dcn401_dpp.h.
// The included register-field dependencies and the TF_SF/TF2_SF machinery are
// supplied by the surrounding translation unit.

// C includes:
// dcn20/dcn20_dpp.h, dcn30/dcn30_dpp.h, dcn32/dcn32_dpp.h

#[macro_export]
macro_rules! TO_DCN401_DPP {
    ($dpp:expr) => { container_of!($dpp, dcn401_dpp, base) };
}

// DPP_REG_LIST_SH_MASK_DCN401_COMMON is a register-field list consumed by the
// register-generation macros.  Its entries are retained verbatim here as a
// token-list macro; the referenced register and field identifiers are external.
#[macro_export]
macro_rules! DPP_REG_LIST_SH_MASK_DCN401_COMMON {
    ($mask_sh:expr) => {
        TF_SF!(CM0_CM_MEM_PWR_STATUS, GAMCOR_MEM_PWR_STATE, $mask_sh);
        TF_SF!(CM0_CM_DEALPHA, CM_DEALPHA_EN, $mask_sh);
        TF_SF!(CM0_CM_DEALPHA, CM_DEALPHA_ABLND, $mask_sh);
        TF_SF!(CM0_CM_BIAS_CR_R, CM_BIAS_CR_R, $mask_sh);
        TF_SF!(CM0_CM_BIAS_Y_G_CB_B, CM_BIAS_Y_G, $mask_sh);
        TF_SF!(CM0_CM_BIAS_Y_G_CB_B, CM_BIAS_CB_B, $mask_sh);
        TF_SF!(CM0_CM_MEM_PWR_CTRL, GAMCOR_MEM_PWR_DIS, $mask_sh);
        TF_SF!(CM0_CM_MEM_PWR_CTRL, GAMCOR_MEM_PWR_FORCE, $mask_sh);
        TF_SF!(CNVC_CFG0_PRE_DEGAM, PRE_DEGAM_MODE, $mask_sh);
        TF_SF!(CNVC_CFG0_PRE_DEGAM, PRE_DEGAM_SELECT, $mask_sh);
        TF_SF!(CM0_CM_GAMCOR_CONTROL, CM_GAMCOR_MODE, $mask_sh);
        TF_SF!(CM0_CM_GAMCOR_CONTROL, CM_GAMCOR_SELECT, $mask_sh);
        TF_SF!(CM0_CM_GAMCOR_LUT_INDEX, CM_GAMCOR_LUT_INDEX, $mask_sh);
        TF_SF!(CM0_CM_GAMCOR_LUT_DATA, CM_GAMCOR_LUT_DATA, $mask_sh);
        TF_SF!(DSCL0_DSCL_EXT_OVERSCAN_LEFT_RIGHT, EXT_OVERSCAN_LEFT, $mask_sh);
        TF_SF!(DSCL0_DSCL_EXT_OVERSCAN_LEFT_RIGHT, EXT_OVERSCAN_RIGHT, $mask_sh);
        TF_SF!(DSCL0_DSCL_EXT_OVERSCAN_TOP_BOTTOM, EXT_OVERSCAN_BOTTOM, $mask_sh);
        TF_SF!(DSCL0_DSCL_EXT_OVERSCAN_TOP_BOTTOM, EXT_OVERSCAN_TOP, $mask_sh);
        TF_SF!(DSCL0_LB_DATA_FORMAT, INTERLEAVE_EN, $mask_sh);
        TF2_SF!(DSCL0, LB_DATA_FORMAT__ALPHA_EN, $mask_sh);
        TF_SF!(DSCL0_LB_MEMORY_CTRL, MEMORY_CONFIG, $mask_sh);
        TF_SF!(DSCL0_LB_MEMORY_CTRL, LB_MAX_PARTITIONS, $mask_sh);
        TF_SF!(DSCL0_DSCL_CONTROL, SCL_BOUNDARY_MODE, $mask_sh);
        TF_SF!(DSCL0_SCL_MODE, SCL_COEF_RAM_SELECT, $mask_sh);
        TF_SF!(DSCL0_SCL_MODE, DSCL_MODE, $mask_sh);
        TF_SF!(DPP_TOP0_DPP_CONTROL, DPP_CLOCK_ENABLE, $mask_sh);
        TF_SF!(CM0_CM_CONTROL, CM_BYPASS, $mask_sh);
        TF_SF!(CNVC_CFG0_FORMAT_CONTROL, CNVC_BYPASS, $mask_sh);
        TF2_SF!(CNVC_CFG0, FORMAT_CONTROL__ALPHA_EN, $mask_sh);
        TF_SF!(CNVC_CFG0_CNVC_SURFACE_PIXEL_FORMAT, CNVC_SURFACE_PIXEL_FORMAT, $mask_sh);
        TF_SF!(CNVC_CFG0_CNVC_SURFACE_PIXEL_FORMAT, CNVC_ALPHA_PLANE_ENABLE, $mask_sh);
        TF_SF!(CM_CUR0_CURSOR0_CONTROL, CUR0_MODE, $mask_sh);
        TF_SF!(CM_CUR0_CURSOR0_CONTROL, CUR0_ENABLE, $mask_sh);
        TF_SF!(DSCL0_ISHARP_MODE, ISHARP_EN, $mask_sh);
        TF_SF!(DSCL0_ISHARP_MODE, ISHARP_NOISEDET_EN, $mask_sh);
        TF_SF!(DSCL0_ISHARP_MODE, ISHARP_LBA_MODE, $mask_sh);
        TF_SF!(DSCL0_ISHARP_DATA, ISHARP_DELTA_DATA, $mask_sh);
        TF_SF!(DSCL0_SCL_VERT_FILTER_INIT_BOT, SCL_V_INIT_FRAC_BOT, $mask_sh);
        TF_SF!(DSCL0_SCL_VERT_FILTER_INIT_BOT, SCL_V_INIT_INT_BOT, $mask_sh);
    };
}

#[repr(C)]
pub struct dcn401_dpp_registers {
    pub common: dcn3_dpp_registers,
    pub CURSOR0_FP_SCALE_BIAS_G_Y: u32,
    pub CURSOR0_FP_SCALE_BIAS_RB_CRCB: u32,
    pub CUR0_MATRIX_MODE: u32,
    pub CUR0_MATRIX_C11_C12_A: u32,
    pub CUR0_MATRIX_C13_C14_A: u32,
    pub CUR0_MATRIX_C21_C22_A: u32,
    pub CUR0_MATRIX_C23_C24_A: u32,
    pub CUR0_MATRIX_C31_C32_A: u32,
    pub CUR0_MATRIX_C33_C34_A: u32,
    pub CUR0_MATRIX_C11_C12_B: u32,
    pub CUR0_MATRIX_C13_C14_B: u32,
    pub CUR0_MATRIX_C21_C22_B: u32,
    pub CUR0_MATRIX_C23_C24_B: u32,
    pub CUR0_MATRIX_C31_C32_B: u32,
    pub CUR0_MATRIX_C33_C34_B: u32,
    pub DSCL_SC_MODE: u32,
    pub DSCL_EASF_H_MODE: u32,
    pub DSCL_EASF_H_BF_CNTL: u32,
    pub DSCL_EASF_H_RINGEST_EVENTAP_REDUCE: u32,
    pub DSCL_EASF_H_RINGEST_EVENTAP_GAIN: u32,
    pub DSCL_EASF_H_BF_FINAL_MAX_MIN: u32,
    pub DSCL_EASF_V_MODE: u32,
    pub DSCL_EASF_V_BF_CNTL: u32,
    pub DSCL_EASF_V_RINGEST_3TAP_CNTL1: u32,
    pub DSCL_EASF_V_RINGEST_3TAP_CNTL2: u32,
    pub DSCL_EASF_V_RINGEST_3TAP_CNTL3: u32,
    pub DSCL_EASF_V_RINGEST_EVENTAP_REDUCE: u32,
    pub DSCL_EASF_V_RINGEST_EVENTAP_GAIN: u32,
    pub DSCL_EASF_V_BF_FINAL_MAX_MIN: u32,
    pub DSCL_SC_MATRIX_C0C1: u32,
    pub DSCL_SC_MATRIX_C2C3: u32,
    pub ISHARP_MODE: u32,
    pub ISHARP_DELTA_LUT_MEM_PWR_CTRL: u32,
    pub ISHARP_NOISEDET_THRESHOLD: u32,
    pub ISHARP_NOISE_GAIN_PWL: u32,
    pub ISHARP_DELTA_CTRL: u32,
    pub ISHARP_DELTA_DATA: u32,
    pub ISHARP_DELTA_INDEX: u32,
    pub ISHARP_NLDELTA_SOFT_CLIP: u32,
    pub ALPHA_2BIT_LUT01: u32,
    pub ALPHA_2BIT_LUT23: u32,
}

#[repr(C)]
pub struct dcn401_dpp_shift { pub common: dcn3_dpp_shift }
#[repr(C)]
pub struct dcn401_dpp_mask { pub common: dcn3_dpp_mask }

#[repr(C)]
pub struct dcn401_dpp {
    pub base: dpp,
    pub tf_regs: *const dcn401_dpp_registers,
    pub tf_shift: *const dcn401_dpp_shift,
    pub tf_mask: *const dcn401_dpp_mask,
    pub filter_v: *const u16,
    pub filter_h: *const u16,
    pub filter_v_c: *const u16,
    pub filter_h_c: *const u16,
    pub lb_pixel_depth_supported: i32,
    pub lb_memory_size: i32,
    pub lb_bits_per_entry: i32,
    pub is_write_to_ram_a_safe: bool,
    pub scl_data: scaler_data,
    pub pwl_data: pwl_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dcn401_dscl_mode_sel {
    DCN401_DSCL_MODE_SCALING_444_BYPASS = 0,
    DCN401_DSCL_MODE_SCALING_444_RGB_ENABLE = 1,
    DCN401_DSCL_MODE_SCALING_444_YCBCR_ENABLE = 2,
    DCN401_DSCL_MODE_SCALING_420_YCBCR_ENABLE = 3,
    DCN401_DSCL_MODE_SCALING_420_LUMA_BYPASS = 4,
    DCN401_DSCL_MODE_SCALING_420_CHROMA_BYPASS = 5,
    DCN401_DSCL_MODE_DSCL_BYPASS = 6,
}

extern "C" {
    pub fn dpp401_construct(dpp401: *mut dcn401_dpp, ctx: *mut dc_context, inst: u32, tf_regs: *const dcn401_dpp_registers, tf_shift: *const dcn401_dpp_shift, tf_mask: *const dcn401_dpp_mask) -> bool;
    pub fn dpp401_dscl_set_scaler_manual_scale(dpp_base: *mut dpp, scl_data: *const scaler_data);
    pub fn dpp401_dpp_setup(dpp_base: *mut dpp, format: surface_pixel_format, mode: expansion_mode, input_csc_color_matrix: dc_csc_transform, input_color_space: dc_color_space, alpha_2bit_lut: *mut cnv_alpha_2bit_lut);
    pub fn dpp401_set_cursor_attributes(dpp_base: *mut dpp, cursor_attributes: *mut dc_cursor_attributes);
    pub fn dpp401_set_cursor_position(dpp_base: *mut dpp, pos: *const dc_cursor_position, param: *const dc_cursor_mi_param, width: u32, height: u32);
    pub fn dpp401_set_optional_cursor_attributes(dpp_base: *mut dpp, attr: *mut dpp_cursor_attributes);
    pub fn dscl401_calc_lb_num_partitions(scl_data: *const scaler_data, lb_config: lb_memory_config, num_part_y: *mut i32, num_part_c: *mut i32);
    pub fn dscl401_spl_calc_lb_num_partitions(alpha_en: bool, scl_data: *const spl_scaler_data, lb_config: lb_memory_config, num_part_y: *mut i32, num_part_c: *mut i32);
    pub fn dpp401_read_state(dpp_base: *mut dpp, s: *mut dcn_dpp_state);
    pub fn dpp401_set_cursor_matrix(dpp_base: *mut dpp, color_space: dc_color_space, cursor_csc_color_matrix: dc_csc_transform);
    pub fn dpp401_dscl_get_dscl_mode(dpp_base: *mut dpp, data: *const scaler_data, dbg_always_scale: bool) -> dcn401_dscl_mode_sel;
    pub fn dpp401_power_on_dscl(dpp_base: *mut dpp, power_on: bool);
    pub fn dpp401_dscl_set_recout(dpp: *mut dcn401_dpp, recout: *const rect);
    pub fn dpp401_dscl_find_lb_memory_config(dpp: *mut dcn401_dpp, scl_data: *const scaler_data) -> lb_memory_config;
    pub fn dpp401_dscl_program_isharp(dpp_base: *mut dpp, scl_data: *const scaler_data, program_isharp_1dlut: bool, bs_coeffs_updated: *mut bool);
    pub fn dpp401_dscl_set_isharp_filter(dpp: *mut dcn401_dpp, filter: *const u32);
    pub fn dpp401_dscl_set_scl_filter(dpp: *mut dcn401_dpp, scl_data: *const scaler_data, chroma_coef_mode: bool, force_coeffs_update: bool);
    pub fn dpp401_dscl_disable_easf(dpp_base: *mut dpp, scl_data: *const scaler_data);
    pub fn dpp401_dscl_program_easf(dpp_base: *mut dpp, scl_data: *const scaler_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
