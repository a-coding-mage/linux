/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * include/linux/mfd/wm831x/status.h -- Status LEDs for WM831x
 *
 * Copyright 2009 Wolfson Microelectronics PLC.
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

// Translated from the C header; the original include guard is omitted.

pub const WM831X_LED_SRC_MASK: u16 = 0xC000; // LED_SRC - [15:14]
pub const WM831X_LED_SRC_SHIFT: u32 = 14; // LED_SRC - [15:14]
pub const WM831X_LED_SRC_WIDTH: u32 = 2; // LED_SRC - [15:14]
pub const WM831X_LED_MODE_MASK: u16 = 0x0300; // LED_MODE - [9:8]
pub const WM831X_LED_MODE_SHIFT: u32 = 8; // LED_MODE - [9:8]
pub const WM831X_LED_MODE_WIDTH: u32 = 2; // LED_MODE - [9:8]
pub const WM831X_LED_SEQ_LEN_MASK: u16 = 0x0030; // LED_SEQ_LEN - [5:4]
pub const WM831X_LED_SEQ_LEN_SHIFT: u32 = 4; // LED_SEQ_LEN - [5:4]
pub const WM831X_LED_SEQ_LEN_WIDTH: u32 = 2; // LED_SEQ_LEN - [5:4]
pub const WM831X_LED_DUR_MASK: u16 = 0x000C; // LED_DUR - [3:2]
pub const WM831X_LED_DUR_SHIFT: u32 = 2; // LED_DUR - [3:2]
pub const WM831X_LED_DUR_WIDTH: u32 = 2; // LED_DUR - [3:2]
pub const WM831X_LED_DUTY_CYC_MASK: u16 = 0x0003; // LED_DUTY_CYC - [1:0]
pub const WM831X_LED_DUTY_CYC_SHIFT: u32 = 0; // LED_DUTY_CYC - [1:0]
pub const WM831X_LED_DUTY_CYC_WIDTH: u32 = 2; // LED_DUTY_CYC - [1:0]

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
