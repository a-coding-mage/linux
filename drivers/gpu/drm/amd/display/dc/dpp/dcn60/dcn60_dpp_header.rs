// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Translated from dcn60_dpp.h.  The included register-definition headers and
// register-field construction macros are supplied by the surrounding tree.
// C includes intentionally remain represented as dependency notes.
// Dependencies: dcn20/dcn20_dpp.h, dcn30/dcn30_dpp.h, dcn32/dcn32_dpp.h,
// dcn35/dcn35_dpp.h, dcn401/dcn401_dpp.h, dcn42/dcn42_dpp.h,
// dcn50/dcn50_dpp.h.

#[allow(unused_macros)]
macro_rules! to_dcn60_dpp {
    ($dpp:expr) => { container_of!($dpp, dcn60_dpp, base) };
}

// DPP_REG_LIST_SH_MASK_DCN60(mask_sh) is the register-field list from the C
// header.  Its entries are consumed by the generated register infrastructure;
// retain the invocation interface here so dependent declarations can use it.
#[allow(unused_macros)]
macro_rules! dpp_reg_list_sh_mask_dcn60 {
    ($mask_sh:expr) => {
        dpp_reg_list_sh_mask_dcn42!($mask_sh);
        TF_SF!(CM0_CM_MEM_PWR_STATUS, GAMCOR_MEM_PWR_STATE, $mask_sh);
        TF_SF!(CM0_CM_DEALPHA, CM_DEALPHA_EN, $mask_sh);
        TF_SF!(CM0_CM_DEALPHA, CM_DEALPHA_ABLND, $mask_sh);
        TF_SF!(CM0_CM_BIAS_CR_R, CM_BIAS_CR_R, $mask_sh);
        TF_SF!(CM0_CM_BIAS_Y_G_CB_B, CM_BIAS_Y_G, $mask_sh);
        TF_SF!(CM0_CM_BIAS_Y_G_CB_B, CM_BIAS_CB_B, $mask_sh);
        TF_SF!(CM0_CM_MEM_PWR_CTRL, GAMCOR_MEM_PWR_DIS, $mask_sh);
        TF_SF!(CM0_CM_MEM_PWR_CTRL, GAMCOR_MEM_PWR_FORCE, $mask_sh);
        TF_SF!(CM0_CM_GAMCOR_CONTROL, CM_GAMCOR_MODE, $mask_sh);
        TF_SF!(CM0_CM_GAMCOR_CONTROL, CM_GAMCOR_SELECT, $mask_sh);
        TF_SF!(CM0_CM_GAMCOR_LUT_INDEX, CM_GAMCOR_LUT_INDEX, $mask_sh);
        TF_SF!(CM0_CM_GAMCOR_LUT_DATA, CM_GAMCOR_LUT_DATA, $mask_sh);
        TF_SF!(DSCL0_DSCL_EXT_OVERSCAN_LEFT_RIGHT, EXT_OVERSCAN_LEFT, $mask_sh);
        TF_SF!(DSCL0_DSCL_EXT_OVERSCAN_LEFT_RIGHT, EXT_OVERSCAN_RIGHT, $mask_sh);
        TF_SF!(DSCL0_DSCL_EXT_OVERSCAN_TOP_BOTTOM, EXT_OVERSCAN_BOTTOM, $mask_sh);
        TF_SF!(DSCL0_DSCL_EXT_OVERSCAN_TOP_BOTTOM, EXT_OVERSCAN_TOP, $mask_sh);
        TF2_SF!(DSCL0, LB_DATA_FORMAT__ALPHA_EN, $mask_sh);
        TF_SF!(DSCL0_LB_MEMORY_CTRL, MEMORY_CONFIG, $mask_sh);
        TF_SF!(DSCL0_LB_MEMORY_CTRL, LB_MAX_PARTITIONS, $mask_sh);
        TF_SF!(DSCL0_DSCL_CONTROL, SCL_BOUNDARY_MODE, $mask_sh);
        TF_SF!(DSCL0_SCL_TAP_CONTROL, SCL_V_NUM_TAPS, $mask_sh);
        TF_SF!(DSCL0_SCL_TAP_CONTROL, SCL_H_NUM_TAPS, $mask_sh);
        TF_SF!(DSCL0_SCL_MODE, SCL_COEF_RAM_SELECT, $mask_sh);
        TF_SF!(DSCL0_SCL_MODE, DSCL_MODE, $mask_sh);
        TF_SF!(DPP_TOP0_DPP_CONTROL, DPP_CLOCK_ENABLE, $mask_sh);
        TF_SF!(CM0_CM_CONTROL, CM_BYPASS, $mask_sh);
        TF_SF!(CNVC_CFG0_FORMAT_CONTROL, CNVC_BYPASS, $mask_sh);
        TF2_SF!(CNVC_CFG0, FORMAT_CONTROL__ALPHA_EN, $mask_sh);
        TF_SF!(CNVC_CFG0_CNVC_SURFACE_PIXEL_FORMAT, CNVC_SURFACE_PIXEL_FORMAT, $mask_sh);
        TF_SF!(CURSOR0_0_CURSOR_CONTROL, CURSOR_MODE, $mask_sh);
        TF_SF!(CURSOR0_0_CURSOR_CONTROL, CURSOR_ENABLE, $mask_sh);
        TF_SF!(DSCL0_ISHARP_MODE, ISHARP_EN, $mask_sh);
        TF_SF!(DSCL0_ISHARP_MODE, ISHARP_NOISEDET_EN, $mask_sh);
        TF_SF!(CNVC_CFG0_CNVC_UPSP_MODE, UPSP_MODE, $mask_sh);
        TF_SF!(CNVC_CFG0_CNVC_UPSP_CLAMP, UPSP_CLAMP_MAX, $mask_sh);
        TF_SF!(CNVC_CFG0_CNVC_UPSP_CLAMP, UPSP_CLAMP_MIN, $mask_sh);
    };
}

