/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2008 Openmoko, Inc.
 * Copyright 2008 Simtec Electronics
 *	http://armlinux.simtec.co.uk/
 *	Ben Dooks <ben@simtec.co.uk>
 *
 * S3C Platform - GPIO pin configuration
 */

/* This file contains the necessary definitions to get the basic gpio
 * pin configuration done such as setting a pin to input or output or
 * changing the pull-{up,down} configurations.
 */

/* Note, this interface is being added to the s3c64xx arch first and will
 * be added to the s3c24xx systems later.
 */

/// Equivalent to C's `unsigned int __bitwise` GPIO pull type.
pub type samsung_gpio_pull_t = u32;

/* forward declaration if gpio-core.h hasn't been included */
#[repr(C)]
pub struct samsung_gpio_chip {
    _private: [u8; 0],
}

/**
 * struct samsung_gpio_cfg GPIO configuration
 * @cfg_eint: Configuration setting when used for external interrupt source
 * @get_pull: Read the current pull configuration for the GPIO
 * @set_pull: Set the current pull configuration for the GPIO
 * @set_config: Set the current configuration for the GPIO
 * @get_config: Read the current configuration for the GPIO
 *
 * Each chip can have more than one type of GPIO bank available and some
 * have different capabilites even when they have the same control register
 * layouts. Provide an point to vector control routine and provide any
 * per-bank configuration information that other systems such as the
 * external interrupt code will need.
 *
 * @sa samsung_gpio_cfgpin
 * @sa s3c_gpio_getcfg
 * @sa s3c_gpio_setpull
 * @sa s3c_gpio_getpull
 */
#[repr(C)]
pub struct samsung_gpio_cfg {
    pub cfg_eint: u32,
    pub get_pull: Option<unsafe extern "C" fn(*mut samsung_gpio_chip, u32) -> samsung_gpio_pull_t>,
    pub set_pull: Option<unsafe extern "C" fn(*mut samsung_gpio_chip, u32, samsung_gpio_pull_t) -> i32>,
    pub get_config: Option<unsafe extern "C" fn(*mut samsung_gpio_chip, u32) -> u32>,
    pub set_config: Option<unsafe extern "C" fn(*mut samsung_gpio_chip, u32, u32) -> i32>,
}

pub const S3C_GPIO_SPECIAL_MARK: u32 = 0xfffffff0;

#[inline]
pub const fn S3C_GPIO_SPECIAL(x: u32) -> u32 {
    S3C_GPIO_SPECIAL_MARK | x
}

/* Defines for generic pin configurations */
pub const S3C_GPIO_INPUT: u32 = S3C_GPIO_SPECIAL(0);
pub const S3C_GPIO_OUTPUT: u32 = S3C_GPIO_SPECIAL(1);

#[inline]
pub const fn S3C_GPIO_SFN(x: u32) -> u32 {
    S3C_GPIO_SPECIAL(x)
}

#[inline]
pub const fn samsung_gpio_is_cfg_special(cfg: u32) -> bool {
    (cfg & S3C_GPIO_SPECIAL_MARK) == S3C_GPIO_SPECIAL_MARK
}

/**
 * s3c_gpio_cfgpin() - Change the GPIO function of a pin.
 * @pin pin The pin number to configure.
 * @to to The configuration for the pin's function.
 *
 * Configure which function is actually connected to the external
 * pin, such as an gpio input, output or some form of special function
 * connected to an internal peripheral block.
 *
 * The @to parameter can be one of the generic S3C_GPIO_INPUT, S3C_GPIO_OUTPUT
 * or S3C_GPIO_SFN() to indicate one of the possible values that the helper
 * will then generate the correct bit mask and shift for the configuration.
 *
 * If a bank of GPIOs all needs to be set to special-function 2, then
 * the following code will work:
 *
 *	for (gpio = start; gpio < end; gpio++)
 *		s3c_gpio_cfgpin(gpio, S3C_GPIO_SFN(2));
 *
 * The @to parameter can also be a specific value already shifted to the
 * correct position in the control register, although these are discouraged
 * in newer kernels and are only being kept for compatibility.
 */
unsafe extern "C" {
    pub fn s3c_gpio_cfgpin(pin: u32, to: u32) -> i32;
    pub fn s3c_gpio_cfgpin_range(start: u32, nr: u32, cfg: u32) -> i32;
    pub fn s3c_gpio_setpull(pin: u32, pull: samsung_gpio_pull_t) -> i32;
    pub fn s3c_gpio_cfgall_range(start: u32, nr: u32, cfg: u32, pull: samsung_gpio_pull_t) -> i32;
}

/* Define values for the pull-{up,down} available for each gpio pin.
 *
 * These values control the state of the weak pull-{up,down} resistors
 * available on most pins on the S3C series. Not all chips support both
 * up or down settings, and it may be dependent on the chip that is being
 * used to whether the particular mode is available.
 */
pub const S3C_GPIO_PULL_NONE: samsung_gpio_pull_t = 0x00;
pub const S3C_GPIO_PULL_DOWN: samsung_gpio_pull_t = 0x01;
pub const S3C_GPIO_PULL_UP: samsung_gpio_pull_t = 0x02;

/* configure `all` aspects of an gpio */

/**
 * s3c_gpio_cfgall_range() - configure range of gpio functtion and pull.
 * @start: The gpio number to start at.
 * @nr: The number of gpio to configure from @start.
 * @cfg: The configuration to use
 * @pull: The pull setting to use.
 *
 * Run s3c_gpio_cfgpin() and s3c_gpio_setpull() over the gpio range starting
 * @gpio and running for @size.
 *
 * @sa s3c_gpio_cfgpin
 * @sa s3c_gpio_setpull
 * @sa s3c_gpio_cfgpin_range
 */
#[inline]
pub unsafe fn s3c_gpio_cfgrange_nopull(pin: u32, size: u32, cfg: u32) -> i32 {
    s3c_gpio_cfgall_range(pin, size, cfg, S3C_GPIO_PULL_NONE)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
