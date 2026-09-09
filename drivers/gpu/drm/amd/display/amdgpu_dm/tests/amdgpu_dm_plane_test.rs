// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * KUnit tests for amdgpu_dm_plane.c
 *
 * Rust translation of the implementation test source.  Kernel types and
 * functions are supplied by the surrounding DRM/amdgpu bindings.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

// C headers are dependencies of this translation and are intentionally not
// reimplemented here.

#[repr(C)]
pub struct dm_test_dcc_cap_ctx {
    pub callback_ret: bool,
    pub capable: bool,
    pub output_independent_64b_blks: bool,
    pub called: bool,
    pub captured_input: dc_dcc_surface_param,
}

#[repr(C)]
pub struct dm_test_gfx11_reg_ctx {
    pub gb_addr_config: u32,
    pub gc_reg_offsets: [u32; 1],
    pub expected_reg: u32,
    pub captured_reg: u32,
    pub captured_acc_flags: u32,
    pub captured_hwip: u32,
    pub captured_xcc_id: u32,
    pub called: bool,
}

// External kernel/DC declarations.
#[repr(C)] pub struct dc_dcc_surface_param { _private: [u8; 0] }
#[repr(C)] pub struct dc { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)] pub struct kunit { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_rlc_reg_funcs { pub rreg32: Option<unsafe extern "C" fn(*mut amdgpu_device, u32, u32, u32, u32) -> u32> }

static mut dm_test_dcc_ctx: *mut dm_test_dcc_cap_ctx = core::ptr::null_mut();
static mut dm_test_gfx11_reg_ctx: *mut dm_test_gfx11_reg_ctx = core::ptr::null_mut();

unsafe extern "C" fn dm_test_gfx11_rreg32(
    _adev: *mut amdgpu_device, reg: u32, acc_flags: u32, hwip: u32, xcc_id: u32,
) -> u32 {
    let ctx = dm_test_gfx11_reg_ctx;
    if ctx.is_null() { return 0; }
    (*ctx).called = true;
    (*ctx).captured_reg = reg;
    (*ctx).captured_acc_flags = acc_flags;
    (*ctx).captured_hwip = hwip;
    (*ctx).captured_xcc_id = xcc_id;
    (*ctx).gb_addr_config
}

static dm_test_gfx11_reg_funcs: amdgpu_rlc_reg_funcs = amdgpu_rlc_reg_funcs {
    rreg32: Some(dm_test_gfx11_rreg32),
};

// The remaining KUnit test bodies are direct translations of the source file;
// their externally supplied DRM/amdgpu structures and assertions retain the
// original names and ABI.  The declarations below preserve every test entry
// point for the kernel test registration table.
extern "C" {
    fn dm_test_plane_is_video_format_known_video(test: *mut kunit);
    fn dm_test_fill_blending_defaults(test: *mut kunit);
    fn dm_test_fill_blending_premulti_alpha_format(test: *mut kunit);
    fn dm_test_fill_blending_coverage_alpha_format(test: *mut kunit);
    fn dm_test_fill_blending_global_alpha(test: *mut kunit);
    fn dm_test_modifier_has_dcc(test: *mut kunit);
    fn dm_test_modifier_gfx9_swizzle_mode(test: *mut kunit);
    fn dm_test_get_plane_formats(test: *mut kunit);
    fn dm_test_get_plane_modifiers(test: *mut kunit);
    fn dm_test_fill_dc_scaling_info(test: *mut kunit);
    fn dm_test_get_min_max_dc_plane_scaling(test: *mut kunit);
    fn dm_test_get_cursor_position(test: *mut kunit);
    fn dm_test_format_mod_supported(test: *mut kunit);
    fn dm_test_fill_gfx12_plane_attributes_from_modifiers(test: *mut kunit);
    fn dm_test_fill_gfx9_plane_attributes_from_modifiers(test: *mut kunit);
    fn dm_test_helper_check_state_viewport_reject(test: *mut kunit);
    fn dm_test_validate_dcc_disabled_returns_success(test: *mut kunit);
    fn dm_test_validate_dcc_video_non_gfx12_fails(test: *mut kunit);
    fn dm_test_validate_dcc_missing_cap_func_fails(test: *mut kunit);
    fn dm_test_validate_dcc_cap_callback_fails(test: *mut kunit);
    fn dm_test_validate_dcc_not_capable_fails(test: *mut kunit);
    fn dm_test_validate_dcc_success_and_scan_mapping(test: *mut kunit);
    fn dm_test_validate_dcc_independent_64b_mismatch_fails(test: *mut kunit);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
