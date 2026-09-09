/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by another translation unit/header:
#[repr(C)]
pub struct mtd_partition {
    _private: [u8; 0],
}

use core::ffi::c_char;

/**
 * struct flash_platform_data: board-specific flash data
 * @name: optional flash device name (eg, as used with mtdparts=)
 * @parts: optional array of mtd_partitions for static partitioning
 * @nr_parts: number of mtd_partitions for static partitioning
 * @type: optional flash device type (e.g. m25p80 vs m25p64), for use
 *\twith chips that can't be queried for JEDEC or other IDs
 *
 * Board init code (in arch/.../mach-xxx/board-yyy.c files) can
 * provide information about SPI flash parts (such as DataFlash) to
 * help set up the device and its appropriate default partitioning.
 *
 * Note that for DataFlash, sizes for pages, blocks, and sectors are
 * rarely powers of two; and partitions should be sector-aligned.
 */
#[repr(C)]
pub struct flash_platform_data {
    pub name: *mut c_char,
    pub parts: *mut mtd_partition,
    pub nr_parts: u32,
    pub r#type: *mut c_char,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
