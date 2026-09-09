// SPDX-License-Identifier: MIT
/*
 * Copyright (C) 2019 Google, Inc.
 *
 * Authors:
 * Sean Paul <seanpaul@chromium.org>
 */

#[repr(C)]
pub struct drm_atomic_commit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_crtc {
    _private: [u8; 0],
}

extern "C" {
    pub fn drm_self_refresh_helper_alter_state(state: *mut drm_atomic_commit);

    pub fn drm_self_refresh_helper_update_avg_times(
        state: *mut drm_atomic_commit,
        commit_time_ms: core::ffi::c_uint,
        new_self_refresh_mask: core::ffi::c_uint,
    );

    pub fn drm_self_refresh_helper_init(crtc: *mut drm_crtc) -> core::ffi::c_int;
    pub fn drm_self_refresh_helper_cleanup(crtc: *mut drm_crtc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
