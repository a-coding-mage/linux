/*
 * Copyright © 2010 ST Microelectronics
 * Shiraz Hashim <shiraz.linux.kernel@gmail.com>
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2. This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

// Dependencies supplied by the surrounding kernel translation.

/* max possible slots for serial-nor flash chip in the SMI controller */
pub const MAX_NUM_FLASH_CHIP: usize = 4;

/* macro to define partitions for flash devices */
#[macro_export]
macro_rules! DEFINE_PARTS {
    ($n:expr, $of:expr, $s:expr) => {
        mtd_partition {
            name: $n,
            offset: $of,
            size: $s,
        }
    };
}

/**
 * struct spear_smi_flash_info - platform structure for passing flash
 * information
 *
 * @name: name of the serial nor flash for identification
 * @mem_base: the memory base on which the flash is mapped
 * @size: size of the flash in bytes
 * @partitions: parition details
 * @nr_partitions: number of partitions
 * @fast_mode: whether flash supports fast mode
 */
#[repr(C)]
pub struct spear_smi_flash_info {
    pub name: *mut ::core::ffi::c_char,
    pub mem_base: ::core::ffi::c_ulong,
    pub size: ::core::ffi::c_ulong,
    pub partitions: *mut mtd_partition,
    pub nr_partitions: ::core::ffi::c_int,
    pub fast_mode: u8,
}

/**
 * struct spear_smi_plat_data - platform structure for configuring smi
 *
 * @clk_rate: clk rate at which SMI must operate
 * @num_flashes: number of flashes present on board
 * @board_flash_info: specific details of each flash present on board
 * @np: array of DT node pointers for all possible flash chip devices
 */
#[repr(C)]
pub struct spear_smi_plat_data {
    pub clk_rate: ::core::ffi::c_ulong,
    pub num_flashes: ::core::ffi::c_int,
    pub board_flash_info: *mut spear_smi_flash_info,
    pub np: [*mut device_node; MAX_NUM_FLASH_CHIP],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
