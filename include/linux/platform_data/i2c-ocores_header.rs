/* SPDX-License-Identifier: GPL-2.0 */
/*
 * i2c-ocores.h - definitions for the i2c-ocores interface
 *
 * Peter Korsgaard <peter@korsgaard.com>
 */

#[repr(C)]
pub struct ocores_i2c_platform_data {
    pub reg_shift: u32, /* register offset shift value */
    pub reg_io_width: u32, /* register io read/write width */
    pub clock_khz: u32, /* input clock in kHz */
    pub bus_khz: u32, /* bus clock in kHz */
    pub big_endian: bool, /* registers are big endian */
    pub num_devices: u8, /* number of devices in the devices list */
    pub devices: *const i2c_board_info, /* devices connected to the bus */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
