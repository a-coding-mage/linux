/*
 * SPDX-License-Identifier: MIT
 *
 * Copyright (c) 2026 Advanced Micro Devices, Inc. All rights reserved.
 */

// Dependencies supplied by the corresponding DPP headers are intentionally
// left external to this translation.

// C macro: container_of(dpp, struct dcn50_dpp, base)
macro_rules! TO_DCN50_DPP {
    ($dpp:expr) => {{
        unsafe {
            &mut *((($dpp as *mut u8).sub(core::mem::offset_of!(dcn50_dpp, base)))
                as *mut dcn50_dpp)
        }
    }};
}

// C preprocessor field-list macros.  The inherited portions expand from the
// corresponding external DPP header definitions.
// DPP_REG_LIST_SH_MASK_DCN50_COMMON(mask_sh) expands
// DPP_REG_LIST_SH_MASK_DCN401_COMMON(mask_sh), plus:
//   TF_SF(CNVC_CFG0_PRE_GAM, PRE_GAM_MODE, mask_sh)
//   TF_SF(CNVC_CFG0_PRE_GAM, PRE_DEGAM_SELECT, mask_sh)
//   TF_SF(CNVC_CFG0_PRE_GAM, PRE_REGAM_SELECT, mask_sh)
//
// DPP_REG_FIELD_LIST_DCN50(type) expands the DCN401 field list and adds
// PRE_GAM_MODE and PRE_REGAM_SELECT.
// DPP_REG_VARIABLE_LIST_DCN50 expands the DCN401 variable list and adds PRE_GAM.

#[repr(C)]
pub struct dcn50_dpp_registers {
    // DPP_REG_VARIABLE_LIST_DCN401;
    pub PRE_GAM: u32,
}

#[repr(C)]
pub struct dcn50_dpp_shift {
    // DPP_REG_FIELD_LIST_DCN401(u8);
    pub PRE_GAM_MODE: u8,
    pub PRE_REGAM_SELECT: u8,
}

#[repr(C)]
pub struct dcn50_dpp_mask {
    // DPP_REG_FIELD_LIST_DCN401(u32);
    pub PRE_GAM_MODE: u32,
    pub PRE_REGAM_SELECT: u32,
}

#[repr(C)]
pub struct dcn50_dpp {
    pub base: dpp,

    pub tf_regs: *const dcn50_dpp_registers,
    pub tf_shift: *const dcn50_dpp_shift,
    pub tf_mask: *const dcn50_dpp_mask,

    pub filter_v: *const u16,
    pub filter_h: *const u16,
    pub filter_v_c: *const u16,
    pub filter_h_c: *const u16,
    pub lb_pixel_depth_supported: core::ffi::c_int,
    pub lb_memory_size: core::ffi::c_int,
    pub lb_bits_per_entry: core::ffi::c_int,
    pub is_write_to_ram_a_safe: bool,
    pub scl_data: scaler_data,
    pub pwl_data: pwl_params,
}

extern "C" {
    pub fn dpp50_construct(
        dpp50: *mut dcn50_dpp,
        ctx: *mut dc_context,
        inst: u32,
        tf_regs: *const dcn50_dpp_registers,
        tf_shift: *const dcn50_dpp_shift,
        tf_mask: *const dcn50_dpp_mask,
    ) -> bool;

    pub fn dpp50_dpp_setup(
        dpp_base: *mut dpp,
        format: surface_pixel_format,
        mode: expansion_mode,
        input_csc_color_matrix: dc_csc_transform,
        input_color_space: dc_color_space,
        alpha_2bit_lut: *mut cnv_alpha_2bit_lut,
    );

    pub fn dpp50_set_pregam_state(
        dpp_base: *mut dpp,
        tr: dc_transfer_func_predefined,
        scaling: dc_scaling_linearity,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
