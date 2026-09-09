/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Platform profile sysfs interface
 *
 * See Documentation/userspace-api/sysfs-platform_profile.rst for more
 * information.
 */

use std::os::raw::{c_char, c_int, c_ulong};

/* Dependency supplied by linux/device.h. */
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/*
 * If more options are added please update profile_names array in
 * platform_profile.c and sysfs-platform_profile documentation.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum platform_profile_option {
    PLATFORM_PROFILE_LOW_POWER,
    PLATFORM_PROFILE_COOL,
    PLATFORM_PROFILE_QUIET,
    PLATFORM_PROFILE_BALANCED,
    PLATFORM_PROFILE_BALANCED_PERFORMANCE,
    PLATFORM_PROFILE_PERFORMANCE,
    PLATFORM_PROFILE_MAX_POWER,
    PLATFORM_PROFILE_CUSTOM,
    PLATFORM_PROFILE_LAST, /*must always be last */
}

/**
 * struct platform_profile_ops - platform profile operations
 * @probe: Callback to setup choices available to the new class device. These
 *     choices will only be enforced when setting a new profile, not when
 *     getting the current one.
 * @hidden_choices: Callback to setup choices that are not visible to the user
 *          but can be set by the driver.
 * @profile_get: Callback that will be called when showing the current platform
 *       profile in sysfs.
 * @profile_set: Callback that will be called when storing a new platform
 *       profile in sysfs.
 */
#[repr(C)]
pub struct platform_profile_ops {
    pub probe: Option<unsafe extern "C" fn(drvdata: *mut core::ffi::c_void, choices: *mut c_ulong) -> c_int>,
    pub hidden_choices: Option<unsafe extern "C" fn(drvdata: *mut core::ffi::c_void, choices: *mut c_ulong) -> c_int>,
    pub profile_get: Option<unsafe extern "C" fn(dev: *mut device, profile: *mut platform_profile_option) -> c_int>,
    pub profile_set: Option<unsafe extern "C" fn(dev: *mut device, profile: platform_profile_option) -> c_int>,
}

extern "C" {
    pub fn platform_profile_register(
        dev: *mut device,
        name: *const c_char,
        drvdata: *mut core::ffi::c_void,
        ops: *const platform_profile_ops,
    ) -> *mut device;
    pub fn platform_profile_remove(dev: *mut device);
    pub fn devm_platform_profile_register(
        dev: *mut device,
        name: *const c_char,
        drvdata: *mut core::ffi::c_void,
        ops: *const platform_profile_ops,
    ) -> *mut device;
    pub fn platform_profile_cycle() -> c_int;
    pub fn platform_profile_notify(dev: *mut device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
