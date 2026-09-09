/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Samsung Platform - Keypad platform data definitions
 *
 * Copyright (C) 2010 Samsung Electronics Co.Ltd
 * Author: Joonyoung Shim <jy0922.shim@samsung.com>
 */

// Dependency corresponding to <linux/input/samsung-keypad.h>.

/// Platform data for the Samsung Keypad device.
///
/// The complete definition is supplied by the corresponding dependency.
#[repr(C)]
pub struct samsung_keypad_platdata {
    _private: [u8; 0],
}

/// Set platform data for Samsung Keypad device.
///
/// Register the given platform data for use with Samsung Keypad device. The
/// call will copy the platform data, so board definitions can make the
/// structure itself `__initdata`.
extern "C" {
    pub fn samsung_keypad_set_platdata(pd: *mut samsung_keypad_platdata);

    // Defined by architecture to configure GPIO.
    pub fn samsung_keypad_cfg_gpio(rows: ::core::ffi::c_uint, cols: ::core::ffi::c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
