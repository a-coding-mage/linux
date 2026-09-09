// SPDX-License-Identifier: GPL-2.0-only
/*
 * Compatibility for old (not device tree aware) U-Boot versions
 *
 * Author: Scott Wood <scottwood@freescale.com>
 * Consolidated using macros by David Gibson <david@gibson.dropbear.id.au>
 *
 * Copyright 2007 David Gibson, IBM Corporation.
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the corresponding platform headers.
#[repr(C)]
pub struct LoaderInfo {
    pub initrd_addr: ::core::ffi::c_ulong,
    pub initrd_size: ::core::ffi::c_ulong,
    pub cmdline: *mut ::core::ffi::c_char,
    pub cmdline_len: ::core::ffi::c_ulong,
}

unsafe extern "C" {
    static mut _end: ::core::ffi::c_uchar;
    static mut loader_info: LoaderInfo;
    fn simple_alloc_init(
        base: *mut ::core::ffi::c_void,
        size: ::core::ffi::c_ulong,
        align: ::core::ffi::c_ulong,
        boundary: ::core::ffi::c_ulong,
    );
}

pub unsafe fn cuboot_init(
    r4: ::core::ffi::c_ulong,
    r5: ::core::ffi::c_ulong,
    r6: ::core::ffi::c_ulong,
    r7: ::core::ffi::c_ulong,
    end_of_ram: ::core::ffi::c_ulong,
) {
    let avail_ram = end_of_ram - (&raw mut _end as *mut ::core::ffi::c_uchar as ::core::ffi::c_ulong);

    loader_info.initrd_addr = r4;
    loader_info.initrd_size = if r4 != 0 { r5 - r4 } else { 0 };
    loader_info.cmdline = r6 as *mut ::core::ffi::c_char;
    loader_info.cmdline_len = r7 - r6;

    simple_alloc_init(
        &raw mut _end as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        avail_ram - 1024 * 1024,
        32,
        64,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
