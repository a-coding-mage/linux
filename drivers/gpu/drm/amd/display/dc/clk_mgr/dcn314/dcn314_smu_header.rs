/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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
 *
 */

// Dependency supplied by smu13_driver_if_v13_0_4.h.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WCK_RATIO_e {
    WCK_RATIO_1_1 = 0, // DDR5, Wck:ck is always 1:1;
    WCK_RATIO_1_2,
    WCK_RATIO_1_4,
    WCK_RATIO_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DfPstateTable314_t {
    pub FClk: u32,
    pub MemClk: u32,
    pub Voltage: u32,
    pub WckRatio: u8,
    pub Spare: [u8; 3],
}

// Freq in MHz
// Voltage in milli volts with 2 fractional bits
#[repr(C)]
pub struct DpmClocks314_t {
    pub DcfClocks: [u32; NUM_DCFCLK_DPM_LEVELS],
    pub DispClocks: [u32; NUM_DISPCLK_DPM_LEVELS],
    pub DppClocks: [u32; NUM_DPPCLK_DPM_LEVELS],
    pub SocClocks: [u32; NUM_SOCCLK_DPM_LEVELS],
    pub VClocks: [u32; NUM_VCN_DPM_LEVELS],
    pub DClocks: [u32; NUM_VCN_DPM_LEVELS],
    pub SocVoltage: [u32; NUM_SOC_VOLTAGE_LEVELS],
    pub DfPstateTable: [DfPstateTable314_t; NUM_DF_PSTATE_LEVELS],

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
pub struct dcn314_watermarks {
    // Watermarks
    pub WatermarkRow: [[WatermarkRowGeneric_t; NUM_WM_RANGES]; WM_COUNT],
    pub MmHubPadding: [u32; 7], // SMU internal use
}

#[repr(C)]
pub struct dcn314_smu_dpm_clks {
    pub dpm_clks: *mut DpmClocks314_t,
    pub mc_address: large_integer,
}

#[repr(C)]
pub struct display_idle_optimization {
    // C bitfields: df_request_disabled:1, phy_ref_clk_off:1, s0i2_rdy:1,
    // reserved:29. Stored as a 32-bit bitfield word to preserve layout.
    pub data: u32,
}

#[repr(C)]
pub union display_idle_optimization_u {
    pub idle_info: display_idle_optimization,
    pub data: u32,
}

extern "C" {
    pub fn dcn314_smu_get_smu_version(clk_mgr: *mut clk_mgr_internal) -> i32;
    pub fn dcn314_smu_set_dispclk(clk_mgr: *mut clk_mgr_internal, requested_dispclk_khz: i32) -> i32;
    pub fn dcn314_smu_set_dprefclk(clk_mgr: *mut clk_mgr_internal) -> i32;
    pub fn dcn314_smu_set_hard_min_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_dcfclk_khz: i32) -> i32;
    pub fn dcn314_smu_set_min_deep_sleep_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_min_ds_dcfclk_khz: i32) -> i32;
    pub fn dcn314_smu_set_dppclk(clk_mgr: *mut clk_mgr_internal, requested_dpp_khz: i32) -> i32;
    pub fn dcn314_smu_set_display_idle_optimization(clk_mgr: *mut clk_mgr_internal, idle_info: u32);
    pub fn dcn314_smu_enable_phy_refclk_pwrdwn(clk_mgr: *mut clk_mgr_internal, enable: bool);
    pub fn dcn314_smu_enable_pme_wa(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn314_smu_set_dram_addr_high(clk_mgr: *mut clk_mgr_internal, addr_high: u32);
    pub fn dcn314_smu_set_dram_addr_low(clk_mgr: *mut clk_mgr_internal, addr_low: u32);
    pub fn dcn314_smu_transfer_dpm_table_smu_2_dram(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn314_smu_transfer_wm_table_dram_2_smu(clk_mgr: *mut clk_mgr_internal);

    pub fn dcn314_smu_set_zstate_support(clk_mgr: *mut clk_mgr_internal, support: dcn_zstate_support_state);
    pub fn dcn314_smu_set_dtbclk(clk_mgr: *mut clk_mgr_internal, enable: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
