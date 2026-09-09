/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * I2C multiplexer using a single register
 *
 * Copyright 2015 Freescale Semiconductor
 * York Sun <yorksun@freescale.com>
 */

/**
 * struct i2c_mux_reg_platform_data - Platform-dependent data for i2c-mux-reg
 * @parent: Parent I2C bus adapter number
 * @base_nr: Base I2C bus number to number adapters from or zero for dynamic
 * @values: Array of value for each channel
 * @n_values: Number of multiplexer channels
 * @little_endian: Indicating if the register is in little endian
 * @write_only: Reading the register is not allowed by hardware
 * @idle: Value to write to mux when idle
 * @idle_in_use: indicate if idle value is in use
 * @reg: Virtual address of the register to switch channel
 * @reg_size: register size in bytes
 */
#[repr(C)]
pub struct i2c_mux_reg_platform_data {
    pub parent: core::ffi::c_int,
    pub base_nr: core::ffi::c_int,
    pub values: *const core::ffi::c_uint,
    pub n_values: core::ffi::c_int,
    pub little_endian: bool,
    pub write_only: bool,
    pub idle: u32,
    pub idle_in_use: bool,
    pub reg: *mut core::ffi::c_void,
    pub reg_size: resource_size_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
