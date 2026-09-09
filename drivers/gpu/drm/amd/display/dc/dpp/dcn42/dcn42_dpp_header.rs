/* SPDX-License-Identifier: MIT */
/* Copyright 2026 Advanced Micro Devices, Inc. */

// Translation of dcn42_dpp.h.  The register-field list macros are retained as
// token-level Rust macros because their expansion is supplied by the display
// controller register-definition layer.

#[macro_export]
macro_rules! TO_DCN42_DPP {
    ($dpp:expr) => { container_of!($dpp, dcn42_dpp, base) };
}

// DPP_REG_LIST_SH_MASK_DCN42_COMMON consists of the complete register-field
// table from the C header; TF_SF/TF2_SF are external register-table macros.
#[macro_export]
macro_rules! DPP_REG_LIST_SH_MASK_DCN42_COMMON {
    ($mask_sh:expr) => {
        dpp_reg_list_sh_mask_dcn42_common!($mask_sh)
    };
}

#[macro_export]
macro_rules! DPP_REG_FIELD_LIST_DCN42 {
    ($type:ty) => {
        DPP_REG_FIELD_LIST_DCN401!($type);
        $type CM_HIST_SEL;
        $type CM_HIST_CH_EN;
        $type CM_HIST_SRC1_SEL;
        $type CM_HIST_SRC2_SEL;
        $type CM_HIST_SRC3_SEL;
        $type CM_HIST_CH1_XBAR;
        $type CM_HIST_CH2_XBAR;
        $type CM_HIST_CH3_XBAR;
        $type CM_HIST_FORMAT;
        $type CM_HIST_READ_CHANNEL_MASK;
        $type CM_HIST_LOCK;
        $type CM_HIST_INDEX;
        $type CM_HIST_DATA;
        $type CM_HIST_BUFA_RDY_STATUS;
        $type CM_HIST_BUFB_RDY_STATUS;
        $type CM_HIST_SCALE_SRC1;
        $type CM_HIST_COEFA_SRC2;
        $type CM_HIST_COEFB_SRC2;
        $type CM_HIST_COEFC_SRC2;
        $type CM_HIST_SCALE_SRC3;
        $type CM_HIST_BIAS_SRC1;
        $type CM_HIST_BIAS_SRC2;
        $type CM_HIST_BIAS_SRC3;
    };
}

#[repr(C)]
pub struct dcn42_dpp_registers {
    pub base: dcn401_dpp_registers,
    pub ALPHA_2BIT_LUT01: u32,
    pub ALPHA_2BIT_LUT23: u32,
    pub CM_HIST_CNTL: u32,
    pub CM_HIST_INDEX: u32,
    pub CM_HIST_LOCK: u32,
    pub CM_HIST_DATA: u32,
    pub CM_HIST_STATUS: u32,
    pub CM_HIST_SCALE_SRC1: u32,
    pub CM_HIST_COEFA_SRC2: u32,
    pub CM_HIST_COEFB_SRC2: u32,
    pub CM_HIST_COEFC_SRC2: u32,
    pub CM_HIST_SCALE_SRC3: u32,
    pub CM_HIST_BIAS_SRC1: u32,
    pub CM_HIST_BIAS_SRC2: u32,
    pub CM_HIST_BIAS_SRC3: u32,
}

#[repr(C)]
pub struct dcn42_dpp_shift { pub base: dcn401_dpp_shift }

#[repr(C)]
pub struct dcn42_dpp_mask { pub base: dcn401_dpp_mask }

#[repr(C)]
pub struct dcn42_dpp {
    pub base: dpp,
    pub tf_regs: *const dcn42_dpp_registers,
    pub tf_shift: *const dcn42_dpp_shift,
    pub tf_mask: *const dcn42_dpp_mask,
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
    pub fn dpp42_construct(
        dpp42: *mut dcn42_dpp, ctx: *mut dc_context, inst: u32,
        tf_regs: *const dcn42_dpp_registers, tf_shift: *const dcn42_dpp_shift,
        tf_mask: *const dcn42_dpp_mask,
    ) -> bool;
    pub fn dpp42_dpp_cm_hist_control(
        dpp_base: *mut dpp, cntl: cm_hist_control, color_space: dc_color_space,
    );
    pub fn dpp42_dpp_cm_hist_read(dpp_base: *mut dpp, hist_out: *mut cm_hist) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
