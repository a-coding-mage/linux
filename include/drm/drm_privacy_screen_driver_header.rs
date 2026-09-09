/* SPDX-License-Identifier: MIT */
/*
 * Copyright (C) 2020 Red Hat, Inc.
 *
 * Authors:
 * Hans de Goede <hdegoede@redhat.com>
 */

// Translated from drm_privacy_screen_driver.h.
// Linux and DRM types referenced by this header are supplied by other files.

#[allow(non_camel_case_types)]
pub type drm_privacy_screen_status = i32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct blocking_notifier_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_privacy_screen_ops {
    /// Called to request a change of the privacy-screen state.
    /// The privacy-screen class code contains a check to avoid this getting
    /// called when the hw_state reports the state is locked. It is the
    /// driver's responsibility to update sw_state and hw_state. This is
    /// always called with the drm_privacy_screen's lock held.
    pub set_sw_state: Option<
        unsafe extern "C" fn(
            priv_: *mut drm_privacy_screen,
            sw_state: drm_privacy_screen_status,
        ) -> i32,
    >,

    /// Called to request that the driver gets the current privacy-screen
    /// state from the hardware and then updates sw_state and hw_state
    /// accordingly. This will be called by the core just before the
    /// privacy-screen is registered in sysfs.
    pub get_hw_state:
        Option<unsafe extern "C" fn(priv_: *mut drm_privacy_screen)>,
}

#[repr(C)]
pub struct drm_privacy_screen {
    /// Device used to register the privacy-screen in sysfs.
    pub dev: device,
    /// Mutex protecting all fields in this struct.
    pub lock: mutex,
    /// Privacy-screen devices list list-entry.
    pub list: list_head,
    /// Privacy-screen notifier head.
    pub notifier_head: blocking_notifier_head,
    /// This is null if the driver has unregistered the privacy-screen.
    pub ops: *const drm_privacy_screen_ops,
    /// The privacy-screen's software state.
    pub sw_state: drm_privacy_screen_status,
    /// The privacy-screen's hardware state.
    pub hw_state: drm_privacy_screen_status,
    /// Private data owned by the privacy screen provider.
    pub drvdata: *mut core::ffi::c_void,
}

#[inline]
pub unsafe fn drm_privacy_screen_get_drvdata(
    priv_: *mut drm_privacy_screen,
) -> *mut core::ffi::c_void {
    (*priv_).drvdata
}

extern "C" {
    pub fn drm_privacy_screen_register(
        parent: *mut device,
        ops: *const drm_privacy_screen_ops,
        data: *mut core::ffi::c_void,
    ) -> *mut drm_privacy_screen;

    pub fn drm_privacy_screen_unregister(priv_: *mut drm_privacy_screen);

    pub fn drm_privacy_screen_call_notifier_chain(priv_: *mut drm_privacy_screen);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
