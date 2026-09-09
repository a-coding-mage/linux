/* SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause) */
/* Copyright 2024 NXP
 */

// C dependencies: <linux/io.h> and <linux/io-64-nonatomic-lo-hi.h>

use core::ffi::c_void;

extern "C" {
    pub fn ioread32(reg: *mut c_void) -> u32;
    pub fn iowrite32(val: u32, reg: *mut c_void);
    pub fn ioread64(reg: *mut c_void) -> u64;
}

pub unsafe fn netc_read(reg: *mut c_void) -> u32 {
    ioread32(reg)
}

pub unsafe fn netc_write(reg: *mut c_void, val: u32) {
    iowrite32(val, reg);
}

pub unsafe fn netc_read64(reg: *mut c_void) -> u64 {
    ioread64(reg)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
