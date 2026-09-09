/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * i2c-gpio interface to platform code
 *
 * Copyright (C) 2007 Atmel Corporation
 */

/**
 * struct i2c_gpio_platform_data - Platform-dependent data for i2c-gpio
 * @udelay: signal toggle delay. SCL frequency is (500 / udelay) kHz
 * @timeout: clock stretching timeout in jiffies. If the slave keeps
 *	SCL low for longer than this, the transfer will time out.
 * @sda_is_open_drain: SDA is configured as open drain, i.e. the pin
 *	isn't actively driven high when setting the output value high.
 *	gpio_get_value() must return the actual pin state even if the
 *	pin is configured as an output.
 * @sda_is_output_only: SDA output drivers can't be turned off.
 *	This is for clients that can only read SDA/SCL.
 * @sda_has_no_pullup: SDA is used in a non-compliant way and has no pull-up.
 *	Therefore disable open-drain.
 * @scl_is_open_drain: SCL is set up as open drain. Same requirements
 *	as for sda_is_open_drain apply.
 * @scl_is_output_only: SCL output drivers cannot be turned off.
 * @scl_has_no_pullup: SCL is used in a non-compliant way and has no pull-up.
 *	Therefore disable open-drain.
 */
#[repr(C)]
pub struct i2c_gpio_platform_data {
    pub udelay: i32,
    pub timeout: i32,
    // C unsigned int bit-fields; represented as their underlying 32-bit storage values.
    pub sda_is_open_drain: u32,
    pub sda_is_output_only: u32,
    pub sda_has_no_pullup: u32,
    pub scl_is_open_drain: u32,
    pub scl_is_output_only: u32,
    pub scl_has_no_pullup: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
