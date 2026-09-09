/*
 * Copyright 2013 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the corresponding C headers.

pub const SMU__NUM_SCLK_DPM_STATE: usize = 8;
pub const SMU__NUM_MCLK_DPM_LEVELS: usize = 4;
pub const SMU__NUM_LCLK_DPM_LEVELS: usize = 8;
pub const SMU__NUM_PCIE_DPM_LEVELS: usize = 0; // ???

pub const SUMO_MAX_HARDWARE_POWERLEVELS: usize = 5;
pub const SUMO_MAX_NUMBER_VOLTAGES: usize = 4;

#[repr(C)]
pub struct sumo_vid_mapping_entry {
    pub vid_2bit: u16,
    pub vid_7bit: u16,
}

#[repr(C)]
pub struct sumo_vid_mapping_table {
    pub num_entries: u32,
    pub entries: [sumo_vid_mapping_entry; SUMO_MAX_NUMBER_VOLTAGES],
}

#[repr(C)]
pub struct sumo_sclk_voltage_mapping_entry {
    pub sclk_frequency: u32,
    pub vid_2bit: u16,
    pub rsv: u16,
}

#[repr(C)]
pub struct sumo_sclk_voltage_mapping_table {
    pub num_max_dpm_entries: u32,
    pub entries: [sumo_sclk_voltage_mapping_entry; SUMO_MAX_HARDWARE_POWERLEVELS],
}

pub const TRINITY_AT_DFLT: u32 = 30;
pub const KV_NUM_NBPSTATES: usize = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum kv_pt_config_reg_type {
    KV_CONFIGREG_MMR = 0,
    KV_CONFIGREG_SMC_IND,
    KV_CONFIGREG_DIDT_IND,
    KV_CONFIGREG_CACHE,
    KV_CONFIGREG_MAX,
}

#[repr(C)]
pub struct kv_pt_config_reg {
    pub offset: u32,
    pub mask: u32,
    pub shift: u32,
    pub value: u32,
    pub type_: kv_pt_config_reg_type,
}

#[repr(C)]
pub struct kv_lcac_config_values { pub block_id: u32, pub signal_id: u32, pub t: u32 }

#[repr(C)]
pub struct kv_lcac_config_reg {
    pub cntl: u32, pub block_mask: u32, pub block_shift: u32,
    pub signal_mask: u32, pub signal_shift: u32, pub t_mask: u32,
    pub t_shift: u32, pub enable_mask: u32, pub enable_shift: u32,
}

#[repr(C)]
pub struct kv_pl {
    pub sclk: u32,
    pub vddc_index: u8,
    pub ds_divider_index: u8,
    pub ss_divider_index: u8,
    pub allow_gnb_slow: u8,
    pub force_nbp_state: u8,
    pub display_wm: u8,
    pub vce_wm: u8,
}

#[repr(C)]
pub struct kv_ps {
    pub levels: [kv_pl; SUMO_MAX_HARDWARE_POWERLEVELS],
    pub num_levels: u32,
    pub need_dfs_bypass: bool,
    pub dpm0_pg_nb_ps_lo: u8, pub dpm0_pg_nb_ps_hi: u8,
    pub dpmx_nb_ps_lo: u8, pub dpmx_nb_ps_hi: u8,
}

#[repr(C)]
pub struct kv_sys_info {
    pub bootup_uma_clk: u32, pub bootup_sclk: u32, pub dentist_vco_freq: u32,
    pub nb_dpm_enable: u32,
    pub nbp_memory_clock: [u32; KV_NUM_NBPSTATES],
    pub nbp_n_clock: [u32; KV_NUM_NBPSTATES],
    pub bootup_nb_voltage_index: u16, pub htc_tmp_lmt: u8, pub htc_hyst_lmt: u8,
    pub sclk_voltage_mapping_table: sumo_sclk_voltage_mapping_table,
    pub vid_mapping_table: sumo_vid_mapping_table,
    pub uma_channel_number: u32,
}

#[repr(C)]
pub struct kv_power_info {
    pub at: [u32; SUMO_MAX_HARDWARE_POWERLEVELS], pub voltage_drop_t: u32,
    pub sys_info: kv_sys_info, pub boot_pl: kv_pl,
    pub enable_nb_ps_policy: bool, pub disable_nb_ps3_in_battery: bool,
    pub video_start: bool, pub battery_state: bool, pub lowest_valid: u32,
    pub highest_valid: u32, pub high_voltage_t: u16, pub cac_enabled: bool,
    pub bapm_enable: bool, pub sram_end: u32, pub dpm_table_start: u32,
    pub soft_regs_start: u32, pub graphics_dpm_level_count: u8,
    pub uvd_level_count: u8, pub vce_level_count: u8, pub acp_level_count: u8,
    pub samu_level_count: u8, pub fps_high_t: u16,
    pub graphics_level: [SMU7_Fusion_GraphicsLevel; SMU__NUM_SCLK_DPM_STATE],
    pub acpi_level: SMU7_Fusion_ACPILevel,
    pub uvd_level: [SMU7_Fusion_UvdLevel; SMU7_MAX_LEVELS_UVD],
    pub vce_level: [SMU7_Fusion_ExtClkLevel; SMU7_MAX_LEVELS_VCE],
    pub acp_level: [SMU7_Fusion_ExtClkLevel; SMU7_MAX_LEVELS_ACP],
    pub samu_level: [SMU7_Fusion_ExtClkLevel; SMU7_MAX_LEVELS_SAMU],
    pub uvd_boot_level: u8, pub vce_boot_level: u8, pub acp_boot_level: u8,
    pub samu_boot_level: u8, pub uvd_interval: u8, pub vce_interval: u8,
    pub acp_interval: u8, pub samu_interval: u8, pub graphics_boot_level: u8,
    pub graphics_interval: u8, pub graphics_therm_throttle_enable: u8,
    pub graphics_voltage_change_enable: u8, pub graphics_clk_slow_enable: u8,
    pub graphics_clk_slow_divider: u8, pub fps_low_t: u8,
    pub low_sclk_interrupt_t: u32, pub uvd_power_gated: bool,
    pub vce_power_gated: bool, pub acp_power_gated: bool, pub samu_power_gated: bool,
    pub nb_dpm_enabled: bool, pub enable_didt: bool, pub enable_dpm: bool,
    pub enable_auto_thermal_throttling: bool, pub enable_nb_dpm: bool,
    pub caps_cac: bool, pub caps_power_containment: bool, pub caps_sq_ramping: bool,
    pub caps_db_ramping: bool, pub caps_td_ramping: bool, pub caps_tcp_ramping: bool,
    pub caps_sclk_throttle_low_notification: bool, pub caps_fps: bool,
    pub caps_uvd_dpm: bool, pub caps_uvd_pg: bool, pub caps_vce_pg: bool,
    pub caps_samu_pg: bool, pub caps_acp_pg: bool, pub caps_stable_p_state: bool,
    pub caps_enable_dfs_bypass: bool, pub caps_sclk_ds: bool,
    pub current_rps: amdgpu_ps, pub current_ps: kv_ps,
    pub requested_rps: amdgpu_ps, pub requested_ps: kv_ps,
}

pub const KV_TEMP_RANGE_MIN: u32 = 90 * 1000;
pub const KV_TEMP_RANGE_MAX: u32 = 120 * 1000;

extern "C" {
    pub fn amdgpu_kv_notify_message_to_smu(adev: *mut amdgpu_device, id: u32) -> i32;
    pub fn amdgpu_kv_dpm_get_enable_mask(adev: *mut amdgpu_device, enable_mask: *mut u32) -> i32;
    pub fn amdgpu_kv_send_msg_to_smc_with_parameter(adev: *mut amdgpu_device, msg: PPSMC_Msg, parameter: u32) -> i32;
    pub fn amdgpu_kv_read_smc_sram_dword(adev: *mut amdgpu_device, smc_address: u32, value: *mut u32, limit: u32) -> i32;
    pub fn amdgpu_kv_smc_dpm_enable(adev: *mut amdgpu_device, enable: bool) -> i32;
    pub fn amdgpu_kv_smc_bapm_enable(adev: *mut amdgpu_device, enable: bool) -> i32;
    pub fn amdgpu_kv_copy_bytes_to_smc(adev: *mut amdgpu_device, smc_start_address: u32, src: *const u8, byte_count: u32, limit: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
