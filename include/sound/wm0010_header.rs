/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * wm0010.h -- Platform data for WM0010 DSP Driver
 *
 * Copyright 2012 Wolfson Microelectronics PLC.
 *
 * Author: Dimitris Papastamos <dp@opensource.wolfsonmicro.com>
 */

#[repr(C)]
pub struct wm0010_pdata {
    pub irq_flags: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
