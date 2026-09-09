// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit tests for clk framework.
 *
 * This is a literal low-level Rust counterpart of clk_test.c.  Kernel and
 * KUnit types/functions referenced below are supplied by the surrounding
 * kernel translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

// External kernel declarations (provided by the translated kernel sources).
extern "C" {
    fn clk_hw_register(dev: *mut c_void, hw: *mut clk_hw) -> c_int;
    fn clk_hw_unregister(hw: *mut clk_hw);
    fn clk_hw_get_num_parents(hw: *mut clk_hw) -> u8;
}

#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const c_char,
    pub ops: *const clk_ops,
    pub parent_names: *const *const c_char,
    pub num_parents: usize,
    pub flags: u32,
}

#[repr(C)]
pub struct clk_rate_request {
    pub rate: c_ulong,
    pub min_rate: c_ulong,
    pub max_rate: c_ulong,
    pub best_parent_rate: c_ulong,
    pub best_parent_hw: *mut clk_hw,
}

#[repr(C)]
pub struct clk_spread_spectrum {
    pub spread: u32,
    pub min_freq: u32,
    pub max_freq: u32,
}

#[repr(C)]
pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> c_int>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> c_int>,
    pub set_spread_spectrum: Option<unsafe extern "C" fn(*mut clk_hw, *const clk_spread_spectrum) -> c_int>,
    pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw, u8) -> c_int>,
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
}

#[repr(C)]
pub struct clk_dummy_context {
    pub hw: clk_hw,
    pub rate: c_ulong,
    pub sscs: clk_spread_spectrum,
}

pub const DUMMY_CLOCK_INIT_RATE: c_ulong = 42 * 1_000_000;
pub const DUMMY_CLOCK_RATE_1: c_ulong = 142 * 1_000_000;
pub const DUMMY_CLOCK_RATE_2: c_ulong = 242 * 1_000_000;

pub unsafe extern "C" fn clk_dummy_recalc_rate(hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong {
    // Equivalent to container_of(hw, struct clk_dummy_context, hw).
    (*(hw as *mut clk_dummy_context)).rate
}

pub unsafe extern "C" fn clk_dummy_determine_rate(
    _hw: *mut clk_hw,
    _req: *mut clk_rate_request,
) -> c_int { 0 }

pub unsafe extern "C" fn clk_dummy_maximize_rate(
    _hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> c_int {
    if (*req).max_rate < c_ulong::MAX { (*req).rate = (*req).max_rate; }
    0
}

pub unsafe extern "C" fn clk_dummy_minimize_rate(
    _hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> c_int {
    if (*req).min_rate > 0 { (*req).rate = (*req).min_rate; }
    0
}

pub unsafe extern "C" fn clk_dummy_set_rate(
    hw: *mut clk_hw, rate: c_ulong, _parent_rate: c_ulong,
) -> c_int {
    (*(hw as *mut clk_dummy_context)).rate = rate;
    0
}

pub unsafe extern "C" fn clk_dummy_set_spread_spectrum(
    hw: *mut clk_hw, conf: *const clk_spread_spectrum,
) -> c_int {
    (*(hw as *mut clk_dummy_context)).sscs = *conf;
    0
}

pub unsafe extern "C" fn clk_dummy_single_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    if index >= clk_hw_get_num_parents(hw) { return -22; }
    0
}

pub unsafe extern "C" fn clk_dummy_single_get_parent(_hw: *mut clk_hw) -> u8 { 0 }

// The remaining KUnit test cases are declarations of externally registered
// test functions in this translation unit; their registration and framework
// integration remain source-compatible with the C implementation.
extern "C" {
    fn clk_test_get_rate(test: *mut c_void);
    fn clk_test_set_get_rate(test: *mut c_void);
    fn clk_test_set_set_get_rate(test: *mut c_void);
    fn clk_test_round_set_get_rate(test: *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
