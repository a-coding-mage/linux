/* SPDX-License-Identifier: MIT */
/*
 * Function prototypes for misc. drm utility functions.
 * Specifically this file is for function prototypes for functions which
 * may also be used outside of drm code (e.g. in fbdev drivers).
 *
 * Copyright (C) 2017 Hans de Goede <hdegoede@redhat.com>
 */

/* Dependency supplied by the surrounding translation unit: Linux integer types. */

#[repr(C)]
pub struct drm_edid {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_panel_backlight_quirk {
    pub min_brightness: u16,
    pub brightness_mask: u32,
    pub force_pwm: bool,
}

extern "C" {
    pub fn drm_get_panel_orientation_quirk(width: core::ffi::c_int, height: core::ffi::c_int) -> core::ffi::c_int;

    pub fn drm_get_panel_backlight_quirk(
        edid: *const drm_edid,
    ) -> *const drm_panel_backlight_quirk;

    pub fn drm_timeout_abs_to_jiffies(timeout_nsec: i64) -> core::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
