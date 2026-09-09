/* SPDX-License-Identifier: GPL-2.0-only */
/* linux/include/linux/mtd/plat-ram.h
 *
 * (c) 2004 Simtec Electronics
 *	http://www.simtec.co.uk/products/SWLINUX/
 *	Ben Dooks <ben@simtec.com>
 *
 * Generic platform device based RAM map
 */

// Original header guard: __LINUX_MTD_PLATRAM_H

pub const PLATRAM_RO: i32 = 0;
pub const PLATRAM_RW: i32 = 1;

// External types supplied by the surrounding kernel translation.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtd_partition {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platdata_mtd_ram {
    pub mapname: *const core::ffi::c_char,
    pub map_probes: *const *const core::ffi::c_char,
    pub probes: *const *const core::ffi::c_char,
    pub partitions: *mut mtd_partition,
    pub nr_partitions: core::ffi::c_int,
    pub bankwidth: core::ffi::c_int,

    /* control callbacks */
    pub set_rw: Option<unsafe extern "C" fn(dev: *mut device, to: core::ffi::c_int)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
