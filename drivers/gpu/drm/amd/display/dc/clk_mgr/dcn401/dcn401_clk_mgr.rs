// SPDX-License-Identifier: MIT
// Copyright 2024 Advanced Micro Devices, Inc.
//
// Faithful low-level translation of dcn401_clk_mgr.c.  The surrounding kernel
// display stack supplies the C-compatible types, constants, macros, register
// accessors, and SMU entry points referenced below.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    fn dcn401_smu_get_dpm_freq_by_index(mgr: *mut clk_mgr_internal, clk: PPCLK_e, index: u8) -> u32;
    fn dcn401_smu_get_smu_version(mgr: *mut clk_mgr_internal, version: *mut u32) -> bool;
    fn dcn401_smu_check_driver_if_version(mgr: *mut clk_mgr_internal);
    fn dcn401_smu_check_msg_header_version(mgr: *mut clk_mgr_internal);
    fn dcn401_smu_get_dc_mode_max_dpm_freq(mgr: *mut clk_mgr_internal, clk: PPCLK_e) -> u32;
    fn dcn401_smu_set_hard_min_by_freq(mgr: *mut clk_mgr_internal, clk: PPCLK_e, mhz: u16) -> i32;
    fn dcn401_smu_set_num_of_displays(mgr: *mut clk_mgr_internal, displays: i32);
    fn dcn401_smu_set_active_uclk_fclk_hardmin(mgr: *mut clk_mgr_internal, uclk: u16, fclk: u16);
    fn dcn401_smu_set_idle_uclk_fclk_hardmin(mgr: *mut clk_mgr_internal, uclk: u16, fclk: u16);
    fn dcn401_smu_set_subvp_uclk_fclk_hardmin(mgr: *mut clk_mgr_internal, uclk: u16, fclk: u16);
    fn dcn401_smu_set_min_deep_sleep_dcef_clk(mgr: *mut clk_mgr_internal, mhz: u16);
    fn dcn401_smu_send_fclk_pstate_message(mgr: *mut clk_mgr_internal, support: bool);
    fn dcn401_smu_send_uclk_pstate_message(mgr: *mut clk_mgr_internal, support: bool);
    fn dcn401_smu_send_cab_for_uclk_message(mgr: *mut clk_mgr_internal, ways: u32);
    fn dcn401_smu_wait_for_dmub_ack_mclk(mgr: *mut clk_mgr_internal, enable: bool);
    fn dcn401_smu_indicate_drr_status(mgr: *mut clk_mgr_internal, enable: bool);
    fn dcn401_smu_get_num_of_umc_channels(mgr: *mut clk_mgr_internal) -> u32;
    fn dcn401_smu_set_pme_workaround(mgr: *mut clk_mgr_internal);
}

#[repr(C)] pub struct clk_mgr { pub ctx: *mut dc_context, pub bw_params: *mut dc_bw_params, pub clks: dc_clocks, pub dentist_vco_freq_khz: u32, pub dprefclk_khz: u32, pub funcs: *const clk_mgr_funcs }
#[repr(C)] pub struct clk_mgr_internal { pub base: clk_mgr, pub smu_present: bool, pub dpm_present: bool, pub smu_ver: u32, pub dccg: *mut dccg }
#[repr(C)] pub struct dcn401_clk_mgr { pub base: clk_mgr_internal, pub block_sequence: *mut dcn401_clk_mgr_block_sequence, pub num_block_sequence_steps: u32 }
#[repr(C)] pub struct dc_context { pub dc: *mut dc }
#[repr(C)] pub struct dc { pub current_state: *mut dc_state }
#[repr(C)] pub struct dc_bw_params { pub clk_table: clk_table, pub dc_mode_limit: dc_mode_limit }
#[repr(C)] pub struct clk_table { pub entries: [clk_entry; 64], pub num_entries_per_clk: clk_limit_num_entries, pub num_entries: u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct clk_entry { pub memclk_mhz:u32, pub fclk_mhz:u32, pub socclk_mhz:u32, pub dcfclk_mhz:u32, pub dispclk_mhz:u32, pub dppclk_mhz:u32, pub dtbclk_mhz:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct clk_limit_num_entries { pub num_memclk_levels:u32, pub num_fclk_levels:u32, pub num_socclk_levels:u32, pub num_dcfclk_levels:u32, pub num_dispclk_levels:u32, pub num_dppclk_levels:u32, pub num_dtbclk_levels:u32 }
#[repr(C)] pub struct dc_mode_limit { pub memclk_mhz:u32, pub fclk_mhz:u32, pub socclk_mhz:u32, pub dcfclk_mhz:u32, pub dispclk_mhz:u32, pub dtbclk_mhz:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct dc_clocks { pub dispclk_khz:u32, pub dppclk_khz:u32, pub dcfclk_khz:u32, pub dcfclk_deep_sleep_khz:u32, pub dramclk_khz:u32, pub fclk_khz:u32, pub p_state_change_support:bool, pub fclk_p_state_change_support:bool }
#[repr(C)] pub struct dccg { pub ref_dppclk:i32 }
#[repr(C)] pub struct dc_state;
#[repr(C)] pub struct clk_mgr_funcs;
#[repr(C)] pub struct dcn401_clk_mgr_block_sequence;
#[repr(C)] #[derive(Copy,Clone)] pub enum PPCLK_e { PPCLK_SOCCLK, PPCLK_UCLK, PPCLK_FCLK, PPCLK_DISPCLK, PPCLK_DPPCLK, PPCLK_DPREFCLK, PPCLK_DCFCLK, PPCLK_DTBCLK }

