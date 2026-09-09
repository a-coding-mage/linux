/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Counter sysfs interface
 * Copyright (C) 2020 William Breathitt Gray
 */

// Dependency corresponding to <linux/counter.h>.

use core::ffi::c_int;

#[repr(C)]
pub struct counter_device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn counter_sysfs_add(counter: *mut counter_device) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
