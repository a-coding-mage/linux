/* SPDX-License-Identifier: GPL-2.0 */

// Dependency provided by <linux/mtd/rawnand.h>.

#[repr(C)]
pub struct gpio_nand_platdata {
    pub adjust_parts:
        Option<unsafe extern "C" fn(*mut gpio_nand_platdata, usize)>,
    pub parts: *mut mtd_partition,
    pub num_parts: core::ffi::c_uint,
    pub options: core::ffi::c_uint,
    pub chip_delay: core::ffi::c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
