/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * LP855x Backlight Driver
 *
 *			Copyright (C) 2011 Texas Instruments
 */

// C header dependencies: BIT and the LP855x symbols referenced below are
// supplied by other headers/source files.

pub const BL_CTL_SHFT: u32 = 0;
pub const BRT_MODE_SHFT: u32 = 1;
pub const BRT_MODE_MASK: u32 = 0x06;

/* Enable backlight. Only valid when BRT_MODE=10(I2C only) */
pub const ENABLE_BL: u32 = 1;
pub const DISABLE_BL: u32 = 0;

#[macro_export]
macro_rules! I2C_CONFIG {
    (LP8550) => { LP8550_I2C_CONFIG };
    (LP8551) => { LP8551_I2C_CONFIG };
    (LP8552) => { LP8552_I2C_CONFIG };
    (LP8553) => { LP8553_I2C_CONFIG };
    (LP8555) => { LP8555_I2C_CONFIG };
    (LP8556) => { LP8556_I2C_CONFIG };
    (LP8557) => { LP8557_I2C_CONFIG };
}

#[macro_export]
macro_rules! PWM_CONFIG {
    (LP8550) => { LP8550_PWM_CONFIG };
    (LP8551) => { LP8551_PWM_CONFIG };
    (LP8552) => { LP8552_PWM_CONFIG };
    (LP8553) => { LP8553_PWM_CONFIG };
    (LP8555) => { LP8555_PWM_CONFIG };
    (LP8556) => { LP8556_PWM_CONFIG };
    (LP8557) => { LP8557_PWM_CONFIG };
}

/* DEVICE CONTROL register - LP8550 */
pub const LP8550_PWM_CONFIG: u32 = LP8550_PWM_ONLY << BRT_MODE_SHFT;
pub const LP8550_I2C_CONFIG: u32 =
    (ENABLE_BL << BL_CTL_SHFT) | (LP8550_I2C_ONLY << BRT_MODE_SHFT);

/* DEVICE CONTROL register - LP8551 */
pub const LP8551_PWM_CONFIG: u32 = LP8550_PWM_CONFIG;
pub const LP8551_I2C_CONFIG: u32 = LP8550_I2C_CONFIG;

/* DEVICE CONTROL register - LP8552 */
pub const LP8552_PWM_CONFIG: u32 = LP8550_PWM_CONFIG;
pub const LP8552_I2C_CONFIG: u32 = LP8550_I2C_CONFIG;

/* DEVICE CONTROL register - LP8553 */
pub const LP8553_PWM_CONFIG: u32 = LP8550_PWM_CONFIG;
pub const LP8553_I2C_CONFIG: u32 = LP8550_I2C_CONFIG;

/* CONFIG register - LP8555 */
pub const LP8555_PWM_STANDBY: u32 = 1 << 7;
pub const LP8555_PWM_FILTER: u32 = 1 << 6;
pub const LP8555_RELOAD_EPROM: u32 = 1 << 3; /* use it if EPROMs should be reset
                                                  when the backlight turns on */
pub const LP8555_OFF_OPENLEDS: u32 = 1 << 2;
pub const LP8555_PWM_CONFIG: u32 = LP8555_PWM_ONLY;
pub const LP8555_I2C_CONFIG: u32 = LP8555_I2C_ONLY;
pub const LP8555_COMB1_CONFIG: u32 = LP8555_COMBINED1;
pub const LP8555_COMB2_CONFIG: u32 = LP8555_COMBINED2;

/* DEVICE CONTROL register - LP8556 */
pub const LP8556_PWM_CONFIG: u32 = LP8556_PWM_ONLY << BRT_MODE_SHFT;
pub const LP8556_COMB1_CONFIG: u32 = LP8556_COMBINED1 << BRT_MODE_SHFT;
pub const LP8556_I2C_CONFIG: u32 =
    (ENABLE_BL << BL_CTL_SHFT) | (LP8556_I2C_ONLY << BRT_MODE_SHFT);
