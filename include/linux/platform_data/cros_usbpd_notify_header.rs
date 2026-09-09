// SPDX-License-Identifier: GPL-2.0-only
/*
 * ChromeOS EC Power Delivery Notifier Driver
 *
 * Copyright 2020 Google LLC
 */

// Dependency supplied by linux/notifier.h in the original source.
#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn cros_usbpd_register_notify(nb: *mut notifier_block) -> ::core::ffi::c_int;

    pub fn cros_usbpd_unregister_notify(nb: *mut notifier_block);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
