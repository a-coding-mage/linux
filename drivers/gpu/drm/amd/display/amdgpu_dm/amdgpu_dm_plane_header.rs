// SPDX-License-Identifier: MIT
/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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
 *
 * Authors: AMD
 */

unsafe extern "C" {
    pub fn amdgpu_dm_plane_get_cursor_position(
        plane: *mut drm_plane,
        crtc: *mut drm_crtc,
        position: *mut dc_cursor_position,
    ) -> i32;

    pub fn amdgpu_dm_plane_handle_cursor_update(
        plane: *mut drm_plane,
        old_plane_state: *mut drm_plane_state,
    );

    pub fn amdgpu_dm_plane_fill_dc_scaling_info(
        adev: *mut amdgpu_device,
        state: *const drm_plane_state,
        scaling_info: *mut dc_scaling_info,
    ) -> i32;

    pub fn amdgpu_dm_plane_helper_check_state(
        state: *mut drm_plane_state,
        new_crtc_state: *mut drm_crtc_state,
    ) -> i32;

    pub fn amdgpu_dm_plane_fill_plane_buffer_attributes(
        adev: *mut amdgpu_device,
        afb: *const amdgpu_framebuffer,
        format: surface_pixel_format,
        rotation: dc_rotation_angle,
        tiling_info: *mut dc_tiling_info,
        plane_size: *mut plane_size,
        dcc: *mut dc_plane_dcc_param,
        address: *mut dc_plane_address,
        tmz_surface: bool,
    ) -> i32;

    pub fn amdgpu_dm_plane_init(
        dm: *mut amdgpu_display_manager,
        plane: *mut drm_plane,
        possible_crtcs: ::core::ffi::c_ulong,
        plane_cap: *const dc_plane_cap,
    ) -> i32;

    pub fn amdgpu_dm_plane_get_format_info(
        pixel_format: u32,
        modifier: u64,
    ) -> *const drm_format_info;

    pub fn amdgpu_dm_plane_fill_blending_from_plane_state(
        plane_state: *const drm_plane_state,
        per_pixel_alpha: *mut bool,
        pre_multiplied_alpha: *mut bool,
        global_alpha: *mut bool,
        global_alpha_value: *mut i32,
    );

    pub fn amdgpu_dm_plane_is_video_format(format: u32) -> bool;
}

// Corresponds to: #if IS_ENABLED(CONFIG_DRM_AMD_DC_KUNIT_TEST)
#[cfg(any(CONFIG_DRM_AMD_DC_KUNIT_TEST, feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST"))]
unsafe extern "C" {
    pub fn amdgpu_dm_plane_add_modifier(mods: *mut *mut u64, size: *mut u64, cap: *mut u64, mod_: u64);
    pub fn amdgpu_dm_plane_fill_gfx9_tiling_info_from_device(adev: *const amdgpu_device, tiling_info: *mut dc_tiling_info);
    pub fn amdgpu_dm_plane_fill_gfx9_tiling_info_from_modifier(adev: *const amdgpu_device, tiling_info: *mut dc_tiling_info, modifier: u64);
    pub fn amdgpu_dm_plane_validate_dcc(adev: *mut amdgpu_device, format: surface_pixel_format, rotation: dc_rotation_angle, tiling_info: *const dc_tiling_info, dcc: *const dc_plane_dcc_param, address: *const dc_plane_address, plane_size: *const plane_size) -> i32;
    pub fn amdgpu_dm_plane_modifier_has_dcc(modifier: u64) -> bool;
    pub fn amdgpu_dm_plane_modifier_gfx9_swizzle_mode(modifier: u64) -> u32;
    pub fn amdgpu_dm_plane_get_plane_modifiers(adev: *mut amdgpu_device, plane_type: u32, mods: *mut *mut u64) -> i32;
    pub fn amdgpu_dm_plane_get_plane_formats(plane: *const drm_plane, plane_cap: *const dc_plane_cap, formats: *mut u32, max_formats: i32) -> i32;
    pub fn amdgpu_dm_plane_fill_gfx9_attrs_from_modifiers(adev: *mut amdgpu_device, afb: *const amdgpu_framebuffer, format: surface_pixel_format, rotation: dc_rotation_angle, plane_size: *const plane_size, tiling_info: *mut dc_tiling_info, dcc: *mut dc_plane_dcc_param, address: *mut dc_plane_address) -> i32;
    pub fn amdgpu_dm_plane_fill_gfx12_attrs_from_modifiers(adev: *mut amdgpu_device, afb: *const amdgpu_framebuffer, format: surface_pixel_format, rotation: dc_rotation_angle, plane_size: *const plane_size, tiling_info: *mut dc_tiling_info, dcc: *mut dc_plane_dcc_param, address: *mut dc_plane_address) -> i32;
    pub fn amdgpu_dm_plane_format_mod_supported(plane: *mut drm_plane, format: u32, modifier: u64) -> bool;
    pub fn amdgpu_dm_plane_get_min_max_dc_plane_scaling(dev: *mut drm_device, fb: *mut drm_framebuffer, min_downscale: *mut i32, max_upscale: *mut i32);
    pub fn amdgpu_dm_plane_atomic_async_check(plane: *mut drm_plane, state: *mut drm_atomic_commit, flip: bool) -> i32;
    pub fn amdgpu_dm_plane_atomic_check(plane: *mut drm_plane, state: *mut drm_atomic_commit) -> i32;
    pub fn amdgpu_dm_plane_panic_flush(plane: *mut drm_plane);
    pub fn amdgpu_dm_plane_drm_plane_reset(plane: *mut drm_plane);
    pub fn amdgpu_dm_plane_drm_plane_duplicate_state(plane: *mut drm_plane) -> *mut drm_plane_state;
    pub fn amdgpu_dm_plane_drm_plane_destroy_state(plane: *mut drm_plane, state: *mut drm_plane_state);
    pub fn amdgpu_dm_plane_add_modifier_dedup(mods: *mut *mut u64, size: *mut u64, cap: *mut u64, mod_: u64);
    pub fn amdgpu_dm_plane_fill_gfx6_tiling_info_from_modifier(tiling_info: *mut dc_tiling_info, modifier: u64) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
