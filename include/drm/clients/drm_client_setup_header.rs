/* SPDX-License-Identifier: MIT */

// C dependency: <linux/types.h>
// The build-time CONFIG_DRM_CLIENT_SETUP condition is represented by the
// `CONFIG_DRM_CLIENT_SETUP` Cargo feature.

use core::ffi::c_uint;

#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_format_info {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_DRM_CLIENT_SETUP")]
extern "C" {
    pub fn drm_client_setup(
        dev: *mut drm_device,
        format: *const drm_format_info,
    );
    pub fn drm_client_setup_with_fourcc(dev: *mut drm_device, fourcc: u32);
    pub fn drm_client_setup_with_color_mode(dev: *mut drm_device, color_mode: c_uint);
}

#[cfg(not(feature = "CONFIG_DRM_CLIENT_SETUP"))]
#[inline]
pub unsafe fn drm_client_setup(
    _dev: *mut drm_device,
    _format: *const drm_format_info,
) {
}

#[cfg(not(feature = "CONFIG_DRM_CLIENT_SETUP"))]
#[inline]
pub unsafe fn drm_client_setup_with_fourcc(_dev: *mut drm_device, _fourcc: u32) {
}

#[cfg(not(feature = "CONFIG_DRM_CLIENT_SETUP"))]
#[inline]
pub unsafe fn drm_client_setup_with_color_mode(_dev: *mut drm_device, _color_mode: c_uint) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
