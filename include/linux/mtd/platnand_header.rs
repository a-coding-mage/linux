/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Copyright © 2000-2010 David Woodhouse <dwmw2@infradead.org>
 *                         Steven J. Hill <sjhill@realitydiluted.com>
 *                         Thomas Gleixner <tglx@kernel.org>
 *
 * Contains all platform NAND related definitions.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/mtd/partitions.h, linux/mtd/rawnand.h, and linux/platform_device.h

/**
 * struct platform_nand_chip - chip level device structure
 * @nr_chips: max. number of chips to scan for
 * @chip_offset: chip number offset
 * @nr_partitions: number of partitions pointed to by partitions (or zero)
 * @partitions: mtd partition list
 * @chip_delay: R/B delay value in us
 * @options: Option flags, e.g. 16bit buswidth
 * @bbt_options: BBT option flags, e.g. NAND_BBT_USE_FLASH
 * @part_probe_types: NULL-terminated array of probe types
 */
#[repr(C)]
pub struct platform_nand_chip {
    pub nr_chips: ::core::ffi::c_int,
    pub chip_offset: ::core::ffi::c_int,
    pub nr_partitions: ::core::ffi::c_int,
    pub partitions: *mut mtd_partition,
    pub chip_delay: ::core::ffi::c_int,
    pub options: ::core::ffi::c_uint,
    pub bbt_options: ::core::ffi::c_uint,
    pub part_probe_types: *const *const ::core::ffi::c_char,
}

/**
 * struct platform_nand_ctrl - controller level device structure
 * @probe: platform specific function to probe/setup hardware
 * @remove: platform specific function to remove/teardown hardware
 * @dev_ready: platform specific function to read ready/busy pin
 * @select_chip: platform specific chip select function
 * @cmd_ctrl: platform specific function for controlling
 *           ALE/CLE/nCE. Also used to write command and address
 * @write_buf: platform specific function for write buffer
 * @read_buf: platform specific function for read buffer
 * @priv: private data to transport driver specific settings
 *
 * All fields are optional and depend on the hardware driver requirements
 */
#[repr(C)]
pub struct platform_nand_ctrl {
    pub probe: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> ::core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(pdev: *mut platform_device)>,
    pub dev_ready: Option<unsafe extern "C" fn(chip: *mut nand_chip) -> ::core::ffi::c_int>,
    pub select_chip: Option<unsafe extern "C" fn(chip: *mut nand_chip, cs: ::core::ffi::c_int)>,
    pub cmd_ctrl: Option<unsafe extern "C" fn(
        chip: *mut nand_chip,
        dat: ::core::ffi::c_int,
        ctrl: ::core::ffi::c_uint,
    )>,
    pub write_buf: Option<unsafe extern "C" fn(
        chip: *mut nand_chip,
        buf: *const u8,
        len: ::core::ffi::c_int,
    )>,
    pub read_buf: Option<unsafe extern "C" fn(
        chip: *mut nand_chip,
        buf: *mut u8,
        len: ::core::ffi::c_int,
    )>,
    pub priv_: *mut ::core::ffi::c_void,
}

/**
 * struct platform_nand_data - container structure for platform-specific data
 * @chip: chip level chip structure
 * @ctrl: controller level device structure
 */
#[repr(C)]
pub struct platform_nand_data {
    pub chip: platform_nand_chip,
    pub ctrl: platform_nand_ctrl,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
