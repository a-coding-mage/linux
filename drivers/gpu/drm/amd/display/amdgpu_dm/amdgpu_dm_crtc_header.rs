/* SPDX-License-Identifier: MIT */
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
 *
 */

extern "C" {
    pub fn amdgpu_dm_crtc_set_static_screen_optimze(
        dm: *mut amdgpu_display_manager,
        stream: *mut dc_stream_state,
        sso_enable: bool,
        allow_sr_entry: bool,
    );

    pub fn amdgpu_dm_crtc_handle_vblank(acrtc: *mut amdgpu_crtc);

    pub fn amdgpu_dm_crtc_modeset_required(
        crtc_state: *mut drm_crtc_state,
        new_stream: *mut dc_stream_state,
        old_stream: *mut dc_stream_state,
    ) -> bool;

    pub fn amdgpu_dm_crtc_set_vupdate_irq(crtc: *mut drm_crtc, enable: bool) -> ::core::ffi::c_int;

    pub fn amdgpu_dm_crtc_vrr_active_irq(acrtc: *mut amdgpu_crtc) -> bool;

    // Preserved from #if IS_ENABLED(CONFIG_DRM_AMD_DC_KUNIT_TEST).
    #[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
    pub fn amdgpu_dm_crtc_helper_mode_fixup(
        crtc: *mut drm_crtc,
        mode: *const drm_display_mode,
        adjusted_mode: *mut drm_display_mode,
    ) -> bool;

    #[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
    pub fn amdgpu_dm_crtc_destroy_state(crtc: *mut drm_crtc, state: *mut drm_crtc_state);

    #[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
    pub fn amdgpu_dm_crtc_duplicate_state(crtc: *mut drm_crtc) -> *mut drm_crtc_state;

    #[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
    pub fn amdgpu_dm_crtc_reset_state(crtc: *mut drm_crtc);

    #[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
    pub fn amdgpu_dm_crtc_count_crtc_active_planes(
        new_crtc_state: *mut drm_crtc_state,
    ) -> ::core::ffi::c_int;

    #[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
    pub fn amdgpu_dm_crtc_update_crtc_active_planes(
        crtc: *mut drm_crtc,
        new_crtc_state: *mut drm_crtc_state,
    );

    #[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
    pub fn amdgpu_dm_crtc_vblank_control_worker(work: *mut work_struct);

    #[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
    pub fn amdgpu_dm_idle_worker(work: *mut work_struct);

    pub fn amdgpu_dm_crtc_vrr_active(dm_state: *const dm_crtc_state) -> bool;

    pub fn amdgpu_dm_crtc_enable_vblank(crtc: *mut drm_crtc) -> ::core::ffi::c_int;

    pub fn amdgpu_dm_crtc_disable_vblank(crtc: *mut drm_crtc);

    pub fn amdgpu_dm_crtc_init(
        dm: *mut amdgpu_display_manager,
        plane: *mut drm_plane,
        link_index: u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
