/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * include/linux/mfd/wm831x/watchdog.h -- Watchdog for WM831x
 *
 * Copyright 2009 Wolfson Microelectronics PLC.
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

/*
 * R16388 (0x4004) - Watchdog
 */
pub const WM831X_WDOG_ENA: u32 = 0x8000; // WDOG_ENA
pub const WM831X_WDOG_ENA_MASK: u32 = 0x8000; // WDOG_ENA
pub const WM831X_WDOG_ENA_SHIFT: u32 = 15; // WDOG_ENA
pub const WM831X_WDOG_ENA_WIDTH: u32 = 1; // WDOG_ENA
pub const WM831X_WDOG_DEBUG: u32 = 0x4000; // WDOG_DEBUG
pub const WM831X_WDOG_DEBUG_MASK: u32 = 0x4000; // WDOG_DEBUG
pub const WM831X_WDOG_DEBUG_SHIFT: u32 = 14; // WDOG_DEBUG
pub const WM831X_WDOG_DEBUG_WIDTH: u32 = 1; // WDOG_DEBUG
pub const WM831X_WDOG_RST_SRC: u32 = 0x2000; // WDOG_RST_SRC
pub const WM831X_WDOG_RST_SRC_MASK: u32 = 0x2000; // WDOG_RST_SRC
pub const WM831X_WDOG_RST_SRC_SHIFT: u32 = 13; // WDOG_RST_SRC
pub const WM831X_WDOG_RST_SRC_WIDTH: u32 = 1; // WDOG_RST_SRC
pub const WM831X_WDOG_SLPENA: u32 = 0x1000; // WDOG_SLPENA
pub const WM831X_WDOG_SLPENA_MASK: u32 = 0x1000; // WDOG_SLPENA
pub const WM831X_WDOG_SLPENA_SHIFT: u32 = 12; // WDOG_SLPENA
pub const WM831X_WDOG_SLPENA_WIDTH: u32 = 1; // WDOG_SLPENA
pub const WM831X_WDOG_RESET: u32 = 0x0800; // WDOG_RESET
pub const WM831X_WDOG_RESET_MASK: u32 = 0x0800; // WDOG_RESET
pub const WM831X_WDOG_RESET_SHIFT: u32 = 11; // WDOG_RESET
pub const WM831X_WDOG_RESET_WIDTH: u32 = 1; // WDOG_RESET
pub const WM831X_WDOG_SECACT_MASK: u32 = 0x0300; // WDOG_SECACT - [9:8]
pub const WM831X_WDOG_SECACT_SHIFT: u32 = 8; // WDOG_SECACT - [9:8]
pub const WM831X_WDOG_SECACT_WIDTH: u32 = 2; // WDOG_SECACT - [9:8]
pub const WM831X_WDOG_PRIMACT_MASK: u32 = 0x0030; // WDOG_PRIMACT - [5:4]
pub const WM831X_WDOG_PRIMACT_SHIFT: u32 = 4; // WDOG_PRIMACT - [5:4]
pub const WM831X_WDOG_PRIMACT_WIDTH: u32 = 2; // WDOG_PRIMACT - [5:4]
pub const WM831X_WDOG_TO_MASK: u32 = 0x0007; // WDOG_TO - [2:0]
pub const WM831X_WDOG_TO_SHIFT: u32 = 0; // WDOG_TO - [2:0]
pub const WM831X_WDOG_TO_WIDTH: u32 = 3; // WDOG_TO - [2:0]

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
