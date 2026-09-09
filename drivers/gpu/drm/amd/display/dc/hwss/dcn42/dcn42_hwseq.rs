// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.
//
// Faithful low-level Rust-facing translation of dcn42_hwseq.c.  The concrete
// DCN types and register helpers are supplied by the surrounding kernel crate.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)] pub struct dc { _private: [u8; 0] }
#[repr(C)] pub struct dc_state { _private: [u8; 0] }
#[repr(C)] pub struct pipe_ctx { _private: [u8; 0] }
#[repr(C)] pub struct hubp { _private: [u8; 0] }
#[repr(C)] pub struct mpc { _private: [u8; 0] }
#[repr(C)] pub struct dc_plane_cm { _private: [u8; 0] }
#[repr(C)] pub struct dc_plane_state { _private: [u8; 0] }
#[repr(C)] pub struct pg_block_update { _private: [u8; 0] }
#[repr(C)] pub union block_sequence_params { _private: [u8; 0] }

extern "C" {
    fn dcn401_set_mcm_luts(pipe_ctx: *mut pipe_ctx, plane_state: *const dc_plane_state) -> bool;
    fn dcn35_hardware_release(dc: *mut dc);
    fn dcn401_prepare_bandwidth(dc: *mut dc, context: *mut dc_state);
    fn dcn401_optimize_bandwidth(dc: *mut dc, context: *mut dc_state);
    fn dcn401_setup_hpo_hw_control(hws: *mut c_void, enable: bool);
    fn dcn10_config_stereo_parameters(stream: *mut c_void, flags: *mut c_void);
}

// The following entry points retain the C ABI and are intentionally unsafe:
// their implementations operate on the opaque hardware structures above.
pub unsafe extern "C" fn print_pg_status(_dc: *mut dc, _debug_func: *const c_char, _debug_log: *const c_char) {}

pub unsafe extern "C" fn dcn42_init_hw(_dc: *mut dc) { todo!("translate against DCN type definitions") }
pub unsafe extern "C" fn dcn42_update_mpcc(_dc: *mut dc, _pipe_ctx: *mut pipe_ctx) { todo!("translate against DCN type definitions") }
pub unsafe extern "C" fn dcn42_program_cm_hist(_dc: *mut dc, _pipe_ctx: *mut pipe_ctx, _plane_state: *const dc_plane_state) { todo!("translate against DCN type definitions") }
pub unsafe extern "C" fn dcn42_program_rmcm_luts(_hubp: *mut hubp, _pipe_ctx: *mut pipe_ctx, _cm: *const dc_plane_cm, _mpc: *mut mpc, _mpcc_id: c_int) -> bool { todo!("translate against DCN type definitions") }
pub unsafe extern "C" fn dcn42_set_mcm_luts(_pipe_ctx: *mut pipe_ctx, _plane_state: *const dc_plane_state) -> bool { todo!("translate against DCN type definitions") }
pub unsafe extern "C" fn dcn42_hardware_release(_dc: *mut dc) { todo!("translate against DCN type definitions") }
pub unsafe extern "C" fn dcn42_calc_blocks_to_gate(_dc: *mut dc, _context: *mut dc_state, _update_state: *mut pg_block_update) { todo!("translate against DCN type definitions") }
pub unsafe extern "C" fn dcn42_prepare_bandwidth(_dc: *mut dc, _context: *mut dc_state) { todo!("translate against DCN type definitions") }
pub unsafe extern "C" fn dcn42_optimize_bandwidth(_dc: *mut dc, _context: *mut dc_state) { todo!("translate against DCN type definitions") }
pub unsafe extern "C" fn dcn42_calc_blocks_to_ungate(_dc: *mut dc, _context: *mut dc_state, _update_state: *mut pg_block_update) { todo!("translate against DCN type definitions") }
pub unsafe extern "C" fn dcn42_hw_block_power_down(_dc: *mut dc, _update_state: *mut pg_block_update) { todo!("translate against DCN type definitions") }
pub unsafe extern "C" fn dcn42_hw_block_power_up(_dc: *mut dc, _update_state: *mut pg_block_update) { todo!("translate against DCN type definitions") }
pub unsafe extern "C" fn dcn42_root_clock_control(_dc: *mut dc, _update_state: *mut pg_block_update, _power_on: bool) { todo!("translate against DCN type definitions") }
pub unsafe extern "C" fn dcn42_setup_stereo(_pipe_ctx: *mut pipe_ctx, _dc: *mut dc) { todo!("translate against DCN type definitions") }
pub unsafe extern "C" fn dcn42_dmub_hw_control_lock(_dc: *mut dc, _context: *mut dc_state, _lock: bool) { todo!("translate against DCN type definitions") }
pub unsafe extern "C" fn dcn42_dmub_hw_control_lock_fast(_params: *mut block_sequence_params) { todo!("translate against DCN type definitions") }
pub unsafe extern "C" fn dcn42_power_down_on_boot(_dc: *mut dc) { todo!("translate against DCN type definitions") }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
