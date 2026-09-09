/* SPDX-License-Identifier: GPL-2.0 */

// C forward declaration: struct sh_spi_info;

#[repr(C)]
pub struct sh_spi_info {
    pub bus_num: i32,
    pub num_chipselect: i32,
    pub chip_select: Option<unsafe extern "C" fn(spi: *mut sh_spi_info, cs: i32, state: i32)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
