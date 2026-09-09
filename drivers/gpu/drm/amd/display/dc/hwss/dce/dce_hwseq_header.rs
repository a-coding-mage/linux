/*
 * Copyright 2016, 2026 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// The register-list and mask/shift-list preprocessor macros from the C header
// are retained as declarative Rust macro names. Their register symbols are
// supplied by the generated hardware-register dependencies.
macro_rules! HWSEQ_DCEF_REG_LIST_DCE8 { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCEF_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_BLND_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HSWEQ_DCN_PIXEL_RATE_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_PIXEL_RATE_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_PIXEL_RATE_REG_LIST_201 { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_PHYPLL_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_PIXEL_RATE_REG_LIST_3 { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_PHYPLL_REG_LIST_3 { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_PIXEL_RATE_REG_LIST_302 { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_PHYPLL_REG_LIST_302 { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_PIXEL_RATE_REG_LIST_303 { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_PHYPLL_REG_LIST_303 { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_PHYPLL_REG_LIST_201 { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCE6_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCE8_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCE10_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_ST_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_CZ_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCE120_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_VG20_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCE112_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! MMHUB_DCN_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN1_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN2_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN21_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN201_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN30_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN301_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN302_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN303_REG_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWS_SF { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWS_SF1 { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCEF_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_BLND_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_PIXEL_RATE_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_PHYPLL_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCE6_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCE8_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCE10_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCE11_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCE112_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_GFX9_DCHUB_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCE12_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_VG20_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN1_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN2_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN21_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN201_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN30_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN301_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN302_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN303_MASK_SH_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_REG_FIELD_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN_REG_FIELD_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN3_REG_FIELD_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN301_REG_FIELD_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN31_REG_FIELD_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN35_REG_FIELD_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN401_REG_FIELD_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN42_REG_FIELD_LIST { ($($t:tt)*) => { $($t)* }; }
macro_rules! HWSEQ_DCN60_REG_FIELD_LIST { ($($t:tt)*) => { $($t)* }; }

#[repr(C)]
pub struct dce_hwseq_registers {
    pub DCFE_CLOCK_CONTROL: [u32; 6], pub DCFEV_CLOCK_CONTROL: u32,
    pub DC_MEM_GLOBAL_PWR_REQ_CNTL: u32, pub BLND_V_UPDATE_LOCK: [u32; 6],
    pub BLND_CONTROL: [u32; 6], pub BLNDV_CONTROL: u32,
    pub CRTC_H_BLANK_START_END: [u32; 6], pub PIXEL_RATE_CNTL: [u32; 6],
    pub PHYPLL_PIXEL_RATE_CNTL: [u32; 6],
    pub DCHUB_FB_LOCATION: u32, pub DCHUB_AGP_BASE: u32, pub DCHUB_AGP_BOT: u32, pub DCHUB_AGP_TOP: u32,
    pub REFCLK_CNTL: u32, pub DCHUBBUB_GLOBAL_TIMER_CNTL: u32,
    pub DCHUBBUB_SDPIF_FB_BASE: u32, pub DCHUBBUB_SDPIF_FB_OFFSET: u32,
    pub DCHUBBUB_SDPIF_AGP_BASE: u32, pub DCHUBBUB_SDPIF_AGP_BOT: u32, pub DCHUBBUB_SDPIF_AGP_TOP: u32,
    pub DC_IP_REQUEST_CNTL: u32,
    pub DOMAIN0_PG_CONFIG: u32, pub DOMAIN1_PG_CONFIG: u32, pub DOMAIN2_PG_CONFIG: u32,
    pub DOMAIN3_PG_CONFIG: u32, pub DOMAIN4_PG_CONFIG: u32, pub DOMAIN5_PG_CONFIG: u32,
    pub DOMAIN6_PG_CONFIG: u32, pub DOMAIN7_PG_CONFIG: u32, pub DOMAIN8_PG_CONFIG: u32,
    pub DOMAIN9_PG_CONFIG: u32, pub DOMAIN10_PG_CONFIG: u32, pub DOMAIN11_PG_CONFIG: u32,
    pub DOMAIN16_PG_CONFIG: u32, pub DOMAIN17_PG_CONFIG: u32, pub DOMAIN18_PG_CONFIG: u32,
    pub DOMAIN19_PG_CONFIG: u32, pub DOMAIN20_PG_CONFIG: u32, pub DOMAIN21_PG_CONFIG: u32,
    pub DOMAIN0_PG_STATUS: u32, pub DOMAIN1_PG_STATUS: u32, pub DOMAIN2_PG_STATUS: u32,
    pub DOMAIN3_PG_STATUS: u32, pub DOMAIN4_PG_STATUS: u32, pub DOMAIN5_PG_STATUS: u32,
    pub DOMAIN6_PG_STATUS: u32, pub DOMAIN7_PG_STATUS: u32, pub DOMAIN8_PG_STATUS: u32,
    pub DOMAIN9_PG_STATUS: u32, pub DOMAIN10_PG_STATUS: u32, pub DOMAIN11_PG_STATUS: u32,
    pub DOMAIN16_PG_STATUS: u32, pub DOMAIN17_PG_STATUS: u32, pub DOMAIN18_PG_STATUS: u32,
    pub DOMAIN19_PG_STATUS: u32, pub DOMAIN20_PG_STATUS: u32, pub DOMAIN21_PG_STATUS: u32,
    pub DIO_MEM_PWR_CTRL: u32, pub DCCG_GATE_DISABLE_CNTL: u32, pub DCCG_GATE_DISABLE_CNTL2: u32,
    pub DCFCLK_CNTL: u32, pub MICROSECOND_TIME_BASE_DIV: u32, pub MILLISECOND_TIME_BASE_DIV: u32,
    pub DISPCLK_FREQ_CHANGE_CNTL: u32, pub RBBMIF_TIMEOUT_DIS: u32, pub RBBMIF_TIMEOUT_DIS_2: u32,
    pub DCHUBBUB_CRC_CTRL: u32, pub DPP_TOP0_DPP_CRC_CTRL: u32, pub DPP_TOP0_DPP_CRC_VAL_R_G: u32,
    pub DPP_TOP0_DPP_CRC_VAL_B_A: u32, pub DPP_TOP0_DPP_CRC_VAL_R: u32, pub DPP_TOP0_DPP_CRC_VAL_G: u32,
    pub DPP_TOP0_DPP_CRC_VAL_B: u32, pub DPP_TOP0_DPP_CRC_VAL_A: u32, pub MPC_CRC_CTRL: u32,
    pub MPC_CRC_RESULT_GB: u32, pub MPC_CRC_RESULT_C: u32, pub MPC_CRC_RESULT_AR: u32,
    pub MPC_CRC_RESULT_R: u32, pub MPC_CRC_RESULT_G: u32, pub MPC_CRC_RESULT_B: u32, pub MPC_CRC_RESULT_A: u32,
    pub D1VGA_CONTROL: u32, pub D2VGA_CONTROL: u32, pub D3VGA_CONTROL: u32, pub D4VGA_CONTROL: u32,
    pub D5VGA_CONTROL: u32, pub D6VGA_CONTROL: u32, pub VGA_TEST_CONTROL: u32,
    pub VM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32: u32, pub VM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32: u32,
    pub VM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32: u32, pub VM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32: u32,
    pub VM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32: u32, pub VM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32: u32,
    pub VM_L2_PROTECTION_FAULT_DEFAULT_ADDR_HI32: u32, pub VM_L2_PROTECTION_FAULT_DEFAULT_ADDR_LO32: u32,
    pub MC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB: u32, pub MC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB: u32,
    pub MC_VM_SYSTEM_APERTURE_LOW_ADDR: u32, pub MC_VM_SYSTEM_APERTURE_HIGH_ADDR: u32,
    pub MC_VM_XGMI_LFB_CNTL: u32, pub AZALIA_AUDIO_DTO: u32, pub AZALIA_CONTROLLER_CLOCK_GATING: u32,
    pub MC_VM_FB_LOCATION_BASE: u32, pub MC_VM_FB_LOCATION_TOP: u32, pub MC_VM_FB_OFFSET: u32,
    pub MMHUBBUB_MEM_PWR_CNTL: u32, pub HPO_TOP_CLOCK_CONTROL: u32, pub ODM_MEM_PWR_CTRL3: u32,
    pub DMU_MEM_PWR_CNTL: u32, pub DCHUBBUB_ARB_HOSTVM_CNTL: u32, pub HPO_TOP_HW_CONTROL: u32,
    pub DMU_CLK_CNTL: u32, pub DCCG_GATE_DISABLE_CNTL4: u32, pub DCCG_GATE_DISABLE_CNTL5: u32,
    pub DOMAIN22_PG_CONFIG: u32, pub DOMAIN23_PG_CONFIG: u32, pub DOMAIN24_PG_CONFIG: u32, pub DOMAIN25_PG_CONFIG: u32,
    pub DOMAIN22_PG_STATUS: u32, pub DOMAIN23_PG_STATUS: u32, pub DOMAIN24_PG_STATUS: u32, pub DOMAIN25_PG_STATUS: u32,
    pub DOMAIN26_PG_CONFIG: u32, pub DOMAIN26_PG_STATUS: u32, pub HDCP_INTERRUPT_DEST: u32,
}

#[repr(C)]
pub struct dce_hwseq_shift { pub fields: [u8; 0] }
#[repr(C)]
pub struct dce_hwseq_mask { pub fields: [u32; 0] }

#[repr(i32)]
pub enum blnd_mode {
    BLND_MODE_CURRENT_PIPE = 0,
    BLND_MODE_OTHER_PIPE,
    BLND_MODE_BLENDING,
}

#[repr(C)] pub struct dce_hwseq { _private: [u8; 0] }
#[repr(C)] pub struct pipe_ctx { _private: [u8; 0] }
#[repr(C)] pub struct clock_source { _private: [u8; 0] }
#[repr(C)] pub struct dc { _private: [u8; 0] }

extern "C" {
    pub fn dce_enable_fe_clock(hwss: *mut dce_hwseq, inst: ::core::ffi::c_uint, enable: bool);
    pub fn dce_pipe_control_lock(dc: *mut dc, pipe: *mut pipe_ctx, lock: bool);
    pub fn dce_set_blender_mode(hws: *mut dce_hwseq, blnd_inst: ::core::ffi::c_uint, mode: blnd_mode);
    pub fn dce60_pipe_control_lock(dc: *mut dc, pipe: *mut pipe_ctx, lock: bool);
    pub fn dce_clock_gating_power_up(hws: *mut dce_hwseq, enable: bool);
    pub fn dce_crtc_switch_to_clk_src(hws: *mut dce_hwseq, clk_src: *mut clock_source, tg_inst: ::core::ffi::c_uint);
    pub fn dce_use_lut(format: ::core::ffi::c_int) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
