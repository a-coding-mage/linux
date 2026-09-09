/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

// External C dependencies are supplied by the surrounding translation unit.

#[repr(C)]
pub struct FloatInIntFormat_t { pub value: i32, pub numFractionalBits: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum DSPCLK_e { DSPCLK_DCFCLK = 0, DSPCLK_DISPCLK, DSPCLK_PIXCLK, DSPCLK_PHYCLK, DSPCLK_COUNT }

#[repr(C)] pub struct DisplayClockTable_t { pub Freq: u16, pub Vid: u16 }
#[repr(C)] pub struct WatermarkRowGeneric_t { pub MinClock: u16, pub MaxClock: u16, pub MinMclk: u16, pub MaxMclk: u16, pub WmSetting: u8, pub WmType: u8, pub Padding: [u8; 2] }

pub const NUM_WM_RANGES: usize = 4;
pub const WM_PSTATE_CHG: u32 = 0;
pub const WM_RETRAINING: u32 = 1;
#[repr(C)] pub enum WM_CLOCK_e { WM_SOCCLK = 0, WM_DCFCLK, WM_COUNT }
#[repr(C)] pub struct Watermarks_t { pub WatermarkRow: [[WatermarkRowGeneric_t; NUM_WM_RANGES]; 2], pub MmHubPadding: [u32; 7] }

#[repr(C)] pub enum CUSTOM_DPM_SETTING_e { CUSTOM_DPM_SETTING_GFXCLK, CUSTOM_DPM_SETTING_CCLK, CUSTOM_DPM_SETTING_FCLK_CCX, CUSTOM_DPM_SETTING_FCLK_GFX, CUSTOM_DPM_SETTING_FCLK_STALLS, CUSTOM_DPM_SETTING_LCLK, CUSTOM_DPM_SETTING_COUNT }
#[repr(C)] pub struct DpmActivityMonitorCoeffExt_t { pub ActiveHystLimit: u8, pub IdleHystLimit: u8, pub FPS: u8, pub MinActiveFreqType: u8, pub MinActiveFreq: FloatInIntFormat_t, pub PD_Data_limit: FloatInIntFormat_t, pub PD_Data_time_constant: FloatInIntFormat_t, pub PD_Data_error_coeff: FloatInIntFormat_t, pub PD_Data_error_rate_coeff: FloatInIntFormat_t }
#[repr(C)] pub struct CustomDpmSettings_t { pub DpmActivityMonitorCoeff: [DpmActivityMonitorCoeffExt_t; 6] }

pub const NUM_DCFCLK_DPM_LEVELS: usize = 8;
pub const NUM_DISPCLK_DPM_LEVELS: usize = 8;
pub const NUM_DPPCLK_DPM_LEVELS: usize = 8;
pub const NUM_SOCCLK_DPM_LEVELS: usize = 8;
pub const NUM_VCN_DPM_LEVELS: usize = 8;
pub const NUM_SOC_VOLTAGE_LEVELS: usize = 8;
pub const NUM_DF_PSTATE_LEVELS: usize = 4;
#[repr(C)] pub enum WCK_RATIO_e { WCK_RATIO_1_1 = 0, WCK_RATIO_1_2, WCK_RATIO_1_4, WCK_RATIO_MAX }
#[repr(C)] pub struct DfPstateTable_t { pub FClk: u32, pub MemClk: u32, pub Voltage: u32, pub WckRatio: u8, pub Spare: [u8; 3] }
#[repr(C)] pub struct DpmClocks_t { pub DcfClocks: [u32; 8], pub DispClocks: [u32; 8], pub DppClocks: [u32; 8], pub SocClocks: [u32; 8], pub VClocks: [u32; 8], pub DClocks: [u32; 8], pub SocVoltage: [u32; 8], pub DfPstateTable: [DfPstateTable_t; 4], pub NumDcfClkLevelsEnabled: u8, pub NumDispClkLevelsEnabled: u8, pub NumSocClkLevelsEnabled: u8, pub VcnClkLevelsEnabled: u8, pub NumDfPstatesEnabled: u8, pub spare: [u8; 3], pub MinGfxClk: u32, pub MaxGfxClk: u32 }

pub const THROTTLER_STATUS_BIT_SPL: u32 = 0; pub const THROTTLER_STATUS_BIT_FPPT: u32 = 1; pub const THROTTLER_STATUS_BIT_SPPT: u32 = 2; pub const THROTTLER_STATUS_BIT_SPPT_APU: u32 = 3; pub const THROTTLER_STATUS_BIT_THM_CORE: u32 = 4; pub const THROTTLER_STATUS_BIT_THM_GFX: u32 = 5; pub const THROTTLER_STATUS_BIT_THM_SOC: u32 = 6; pub const THROTTLER_STATUS_BIT_TDC_VDD: u32 = 7; pub const THROTTLER_STATUS_BIT_TDC_SOC: u32 = 8; pub const THROTTLER_STATUS_BIT_PROCHOT_CPU: u32 = 9; pub const THROTTLER_STATUS_BIT_PROCHOT_GFX: u32 = 10; pub const THROTTLER_STATUS_BIT_EDC_CPU: u32 = 11; pub const THROTTLER_STATUS_BIT_EDC_GFX: u32 = 12;

#[repr(C)] pub struct SmuMetrics_t { pub GfxclkFrequency:u16, pub SocclkFrequency:u16, pub VclkFrequency:u16, pub DclkFrequency:u16, pub MemclkFrequency:u16, pub spare:u16, pub GfxActivity:u16, pub UvdActivity:u16, pub Voltage:[u16;2], pub Current:[u16;2], pub Power:[u16;2], pub CoreFrequency:[u16;8], pub CorePower:[u16;8], pub CoreTemperature:[u16;8], pub L3Frequency:u16, pub L3Temperature:u16, pub GfxTemperature:u16, pub SocTemperature:u16, pub ThrottlerStatus:u16, pub CurrentSocketPower:u16, pub StapmOriginalLimit:u16, pub StapmCurrentLimit:u16, pub ApuPower:u16, pub dGpuPower:u16, pub VddTdcValue:u16, pub SocTdcValue:u16, pub VddEdcValue:u16, pub SocEdcValue:u16, pub InfrastructureCpuMaxFreq:u16, pub InfrastructureGfxMaxFreq:u16 }

pub const WORKLOAD_PPLIB_FULL_SCREEN_3D_BIT:u32=0; pub const WORKLOAD_PPLIB_VIDEO_BIT:u32=2; pub const WORKLOAD_PPLIB_VR_BIT:u32=3; pub const WORKLOAD_PPLIB_COMPUTE_BIT:u32=4; pub const WORKLOAD_PPLIB_CUSTOM_BIT:u32=5; pub const WORKLOAD_PPLIB_COUNT:u32=6;
pub const TABLE_BIOS_IF:u32=0; pub const TABLE_WATERMARKS:u32=1; pub const TABLE_CUSTOM_DPM:u32=2; pub const TABLE_SPARE1:u32=3; pub const TABLE_DPMCLOCKS:u32=4; pub const TABLE_MOMENTARY_PM:u32=5; pub const TABLE_MODERN_STDBY:u32=6; pub const TABLE_SMU_METRICS:u32=7; pub const TABLE_COUNT:u32=8;

#[repr(C)] pub struct dcn31_watermarks { pub WatermarkRow: [[WatermarkRowGeneric_t; 4]; 2], pub MmHubPadding: [u32; 7] }
#[repr(C)] pub struct dcn31_smu_dpm_clks { pub dpm_clks: *mut DpmClocks_t, pub mc_address: large_integer }
#[repr(C)] pub struct display_idle_optimization { pub df_request_disabled:u32, pub phy_ref_clk_off:u32, pub s0i2_rdy:u32, pub reserved:u32 }
#[repr(C)] pub union display_idle_optimization_u { pub idle_info: display_idle_optimization, pub data: u32 }

extern "C" {
    pub fn dcn31_smu_get_smu_version(clk_mgr: *mut clk_mgr_internal) -> i32;
    pub fn dcn31_smu_set_dispclk(clk_mgr: *mut clk_mgr_internal, requested_dispclk_khz: i32) -> i32;
    pub fn dcn31_smu_set_dprefclk(clk_mgr: *mut clk_mgr_internal) -> i32;
    pub fn dcn31_smu_set_hard_min_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_dcfclk_khz: i32) -> i32;
    pub fn dcn31_smu_set_min_deep_sleep_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_min_ds_dcfclk_khz: i32) -> i32;
    pub fn dcn31_smu_set_dppclk(clk_mgr: *mut clk_mgr_internal, requested_dpp_khz: i32) -> i32;
    pub fn dcn31_smu_set_display_idle_optimization(clk_mgr: *mut clk_mgr_internal, idle_info: u32);
    pub fn dcn31_smu_enable_phy_refclk_pwrdwn(clk_mgr: *mut clk_mgr_internal, enable: bool);
    pub fn dcn31_smu_enable_pme_wa(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn31_smu_set_dram_addr_high(clk_mgr: *mut clk_mgr_internal, addr_high: u32);
    pub fn dcn31_smu_set_dram_addr_low(clk_mgr: *mut clk_mgr_internal, addr_low: u32);
    pub fn dcn31_smu_transfer_dpm_table_smu_2_dram(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn31_smu_transfer_wm_table_dram_2_smu(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn31_smu_set_zstate_support(clk_mgr: *mut clk_mgr_internal, support: dcn_zstate_support_state);
    pub fn dcn31_smu_set_dtbclk(clk_mgr: *mut clk_mgr_internal, enable: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
