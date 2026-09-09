/*
 * Copyright 2015 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the corresponding translated headers:
// hwmgr.h, ppatomctrl.h

pub const SMU7_MAX_HARDWARE_POWERLEVELS: usize = 2;
pub const SMU7_VOLTAGE_CONTROL_NONE: u32 = 0x0;
pub const SMU7_VOLTAGE_CONTROL_BY_GPIO: u32 = 0x1;
pub const SMU7_VOLTAGE_CONTROL_BY_SVID2: u32 = 0x2;
pub const SMU7_VOLTAGE_CONTROL_MERGED: u32 = 0x3;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum gpu_pt_config_reg_type {
    GPU_CONFIGREG_MMR = 0,
    GPU_CONFIGREG_SMC_IND,
    GPU_CONFIGREG_DIDT_IND,
    GPU_CONFIGREG_GC_CAC_IND,
    GPU_CONFIGREG_CACHE,
    GPU_CONFIGREG_MAX,
}

#[repr(C)]
pub struct gpu_pt_config_reg { pub offset: u32, pub mask: u32, pub shift: u32, pub value: u32, pub type_: gpu_pt_config_reg_type }

#[repr(C)]
pub struct smu7_performance_level { pub memory_clock: u32, pub engine_clock: u32, pub pcie_gen: u16, pub pcie_lane: u16 }
#[repr(C)]
pub struct smu7_thermal_temperature_setting { pub temperature_low: libc::c_long, pub temperature_high: libc::c_long, pub temperature_shutdown: libc::c_long }
#[repr(C)]
pub struct smu7_uvd_clocks { pub vclk: u32, pub dclk: u32 }
#[repr(C)]
pub struct smu7_vce_clocks { pub evclk: u32, pub ecclk: u32 }

#[repr(C)]
pub struct smu7_power_state {
    pub magic: u32, pub uvd_clks: smu7_uvd_clocks, pub vce_clks: smu7_vce_clocks,
    pub sam_clk: u32, pub performance_level_count: u16, pub dc_compatible: bool,
    pub sclk_threshold: u32,
    pub performance_levels: [smu7_performance_level; SMU7_MAX_HARDWARE_POWERLEVELS],
}
#[repr(C)]
pub struct smu7_dpm_level { pub enabled: bool, pub value: u32, pub param1: u32 }

pub const SMU7_MAX_DEEPSLEEP_DIVIDER_ID: u32 = 5;
pub const MAX_REGULAR_DPM_NUMBER: usize = 8;
pub const SMU7_MINIMUM_ENGINE_CLOCK: u32 = 2500;

#[repr(C)]
pub struct smu7_single_dpm_table { pub count: u32, pub dpm_levels: [smu7_dpm_level; MAX_REGULAR_DPM_NUMBER] }
#[repr(C)]
pub struct smu7_dpm_table {
    pub sclk_table: smu7_single_dpm_table, pub mclk_table: smu7_single_dpm_table,
    pub pcie_speed_table: smu7_single_dpm_table, pub vddc_table: smu7_single_dpm_table,
    pub vddci_table: smu7_single_dpm_table, pub mvdd_table: smu7_single_dpm_table,
}
#[repr(C)]
pub struct smu7_clock_registers {
    pub vCG_SPLL_FUNC_CNTL: u32, pub vCG_SPLL_FUNC_CNTL_2: u32, pub vCG_SPLL_FUNC_CNTL_3: u32,
    pub vCG_SPLL_FUNC_CNTL_4: u32, pub vCG_SPLL_SPREAD_SPECTRUM: u32, pub vCG_SPLL_SPREAD_SPECTRUM_2: u32,
    pub vDLL_CNTL: u32, pub vMCLK_PWRMGT_CNTL: u32, pub vMPLL_AD_FUNC_CNTL: u32,
    pub vMPLL_DQ_FUNC_CNTL: u32, pub vMPLL_FUNC_CNTL: u32, pub vMPLL_FUNC_CNTL_1: u32,
    pub vMPLL_FUNC_CNTL_2: u32, pub vMPLL_SS1: u32, pub vMPLL_SS2: u32,
}
pub const DISABLE_MC_LOADMICROCODE: u32 = 1;
pub const DISABLE_MC_CFGPROGRAMMING: u32 = 2;
#[repr(C)] pub struct smu7_voltage_smio_registers { pub vS0_VID_LOWER_SMIO_CNTL: u32 }
pub const SMU7_MAX_LEAKAGE_COUNT: usize = 8;
#[repr(C)] pub struct smu7_leakage_voltage { pub count: u16, pub leakage_id: [u16; SMU7_MAX_LEAKAGE_COUNT], pub actual_voltage: [u16; SMU7_MAX_LEAKAGE_COUNT] }
#[repr(C)] pub struct smu7_vbios_boot_state { pub mvdd_bootup_value: u16, pub vddc_bootup_value: u16, pub vddci_bootup_value: u16, pub vddgfx_bootup_value: u16, pub sclk_bootup_value: u32, pub mclk_bootup_value: u32, pub pcie_gen_bootup_value: u16, pub pcie_lane_bootup_value: u16 }
#[repr(C)] pub struct smu7_display_timing { pub min_clock_in_sr: u32, pub num_existing_displays: u32, pub vrefresh: u32 }
#[repr(C)] pub struct smu7_dpmlevel_enable_mask { pub uvd_dpm_enable_mask: u32, pub vce_dpm_enable_mask: u32, pub acp_dpm_enable_mask: u32, pub samu_dpm_enable_mask: u32, pub sclk_dpm_enable_mask: u32, pub mclk_dpm_enable_mask: u32, pub pcie_dpm_enable_mask: u32 }
#[repr(C)] pub struct smu7_pcie_perf_range { pub max: u16, pub min: u16 }
#[repr(C)] pub struct smu7_odn_clock_voltage_dependency_table { pub count: u32, pub entries: [phm_ppt_v1_clock_voltage_dependency_record; MAX_REGULAR_DPM_NUMBER] }
#[repr(C)] pub struct smu7_odn_dpm_table { pub odn_core_clock_dpm_levels: phm_odn_clock_levels, pub odn_memory_clock_dpm_levels: phm_odn_clock_levels, pub vdd_dependency_on_sclk: smu7_odn_clock_voltage_dependency_table, pub vdd_dependency_on_mclk: smu7_odn_clock_voltage_dependency_table, pub odn_mclk_min_limit: u32, pub min_vddc: u32, pub max_vddc: u32 }
#[repr(C)] pub struct profile_mode_setting { pub bupdate_sclk: u8, pub sclk_up_hyst: u8, pub sclk_down_hyst: u8, pub sclk_activity: u16, pub bupdate_mclk: u8, pub mclk_up_hyst: u8, pub mclk_down_hyst: u8, pub mclk_activity: u16 }
#[repr(C)] pub struct smu7_mclk_latency_entries { pub frequency: u32, pub latency: u32 }
#[repr(C)] pub struct smu7_mclk_latency_table { pub count: u32, pub entries: [smu7_mclk_latency_entries; MAX_REGULAR_DPM_NUMBER] }

#[repr(C)]
pub struct smu7_hwmgr {
    pub dpm_table: smu7_dpm_table, pub golden_dpm_table: smu7_dpm_table, pub odn_dpm_table: smu7_odn_dpm_table, pub mclk_latency_table: smu7_mclk_latency_table,
    pub voting_rights_clients: [u32; 8], pub static_screen_threshold_unit: u32, pub static_screen_threshold: u32, pub voltage_control: u32, pub vdd_gfx_control: u32, pub vddc_vddgfx_delta: u32, pub active_auto_throttle_sources: u32,
    pub clock_registers: smu7_clock_registers, pub is_memory_gddr5: bool, pub acpi_vddc: u16, pub pspp_notify_required: bool, pub force_pcie_gen: u16, pub acpi_pcie_gen: u16, pub pcie_gen_cap: u32, pub pcie_lane_cap: u32, pub pcie_spc_cap: u32, pub sclk_cap: u32,
    pub vddc_leakage: smu7_leakage_voltage, pub vddci_leakage: smu7_leakage_voltage, pub vddcgfx_leakage: smu7_leakage_voltage,
    pub mvdd_control: u32, pub vddc_mask_low: u32, pub mvdd_mask_low: u32, pub max_vddc_in_pptable: u16, pub min_vddc_in_pptable: u16, pub max_vddci_in_pptable: u16, pub min_vddci_in_pptable: u16, pub is_uvd_enabled: bool, pub vbios_boot_state: smu7_vbios_boot_state,
    pub pcie_performance_request: bool, pub battery_state: bool, pub mclk_ignore_signal: bool, pub is_tlu_enabled: bool, pub disable_handshake: bool, pub smc_voltage_control_enabled: bool, pub vbi_time_out_support: bool,
    pub soft_regs_start: u32, pub vddci_control: u32, pub vddc_voltage_table: pp_atomctrl_voltage_table, pub vddci_voltage_table: pp_atomctrl_voltage_table, pub mvdd_voltage_table: pp_atomctrl_voltage_table, pub vddgfx_voltage_table: pp_atomctrl_voltage_table,
    pub mgcg_cgtt_local2: u32, pub mgcg_cgtt_local3: u32, pub gpio_debug: u32, pub mc_micro_code_feature: u32, pub highest_mclk: u32, pub acpi_vddci: u16, pub mvdd_high_index: u8, pub mvdd_low_index: u8, pub dll_default_on: bool, pub performance_request_registered: bool,
    pub ulv_supported: bool, pub cac_table_start: u32, pub cac_configuration_required: bool, pub driver_calculate_cac_leakage: bool, pub cac_enabled: bool, pub power_containment_features: u32, pub enable_dte_feature: bool, pub enable_tdc_limit_feature: bool, pub enable_pkg_pwr_tracking_feature: bool, pub disable_uvd_power_tune_feature: bool,
    pub dte_tj_offset: u32, pub fast_watermark_threshold: u32, pub vddc_phase_shed_control: u8, pub display_timing: smu7_display_timing, pub thermal_temp_setting: smu7_thermal_temperature_setting, pub dpm_level_enable_mask: smu7_dpmlevel_enable_mask, pub need_update_smu7_dpm_table: u32, pub sclk_dpm_key_disabled: u32, pub mclk_dpm_key_disabled: u32, pub pcie_dpm_key_disabled: u32, pub min_engine_clocks: u32,
    pub pcie_gen_performance: smu7_pcie_perf_range, pub pcie_lane_performance: smu7_pcie_perf_range, pub pcie_gen_power_saving: smu7_pcie_perf_range, pub pcie_lane_power_saving: smu7_pcie_perf_range, pub use_pcie_performance_levels: bool, pub use_pcie_power_saving_levels: bool, pub mclk_dpm0_activity_target: u32, pub low_sclk_interrupt_threshold: u32, pub last_mclk_dpm_enable_mask: u32, pub uvd_enabled: bool,
    pub uvd_power_gated: bool, pub vce_power_gated: bool, pub need_long_memory_training: bool, pub update_up_hyst: bool, pub update_down_hyst: bool, pub down_hyst: u32, pub up_hyst: u32, pub disable_dpm_mask: u32, pub apply_optimized_settings: bool,
    pub avfs_vdroop_override_setting: u32, pub apply_avfs_cks_off_voltage: bool, pub frame_time_x2: u32, pub last_sent_vbi_timeout: u32, pub mem_latency_high: u16, pub mem_latency_low: u16, pub vr_config: u32, pub current_profile_setting: profile_mode_setting, pub ro_range_minimum: u32, pub ro_range_maximum: u32, pub disable_edc_leakage_controller: bool, pub edc_hilo_leakage_offset_from_vbios: AtomCtrl_HiLoLeakageOffsetTable, pub edc_leakage_table: AtomCtrl_EDCLeakgeTable,
}

pub const SMU7_Q88_FORMAT_CONVERSION_UNIT: u32 = 256;
#[repr(C)] pub enum SMU7_I2CLineID { SMU7_I2CLineID_DDC1 = 0x90, SMU7_I2CLineID_DDC2, SMU7_I2CLineID_DDC3, SMU7_I2CLineID_DDC4, SMU7_I2CLineID_DDC5, SMU7_I2CLineID_DDC6, SMU7_I2CLineID_SCLSDA, SMU7_I2CLineID_DDCVGA }
pub const SMU7_I2C_DDC1DATA: u32 = 0; pub const SMU7_I2C_DDC1CLK: u32 = 1; pub const SMU7_I2C_DDC2DATA: u32 = 2; pub const SMU7_I2C_DDC2CLK: u32 = 3; pub const SMU7_I2C_DDC3DATA: u32 = 4; pub const SMU7_I2C_DDC3CLK: u32 = 5; pub const SMU7_I2C_SDA: u32 = 40; pub const SMU7_I2C_SCL: u32 = 41; pub const SMU7_I2C_DDC4DATA: u32 = 65; pub const SMU7_I2C_DDC4CLK: u32 = 66; pub const SMU7_I2C_DDC5DATA: u32 = 0x48; pub const SMU7_I2C_DDC5CLK: u32 = 0x49; pub const SMU7_I2C_DDC6DATA: u32 = 0x4a; pub const SMU7_I2C_DDC6CLK: u32 = 0x4b; pub const SMU7_I2C_DDCVGADATA: u32 = 0x4c; pub const SMU7_I2C_DDCVGACLK: u32 = 0x4d; pub const SMU7_UNUSED_GPIO_PIN: u32 = 0x7F;
extern "C" { pub fn smu7_get_sleep_divider_id_from_clock(clock: u32, clock_insr: u32) -> u8; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
