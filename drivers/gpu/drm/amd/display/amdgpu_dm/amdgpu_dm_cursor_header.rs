/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2026 Advanced Micro Devices, Inc.
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
 *
 */

// Declarations supplied by the surrounding kernel/Rust translation.
unsafe extern "C" {
    pub fn amdgpu_dm_check_native_cursor_state(
        new_plane_crtc: *mut drm_crtc,
        plane: *mut drm_plane,
        new_plane_state: *mut drm_plane_state,
        enable: bool,
    ) -> i32;

    pub fn amdgpu_dm_should_update_native_cursor(
        state: *mut drm_atomic_commit,
        old_plane_crtc: *mut drm_crtc,
        new_plane_crtc: *mut drm_crtc,
        enable: bool,
    ) -> bool;

    pub fn amdgpu_dm_crtc_get_cursor_mode(
        adev: *mut amdgpu_device,
        state: *mut drm_atomic_commit,
        dm_crtc_state: *mut dm_crtc_state,
        cursor_mode: *mut amdgpu_dm_cursor_mode,
    ) -> i32;
}

// C condition: IS_ENABLED(CONFIG_DRM_AMD_DC_KUNIT_TEST)
// The declarations are retained here for builds enabling that configuration.
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
unsafe extern "C" {
    pub fn dm_get_oriented_plane_size(
        plane_state: *mut drm_plane_state,
        src_w: *mut i32,
        src_h: *mut i32,
    );

    pub fn dm_get_plane_scale(
        plane_state: *mut drm_plane_state,
        out_plane_scale_w: *mut i32,
        out_plane_scale_h: *mut i32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
