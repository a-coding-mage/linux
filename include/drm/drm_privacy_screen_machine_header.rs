/* SPDX-License-Identifier: MIT */
/*
 * Copyright (C) 2020 Red Hat, Inc.
 *
 * Authors:
 * Hans de Goede <hdegoede@redhat.com>
 */

// Translated from drm_privacy_screen_machine.h.
// Dependency: linux/list.h supplies `list_head`.

/**
 * struct drm_privacy_screen_lookup - static privacy-screen lookup list entry
 *
 * Used for the static lookup-list for mapping privacy-screen consumer
 * dev-connector pairs to a privacy-screen provider.
 */
#[repr(C)]
pub struct drm_privacy_screen_lookup {
    /** @list: Lookup list list-entry. */
    pub list: list_head,
    /** @dev_id: Consumer device name or NULL to match all devices. */
    pub dev_id: *const ::core::ffi::c_char,
    /** @con_id: Consumer connector name or NULL to match all connectors. */
    pub con_id: *const ::core::ffi::c_char,
    /** @provider: dev_name() of the privacy_screen provider. */
    pub provider: *const ::core::ffi::c_char,
}

unsafe extern "C" {
    pub fn drm_privacy_screen_lookup_add(lookup: *mut drm_privacy_screen_lookup);
    pub fn drm_privacy_screen_lookup_remove(lookup: *mut drm_privacy_screen_lookup);
}

// Preserved build-time condition:
// #if IS_ENABLED(CONFIG_DRM_PRIVACY_SCREEN) && IS_ENABLED(CONFIG_X86)
#[cfg(all(
    feature = "CONFIG_DRM_PRIVACY_SCREEN",
    feature = "CONFIG_X86"
))]
unsafe extern "C" {
    pub fn drm_privacy_screen_lookup_init();
    pub fn drm_privacy_screen_lookup_exit();
}
// #else: the C header provides empty inline functions.
#[cfg(not(all(
    feature = "CONFIG_DRM_PRIVACY_SCREEN",
    feature = "CONFIG_X86"
)))]
#[inline]
pub fn drm_privacy_screen_lookup_init() {}

#[cfg(not(all(
    feature = "CONFIG_DRM_PRIVACY_SCREEN",
    feature = "CONFIG_X86"
)))]
#[inline]
pub fn drm_privacy_screen_lookup_exit() {}
// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
