/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
 *
 * Rust translation of dcn35_dccg.h.  The dcn314 declarations and the
 * register/field-list helper macros are supplied by the surrounding build.
 */

// Dependency equivalent of: #include "dcn314/dcn314_dccg.h"

#[macro_export]
macro_rules! DCCG_SFII {
    ($block:ident, $reg_name:ident, $field_prefix:ident, $field_name:ident, $inst:expr, $post_fix:ident) => {
        $crate::DCCG_SFII_ENTRY!($block, $reg_name, $field_prefix, $field_name, $inst, $post_fix)
    };
}

#[macro_export]
macro_rules! DCCG_REG_LIST_DCN35 {
    () => {
        DCCG_REG_LIST_DCN314!(),
        SR!(DPPCLK_CTRL),
        SR!(DCCG_GATE_DISABLE_CNTL4),
        SR!(DCCG_GATE_DISABLE_CNTL5),
        SR!(DCCG_GATE_DISABLE_CNTL6),
        SR!(DCCG_GLOBAL_FGCG_REP_CNTL),
        SR!(SYMCLKA_CLOCK_ENABLE), SR!(SYMCLKB_CLOCK_ENABLE),
        SR!(SYMCLKC_CLOCK_ENABLE), SR!(SYMCLKD_CLOCK_ENABLE),
        SR!(SYMCLKE_CLOCK_ENABLE), SR!(SYMCLK_PSP_CNTL)
    };
}

// The following field list is intentionally retained as a token macro: its
// entries are consumed by the register-generation macros provided by dcn314.
#[macro_export]
macro_rules! DCCG_MASK_SH_LIST_DCN35 {
    ($mask_sh:ident) => {
        DCCG_SFI!(DPPCLK_DTO_CTRL, DTO_DB_EN, DPPCLK, 0, $mask_sh),
        DCCG_SFI!(DPPCLK_DTO_CTRL, DTO_DB_EN, DPPCLK, 1, $mask_sh),
        DCCG_SFI!(DPPCLK_DTO_CTRL, DTO_DB_EN, DPPCLK, 2, $mask_sh),
        DCCG_SFI!(DPPCLK_DTO_CTRL, DTO_DB_EN, DPPCLK, 3, $mask_sh),
        DCCG_SF!(DPPCLK_CTRL, DPPCLK0_EN, $mask_sh),
        DCCG_SF!(DPPCLK_CTRL, DPPCLK1_EN, $mask_sh),
        DCCG_SF!(DPPCLK_CTRL, DPPCLK2_EN, $mask_sh),
        DCCG_SF!(DPPCLK_CTRL, DPPCLK3_EN, $mask_sh),
        DCCG_SF!(DPPCLK0_DTO_PARAM, DPPCLK0_DTO_PHASE, $mask_sh),
        DCCG_SF!(DPPCLK0_DTO_PARAM, DPPCLK0_DTO_MODULO, $mask_sh),
        DCCG_SF!(HDMICHARCLK0_CLOCK_CNTL, HDMICHARCLK0_EN, $mask_sh),
        DCCG_SF!(HDMICHARCLK0_CLOCK_CNTL, HDMICHARCLK0_SRC_SEL, $mask_sh),
        DCCG_SF!(PHYASYMCLK_CLOCK_CNTL, PHYASYMCLK_EN, $mask_sh),
        DCCG_SF!(PHYASYMCLK_CLOCK_CNTL, PHYASYMCLK_SRC_SEL, $mask_sh),
        DCCG_SF!(PHYBSYMCLK_CLOCK_CNTL, PHYBSYMCLK_EN, $mask_sh),
        DCCG_SF!(PHYBSYMCLK_CLOCK_CNTL, PHYBSYMCLK_SRC_SEL, $mask_sh),
        DCCG_SF!(PHYCSYMCLK_CLOCK_CNTL, PHYCSYMCLK_EN, $mask_sh),
        DCCG_SF!(PHYCSYMCLK_CLOCK_CNTL, PHYCSYMCLK_SRC_SEL, $mask_sh),
        DCCG_SF!(PHYDSYMCLK_CLOCK_CNTL, PHYDSYMCLK_EN, $mask_sh),
        DCCG_SF!(PHYDSYMCLK_CLOCK_CNTL, PHYDSYMCLK_SRC_SEL, $mask_sh),
        DCCG_SF!(DPSTREAMCLK_CNTL, DPSTREAMCLK0_EN, $mask_sh),
        DCCG_SF!(DPSTREAMCLK_CNTL, DPSTREAMCLK1_EN, $mask_sh),
        DCCG_SF!(DPSTREAMCLK_CNTL, DPSTREAMCLK2_EN, $mask_sh),
        DCCG_SF!(DPSTREAMCLK_CNTL, DPSTREAMCLK3_EN, $mask_sh),
        DCCG_SF!(DPSTREAMCLK_CNTL, DPSTREAMCLK0_SRC_SEL, $mask_sh),
        DCCG_SF!(DPSTREAMCLK_CNTL, DPSTREAMCLK1_SRC_SEL, $mask_sh),
        DCCG_SF!(DPSTREAMCLK_CNTL, DPSTREAMCLK2_SRC_SEL, $mask_sh),
        DCCG_SF!(DPSTREAMCLK_CNTL, DPSTREAMCLK3_SRC_SEL, $mask_sh),
        DCCG_SF!(HDMISTREAMCLK_CNTL, HDMISTREAMCLK0_EN, $mask_sh),
        DCCG_SF!(HDMISTREAMCLK_CNTL, HDMISTREAMCLK0_SRC_SEL, $mask_sh),
        DCCG_SF!(DSCCLK_DTO_CTRL, DSCCLK0_EN, $mask_sh),
        DCCG_SF!(DSCCLK_DTO_CTRL, DSCCLK1_EN, $mask_sh),
        DCCG_SF!(DSCCLK_DTO_CTRL, DSCCLK2_EN, $mask_sh),
        DCCG_SF!(DSCCLK_DTO_CTRL, DSCCLK3_EN, $mask_sh),
        DCCG_SF!(DSCCLK0_DTO_PARAM, DSCCLK0_DTO_PHASE, $mask_sh),
        DCCG_SF!(DSCCLK0_DTO_PARAM, DSCCLK0_DTO_MODULO, $mask_sh),
        DCCG_SF!(DSCCLK1_DTO_PARAM, DSCCLK1_DTO_PHASE, $mask_sh),
        DCCG_SF!(DSCCLK1_DTO_PARAM, DSCCLK1_DTO_MODULO, $mask_sh),
        DCCG_SF!(DSCCLK2_DTO_PARAM, DSCCLK2_DTO_PHASE, $mask_sh),
        DCCG_SF!(DSCCLK2_DTO_PARAM, DSCCLK2_DTO_MODULO, $mask_sh),
        DCCG_SF!(DSCCLK3_DTO_PARAM, DSCCLK3_DTO_PHASE, $mask_sh),
        DCCG_SF!(DSCCLK3_DTO_PARAM, DSCCLK3_DTO_MODULO, $mask_sh),
        DCCG_SF!(DCCG_GLOBAL_FGCG_REP_CNTL, DCCG_GLOBAL_FGCG_REP_DIS, $mask_sh)
    };
}

