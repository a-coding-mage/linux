/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * pca9532.h - platform data structure for pca9532 led controller
 *
 * Copyright (C) 2008 Riku Voipio <riku.voipio@movial.fi>
 *
 * Datasheet: http://www.nxp.com/acrobat/datasheets/PCA9532_3.pdf
 */

// C dependencies:
// #include <linux/leds.h>
// #include <linux/workqueue.h>
// #include <dt-bindings/leds/leds-pca9532.h>

// External types supplied by the corresponding kernel dependencies.
pub struct i2c_client;
pub struct led_classdev;
pub struct work_struct;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum pca9532_state {
    PCA9532_OFF = 0x0,
    PCA9532_ON = 0x1,
    PCA9532_PWM0 = 0x2,
    PCA9532_PWM1 = 0x3,
    PCA9532_KEEP = 0xff,
}

#[repr(C)]
pub struct pca9532_led {
    pub id: u8,
    pub client: *mut i2c_client,
    pub name: *const core::ffi::c_char,
    pub default_trigger: *const core::ffi::c_char,
    pub ldev: led_classdev,
    pub work: work_struct,
    pub type_: u32,
    pub state: pca9532_state,
}

#[repr(C)]
pub struct pca9532_platform_data {
    pub leds: [pca9532_led; 16],
    pub pwm: [u8; 2],
    pub psc: [u8; 2],
    pub gpio_base: core::ffi::c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
