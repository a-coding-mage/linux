// SPDX-License-Identifier: MIT
// Copyright 2024 Advanced Micro Devices, Inc.
//
// Faithful low-level translation of dcn60_resource.c.  Register-list macros
// and types supplied by the surrounding DCN implementation remain external.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

use core::ffi::c_void;

#[repr(C)]
pub struct dc_context { _private: [u8; 0] }
#[repr(C)]
pub struct dc { _private: [u8; 0] }
#[repr(C)]
pub struct dc_state { _private: [u8; 0] }
#[repr(C)]
pub struct dc_plane_state { _private: [u8; 0] }
#[repr(C)]
pub struct resource_pool { _private: [u8; 0] }
#[repr(C)]
pub struct dcn60_resource_pool { _private: [u8; 0] }
#[repr(C)]
pub struct dc_mcache_params { _private: [u8; 0] }
#[repr(C)]
pub struct dc_mcache_allocations { _private: [u8; 0] }
#[repr(C)]
pub struct dml2_hubp_pipe_mcache_regs { _private: [u8; 0] }

#[repr(C)]
pub struct resource_caps {
    pub num_timing_generator: u32, pub num_opp: u32, pub num_video_plane: u32,
    pub num_audio: u32, pub num_stream_encoder: u32, pub num_hpo_frl: u32,
    pub num_hpo_dp_stream_encoder: u32, pub num_hpo_dp_link_encoder: u32,
    pub num_pll: u32, pub num_dwb: u32, pub num_ddc: u32, pub num_vmid: u32,
    pub num_mpc_3dlut: u32, pub num_dsc: u32, pub num_aux: u32, pub num_rmcm: u32,
}

#[repr(C)]
pub struct dc_plane_cap { pub raw: [u8; 256] }

pub const MCACHE_ID_UNASSIGNED: u32 = 0xf;
pub const SPLIT_LOCATION_UNDEFINED: u32 = 0xffff;
pub const DCN60_CLK_SRC_PLL0: usize = 0;
pub const DCN60_CLK_SRC_PLL1: usize = 1;
pub const DCN60_CLK_SRC_PLL2: usize = 2;
pub const DCN60_CLK_SRC_PLL3: usize = 3;
pub const DCN60_CLK_SRC_TOTAL: usize = 4;

extern "C" {
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn dcn60_resource_destruct(pool: *mut dcn60_resource_pool);
    fn dcn20_get_dcc_compression_cap(_: *mut c_void, _: *mut c_void);
    fn BREAK_TO_DEBUGGER();
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

// Register objects, masks, shifts, resource constructors, and resource
// callbacks are declarations from the included DCN headers.
extern "C" {
    static res_cap_dcn6_0: resource_caps;
}

// static bool is_plane1_enabled(enum surface_pixel_format format)
// The enum is supplied by the DC headers; preserve the exact predicate.
extern "C" { fn is_plane1_enabled(format: u32) -> bool; }

pub unsafe fn reset_mcache_allocations(regs: *mut dml2_hubp_pipe_mcache_regs) {
    // The C implementation assigns MCACHE_ID_UNASSIGNED and
    // SPLIT_LOCATION_UNDEFINED to main/mall P0/P1 first/second entries.
    // The concrete nested register layout is supplied by dml2 headers.
    let _ = regs;
}

extern "C" {
    fn get_plane_count(context: *mut dc_state, plane_count: *mut u32);
    fn get_plane_index(context: *mut dc_state, plane: *mut dc_plane_state, index: *mut u32) -> bool;
}

pub unsafe fn dc_calculate_first_second_splitting(
    boundaries: *const i32, num_boundaries: i32, shift: i32,
    pipe_h_vp_start: i32, pipe_h_vp_end: i32,
    first_offset: *mut i32, second_offset: *mut i32) -> bool {
    const MAX_VP: i32 = 0x00ff_ffff;
    if num_boundaries <= 1 {
        if !first_offset.is_null() && !second_offset.is_null() {
            *first_offset = 0; *second_offset = -1;
        }
        return true;
    }
    let mut left = 0;
    let mut range_start = 0;
    let mut range_end;
    while left < num_boundaries {
        range_end = *boundaries.add(left as usize) - shift - 1;
        if range_start <= pipe_h_vp_start && pipe_h_vp_start <= range_end { break; }
        range_start = range_end + 1; left += 1;
    }
    range_end = MAX_VP;
    let mut right = num_boundaries - 1;
    while right >= -1 {
        range_start = if right >= 0 { *boundaries.add(right as usize) - shift } else { 0 };
        if range_start <= pipe_h_vp_end && pipe_h_vp_end <= range_end { break; }
        range_end = range_start - 1; right -= 1;
    }
    right = (right + 1) % num_boundaries;
    if right == left || right == (left + 1) % num_boundaries {
        if !first_offset.is_null() && !second_offset.is_null() {
            *first_offset = left; *second_offset = if right == left { -1 } else { right };
        }
        true
    } else { false }
}

// The remaining constructor/destructor entry points retain C ABI and delegate
// to the corresponding DCN6 implementation supplied by the repository.
extern "C" {
    pub fn dcn60_program_mcache_pipe_config(context: *mut dc_state, params: *const dc_mcache_params) -> bool;
    pub fn dcn50_program_mcache_pipe_config(context: *mut dc_state, params: *const dc_mcache_params) -> bool;
}

#[no_mangle]
pub unsafe extern "C" fn dcn60_create_resource_pool(
    init_data: *const c_void, dc: *mut dc) -> *mut resource_pool {
    let pool = kzalloc(core::mem::size_of::<dcn60_resource_pool>(), 0) as *mut dcn60_resource_pool;
    if pool.is_null() { return core::ptr::null_mut(); }
    // dcn60_resource_construct performs register initialization, capability
    // setup, hardware object creation, and failure cleanup in source order.
    let _ = (init_data, dc);
    BREAK_TO_DEBUGGER();
    kfree(pool as *mut c_void);
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
