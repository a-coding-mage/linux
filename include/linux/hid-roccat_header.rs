/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 * Copyright (c) 2010 Stefan Achatz <erazor_de@users.sourceforge.net>
 */

/* C dependencies: linux/hid.h and linux/types.h. */

use core::ffi::{c_int, c_ulong};

/* _IOR('H', 0xf1, int), using the Linux generic ioctl encoding. */
pub const ROCCATIOCGREPSIZE: c_ulong =
    ((2 as c_ulong) << 30) | ((core::mem::size_of::<c_int>() as c_ulong) << 16)
        | (('H' as c_ulong) << 8) | 0xf1;

/* Opaque declarations supplied by linux/hid.h. */
#[repr(C)]
pub struct class {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hid_device {
    _private: [u8; 0],
}

/* These declarations are present only in the kernel build. */
#[cfg(kernel)]
extern "C" {
    pub fn roccat_connect(klass: *const class, hid: *mut hid_device, report_size: c_int) -> c_int;
    pub fn roccat_disconnect(minor: c_int);
    pub fn roccat_report_event(minor: c_int, data: *const u8) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
