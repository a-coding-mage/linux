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
 */

// Dependency supplied by the surrounding translation unit: Linux u32.

#[repr(C)]
pub struct amdgpu_display_manager {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dm_crtc_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dm_connector_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_stream_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_plane_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_crtc_state {
    _private: [u8; 0],
}

extern "C" {
    pub fn amdgpu_dm_is_dc_timing_adjust_needed(
        old_state: *mut dm_crtc_state,
        new_state: *mut dm_crtc_state,
    ) -> bool;

    pub fn amdgpu_dm_is_timing_unchanged_for_freesync(
        old_crtc_state: *mut drm_crtc_state,
        new_crtc_state: *mut drm_crtc_state,
    ) -> bool;

    pub fn amdgpu_dm_set_freesync_fixed_config(dm_new_crtc_state: *mut dm_crtc_state);

    pub fn amdgpu_dm_reset_freesync_config_for_crtc(new_crtc_state: *mut dm_crtc_state);

    pub fn amdgpu_dm_get_freesync_config_for_crtc(
        new_crtc_state: *mut dm_crtc_state,
        new_con_state: *mut dm_connector_state,
    );

    pub fn amdgpu_dm_update_freesync_state_on_stream(
        dm: *mut amdgpu_display_manager,
        new_crtc_state: *mut dm_crtc_state,
        new_stream: *mut dc_stream_state,
        surface: *mut dc_plane_state,
        flip_timestamp_in_us: u32,
    );

    pub fn amdgpu_dm_update_stream_irq_parameters(
        dm: *mut amdgpu_display_manager,
        new_crtc_state: *mut dm_crtc_state,
    );

    pub fn amdgpu_dm_handle_vrr_transition(
        dm: *mut amdgpu_display_manager,
        old_state: *mut dm_crtc_state,
        new_state: *mut dm_crtc_state,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
