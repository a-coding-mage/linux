/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Dumb driver for LiIon batteries using TWL4030 madc.
 *
 * Copyright 2013 Golden Delicious Computers
 * Nikolaus Schaller <hns@goldelico.com>
 */

/*
 * Usually we can assume 100% @ 4.15V and 0% @ 3.3V but curves differ for
 * charging and discharging!
 */

#[repr(C)]
pub struct twl4030_madc_bat_calibration {
    pub voltage: i16, /* in mV - specify -1 for end of list */
    pub level: i16,   /* in percent (0 .. 100%) */
}

#[repr(C)]
pub struct twl4030_madc_bat_platform_data {
    pub capacity: u32, /* total capacity in uAh */
    pub charging: *mut twl4030_madc_bat_calibration,
    pub charging_size: i32,
    pub discharging: *mut twl4030_madc_bat_calibration,
    pub discharging_size: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
