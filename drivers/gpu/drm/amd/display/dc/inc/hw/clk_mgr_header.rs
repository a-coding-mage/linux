/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2012-2026 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// C dependencies and build-time conditions are supplied by the surrounding translation unit.

pub const DDR4_DRAM_WIDTH: u32 = 64;
pub const WM_A: u32 = 0;
pub const WM_B: u32 = 1;
pub const WM_C: u32 = 2;
pub const WM_D: u32 = 3;
pub const WM_SET_COUNT: usize = 4;
pub const WM_1A: u32 = 2;
pub const WM_1B: u32 = 3;
pub const DCN_MINIMUM_DISPCLK_KHZ: u32 = 100000;
pub const DCN_MINIMUM_DPPCLK_KHZ: u32 = 100000;
pub const MAX_NUM_DPM_LVL: usize = 8;

#[repr(C)] pub struct dcn3_clk_internal { pub dummy: i32, pub CLK1_CLK0_CURRENT_CNT: u32, pub CLK1_CLK1_CURRENT_CNT: u32, pub CLK1_CLK2_CURRENT_CNT: u32, pub CLK1_CLK3_CURRENT_CNT: u32, pub CLK1_CLK4_CURRENT_CNT: u32, pub CLK1_CLK3_DS_CNTL: u32, pub CLK1_CLK3_ALLOW_DS: u32, pub CLK1_CLK0_BYPASS_CNTL: u32, pub CLK1_CLK1_BYPASS_CNTL: u32, pub CLK1_CLK2_BYPASS_CNTL: u32, pub CLK1_CLK3_BYPASS_CNTL: u32, pub CLK4_CLK0_CURRENT_CNT: u32 }
#[repr(C)] pub struct dcn35_clk_internal { pub dummy: i32, pub CLK1_CLK0_CURRENT_CNT: u32, pub CLK1_CLK1_CURRENT_CNT: u32, pub CLK1_CLK2_CURRENT_CNT: u32, pub CLK1_CLK3_CURRENT_CNT: u32, pub CLK1_CLK4_CURRENT_CNT: u32, pub CLK1_CLK3_DS_CNTL: u32, pub CLK1_CLK3_ALLOW_DS: u32, pub CLK1_CLK0_BYPASS_CNTL: u32, pub CLK1_CLK1_BYPASS_CNTL: u32, pub CLK1_CLK2_BYPASS_CNTL: u32, pub CLK1_CLK3_BYPASS_CNTL: u32, pub CLK1_CLK4_BYPASS_CNTL: u32 }
#[repr(C)] pub struct dcn301_clk_internal { pub dummy: i32, pub CLK1_CLK0_CURRENT_CNT: u32, pub CLK1_CLK1_CURRENT_CNT: u32, pub CLK1_CLK2_CURRENT_CNT: u32, pub CLK1_CLK3_CURRENT_CNT: u32, pub CLK1_CLK3_DS_CNTL: u32, pub CLK1_CLK3_ALLOW_DS: u32, pub CLK1_CLK0_BYPASS_CNTL: u32, pub CLK1_CLK1_BYPASS_CNTL: u32, pub CLK1_CLK2_BYPASS_CNTL: u32, pub CLK1_CLK3_BYPASS_CNTL: u32 }
#[repr(C)] pub struct dcn42_clk_internal { pub dummy: i32, pub CLK8_CLK0_CURRENT_CNT: u32, pub CLK8_CLK1_CURRENT_CNT: u32, pub CLK8_CLK2_CURRENT_CNT: u32, pub CLK8_CLK3_CURRENT_CNT: u32, pub CLK8_CLK4_CURRENT_CNT: u32, pub CLK8_CLK0_DS_CNTL: u32, pub CLK8_CLK1_DS_CNTL: u32, pub CLK8_CLK2_DS_CNTL: u32, pub CLK8_CLK3_DS_CNTL: u32, pub CLK8_CLK4_DS_CNTL: u32, pub CLK8_CLK0_BYPASS_CNTL: u32, pub CLK8_CLK1_BYPASS_CNTL: u32, pub CLK8_CLK2_BYPASS_CNTL: u32, pub CLK8_CLK3_BYPASS_CNTL: u32, pub CLK8_CLK4_BYPASS_CNTL: u32, pub CLK8_CLK_TICK_CNT__TIMER_THRESHOLD: u32 }
#[repr(C)] pub struct dcn42b_clk_internal { pub dummy: i32, pub CLK5_CLK0_CURRENT_CNT: u32, pub CLK5_CLK1_CURRENT_CNT: u32, pub CLK5_CLK2_CURRENT_CNT: u32, pub CLK5_CLK3_CURRENT_CNT: u32, pub CLK5_CLK0_DS_CNTL: u32, pub CLK5_CLK1_DS_CNTL: u32, pub CLK5_CLK2_DS_CNTL: u32, pub CLK5_CLK3_DS_CNTL: u32, pub CLK5_CLK3_ALLOW_DS: u32, pub CLK5_CLK0_BYPASS_CNTL: u32, pub CLK5_CLK1_BYPASS_CNTL: u32, pub CLK5_CLK2_BYPASS_CNTL: u32, pub CLK5_CLK3_BYPASS_CNTL: u32, pub CLK5_CLK_TICK_CNT__TIMER_THRESHOLD: u32 }

