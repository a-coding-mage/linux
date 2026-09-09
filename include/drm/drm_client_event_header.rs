/* SPDX-License-Identifier: GPL-2.0 or MIT */

// Translated from drm_client_event.h.
// The C CONFIG_DRM_CLIENT build condition is represented by the Rust feature
// of the same name.

#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_DRM_CLIENT")]
extern "C" {
    pub fn drm_client_dev_unregister(dev: *mut drm_device);
    pub fn drm_client_dev_hotplug(dev: *mut drm_device);
    pub fn drm_client_dev_restore(dev: *mut drm_device, force: bool);
    pub fn drm_client_dev_suspend(dev: *mut drm_device);
    pub fn drm_client_dev_resume(dev: *mut drm_device);
}

#[cfg(not(feature = "CONFIG_DRM_CLIENT"))]
#[inline]
pub unsafe fn drm_client_dev_unregister(_dev: *mut drm_device) {}

#[cfg(not(feature = "CONFIG_DRM_CLIENT"))]
#[inline]
pub unsafe fn drm_client_dev_hotplug(_dev: *mut drm_device) {}

#[cfg(not(feature = "CONFIG_DRM_CLIENT"))]
#[inline]
pub unsafe fn drm_client_dev_restore(_dev: *mut drm_device, _force: bool) {}

#[cfg(not(feature = "CONFIG_DRM_CLIENT"))]
#[inline]
pub unsafe fn drm_client_dev_suspend(_dev: *mut drm_device) {}

#[cfg(not(feature = "CONFIG_DRM_CLIENT"))]
#[inline]
pub unsafe fn drm_client_dev_resume(_dev: *mut drm_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
