/* SPDX-License-Identifier: MIT */
/*
 * Copyright (C) 2017 Google, Inc.
 *
 * Authors:
 * Sean Paul <seanpaul@chromium.org>
 */

// Dependency supplied by the original DRM header: <drm/display/drm_hdcp.h>

#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_connector {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn drm_hdcp_check_ksvs_revoked(
        dev: *mut drm_device,
        ksvs: *mut u8,
        ksv_count: u32,
    ) -> ::core::ffi::c_int;

    pub fn drm_connector_attach_content_protection_property(
        connector: *mut drm_connector,
        hdcp_content_type: bool,
    ) -> ::core::ffi::c_int;

    pub fn drm_hdcp_update_content_protection(
        connector: *mut drm_connector,
        val: u64,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
