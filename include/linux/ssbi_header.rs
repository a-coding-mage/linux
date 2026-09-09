/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2010 Google, Inc.
 * Copyright (c) 2011, Code Aurora Forum. All rights reserved.
 * Author: Dima Zavin <dima@android.com>
 */

// Dependency equivalent of <linux/types.h> is supplied externally.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn ssbi_write(dev: *mut device, addr: u16, buf: *const u8, len: i32) -> i32;
    pub fn ssbi_read(dev: *mut device, addr: u16, buf: *mut u8, len: i32) -> i32;
}

#[inline]
pub unsafe fn ssbi_reg_read(
    context: *mut core::ffi::c_void,
    reg: u32,
    val: *mut u32,
) -> i32 {
    let mut v: u8;

    let ret = unsafe { ssbi_read(context as *mut device, reg as u16, &mut v, 1) };
    if ret == 0 {
        unsafe {
            *val = v as u32;
        }
    }

    ret
}

#[inline]
pub unsafe fn ssbi_reg_write(
    context: *mut core::ffi::c_void,
    reg: u32,
    val: u32,
) -> i32 {
    let v: u8 = val as u8;
    unsafe { ssbi_write(context as *mut device, reg as u16, &v, 1) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
