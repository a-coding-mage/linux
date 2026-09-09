/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * i2c-mux-gpio interface to platform code
 *
 * Peter Korsgaard <peter.korsgaard@barco.com>
 */

/* MUX has no specific idle mode */
pub const I2C_MUX_GPIO_NO_IDLE: u32 = u32::MAX;

/**
 * struct i2c_mux_gpio_platform_data - Platform-dependent data for i2c-mux-gpio
 * @parent: Parent I2C bus adapter number
 * @base_nr: Base I2C bus number to number adapters from or zero for dynamic
 * @values: Array of bitmasks of GPIO settings (low/high) for each
 *\tposition
 * @n_values: Number of multiplexer positions (busses to instantiate)
 * @idle: Bitmask to write to MUX when idle or GPIO_I2CMUX_NO_IDLE if not used
 * @settle_time: Delay to wait when a new bus is selected
 */
#[repr(C)]
pub struct i2c_mux_gpio_platform_data {
    pub parent: i32,
    pub base_nr: i32,
    pub values: *const u32,
    pub n_values: i32,
    pub idle: u32,
    pub settle_time: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
