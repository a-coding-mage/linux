/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2026 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependency: os_types.h
// The PMFW_DRIVER_IF_H conditional is preserved by defining these declarations
// directly; build systems may gate them externally as appropriate.

pub const PMFW_DRIVER_IF_VERSION: u32 = 7;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DSPCLK_e {
    DSPCLK_DCFCLK = 0,
    DSPCLK_DISPCLK,
    DSPCLK_PIXCLK,
    DSPCLK_PHYCLK,
    DSPCLK_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayClockTable_t {
    pub Freq: u16, // in MHz
    pub Vid: u16, // min voltage in SVI3 VID
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WatermarkRowGeneric_t {
    pub MinClock: u16,
    pub MaxClock: u16,
    pub MinMclk: u16,
    pub MaxMclk: u16,
    pub WmSetting: u8,
    pub WmType: u8,
    pub Padding: [u8; 2],
}

pub const NUM_WM_RANGES: usize = 4;
pub const WM_PSTATE_CHG: u32 = 0;
pub const WM_RETRAINING: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WM_CLOCK_e {
    WM_SOCCLK = 0,
    WM_DCFCLK,
    WM_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Watermarks_t {
    pub WatermarkRow: [[WatermarkRowGeneric_t; NUM_WM_RANGES]; 2],
    pub MmHubPadding: [u32; 7],
}

pub const NUM_DCFCLK_DPM_LEVELS: usize = 8;
pub const NUM_DISPCLK_DPM_LEVELS: usize = 8;
pub const NUM_DPPCLK_DPM_LEVELS: usize = 8;
pub const NUM_SOCCLK_DPM_LEVELS: usize = 8;
pub const NUM_VCN_DPM_LEVELS: usize = 8;
pub const NUM_SOC_VOLTAGE_LEVELS: usize = 8;
pub const NUM_VPE_DPM_LEVELS: usize = 8;
pub const NUM_FCLK_DPM_LEVELS: usize = 8;
pub const NUM_MEM_PSTATE_LEVELS: usize = 4;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WCK_RATIO_e {
    WCK_RATIO_1_1 = 0,
    WCK_RATIO_1_2,
    WCK_RATIO_1_4,
    WCK_RATIO_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemPstateTable_t {
    pub UClk: u32,
    pub MemClk: u32,
    pub Voltage: u32,
    pub WckRatio: u8,
    pub Spare: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DpmClocks_t_dcn42 {
    pub DcfClocks: [u32; NUM_DCFCLK_DPM_LEVELS],
    pub DispClocks: [u32; NUM_DISPCLK_DPM_LEVELS],
    pub DppClocks: [u32; NUM_DPPCLK_DPM_LEVELS],
    pub SocClocks: [u32; NUM_SOCCLK_DPM_LEVELS],
    pub VClocks: [u32; NUM_VCN_DPM_LEVELS],
    pub DClocks: [u32; NUM_VCN_DPM_LEVELS],
    pub VPEClocks: [u32; NUM_VPE_DPM_LEVELS],
    pub FclkClocks_Freq: [u32; NUM_FCLK_DPM_LEVELS],
    pub FclkClocks_Voltage: [u32; NUM_FCLK_DPM_LEVELS],
    pub SocVoltage: [u32; NUM_SOC_VOLTAGE_LEVELS],
    pub MemPstateTable: [MemPstateTable_t; NUM_MEM_PSTATE_LEVELS],
    pub NumDcfClkLevelsEnabled: u8,
    pub NumDispClkLevelsEnabled: u8,
    pub NumSocClkLevelsEnabled: u8,
    pub VcnClkLevelsEnabled: u8,
    pub VpeClkLevelsEnabled: u8,
    pub NumMemPstatesEnabled: u8,
    pub NumFclkLevelsEnabled: u8,
    pub spare: u8,
    pub MinGfxClk: u32,
    pub MaxGfxClk: u32,
}

pub const TABLE_BIOS_IF: u32 = 0;
pub const TABLE_WATERMARKS: u32 = 1;
pub const TABLE_CUSTOM_DPM: u32 = 2;
pub const TABLE_SPARE1: u32 = 3;
pub const TABLE_DPMCLOCKS: u32 = 4;
pub const TABLE_MOMENTARY_PM: u32 = 5;
pub const TABLE_MODERN_STDBY: u32 = 6;
pub const TABLE_SMU_METRICS: u32 = 7;
pub const TABLE_COUNT: u32 = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn42_watermarks {
    pub WatermarkRow: [[WatermarkRowGeneric_t; NUM_WM_RANGES]; 2],
    pub MmHubPadding: [u32; 7],
}

#[repr(C)]
pub struct dcn42_smu_dpm_clks {
    pub dpm_clks: *mut DpmClocks_t_dcn42,
    pub mc_address: large_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct display_idle_optimization {
    // C bitfields occupy one unsigned int. Bits 0, 1, and 2 correspond to
    // df_request_disabled, phy_ref_clk_off, and s0i2_rdy; bits 3..31 are reserved.
    pub bits: u32,
}

#[repr(C)]
pub union display_idle_optimization_u {
    pub idle_info: display_idle_optimization,
    pub data: u32,
}

extern "C" {
    pub fn dcn42_smu_get_pmfw_version(clk_mgr: *mut clk_mgr_internal) -> i32;
    pub fn dcn42_smu_set_dispclk(clk_mgr: *mut clk_mgr_internal, requested_dispclk_khz: i32) -> i32;
    pub fn dcn42_smu_set_hard_min_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_dcfclk_khz: i32) -> i32;
    pub fn dcn42_smu_set_min_deep_sleep_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_min_ds_dcfclk_khz: i32) -> i32;
    pub fn dcn42_smu_set_dppclk(clk_mgr: *mut clk_mgr_internal, requested_dpp_khz: i32) -> i32;
    pub fn dcn42_smu_set_display_idle_optimization(clk_mgr: *mut clk_mgr_internal, idle_info: u32);
    pub fn dcn42_smu_enable_phy_refclk_pwrdwn(clk_mgr: *mut clk_mgr_internal, enable: bool);
    pub fn dcn42_smu_enable_pme_wa(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn42_smu_set_dram_addr_high(clk_mgr: *mut clk_mgr_internal, addr_high: u32);
    pub fn dcn42_smu_set_dram_addr_low(clk_mgr: *mut clk_mgr_internal, addr_low: u32);
    pub fn dcn42_smu_transfer_dpm_table_smu_2_dram(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn42_smu_transfer_wm_table_dram_2_smu(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn42_smu_set_zstate_support(clk_mgr: *mut clk_mgr_internal, support: dcn_zstate_support_state);
    pub fn dcn42_smu_set_dtbclk(clk_mgr: *mut clk_mgr_internal, enable: bool);
    pub fn dcn42_vbios_smu_enable_48mhz_tmdp_refclk_pwrdwn(clk_mgr: *mut clk_mgr_internal, enable: bool);
    pub fn dcn42_smu_get_dtbclk(clk_mgr: *mut clk_mgr_internal) -> i32;
    pub fn dcn42_smu_get_dprefclk(clk_mgr: *mut clk_mgr_internal) -> i32;
    pub fn dcn42_smu_set_df_cstate_disable(clk_mgr: *mut clk_mgr_internal, disable: bool) -> bool;
}

// External types supplied by dependent headers.
extern "C" {
    pub type clk_mgr_internal;
    pub type large_integer;
    pub type dcn_zstate_support_state;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
