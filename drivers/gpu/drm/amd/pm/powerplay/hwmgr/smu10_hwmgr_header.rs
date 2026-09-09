/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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

// Dependencies: hwmgr.h, smu10_inc.h, smu10_driver_if.h, and rv_ppsmc.h.

pub const SMU10_MAX_HARDWARE_POWERLEVELS: usize = 8;
pub const SMU10_DYNCLK_NUMBER_OF_TREND_COEFFICIENTS: u32 = 15;
pub const DPMFlags_SCLK_Enabled: u32 = 0x00000001;
pub const DPMFlags_UVD_Enabled: u32 = 0x00000002;
pub const DPMFlags_VCE_Enabled: u32 = 0x00000004;
pub const DPMFlags_ACP_Enabled: u32 = 0x00000008;
pub const DPMFlags_ForceHighestValid: u32 = 0x40000000;
pub const SMU_EnabledFeatureScoreboard_AcpDpmOn: u32 = 0x00000001;
pub const SMU_EnabledFeatureScoreboard_SclkDpmOn: u32 = 0x00200000;
pub const SMU_EnabledFeatureScoreboard_UvdDpmOn: u32 = 0x01000000;
pub const SMU_EnabledFeatureScoreboard_VceDpmOn: u32 = 0x02000000;
pub const SMU_PHYID_SHIFT: u32 = 8;
pub const SMU10_PCIE_POWERGATING_TARGET_GFX: u32 = 0;
pub const SMU10_PCIE_POWERGATING_TARGET_DDI: u32 = 1;
pub const SMU10_PCIE_POWERGATING_TARGET_PLLCASCADE: u32 = 2;
pub const SMU10_PCIE_POWERGATING_TARGET_PHY: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum VQ_TYPE { CLOCK_TYPE_DCLK = 0, CLOCK_TYPE_ECLK, CLOCK_TYPE_SCLK, CLOCK_TYPE_CCLK, VQ_GFX_CU }

