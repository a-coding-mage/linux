/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * include/linux/mfd/wm831x/gpio.h -- GPIO for WM831x
 *
 * Copyright 2009 Wolfson Microelectronics PLC.
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

/*
 * R16440-16455 (0x4038-0x4047) - GPIOx Control
 */
pub const WM831X_GPN_DIR: u32 = 0x8000; /* GPN_DIR */
pub const WM831X_GPN_DIR_MASK: u32 = 0x8000; /* GPN_DIR */
pub const WM831X_GPN_DIR_SHIFT: u32 = 15; /* GPN_DIR */
pub const WM831X_GPN_DIR_WIDTH: u32 = 1; /* GPN_DIR */
pub const WM831X_GPN_PULL_MASK: u32 = 0x6000; /* GPN_PULL - [14:13] */
pub const WM831X_GPN_PULL_SHIFT: u32 = 13; /* GPN_PULL - [14:13] */
pub const WM831X_GPN_PULL_WIDTH: u32 = 2; /* GPN_PULL - [14:13] */
pub const WM831X_GPN_INT_MODE: u32 = 0x1000; /* GPN_INT_MODE */
pub const WM831X_GPN_INT_MODE_MASK: u32 = 0x1000; /* GPN_INT_MODE */
pub const WM831X_GPN_INT_MODE_SHIFT: u32 = 12; /* GPN_INT_MODE */
pub const WM831X_GPN_INT_MODE_WIDTH: u32 = 1; /* GPN_INT_MODE */
pub const WM831X_GPN_PWR_DOM: u32 = 0x0800; /* GPN_PWR_DOM */
pub const WM831X_GPN_PWR_DOM_MASK: u32 = 0x0800; /* GPN_PWR_DOM */
pub const WM831X_GPN_PWR_DOM_SHIFT: u32 = 11; /* GPN_PWR_DOM */
pub const WM831X_GPN_PWR_DOM_WIDTH: u32 = 1; /* GPN_PWR_DOM */
pub const WM831X_GPN_POL: u32 = 0x0400; /* GPN_POL */
pub const WM831X_GPN_POL_MASK: u32 = 0x0400; /* GPN_POL */
pub const WM831X_GPN_POL_SHIFT: u32 = 10; /* GPN_POL */
pub const WM831X_GPN_POL_WIDTH: u32 = 1; /* GPN_POL */
pub const WM831X_GPN_OD: u32 = 0x0200; /* GPN_OD */
pub const WM831X_GPN_OD_MASK: u32 = 0x0200; /* GPN_OD */
pub const WM831X_GPN_OD_SHIFT: u32 = 9; /* GPN_OD */
pub const WM831X_GPN_OD_WIDTH: u32 = 1; /* GPN_OD */
pub const WM831X_GPN_ENA: u32 = 0x0080; /* GPN_ENA */
pub const WM831X_GPN_ENA_MASK: u32 = 0x0080; /* GPN_ENA */
pub const WM831X_GPN_ENA_SHIFT: u32 = 7; /* GPN_ENA */
pub const WM831X_GPN_ENA_WIDTH: u32 = 1; /* GPN_ENA */
pub const WM831X_GPN_TRI: u32 = 0x0080; /* GPN_TRI */
pub const WM831X_GPN_TRI_MASK: u32 = 0x0080; /* GPN_TRI */
pub const WM831X_GPN_TRI_SHIFT: u32 = 7; /* GPN_TRI */
pub const WM831X_GPN_TRI_WIDTH: u32 = 1; /* GPN_TRI */
pub const WM831X_GPN_FN_MASK: u32 = 0x000F; /* GPN_FN - [3:0] */
pub const WM831X_GPN_FN_SHIFT: u32 = 0; /* GPN_FN - [3:0] */
pub const WM831X_GPN_FN_WIDTH: u32 = 4; /* GPN_FN - [3:0] */

pub const WM831X_GPIO_PULL_NONE: u32 = 0 << WM831X_GPN_PULL_SHIFT;
pub const WM831X_GPIO_PULL_DOWN: u32 = 1 << WM831X_GPN_PULL_SHIFT;
pub const WM831X_GPIO_PULL_UP: u32 = 2 << WM831X_GPN_PULL_SHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
