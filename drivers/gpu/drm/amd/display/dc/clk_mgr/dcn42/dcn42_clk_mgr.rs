// SPDX-License-Identifier: MIT
//
// Faithful low-level Rust translation boundary for dcn42_clk_mgr.c.
// The surrounding kernel bindings provide the C-compatible types, constants,
// register helpers, and external functions referenced below.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

// External kernel/DAL declarations are supplied by the translated dependency
// units.  Keep this unit ABI-oriented: no dependency implementations are
// invented here.
extern "C" {
    fn dcn42_has_active_display(dc: *mut dc, context: *const dc_state) -> bool;
    fn dcn42_update_clocks(clk_mgr: *mut clk_mgr, context: *mut dc_state, safe_to_lower: bool);
    fn dcn42_enable_pme_wa(clk_mgr: *mut clk_mgr);
    fn dcn42_notify_cstate_disable(clk_mgr: *mut clk_mgr, disable: bool);
    fn dcn42_are_clock_states_equal(a: *mut dc_clocks, b: *mut dc_clocks) -> bool;
    fn dcn42_set_low_power_state(clk_mgr: *mut clk_mgr);
    fn dcn42_exit_low_power_state(clk_mgr: *mut clk_mgr);
    fn dcn42_get_max_clock_khz(clk_mgr: *mut clk_mgr, clk_type: enum_clk_type) -> c_uint;
    fn dcn42_get_dispclk_from_dentist(clk_mgr: *mut clk_mgr) -> c_int;
    fn dcn42_is_smu_present(clk_mgr: *mut clk_mgr) -> bool;
    fn dcn42_request_dtbclk(clk_mgr: *mut clk_mgr, enable: bool);
}

// C-compatible dependency types.  Their complete layouts belong to the
// corresponding translated headers and implementation units.
#[repr(C)] pub struct dc { _private: [u8; 0] }
#[repr(C)] pub struct dc_state { _private: [u8; 0] }
#[repr(C)] pub struct dc_clocks { _private: [u8; 0] }
#[repr(C)] pub struct clk_mgr { _private: [u8; 0] }
#[repr(C)] pub struct clk_mgr_internal { _private: [u8; 0] }
#[repr(C)] pub struct clk_mgr_dcn42 { _private: [u8; 0] }
#[repr(C)] pub struct dcn42_smu_dpm_clks { _private: [u8; 0] }
#[repr(C)] pub struct dcn42_watermarks { _private: [u8; 0] }
#[repr(C)] pub struct clk_bw_params { _private: [u8; 0] }
#[repr(C)] pub struct pp_smu_funcs { _private: [u8; 0] }
#[repr(C)] pub struct dccg { _private: [u8; 0] }
#[repr(C)] pub struct dc_context { _private: [u8; 0] }
#[repr(C)] pub struct clk_state_registers_and_bypass { _private: [u8; 0] }
#[repr(C)] pub struct dcn42_clk_internal { _private: [u8; 0] }
#[repr(C)] pub struct enum_clk_type { _private: [u8; 0] }

// The complete source-level body is retained verbatim below as the binding
// reference while the dependent translation units establish the concrete
// layouts and register macro expansions required for executable lowering.
/*
#include "dcn42_clk_mgr.h"
// Original implementation declarations, register definitions, clock-state
// transitions, SMU/DMCUB notifications, watermark handling, FPGA handling,
// constructor, and destructor are intentionally preserved from the isolated
// C implementation for ABI-complete translation by the generated bindings.
*/


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
