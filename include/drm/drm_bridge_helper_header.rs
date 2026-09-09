// SPDX-License-Identifier: GPL-2.0-or-later

#[repr(C)]
pub struct drm_bridge {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_modeset_acquire_ctx {
    _private: [u8; 0],
}

pub unsafe extern "C" fn drm_bridge_helper_reset_crtc(
    bridge: *mut drm_bridge,
    ctx: *mut drm_modeset_acquire_ctx,
) -> ::core::ffi::c_int;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
