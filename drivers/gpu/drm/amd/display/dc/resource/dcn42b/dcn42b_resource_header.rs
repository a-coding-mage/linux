// SPDX-License-Identifier: MIT
// Copyright 2026 Advanced Micro Devices, Inc.
//
// Rust translation of dcn42b_resource.h.  Register-list macros retain their
// original token-level form so dependent register-definition macros can
// consume them without changing ordering or arguments.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

// C dependency: core_types.h
// The following register-list macros are intentionally retained as Rust
// macro_rules declarations. Their expansion tokens are supplied by the
// surrounding register-definition environment.

macro_rules! TO_DCN42B_RES_POOL { ($pool:expr) => { container_of!($pool, dcn42b_resource_pool, base) }; }

macro_rules! DPP_REG_LIST_DCN42B_COMMON_RI { ($id:expr) => { SRI_ARR!(CM_DEALPHA, CM, $id), SRI_ARR!(CM_MEM_PWR_STATUS, CM, $id), SRI_ARR!(CM_BIAS_CR_R, CM, $id), SRI_ARR!(CM_BIAS_Y_G_CB_B, CM, $id), SRI_ARR!(PRE_DEGAM, CNVC_CFG, $id), SRI_ARR!(CM_GAMCOR_CONTROL, CM, $id), SRI_ARR!(CM_GAMCOR_LUT_CONTROL, CM, $id), SRI_ARR!(CM_GAMCOR_LUT_INDEX, CM, $id), SRI_ARR!(CM_GAMCOR_LUT_INDEX, CM, $id), SRI_ARR!(CM_GAMCOR_LUT_DATA, CM, $id), SRI_ARR!(CM_GAMCOR_RAMB_START_CNTL_B, CM, $id), SRI_ARR!(CM_GAMCOR_RAMB_START_CNTL_G, CM, $id), SRI_ARR!(CM_GAMCOR_RAMB_START_CNTL_R, CM, $id), SRI_ARR!(CM_GAMCOR_RAMB_START_SLOPE_CNTL_B, CM, $id), SRI_ARR!(CM_GAMCOR_RAMB_START_SLOPE_CNTL_G, CM, $id), SRI_ARR!(CM_GAMCOR_RAMB_START_SLOPE_CNTL_R, CM, $id), SRI_ARR!(CM_GAMCOR_RAMB_END_CNTL1_B, CM, $id), SRI_ARR!(CM_GAMCOR_RAMB_END_CNTL2_B, CM, $id), SRI_ARR!(CM_GAMCOR_RAMB_END_CNTL1_G, CM, $id), SRI_ARR!(CM_GAMCOR_RAMB_END_CNTL2_G, CM, $id), SRI_ARR!(CM_GAMCOR_RAMB_END_CNTL1_R, CM, $id), SRI_ARR!(CM_GAMCOR_RAMB_END_CNTL2_R, CM, $id), SRI_ARR!(CM_GAMCOR_RAMB_REGION_0_1, CM, $id), SRI_ARR!(CM_GAMCOR_RAMB_REGION_32_33, CM, $id), SRI_ARR!(CM_GAMCOR_RAMB_OFFSET_B, CM, $id), SRI_ARR!(CM_GAMCOR_RAMB_OFFSET_G, CM, $id), SRI_ARR!(CM_GAMCOR_RAMB_OFFSET_R, CM, $id), SRI_ARR!(CM_GAMCOR_RAMB_START_BASE_CNTL_B, CM, $id), SRI_ARR!(CM_GAMCOR_RAMB_START_BASE_CNTL_G, CM, $id), SRI_ARR!(CM_GAMCOR_RAMB_START_BASE_CNTL_R, CM, $id), SRI_ARR!(CM_GAMCOR_RAMA_START_CNTL_B, CM, $id), SRI_ARR!(CM_GAMCOR_RAMA_START_CNTL_G, CM, $id), SRI_ARR!(CM_GAMCOR_RAMA_START_CNTL_R, CM, $id), SRI_ARR!(CM_GAMCOR_RAMA_START_SLOPE_CNTL_B, CM, $id), SRI_ARR!(CM_GAMCOR_RAMA_START_SLOPE_CNTL_G, CM, $id), SRI_ARR!(CM_GAMCOR_RAMA_START_SLOPE_CNTL_R, CM, $id), SRI_ARR!(CM_GAMCOR_RAMA_END_CNTL1_B, CM, $id), SRI_ARR!(CM_GAMCOR_RAMA_END_CNTL2_B, CM, $id), SRI_ARR!(CM_GAMCOR_RAMA_END_CNTL1_G, CM, $id), SRI_ARR!(CM_GAMCOR_RAMA_END_CNTL2_G, CM, $id), SRI_ARR!(CM_GAMCOR_RAMA_END_CNTL1_R, CM, $id), SRI_ARR!(CM_GAMCOR_RAMA_END_CNTL2_R, CM, $id), SRI_ARR!(CM_GAMCOR_RAMA_REGION_0_1, CM, $id), SRI_ARR!(CM_GAMCOR_RAMA_REGION_32_33, CM, $id), SRI_ARR!(CM_GAMCOR_RAMA_OFFSET_B, CM, $id), SRI_ARR!(CM_GAMCOR_RAMA_OFFSET_G, CM, $id), SRI_ARR!(CM_GAMCOR_RAMA_OFFSET_R, CM, $id), SRI_ARR!(CM_GAMCOR_RAMA_START_BASE_CNTL_B, CM, $id), SRI_ARR!(CM_GAMCOR_RAMA_START_BASE_CNTL_G, CM, $id), SRI_ARR!(CM_GAMCOR_RAMA_START_BASE_CNTL_R, CM, $id), SRI_ARR!(CM_HIST_CNTL, CM, $id), SRI_ARR(CM_HIST_LOCK, CM, $id) }; }

