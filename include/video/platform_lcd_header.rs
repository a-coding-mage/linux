/* SPDX-License-Identifier: GPL-2.0-only */
/* include/video/platform_lcd.h
 *
 * Copyright 2008 Simtec Electronics
 *	Ben Dooks <ben@simtec.co.uk>
 *
 * Generic platform-device LCD power control interface.
*/

#[repr(C)]
pub struct plat_lcd_data {
    pub probe: Option<unsafe extern "C" fn(data: *mut plat_lcd_data) -> i32>,
    pub set_power: Option<unsafe extern "C" fn(data: *mut plat_lcd_data, power: u32)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