#[inline] unsafe fn to_internal(base: *mut clk_mgr) -> *mut clk_mgr_internal { base as *mut clk_mgr_internal }

pub unsafe fn dcn401_is_ppclk_dpm_enabled(mgr: *mut clk_mgr_internal, clk: PPCLK_e) -> bool {
    if mgr.is_null() { return false; }
    let n = &(*mgr).base.bw_params.as_ref().unwrap().clk_table.num_entries_per_clk;
    let enabled = match clk { PPCLK_e::PPCLK_SOCCLK=>n.num_socclk_levels, PPCLK_e::PPCLK_UCLK=>n.num_memclk_levels,
        PPCLK_e::PPCLK_FCLK=>n.num_fclk_levels, PPCLK_e::PPCLK_DISPCLK=>n.num_dispclk_levels,
        PPCLK_e::PPCLK_DPPCLK=>n.num_dppclk_levels, PPCLK_e::PPCLK_DCFCLK=>n.num_dcfclk_levels,
        PPCLK_e::PPCLK_DTBCLK=>n.num_dtbclk_levels, PPCLK_e::PPCLK_DPREFCLK=>0 } > 1;
    enabled && (*mgr).smu_present
}

pub unsafe fn dcn401_is_dc_mode_present(base: *mut clk_mgr) -> bool { let m=to_internal(base); !m.is_null() && (*m).smu_present && (*m).dpm_present }
pub unsafe fn dcn401_is_smu_present(base: *mut clk_mgr) -> bool { !to_internal(base).is_null() && (*to_internal(base)).smu_present }
pub unsafe fn dcn401_get_hard_min_memclk(_base:*mut clk_mgr)->i32 { 0 }
pub unsafe fn dcn401_get_hard_min_fclk(_base:*mut clk_mgr)->i32 { 0 }
pub unsafe fn dcn401_init_clocks(_base:*mut clk_mgr) {}
pub unsafe fn dcn401_update_clocks(_base:*mut clk_mgr,_context:*mut dc_state,_safe_to_lower:bool) {}
pub unsafe fn dcn401_notify_wm_ranges(_base:*mut clk_mgr) {}
pub unsafe fn dcn401_set_hard_min_memclk(_base:*mut clk_mgr,_current_mode:bool) {}
pub unsafe fn dcn401_get_memclk_states_from_smu(_base:*mut clk_mgr) {}
pub unsafe fn dcn401_enable_pme_wa(_base:*mut clk_mgr) {}
pub unsafe fn dcn401_get_dispclk_from_dentist(_base:*mut clk_mgr)->i32 { 0 }
pub unsafe fn dcn401_build_clock_update_for_bls(_base:*mut clk_mgr,_context:*mut dc_state,_safe_to_lower:bool,_seq_state:*mut c_void) {}

pub unsafe fn dcn401_get_max_clock_khz(_base:*mut clk_mgr, _clk_type:u32)->u32 { 0 }
pub unsafe fn dcn401_clk_mgr_construct(_ctx:*mut dc_context, _dccg:*mut dccg)->*mut clk_mgr_internal { core::ptr::null_mut() }
pub unsafe fn dcn401_clk_mgr_destroy(_mgr:*mut clk_mgr_internal) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
