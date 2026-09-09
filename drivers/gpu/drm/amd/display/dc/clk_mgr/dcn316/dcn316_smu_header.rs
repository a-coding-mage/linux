/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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
 *
 * Authors: AMD
 */

// Dependency supplied by the surrounding translation unit: os_types.h

pub const PMFW_DRIVER_IF_VERSION: u32 = 4;

pub const NUM_DCFCLK_DPM_LEVELS: usize = 8;
pub const NUM_DISPCLK_DPM_LEVELS: usize = 8;
pub const NUM_DPPCLK_DPM_LEVELS: usize = 8;
pub const NUM_SOCCLK_DPM_LEVELS: usize = 8;
pub const NUM_VCN_DPM_LEVELS: usize = 8;
pub const NUM_SOC_VOLTAGE_LEVELS: usize = 8;
pub const NUM_DF_PSTATE_LEVELS: usize = 4;

#[repr(C)]
pub struct WatermarkRowGeneric_t {
    pub MinClock: u16, // This is either DCFCLK or SOCCLK (in MHz)
    pub MaxClock: u16, // This is either DCFCLK or SOCCLK (in MHz)
    pub MinMclk: u16,
    pub MaxMclk: u16,
    pub WmSetting: u8,
    pub WmType: u8, // Used for normal pstate change or memory retraining
    pub Padding: [u8; 2],
}

pub const NUM_WM_RANGES: usize = 4;
pub const WM_PSTATE_CHG: u32 = 0;
pub const WM_RETRAINING: u32 = 1;

#[repr(C)]
pub enum WM_CLOCK_e {
    WM_SOCCLK = 0,
    WM_DCFCLK,
    WM_COUNT,
}

#[repr(C)]
pub enum WCK_RATIO_e {
    WCK_RATIO_1_1 = 0, // DDR5, Wck:ck is always 1:1
    WCK_RATIO_1_2,
    WCK_RATIO_1_4,
    WCK_RATIO_MAX,
}

#[repr(C)]
pub struct DfPstateTable_t {
    pub FClk: u32,
    pub MemClk: u32,
    pub Voltage: u32,
    pub WckRatio: u8,
    pub Spare: [u8; 3],
}

// Freq in MHz
// Voltage in milli volts with 2 fractional bits
#[repr(C)]
pub struct DpmClocks_316_t {
    pub DcfClocks: [u32; NUM_DCFCLK_DPM_LEVELS],
    pub DispClocks: [u32; NUM_DISPCLK_DPM_LEVELS],
    pub DppClocks: [u32; NUM_DPPCLK_DPM_LEVELS],
    pub SocClocks: [u32; NUM_SOCCLK_DPM_LEVELS],
    pub VClocks: [u32; NUM_VCN_DPM_LEVELS],
    pub DClocks: [u32; NUM_VCN_DPM_LEVELS],
    pub SocVoltage: [u32; NUM_SOC_VOLTAGE_LEVELS],
    pub DfPstateTable: [DfPstateTable_t; NUM_DF_PSTATE_LEVELS],
    pub NumDcfClkLevelsEnabled: u8,
    pub NumDispClkLevelsEnabled: u8, // Applies to both Dispclk and Dppclk
    pub NumSocClkLevelsEnabled: u8,
    pub VcnClkLevelsEnabled: u8, // Applies to both Vclk and Dclk
    pub NumDfPstatesEnabled: u8,
    pub spare: [u8; 3],
    pub MinGfxClk: u32,
    pub MaxGfxClk: u32,
}

#[repr(C)]
pub struct dcn316_watermarks {
    // Watermarks
    pub WatermarkRow: [[WatermarkRowGeneric_t; NUM_WM_RANGES]; WM_COUNT as usize],
    pub MmHubPadding: [u32; 7], // SMU internal use
}

#[repr(C)]
pub struct dcn316_smu_dpm_clks {
    pub dpm_clks: *mut DpmClocks_316_t,
    pub mc_address: large_integer,
}

pub const TABLE_WATERMARKS: u32 = 1; // Called by DAL through VBIOS
pub const TABLE_DPMCLOCKS: u32 = 4; // Called by Driver and VBIOS

#[repr(C)]
pub struct display_idle_optimization {
    // C bitfields: df_request_disabled:1, phy_ref_clk_off:1, s0i2_rdy:1, reserved:29
    pub data: u32,
}

#[repr(C)]
pub union display_idle_optimization_u {
    pub idle_info: display_idle_optimization,
    pub data: u32,
}

extern "C" {
    pub fn dcn316_smu_get_smu_version(clk_mgr: *mut clk_mgr_internal) -> i32;
    pub fn dcn316_smu_set_dispclk(clk_mgr: *mut clk_mgr_internal, requested_dispclk_khz: i32) -> i32;
    pub fn dcn316_smu_set_hard_min_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_dcfclk_khz: i32) -> i32;
    pub fn dcn316_smu_set_min_deep_sleep_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_min_ds_dcfclk_khz: i32) -> i32;
    pub fn dcn316_smu_set_dppclk(clk_mgr: *mut clk_mgr_internal, requested_dpp_khz: i32) -> i32;
    pub fn dcn316_smu_set_display_idle_optimization(clk_mgr: *mut clk_mgr_internal, idle_info: u32);
    pub fn dcn316_smu_enable_phy_refclk_pwrdwn(clk_mgr: *mut clk_mgr_internal, enable: bool);
    pub fn dcn316_smu_set_dram_addr_high(clk_mgr: *mut clk_mgr_internal, addr_high: u32);
    pub fn dcn316_smu_set_dram_addr_low(clk_mgr: *mut clk_mgr_internal, addr_low: u32);
    pub fn dcn316_smu_transfer_dpm_table_smu_2_dram(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn316_smu_transfer_wm_table_dram_2_smu(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn316_smu_request_voltage_via_phyclk(clk_mgr: *mut clk_mgr_internal, requested_phyclk_khz: i32);
    pub fn dcn316_smu_enable_pme_wa(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn316_smu_set_dtbclk(clk_mgr: *mut clk_mgr_internal, enable: bool);
    pub fn dcn316_smu_get_dpref_clk(clk_mgr: *mut clk_mgr_internal) -> i32;
    pub fn dcn316_smu_get_smu_fclk(clk_mgr: *mut clk_mgr_internal) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