#[repr(C)] #[derive(Copy, Clone)] pub enum clk_type { CLK_TYPE_DCFCLK, CLK_TYPE_FCLK, CLK_TYPE_MCLK, CLK_TYPE_SOCCLK, CLK_TYPE_DTBCLK, CLK_TYPE_DISPCLK, CLK_TYPE_DPPCLK, CLK_TYPE_DSCCLK, CLK_TYPE_COUNT }

#[repr(C)] pub struct clk_limit_table_entry { pub voltage: u32, pub dcfclk_mhz: u32, pub fclk_mhz: u32, pub memclk_mhz: u32, pub socclk_mhz: u32, pub dtbclk_mhz: u32, pub dispclk_mhz: u32, pub dppclk_mhz: u32, pub phyclk_mhz: u32, pub phyclk_d18_mhz: u32, pub wck_ratio: u32 }
#[repr(C)] pub struct clk_limit_num_entries { pub num_dcfclk_levels: u32, pub num_fclk_levels: u32, pub num_memclk_levels: u32, pub num_socclk_levels: u32, pub num_dtbclk_levels: u32, pub num_dispclk_levels: u32, pub num_dppclk_levels: u32, pub num_phyclk_levels: u32, pub num_phyclk_d18_levels: u32 }
#[repr(C)] pub struct clk_limit_table { pub entries: [clk_limit_table_entry; MAX_NUM_DPM_LVL], pub num_entries_per_clk: clk_limit_num_entries, pub num_entries: u32 }
#[repr(C)] pub struct wm_range_table_entry { pub wm_inst: u32, pub wm_type: u32, pub pstate_latency_us: f64, pub sr_exit_time_us: f64, pub sr_enter_plus_exit_time_us: f64, pub valid: bool }
#[repr(C)] pub struct nv_wm_range_entry { pub valid: bool, pub pmfw_breakdown: nv_wm_pmfw_breakdown, pub dml_input: nv_wm_dml_input }
#[repr(C)] pub struct nv_wm_pmfw_breakdown { pub wm_type: u8, pub min_dcfclk: u16, pub max_dcfclk: u16, pub min_uclk: u16, pub max_uclk: u16 }
#[repr(C)] pub struct nv_wm_dml_input { pub pstate_latency_us: f64, pub sr_exit_time_us: f64, pub sr_enter_plus_exit_time_us: f64, pub fclk_change_latency_us: f64 }
#[repr(C)] pub struct clk_log_info { pub enabled: bool, pub pBuf: *mut i8, pub bufSize: u32, pub sum_chars_printed: *mut u32 }
#[repr(C)] pub struct clk_state_registers_and_bypass { pub dcfclk: u32, pub dcf_deep_sleep_divider: u32, pub dcf_deep_sleep_allow: u32, pub dprefclk: u32, pub dispclk: u32, pub dppclk: u32, pub dtbclk: u32, pub fclk: u32, pub dppclk_bypass: u32, pub dcfclk_bypass: u32, pub dprefclk_bypass: u32, pub dispclk_bypass: u32, pub timer_threshold: u32 }
#[repr(C)] pub struct rv1_clk_internal { pub CLK0_CLK8_CURRENT_CNT: u32, pub CLK0_CLK8_DS_CNTL: u32, pub CLK0_CLK8_ALLOW_DS: u32, pub CLK0_CLK10_CURRENT_CNT: u32, pub CLK0_CLK11_CURRENT_CNT: u32, pub CLK0_CLK8_BYPASS_CNTL: u32, pub CLK0_CLK10_BYPASS_CNTL: u32, pub CLK0_CLK11_BYPASS_CNTL: u32 }
#[repr(C)] pub struct rn_clk_internal { pub CLK1_CLK0_CURRENT_CNT: u32, pub CLK1_CLK1_CURRENT_CNT: u32, pub CLK1_CLK2_CURRENT_CNT: u32, pub CLK1_CLK3_CURRENT_CNT: u32, pub CLK1_CLK3_DS_CNTL: u32, pub CLK1_CLK3_ALLOW_DS: u32, pub CLK1_CLK0_BYPASS_CNTL: u32, pub CLK1_CLK1_BYPASS_CNTL: u32, pub CLK1_CLK2_BYPASS_CNTL: u32, pub CLK1_CLK3_BYPASS_CNTL: u32 }
#[repr(C)] pub struct clk_state_registers { pub CLK0_CLK8_CURRENT_CNT: u32, pub CLK0_CLK8_DS_CNTL: u32, pub CLK0_CLK8_ALLOW_DS: u32, pub CLK0_CLK10_CURRENT_CNT: u32, pub CLK0_CLK11_CURRENT_CNT: u32 }
#[repr(C)] pub struct clk_bypass { pub dcfclk_bypass: u32, pub dispclk_pypass: u32, pub dprefclk_bypass: u32 }
#[repr(C)] pub union wm_table { pub nv_entries: [nv_wm_range_entry; WM_SET_COUNT], pub entries: [wm_range_table_entry; WM_SET_COUNT] }
#[repr(C)] pub struct dummy_pstate_entry { pub dram_speed_mts: u32, pub dummy_pstate_latency_us: u32 }