pub const LP8556_COMB2_CONFIG: u32 = LP8556_COMBINED2 << BRT_MODE_SHFT;
pub const LP8556_FAST_CONFIG: u32 = 1 << 7; /* use it if EPROMs should be maintained
                                                 when exiting the low power mode */

/* CONFIG register - LP8557 */
pub const LP8557_PWM_STANDBY: u32 = 1 << 7;
pub const LP8557_PWM_FILTER: u32 = 1 << 6;
pub const LP8557_RELOAD_EPROM: u32 = 1 << 3; /* use it if EPROMs should be reset
                                                  when the backlight turns on */
pub const LP8557_OFF_OPENLEDS: u32 = 1 << 2;
pub const LP8557_PWM_CONFIG: u32 = LP8557_PWM_ONLY;
pub const LP8557_I2C_CONFIG: u32 = LP8557_I2C_ONLY;
pub const LP8557_COMB1_CONFIG: u32 = LP8557_COMBINED1;
pub const LP8557_COMB2_CONFIG: u32 = LP8557_COMBINED2;

#[repr(C)]
pub enum lp855x_chip_id {
    LP8550,
    LP8551,
    LP8552,
    LP8553,
    LP8555,
    LP8556,
    LP8557,
}

#[repr(C)]
pub enum lp8550_brighntess_source {
    LP8550_PWM_ONLY,
    LP8550_I2C_ONLY = 2,
}

#[repr(C)]
pub enum lp8551_brighntess_source {
    LP8551_PWM_ONLY = LP8550_PWM_ONLY as isize,
    LP8551_I2C_ONLY = LP8550_I2C_ONLY as isize,
}

#[repr(C)]
pub enum lp8552_brighntess_source {
    LP8552_PWM_ONLY = LP8550_PWM_ONLY as isize,
    LP8552_I2C_ONLY = LP8550_I2C_ONLY as isize,
}

#[repr(C)]
pub enum lp8553_brighntess_source {
    LP8553_PWM_ONLY = LP8550_PWM_ONLY as isize,
    LP8553_I2C_ONLY = LP8550_I2C_ONLY as isize,
}

#[repr(C)]
pub enum lp8555_brightness_source {
    LP8555_PWM_ONLY,
    LP8555_I2C_ONLY,
    LP8555_COMBINED1, /* Brightness register with shaped PWM */
    LP8555_COMBINED2, /* PWM with shaped brightness register */
}

#[repr(C)]
pub enum lp8556_brightness_source {
    LP8556_PWM_ONLY,
    LP8556_COMBINED1, /* pwm + i2c before the shaper block */
    LP8556_I2C_ONLY,
    LP8556_COMBINED2, /* pwm + i2c after the shaper block */
}

#[repr(C)]
pub enum lp8557_brightness_source {
    LP8557_PWM_ONLY,
    LP8557_I2C_ONLY,
    LP8557_COMBINED1, /* pwm + i2c after the shaper block */
    LP8557_COMBINED2, /* pwm + i2c before the shaper block */
}

#[repr(C)]
pub struct lp855x_rom_data {
    pub addr: u8,
    pub val: u8,
}

/**
 * struct lp855x_platform_data - lp855 platform-specific data
 * @name : Backlight driver name. If it is not defined, default name is set.
 * @device_control : value of DEVICE CONTROL register
 * @initial_brightness : initial value of backlight brightness
 * @period_ns : platform specific pwm period value. unit is nano.
 *		Only valid when mode is PWM_BASED.
 * @size_program : total size of lp855x_rom_data
 * @rom_data : list of new eeprom/eprom registers
 */
#[repr(C)]
pub struct lp855x_platform_data {
    pub name: *const core::ffi::c_char,
    pub device_control: u8,
    pub initial_brightness: u8,
    pub period_ns: u32,
    pub size_program: i32,
    pub rom_data: *mut lp855x_rom_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