// The C field-list macro extends the DCN42 list with these DCN60 fields.
macro_rules! dpp_reg_field_list_dcn60 {
    ($ty:ty) => {
        dpp_reg_field_list_dcn42!($ty);
        $ty PRE_GAM_MODE; $ty PRE_REGAM_SELECT; $ty AUTOCAL_FRAC_MODE;
        $ty SCL_BLACK_COLOR_RGB_Y; $ty SCL_BLACK_COLOR_CBCR;
        $ty UPSP_MODE; $ty UPSP_V_NUM_TAPS; $ty UPSP_V_INIT_INT;
        $ty UPSP_V_INIT_FRAC; $ty UPSP_H_NUM_TAPS; $ty UPSP_H_INIT_INT;
        $ty UPSP_H_INIT_FRAC; $ty UPSP_BOUNDARY_MODE;
        $ty UPSP_V_COEF_TAP0_P0; $ty UPSP_V_COEF_TAP1_P0;
        $ty UPSP_V_COEF_TAP2_P0; $ty UPSP_V_COEF_TAP3_P0;
        $ty UPSP_V_COEF_TAP0_P1; $ty UPSP_V_COEF_TAP1_P1;
        $ty UPSP_V_COEF_TAP2_P1; $ty UPSP_V_COEF_TAP3_P1;
        $ty UPSP_H_COEF_TAP0_P0; $ty UPSP_H_COEF_TAP1_P0;
        $ty UPSP_H_COEF_TAP2_P0; $ty UPSP_H_COEF_TAP3_P0;
        $ty UPSP_H_COEF_TAP0_P1; $ty UPSP_H_COEF_TAP1_P1;
        $ty UPSP_H_COEF_TAP2_P1; $ty UPSP_H_COEF_TAP3_P1;
        $ty UPSP_CLAMP_MAX; $ty UPSP_CLAMP_MIN;
    };
}

// DPP_REG_VARIABLE_LIST_DCN60 extends the inherited register variable list.
#[repr(C)]
pub struct dcn60_dpp_registers {
    pub inherited: dcn42_dpp_registers,
    pub pre_gam: u32,
    pub scl_black_color: u32,
    pub upsp_mode: u32,
    pub upsp_v_coef_p0: u32,
    pub upsp_v_coef_p1: u32,
    pub upsp_h_coef_p0: u32,
    pub upsp_h_coef_p1: u32,
    pub upsp_clamp: u32,
}

#[repr(C)]
pub struct dcn60_dpp_shift {
    pub inherited: dcn42_dpp_shift,
    pub pre_gam_mode: u8, pub pre_regam_select: u8, pub autocal_frac_mode: u8,
    pub scl_black_color_rgb_y: u8, pub scl_black_color_cbcr: u8,
    pub upsp_mode: u8, pub upsp_v_num_taps: u8, pub upsp_v_init_int: u8,
    pub upsp_v_init_frac: u8, pub upsp_h_num_taps: u8, pub upsp_h_init_int: u8,
    pub upsp_h_init_frac: u8, pub upsp_boundary_mode: u8,
}

#[repr(C)]
pub struct dcn60_dpp_mask {
    pub inherited: dcn42_dpp_mask,
    pub pre_gam_mode: u32, pub pre_regam_select: u32, pub autocal_frac_mode: u32,
    pub scl_black_color_rgb_y: u32, pub scl_black_color_cbcr: u32,
    pub upsp_mode: u32, pub upsp_v_num_taps: u32, pub upsp_v_init_int: u32,
    pub upsp_v_init_frac: u32, pub upsp_h_num_taps: u32, pub upsp_h_init_int: u32,
    pub upsp_h_init_frac: u32, pub upsp_boundary_mode: u32,
}

#[repr(C)]
pub struct dcn60_dpp {
    pub base: dpp,
    pub tf_regs: *const dcn60_dpp_registers,
    pub tf_shift: *const dcn60_dpp_shift,
    pub tf_mask: *const dcn60_dpp_mask,
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

extern "C" {
    pub fn dpp60_dpp_setup(dpp_base: *mut dpp, format: surface_pixel_format,
        mode: expansion_mode, input_csc_color_matrix: dc_csc_transform,
        input_color_space: dc_color_space, alpha_2bit_lut: *mut cnv_alpha_2bit_lut);
    pub fn dpp60_full_bypass(dpp_base: *mut dpp);
    pub fn dpp60_dscl_set_scaler_manual_scale(dpp_base: *mut dpp, scl_data: *const scaler_data);
    pub fn dpp60_dscl_set_lb(dpp: *mut dcn60_dpp, lb_params: *const line_buffer_params,
        mem_size_config: lb_memory_config);
    pub fn dpp60_dscl_set_manual_ratio_init(dpp: *mut dcn60_dpp, data: *const scaler_data);
    pub fn dpp60_dscl_program_upsp(dpp_base: *mut dpp, dscl_prog_data: *const dscl_prog_data);
    pub fn dpp60_construct(dpp60: *mut dcn60_dpp, ctx: *mut dc_context, inst: u32,
        tf_regs: *const dcn60_dpp_registers, tf_shift: *const dcn60_dpp_shift,
        tf_mask: *const dcn60_dpp_mask) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
