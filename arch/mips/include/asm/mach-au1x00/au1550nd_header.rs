/* SPDX-License-Identifier: GPL-2.0 */
/*
 * platform data for the Au1550 NAND driver
 */

// Dependency intent: `mtd_partition` is supplied by the translated
// linux/mtd/partitions.h header.

#[repr(C)]
pub struct au1550nd_platdata {
    pub parts: *mut mtd_partition,
    pub num_parts: i32,
    pub devwidth: i32, /* 0 = 8bit device, 1 = 16bit device */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
