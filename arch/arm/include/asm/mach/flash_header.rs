/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/mach/flash.h
 *
 *  Copyright (C) 2003 Russell King, All Rights Reserved.
 */

use core::ffi::c_char;

// Forward declarations from the MTD subsystem.
#[repr(C)]
pub struct mtd_partition {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtd_info {
    _private: [u8; 0],
}

/*
 * map_name:\tthe map probe function name
 * name:\tflash device name (eg, as used with mtdparts=)
 * width:\twidth of mapped device
 * init:\tmethod called at driver/device initialisation
 * exit:\tmethod called at driver/device removal
 * set_vpp:\tmethod called to enable or disable VPP
 * mmcontrol:\tmethod called to enable or disable Sync. Burst Read in OneNAND
 * parts:\toptional array of mtd_partitions for static partitioning
 * nr_parts:\tnumber of mtd_partitions for static partitioning
 */
#[repr(C)]
pub struct flash_platform_data {
    pub map_name: *const c_char,
    pub name: *const c_char,
    pub width: u32,
    pub init: Option<unsafe extern "C" fn() -> i32>,
    pub exit: Option<unsafe extern "C" fn()>,
    pub set_vpp: Option<unsafe extern "C" fn(on: i32)>,
    pub mmcontrol:
        Option<unsafe extern "C" fn(mtd: *mut mtd_info, sync_read: i32)>,
    pub parts: *mut mtd_partition,
    pub nr_parts: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
