/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Samsung Keypad platform data definitions
 *
 * Copyright (C) 2010 Samsung Electronics Co.Ltd
 * Author: Joonyoung Shim <jy0922.shim@samsung.com>
 */

// Dependency supplied by the Linux input matrix keypad definitions:
// #include <linux/input/matrix_keypad.h>

pub const SAMSUNG_MAX_ROWS: u32 = 8;
pub const SAMSUNG_MAX_COLS: u32 = 8;

/**
 * struct samsung_keypad_platdata - Platform device data for Samsung Keypad.
 * @keymap_data: pointer to &matrix_keymap_data.
 * @rows: number of keypad row supported.
 * @cols: number of keypad col supported.
 * @no_autorepeat: disable key autorepeat.
 * @wakeup: controls whether the device should be set up as wakeup source.
 * @cfg_gpio: configure the GPIO.
 *
 * Initialisation data specific to either the machine or the platform
 * for the device driver to use or call-back when configuring gpio.
 */
#[repr(C)]
pub struct samsung_keypad_platdata {
    pub keymap_data: *const matrix_keymap_data,
    pub rows: u32,
    pub cols: u32,
    pub no_autorepeat: bool,
    pub wakeup: bool,
    pub cfg_gpio: Option<unsafe extern "C" fn(rows: u32, cols: u32)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
