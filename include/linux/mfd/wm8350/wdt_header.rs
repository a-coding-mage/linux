/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * wdt.h  --  Watchdog Driver for Wolfson WM8350 PMIC
 *
 * Copyright 2007, 2008 Wolfson Microelectronics PLC
 */

// Dependency supplied by the Linux platform-device headers.
pub struct platform_device;

pub const WM8350_WDOG_HIB_MODE: u16 = 0x0080;
pub const WM8350_WDOG_DEBUG: u16 = 0x0040;
pub const WM8350_WDOG_MODE_MASK: u16 = 0x0030;
pub const WM8350_WDOG_TO_MASK: u16 = 0x0007;

pub const WM8350_IRQ_SYS_WDOG_TO: i32 = 24;

#[repr(C)]
pub struct wm8350_wdt {
    pub pdev: *mut platform_device,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