#[repr(C)] pub struct clk_bw_params { pub vram_type: u32, pub num_channels: u32, pub dram_channel_width_bytes: u32, pub dispclk_vco_khz: u32, pub dc_mode_softmax_memclk: u32, pub max_memclk_mhz: u32, pub clk_table: clk_limit_table, pub wm_table: wm_table, pub dummy_pstate_table: [dummy_pstate_entry; 4], pub dc_mode_limit: clk_limit_table_entry, pub utm_qos_model: *const utm_qos_model }
#[repr(C)] pub struct clk_states { pub dprefclk_khz: u32 }

// External types supplied by the included DC headers.
#[repr(C)] pub struct utm_qos_model { _private: [u8; 0] }
#[repr(C)] pub struct dc_context { _private: [u8; 0] }
#[repr(C)] pub struct dc_state { _private: [u8; 0] }
#[repr(C)] pub struct dc_link { _private: [u8; 0] }
#[repr(C)] pub struct dc_clocks { _private: [u8; 0] }
#[repr(C)] pub struct dc_clock_config { _private: [u8; 0] }
#[repr(C)] pub struct dc_requested_memory_qos { _private: [u8; 0] }
#[repr(C)] pub struct pp_smu_wm_range_sets { _private: [u8; 0] }
#[repr(C)] pub struct pp_smu_funcs { _private: [u8; 0] }
#[repr(C)] pub struct dccg { _private: [u8; 0] }
#[repr(C)] pub struct dc { _private: [u8; 0] }
#[repr(C)] pub struct block_sequence_state { _private: [u8; 0] }
#[repr(C)] pub enum dc_clock_type {}

