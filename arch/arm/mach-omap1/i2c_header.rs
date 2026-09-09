/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Helper module for board specific I2C bus registration
 *
 * Copyright (C) 2009 Nokia Corporation.
 */

#[repr(C)]
pub struct i2c_board_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct omap_i2c_bus_platform_data {
    _private: [u8; 0],
}

extern "C" {
    pub fn omap_i2c_add_bus(
        i2c_pdata: *mut omap_i2c_bus_platform_data,
        bus_id: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

/* CONFIG_I2C_OMAP || CONFIG_I2C_OMAP_MODULE */
#[cfg(any(CONFIG_I2C_OMAP, CONFIG_I2C_OMAP_MODULE))]
extern "C" {
    pub fn omap_register_i2c_bus(
        bus_id: ::core::ffi::c_int,
        clkrate: u32,
        info: *const i2c_board_info,
        len: u32,
    ) -> ::core::ffi::c_int;

    pub fn omap_register_i2c_bus_cmdline() -> ::core::ffi::c_int;
}

/* Fallback definitions when CONFIG_I2C_OMAP and CONFIG_I2C_OMAP_MODULE are unset. */
#[cfg(not(any(CONFIG_I2C_OMAP, CONFIG_I2C_OMAP_MODULE)))]
#[inline]
pub fn omap_register_i2c_bus(
    _bus_id: ::core::ffi::c_int,
    _clkrate: u32,
    _info: *const i2c_board_info,
    _len: u32,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(any(CONFIG_I2C_OMAP, CONFIG_I2C_OMAP_MODULE)))]
#[inline]
pub fn omap_register_i2c_bus_cmdline() -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