pub const SUSTAINABLE_SCLK_MASK: u32 = 0x00ffffff;
pub const SUSTAINABLE_SCLK_SHIFT: u32 = 0;
pub const SUSTAINABLE_CU_MASK: u32 = 0xff000000;
pub const SUSTAINABLE_CU_SHIFT: u32 = 24;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu10_dpm_entry { pub soft_min_clk: u32, pub hard_min_clk: u32, pub soft_max_clk: u32, pub hard_max_clk: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu10_power_level {
    pub engine_clock: u32, pub vddc_index: u8, pub ds_divider_index: u8, pub ss_divider_index: u8,
    pub allow_gnb_slow: u8, pub force_nbp_state: u8, pub display_wm: u8, pub vce_wm: u8,
    pub num_simd_to_powerdown: u8, pub hysteresis_up: u8, pub rsv: [u8; 3],
}

pub const SMU10_POWERSTATE_FLAGS_NBPS_FORCEHIGH: u32 = 1 << 0;
pub const SMU10_POWERSTATE_FLAGS_NBPS_LOCKTOHIGH: u32 = 1 << 1;
pub const SMU10_POWERSTATE_FLAGS_NBPS_LOCKTOLOW: u32 = 1 << 2;
pub const SMU10_POWERSTATE_FLAGS_BAPM_DISABLE: u32 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu10_uvd_clocks { pub vclk: u32, pub dclk: u32, pub vclk_low_divider: u32, pub vclk_high_divider: u32, pub dclk_low_divider: u32, pub dclk_high_divider: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_disable_nbpslo_flags_bits { pub entry: u32, pub display: u32, pub driver: u32, pub vce: u32, pub uvd: u32, pub acp: u32, pub reserved: u32 }
#[repr(C)]
pub union pp_disable_nbpslo_flags_union { pub bits: pp_disable_nbpslo_flags_bits, pub u32All: u32 }
#[repr(C)]
pub struct pp_disable_nbpslo_flags { pub data: pp_disable_nbpslo_flags_union }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum smu10_pstate_previous_action { DO_NOTHING = 1, FORCE_HIGH, CANCEL_FORCE_HIGH }

#[repr(C)]
pub struct smu10_power_state {
    pub magic: ::std::os::raw::c_uint, pub level: u32, pub uvd_clocks: smu10_uvd_clocks, pub evclk: u32, pub ecclk: u32, pub samclk: u32, pub acpclk: u32, pub need_dfs_bypass: bool,
    pub nbps_flags: u32, pub bapm_flags: u32, pub dpm0_pg_nbps_low: u8, pub dpm0_pg_nbps_high: u8, pub dpm_x_nbps_low: u8, pub dpm_x_nbps_high: u8,
    pub action: smu10_pstate_previous_action, pub levels: [smu10_power_level; SMU10_MAX_HARDWARE_POWERLEVELS], pub nbpslo_flags: pp_disable_nbpslo_flags,
}

pub const SMU10_NUM_NBPSTATES: u32 = 4;
pub const SMU10_NUM_NBPMEMORYCLOCK: u32 = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu10_display_phy_info_entry { pub phy_present: u8, pub active_lane_mapping: u8, pub display_config_type: u8, pub active_num_of_lanes: u8 }
pub const SMU10_MAX_DISPLAYPHY_IDS: usize = 10;
#[repr(C)]
pub struct smu10_display_phy_info { pub display_phy_access_initialized: bool, pub entries: [smu10_display_phy_info_entry; SMU10_MAX_DISPLAYPHY_IDS] }
pub const MAX_DISPLAY_CLOCK_LEVEL: u32 = 8;
#[repr(C)]
pub struct smu10_system_info { pub htc_tmp_lmt: u8, pub htc_hyst_lmt: u8 }
pub const MAX_REGULAR_DPM_NUMBER: usize = 8;
#[repr(C)]
pub struct smu10_mclk_latency_entries { pub frequency: u32, pub latency: u32 }
#[repr(C)]
pub struct smu10_mclk_latency_table { pub count: u32, pub entries: [smu10_mclk_latency_entries; MAX_REGULAR_DPM_NUMBER] }
#[repr(C)]
pub struct smu10_clock_voltage_dependency_record { pub clk: u32, pub vol: u32 }
#[repr(C)]
pub struct smu10_voltage_dependency_table { pub count: u32, pub entries: [smu10_clock_voltage_dependency_record; 0] }
#[repr(C)]
pub struct smu10_clock_voltage_information {
    pub vdd_dep_on_dcefclk: *mut smu10_voltage_dependency_table, pub vdd_dep_on_socclk: *mut smu10_voltage_dependency_table, pub vdd_dep_on_fclk: *mut smu10_voltage_dependency_table,
    pub vdd_dep_on_mclk: *mut smu10_voltage_dependency_table, pub vdd_dep_on_dispclk: *mut smu10_voltage_dependency_table, pub vdd_dep_on_dppclk: *mut smu10_voltage_dependency_table, pub vdd_dep_on_phyclk: *mut smu10_voltage_dependency_table,
}

// The remaining hwmgr fields use types declared by the included headers.
#[repr(C)]
pub struct smu10_hwmgr {
    pub disable_driver_thermal_policy: u32, pub thermal_auto_throttling_treshold: u32, pub sys_info: smu10_system_info, pub mclk_latency_table: smu10_mclk_latency_table, pub ddi_power_gating_disabled: u32,
    pub display_phy_info: smu10_display_phy_info_entry, pub dce_slow_sclk_threshold: u32, pub disp_clk_bypass: bool, pub disp_clk_bypass_pending: bool, pub bapm_enabled: u32, pub video_start: bool, pub battery_state: bool,
    pub is_nb_dpm_enabled: u32, pub is_voltage_island_enabled: u32, pub disable_smu_acp_s3_handshake: u32, pub disable_notify_smu_vpu_recovery: u32, pub in_vpu_recovery: bool, pub pg_acp_init: bool, pub disp_config: u8,
    pub power_containment_features: u32, pub cac_enabled: bool, pub disable_uvd_power_tune_feature: bool, pub enable_bapm_feature: bool, pub enable_tdc_limit_feature: bool,
    pub sram_end: u32, pub dpm_table_start: u32, pub soft_regs_start: u32, pub uvd_level_count: u8, pub vce_level_count: u8, pub acp_level_count: u8, pub samu_level_count: u8,
    pub fps_high_threshold: u32, pub fps_low_threshold: u32, pub dpm_flags: u32, pub sclk_dpm: smu10_dpm_entry, pub uvd_dpm: smu10_dpm_entry, pub vce_dpm: smu10_dpm_entry, pub acp_dpm: smu10_dpm_entry, pub acp_power_up_no_dsp: bool,
    pub max_sclk_level: u32, pub num_of_clk_entries: u32, pub separation_time: u32, pub cc6_disable: bool, pub pstate_disable: bool, pub cc6_setting_changed: bool, pub ulTotalActiveCUs: u32,
    pub isp_tileA_power_gated: bool, pub isp_tileB_power_gated: bool, pub isp_actual_hard_min_freq: u32, pub soc_actual_hard_min_freq: u32, pub dcf_actual_hard_min_freq: u32, pub f_actual_hard_min_freq: u32, pub fabric_actual_soft_min_freq: u32, pub vclk_soft_min: u32, pub dclk_soft_min: u32, pub gfx_actual_soft_min_freq: u32, pub gfx_actual_soft_max_freq: u32, pub gfx_min_freq_limit: u32, pub gfx_max_freq_limit: u32,
    pub vcn_power_gated: bool, pub vcn_dpg_mode: bool, pub gfx_off_controled_by_driver: bool, pub water_marks_exist: bool, pub water_marks_table: Watermarks_t, pub clock_vol_info: smu10_clock_voltage_information, pub clock_table: DpmClocks_t,
    pub active_process_mask: u32, pub need_min_deep_sleep_dcefclk: bool, pub deep_sleep_dcefclk: u32, pub num_active_display: u32, pub fine_grain_enabled: bool,
}

pub struct pp_hwmgr;
extern "C" { pub fn smu10_init_function_pointers(hwmgr: *mut pp_hwmgr) -> ::std::os::raw::c_int; }

pub const SMU10_UMD_PSTATE_GFXCLK: u32 = 700;
pub const SMU10_UMD_PSTATE_SOCCLK: u32 = 626;
pub const SMU10_UMD_PSTATE_FCLK: u32 = 933;
pub const SMU10_UMD_PSTATE_VCE: u32 = 0x03C00320;
pub const SMU10_UMD_PSTATE_PROFILE_VCE: u32 = 0x02AD0229;
pub const SMU10_UMD_PSTATE_PEAK_SOCCLK: u32 = 757;
pub const SMU10_UMD_PSTATE_PEAK_FCLK: u32 = 1200;
pub const SMU10_UMD_PSTATE_MIN_FCLK: u32 = 400;
pub const SMU10_UMD_PSTATE_MIN_SOCCLK: u32 = 200;
pub const SMU10_UMD_PSTATE_MIN_VCE: u32 = 0x0190012C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
