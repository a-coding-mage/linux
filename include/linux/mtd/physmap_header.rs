/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * For boards with physically mapped flash and using
 * drivers/mtd/maps/physmap.c mapping driver.
 *
 * Copyright (C) 2003 MontaVista Software Inc.
 * Author: Jun Sun, jsun@mvista.com or jsun@junsun.net
 */

// C dependencies supplied by the corresponding Linux MTD headers.
use core::ffi::c_char;

#[repr(C)]
pub struct map_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

// `struct mtd_partition` is declared by linux/mtd/partitions.h.
#[repr(C)]
pub struct mtd_partition {
    _private: [u8; 0],
}

#[repr(C)]
pub struct physmap_flash_data {
    pub width: core::ffi::c_uint,
    pub init: Option<unsafe extern "C" fn(*mut platform_device) -> core::ffi::c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub set_vpp:
        Option<unsafe extern "C" fn(*mut platform_device, core::ffi::c_int)>,
    pub nr_parts: core::ffi::c_uint,
    pub pfow_base: core::ffi::c_uint,
    pub probe_type: *mut c_char,
    pub parts: *mut mtd_partition,
    pub part_probe_types: *const *const c_char,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
