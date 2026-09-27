/* SPDX-License-Identifier: GPL-2.0 */
/*
 * bitext.h: Bit string operations on the sparc, specific to architecture.
 *
 * Copyright 2002 Pete Zaitcev <zaitcev@yahoo.com>
 */

// Dependency supplied externally by the Linux spinlock definitions.

#[repr(C)]
pub struct bit_map {
    pub lock: spinlock_t,
    pub map: *mut ::kernel::ffi::c_ulong,
    pub size: ::kernel::ffi::c_int,
    pub used: ::kernel::ffi::c_int,
    pub last_off: ::kernel::ffi::c_int,
    pub last_size: ::kernel::ffi::c_int,
    pub first_free: ::kernel::ffi::c_int,
    pub num_colors: ::kernel::ffi::c_int,
}

extern "C" {
    pub fn bit_map_string_get(t: *mut bit_map, len: ::kernel::ffi::c_int, align: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn bit_map_clear(t: *mut bit_map, offset: ::kernel::ffi::c_int, len: ::kernel::ffi::c_int);
    pub fn bit_map_init(t: *mut bit_map, map: *mut ::kernel::ffi::c_ulong, size: ::kernel::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
