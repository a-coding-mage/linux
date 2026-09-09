/* SPDX-License-Identifier: MIT */
/*
 * Copyright (C) 2020 Red Hat, Inc.
 *
 * Authors:
 * Hans de Goede <hdegoede@redhat.com>
 */

// Dependencies supplied by the surrounding kernel/drm bindings:
// linux/device.h, drm/drm_connector.h

pub struct drm_privacy_screen;

// IS_ENABLED(CONFIG_DRM_PRIVACY_SCREEN) is represented here by the
// `drm_privacy_screen` feature configuration.
#[cfg(feature = "drm_privacy_screen")]
extern "C" {
    pub fn drm_privacy_screen_get(
        dev: *mut device,
        con_id: *const ::std::os::raw::c_char,
    ) -> *mut drm_privacy_screen;
    pub fn drm_privacy_screen_put(priv_: *mut drm_privacy_screen);

    pub fn drm_privacy_screen_set_sw_state(
        priv_: *mut drm_privacy_screen,
        sw_state: drm_privacy_screen_status,
    ) -> ::std::os::raw::c_int;
    pub fn drm_privacy_screen_get_state(
        priv_: *mut drm_privacy_screen,
        sw_state_ret: *mut drm_privacy_screen_status,
        hw_state_ret: *mut drm_privacy_screen_status,
    );

    pub fn drm_privacy_screen_register_notifier(
        priv_: *mut drm_privacy_screen,
        nb: *mut notifier_block,
    ) -> ::std::os::raw::c_int;
    pub fn drm_privacy_screen_unregister_notifier(
        priv_: *mut drm_privacy_screen,
        nb: *mut notifier_block,
    ) -> ::std::os::raw::c_int;
}

#[cfg(not(feature = "drm_privacy_screen"))]
#[inline]
pub unsafe fn drm_privacy_screen_get(
    _dev: *mut device,
    _con_id: *const ::std::os::raw::c_char,
) -> *mut drm_privacy_screen {
    ERR_PTR(-ENODEV)
}

#[cfg(not(feature = "drm_privacy_screen"))]
#[inline]
pub unsafe fn drm_privacy_screen_put(_priv: *mut drm_privacy_screen) {}

#[cfg(not(feature = "drm_privacy_screen"))]
#[inline]
pub unsafe fn drm_privacy_screen_set_sw_state(
    _priv: *mut drm_privacy_screen,
    _sw_state: drm_privacy_screen_status,
) -> ::std::os::raw::c_int {
    -ENODEV
}

#[cfg(not(feature = "drm_privacy_screen"))]
#[inline]
pub unsafe fn drm_privacy_screen_get_state(
    _priv: *mut drm_privacy_screen,
    sw_state_ret: *mut drm_privacy_screen_status,
    hw_state_ret: *mut drm_privacy_screen_status,
) {
    *sw_state_ret = PRIVACY_SCREEN_DISABLED;
    *hw_state_ret = PRIVACY_SCREEN_DISABLED;
}

#[cfg(not(feature = "drm_privacy_screen"))]
#[inline]
pub unsafe fn drm_privacy_screen_register_notifier(
    _priv: *mut drm_privacy_screen,
    _nb: *mut notifier_block,
) -> ::std::os::raw::c_int {
    -ENODEV
}

#[cfg(not(feature = "drm_privacy_screen"))]
#[inline]
pub unsafe fn drm_privacy_screen_unregister_notifier(
    _priv: *mut drm_privacy_screen,
    _nb: *mut notifier_block,
) -> ::std::os::raw::c_int {
    -ENODEV
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
