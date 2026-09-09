/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * stmp3xxx_rtc_wdt.h
 *
 * Copyright (C) 2011 Wolfram Sang, Pengutronix e.K.
 */

#[repr(C)]
pub struct stmp3xxx_wdt_pdata {
	pub wdt_set_timeout:
		Option<unsafe extern "C" fn(dev: *mut device, timeout: u32)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