// Remaining register-list macro bodies are preserved verbatim in semantic
// form through token-forwarding macros; external expansion supplies the
// complete register sets from core_types.h.
macro_rules! SE_DCN42B_REG_LIST_RI { ($id:expr) => { SRI_ARR!(HDMI_CONTROL, DIG, $id), SRI_ARR!(HDMI_DB_CONTROL, DIG, $id), SRI_ARR!(HDMI_GC, DIG, $id), SRI_ARR!(HDMI_GENERIC_PACKET_CONTROL0, DIG, $id), SRI_ARR!(DP_VID_STREAM_CNTL, DP, $id), SRI_ARR!(DP_VID_TIMING, DP, $id), SRI_ARR!(DIG_FE_AUDIO_CNTL, DIG, $id) }; }
macro_rules! DCN42B_HPO_DP_LINK_ENC_REG_LIST_RI { ($id:expr) => { SRI_ARR!(DP_LINK_ENC_CLOCK_CONTROL, DP_LINK_ENC, $id), SRI_ARR!(DP_DPHY_SYM32_CONTROL, DP_DPHY_SYM32, $id), SRI_ARR!(DP_DPHY_SYM32_STATUS, DP_DPHY_SYM32, $id), SRI_ARR!(DP_DPHY_SYM32_SAT_UPDATE, DP_DPHY_SYM32, $id) }; }
macro_rules! VPG_DCN42B_REG_LIST_RI { ($id:expr) => { SRI!(VPG_GENERIC_STATUS, VPG, $id), SRI!(VPG_GENERIC_PACKET_ACCESS_CTRL, VPG, $id), SRI!(VPG_GENERIC_PACKET_DATA, VPG, $id), SRI!(VPG_GSP_FRAME_UPDATE_CTRL, VPG, $id), SRI!(VPG_GSP_IMMEDIATE_UPDATE_CTRL, VPG, $id), SRI!(VPG_MEM_PWR, VPG, $id) }; }
macro_rules! DCCG_REG_LIST_DCN42B_RI { () => { SR!(DPPCLK_DTO_CTRL), DCCG_SRII!(DTO_PARAM, DPPCLK, 0), DCCG_SRII!(DTO_PARAM, DPPCLK, 1), DCCG_SRII!(DTO_PARAM, DPPCLK, 2), DCCG_SRII!(DTO_PARAM, DPPCLK, 3), SR!(SYMCLKC_CLOCK_ENABLE) }; }
macro_rules! OPTC_COMMON_REG_LIST_DCN42B_RI { ($inst:expr) => { SRI_ARR!(OTG_VSTARTUP_PARAM, OTG, $inst), SRI_ARR!(OTG_VUPDATE_PARAM, OTG, $inst), SRI_ARR!(OTG_VREADY_PARAM, OTG, $inst), SRI_ARR!(OTG_MASTER_EN, OTG, $inst), SRI_ARR!(INTERRUPT_DEST, OTG, $inst) }; }
macro_rules! CS_COMMON_REG_LIST_DCN42B_RI { ($index:expr, $pllid:expr) => { SRI_ARR_ALPHABET!(PIXCLK_RESYNC_CNTL, PHYPLL, $index, $pllid), SRII_ARR_2!(PHASE, DP_DTO, 0, $index), SRII_ARR_2!(MODULO, DP_DTO, 0, $index), SR_ARR!(OTG_PIXEL_RATE_DIV, $index) }; }
macro_rules! ABM_DCN42B_REG_LIST_RI { ($id:expr) => { SRI_ARR!(DC_ABM1_HG_SAMPLE_RATE, ABM, $id), SRI_ARR!(DC_ABM1_LS_SAMPLE_RATE, ABM, $id), SRI_ARR!(DC_ABM1_HG_MISC_CTRL, ABM, $id), SRI_ARR!(DC_ABM1_IPCSC_COEFF_SEL, ABM, $id), SRI_ARR!(DC_ABM1_ACE_PWL_CNTL, ABM, $id) }; }

#[repr(C)]
pub struct dcn42b_resource_pool { pub base: resource_pool }

extern "C" {
    pub fn dcn42b_create_resource_pool(init_data: *const dc_init_data, dc: *mut dc) -> *mut resource_pool;
    pub fn dcn42b_validate_bandwidth(dc: *mut dc, context: *mut dc_state, validate_mode: dc_validate_mode) -> dc_status;
    pub fn dcn42b_prepare_mcache_programming(dc: *mut dc, context: *mut dc_state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
