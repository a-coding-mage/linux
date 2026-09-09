/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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

pub const SMU13_DRIVER_IF_VERSION: u32 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct df_pstate_t { pub fclk: u32, pub memclk: u32, pub voltage: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcn_clk_t { pub vclk: u32, pub dclk: u32 }

pub const DSPCLK_DCFCLK: i32 = 0;
pub const DSPCLK_DISPCLK: i32 = 1;
pub const DSPCLK_PIXCLK: i32 = 2;
pub const DSPCLK_PHYCLK: i32 = 3;
pub const DSPCLK_COUNT: i32 = 4;
pub type DSPCLK_e = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayClockTable_t { pub Freq: u16, pub Vid: u16 }

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
pub const WM_SOCCLK: i32 = 0;
pub const WM_DCFCLK: i32 = 1;
pub const WM_COUNT: usize = 2;
pub type WM_CLOCK_e = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Watermarks_t {
    pub WatermarkRow: [[WatermarkRowGeneric_t; NUM_WM_RANGES]; WM_COUNT],
    pub MmHubPadding: [u32; 7],
}

pub const TABLE_WATERMARKS: u32 = 1;
pub const TABLE_DPMCLOCKS: u32 = 4;
pub const VG_NUM_DCFCLK_DPM_LEVELS: usize = 7;
pub const VG_NUM_DISPCLK_DPM_LEVELS: usize = 7;
pub const VG_NUM_DPPCLK_DPM_LEVELS: usize = 7;
pub const VG_NUM_SOCCLK_DPM_LEVELS: usize = 7;
pub const VG_NUM_ISPICLK_DPM_LEVELS: usize = 7;
pub const VG_NUM_ISPXCLK_DPM_LEVELS: usize = 7;
pub const VG_NUM_VCN_DPM_LEVELS: usize = 5;
pub const VG_NUM_FCLK_DPM_LEVELS: usize = 4;
pub const VG_NUM_SOC_VOLTAGE_LEVELS: usize = 8;

// copy from vgh/vangogh/pmfw_driver_if.h
#[repr(C)]
pub struct vg_dpm_clocks {
    pub DcfClocks: [u32; VG_NUM_DCFCLK_DPM_LEVELS],
    pub DispClocks: [u32; VG_NUM_DISPCLK_DPM_LEVELS],
    pub DppClocks: [u32; VG_NUM_DPPCLK_DPM_LEVELS],
    pub SocClocks: [u32; VG_NUM_SOCCLK_DPM_LEVELS],
    pub IspiClocks: [u32; VG_NUM_ISPICLK_DPM_LEVELS],
    pub IspxClocks: [u32; VG_NUM_ISPXCLK_DPM_LEVELS],
    pub VcnClocks: [vcn_clk_t; VG_NUM_VCN_DPM_LEVELS],
    pub SocVoltage: [u32; VG_NUM_SOC_VOLTAGE_LEVELS],
    pub DfPstateTable: [df_pstate_t; VG_NUM_FCLK_DPM_LEVELS],
    pub MinGfxClk: u32,
    pub MaxGfxClk: u32,
    pub NumDfPstatesEnabled: u8,
    pub NumDcfclkLevelsEnabled: u8,
    pub NumDispClkLevelsEnabled: u8,
    pub NumSocClkLevelsEnabled: u8,
    pub IspClkLevelsEnabled: u8,
    pub VcnClkLevelsEnabled: u8,
    pub spare: [u8; 2],
}

#[repr(C)]
pub struct smu_dpm_clks {
    pub dpm_clks: *mut vg_dpm_clocks,
    pub mc_address: large_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct watermarks {
    pub WatermarkRow: [[WatermarkRowGeneric_t; NUM_WM_RANGES]; WM_COUNT],
    pub MmHubPadding: [u32; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct display_idle_optimization { pub data: u32 }

#[repr(C)]
pub union display_idle_optimization_u {
    pub idle_info: display_idle_optimization,
    pub data: u32,
}

#[repr(C)]
pub struct clk_mgr_internal;
#[repr(C)]
pub union large_integer { pub data: u64 }

extern "C" {
    pub fn dcn301_smu_get_smu_version(clk_mgr: *mut clk_mgr_internal) -> i32;
    pub fn dcn301_smu_set_dispclk(clk_mgr: *mut clk_mgr_internal, requested_dispclk_khz: i32) -> i32;
    pub fn dcn301_smu_set_dprefclk(clk_mgr: *mut clk_mgr_internal) -> i32;
    pub fn dcn301_smu_set_hard_min_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_dcfclk_khz: i32) -> i32;
    pub fn dcn301_smu_set_min_deep_sleep_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_min_ds_dcfclk_khz: i32) -> i32;
    pub fn dcn301_smu_set_dppclk(clk_mgr: *mut clk_mgr_internal, requested_dpp_khz: i32) -> i32;
    pub fn dcn301_smu_set_display_idle_optimization(clk_mgr: *mut clk_mgr_internal, idle_info: u32);
    pub fn dcn301_smu_enable_phy_refclk_pwrdwn(clk_mgr: *mut clk_mgr_internal, enable: bool);
    pub fn dcn301_smu_enable_pme_wa(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn301_smu_set_dram_addr_high(clk_mgr: *mut clk_mgr_internal, addr_high: u32);
    pub fn dcn301_smu_set_dram_addr_low(clk_mgr: *mut clk_mgr_internal, addr_low: u32);
    pub fn dcn301_smu_transfer_dpm_table_smu_2_dram(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn301_smu_transfer_wm_table_dram_2_smu(clk_mgr: *mut clk_mgr_internal);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