#[repr(C)] pub struct clk_mgr_funcs {
    pub update_clocks: Option<unsafe extern "C" fn(*mut clk_mgr, *mut dc_state, bool)>,
    pub get_dp_ref_clk_frequency: Option<unsafe extern "C" fn(*mut clk_mgr) -> i32>,
    pub get_dtb_ref_clk_frequency: Option<unsafe extern "C" fn(*mut clk_mgr) -> i32>,
    pub set_low_power_state: Option<unsafe extern "C" fn(*mut clk_mgr)>,
    pub exit_low_power_state: Option<unsafe extern "C" fn(*mut clk_mgr)>,
    pub is_ips_supported: Option<unsafe extern "C" fn(*mut clk_mgr) -> bool>,
    pub set_idle_power_optimizations: Option<unsafe extern "C" fn(*mut clk_mgr, bool)>,
    pub init_clocks: Option<unsafe extern "C" fn(*mut clk_mgr)>,
    pub dump_clk_registers: Option<unsafe extern "C" fn(*mut clk_state_registers_and_bypass, *mut clk_mgr, *mut clk_log_info)>,
    pub enable_pme_wa: Option<unsafe extern "C" fn(*mut clk_mgr)>,
    pub get_clock: Option<unsafe extern "C" fn(*mut clk_mgr, *mut dc_state, dc_clock_type, *mut dc_clock_config)>,
    pub are_clock_states_equal: Option<unsafe extern "C" fn(*mut dc_clocks, *mut dc_clocks) -> bool>,
    pub notify_wm_ranges: Option<unsafe extern "C" fn(*mut clk_mgr)>,
    pub notify_link_rate_change: Option<unsafe extern "C" fn(*mut clk_mgr, *mut dc_link)>,
    pub set_hard_min_memclk: Option<unsafe extern "C" fn(*mut clk_mgr, bool)>,
    pub get_hard_min_memclk: Option<unsafe extern "C" fn(*mut clk_mgr) -> i32>,
    pub get_hard_min_fclk: Option<unsafe extern "C" fn(*mut clk_mgr) -> i32>,
    pub set_hard_max_memclk: Option<unsafe extern "C" fn(*mut clk_mgr)>,
    pub set_max_memclk: Option<unsafe extern "C" fn(*mut clk_mgr, u32)>,
    pub set_min_memclk: Option<unsafe extern "C" fn(*mut clk_mgr, u32)>,
    pub get_memclk_states_from_smu: Option<unsafe extern "C" fn(*mut clk_mgr)>,
    pub is_smu_present: Option<unsafe extern "C" fn(*mut clk_mgr) -> bool>,
    pub get_dispclk_from_dentist: Option<unsafe extern "C" fn(*mut clk_mgr) -> i32>,
    pub is_dc_mode_present: Option<unsafe extern "C" fn(*mut clk_mgr) -> bool>,
    pub set_smartmux_switch: Option<unsafe extern "C" fn(*mut clk_mgr, u32) -> u32>,
    pub get_max_clock_khz: Option<unsafe extern "C" fn(*mut clk_mgr, clk_type) -> u32>,
    pub override_memory_bandwidth_request: Option<unsafe extern "C" fn(*mut clk_mgr, u32) -> u32>,
    pub get_requested_memory_qos: Option<unsafe extern "C" fn(*mut clk_mgr, *mut dc_requested_memory_qos)>,
    pub notify_cstate_disable: Option<unsafe extern "C" fn(*mut clk_mgr, bool)>,
    pub build_clock_update_for_bls: Option<unsafe extern "C" fn(*mut clk_mgr, *mut dc_state, bool, *mut block_sequence_state)>,
    pub execute_clk_mgr_block_sequence: Option<unsafe extern "C" fn(*mut clk_mgr)>,
    pub request_dtbclk: Option<unsafe extern "C" fn(*mut clk_mgr, bool)>,
}

#[repr(C)] pub struct clk_mgr { pub ctx: *mut dc_context, pub funcs: *mut clk_mgr_funcs, pub clks: dc_clocks, pub psr_allow_active_cache: bool, pub force_smu_not_present: bool, pub dc_mode_softmax_enabled: bool, pub dprefclk_khz: i32, pub dp_dto_source_clock_in_khz: i32, pub dentist_vco_freq_khz: i32, pub boot_snapshot: clk_state_registers_and_bypass, pub bw_params: *mut clk_bw_params, pub ranges: pp_smu_wm_range_sets }

unsafe extern "C" {
    pub fn dc_clk_mgr_create(ctx: *mut dc_context, pp_smu: *mut pp_smu_funcs, dccg: *mut dccg) -> *mut clk_mgr;
    pub fn dc_destroy_clk_mgr(clk_mgr: *mut clk_mgr);
    pub fn clk_mgr_exit_optimized_pwr_state(dc: *const dc, clk_mgr: *mut clk_mgr);
    pub fn clk_mgr_optimize_pwr_state(dc: *const dc, clk_mgr: *mut clk_mgr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
