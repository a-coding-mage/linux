/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 */

// Dependencies corresponding to <linux/types.h> and <linux/power_supply.h>
// are supplied externally.

use core::ffi::c_char;

#[repr(C)]
pub struct bq24735_platform {
    pub charge_current: u32,
    pub charge_voltage: u32,
    pub input_current: u32,

    pub name: *const c_char,

    pub ext_control: bool,

    pub supplied_to: *mut *mut c_char,
    pub num_supplicants: usize,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
