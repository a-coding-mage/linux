/* SPDX-License-Identifier: GPL-2.0 */

/* linux/platform_data/tsc2007.h */

use core::ffi::c_ulong;

#[repr(C)]
pub struct tsc2007_platform_data {
    pub model: u16,             /* 2007. */
    pub x_plate_ohms: u16,      /* must be non-zero value */
    pub max_rt: u16,            /* max. resistance above which samples are ignored */
    pub poll_period: c_ulong,   /* time (in ms) between samples */
    pub fuzzx: i32,             /* fuzz factor for X, Y and pressure axes */
    pub fuzzy: i32,
    pub fuzzz: i32,

    pub get_pendown_state: Option<unsafe extern "C" fn(*mut device) -> i32>,
    /* If needed, clear 2nd level interrupt source */
    pub clear_penirq: Option<unsafe extern "C" fn()>,
    pub init_platform_hw: Option<unsafe extern "C" fn() -> i32>,
    pub exit_platform_hw: Option<unsafe extern "C" fn()>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