extern "C" {
    pub fn dccg35_create(ctx: *mut dc_context, regs: *const dccg_registers,
        dccg_shift: *const dccg_shift, dccg_mask: *const dccg_mask) -> *mut dccg;
    pub fn dccg35_init(dccg: *mut dccg);
    pub fn dccg35_trigger_dio_fifo_resync(dccg: *mut dccg);
    pub fn dccg35_update_dpp_dto(dccg: *mut dccg, dpp_inst: ::core::ffi::c_int, req_dppclk: ::core::ffi::c_int);
    pub fn dccg35_enable_global_fgcg_rep(dccg: *mut dccg, value: bool);
    pub fn dccg35_root_gate_disable_control(dccg: *mut dccg, pipe_idx: u32, disable_clock_gating: u32);
    pub fn dccg35_set_dpstreamclk_root_clock_gating(dccg: *mut dccg, dp_hpo_inst: ::core::ffi::c_int, enable: bool);
    pub fn dccg35_set_hdmistreamclk_root_clock_gating(dccg: *mut dccg, enable: bool);
    pub fn dccg35_dpp_root_clock_control(dccg: *mut dccg, dpp_inst: u32, clock_on: bool);
    pub fn dccg35_disable_symclk32_se(dccg: *mut dccg, hpo_se_inst: ::core::ffi::c_int);
    pub fn dccg35_enable_dscclk(dccg: *mut dccg, inst: ::core::ffi::c_int);
    pub fn dccg35_disable_dscclk(dccg: *mut dccg, inst: ::core::ffi::c_int);
    pub fn dccg35_enable_symclk_se(dccg: *mut dccg, stream_enc_inst: u32, link_enc_inst: u32);
    pub fn dccg35_disable_symclk_se(dccg: *mut dccg, stream_enc_inst: u32, link_enc_inst: u32);
    pub fn dccg35_set_hdmistreamclk(dccg: *mut dccg, src: streamclk_source, otg_inst: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
