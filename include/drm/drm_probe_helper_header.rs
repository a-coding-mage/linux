// SPDX-License-Identifier: GPL-2.0 OR MIT

// Dependency intent: declarations from <drm/drm_modes.h> are supplied by
// other translated files.

use core::ffi::c_uint;

#[repr(C)]
pub struct drm_connector {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct drm_crtc {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct drm_device {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct drm_modeset_acquire_ctx {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct drm_display_mode {
    _opaque: [u8; 0],
}

// Supplied by the translated drm_modes.h dependency; this preserves its C ABI.
pub type drm_mode_status = i32;

extern "C" {
    pub fn drm_helper_probe_single_connector_modes(
        connector: *mut drm_connector,
        maxX: c_uint,
        maxY: c_uint,
    ) -> i32;

    pub fn drm_helper_probe_detect(
        connector: *mut drm_connector,
        ctx: *mut drm_modeset_acquire_ctx,
        force: bool,
    ) -> i32;

    pub fn drmm_kms_helper_poll_init(dev: *mut drm_device);
    pub fn drm_kms_helper_poll_init(dev: *mut drm_device);
    pub fn drm_kms_helper_poll_fini(dev: *mut drm_device);
    pub fn drm_helper_hpd_irq_event(dev: *mut drm_device) -> bool;
    pub fn drm_connector_helper_hpd_irq_event(connector: *mut drm_connector) -> bool;
    pub fn drm_kms_helper_hotplug_event(dev: *mut drm_device);
    pub fn drm_kms_helper_connector_hotplug_event(connector: *mut drm_connector);

    pub fn drm_kms_helper_poll_disable(dev: *mut drm_device);
    pub fn drm_kms_helper_poll_enable(dev: *mut drm_device);
    pub fn drm_kms_helper_poll_reschedule(dev: *mut drm_device);
    pub fn drm_kms_helper_is_poll_worker() -> bool;

    pub fn drm_crtc_helper_mode_valid_fixed(
        crtc: *mut drm_crtc,
        mode: *const drm_display_mode,
        fixed_mode: *const drm_display_mode,
    ) -> drm_mode_status;

    pub fn drm_connector_helper_get_modes_fixed(
        connector: *mut drm_connector,
        fixed_mode: *const drm_display_mode,
    ) -> i32;
    pub fn drm_connector_helper_get_modes(connector: *mut drm_connector) -> i32;
    pub fn drm_connector_helper_tv_get_modes(connector: *mut drm_connector) -> i32;

    pub fn drm_connector_helper_detect_from_ddc(
        connector: *mut drm_connector,
        ctx: *mut drm_modeset_acquire_ctx,
        force: bool,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
