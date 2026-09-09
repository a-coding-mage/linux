/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * lm3533.h -- LM3533 interface
 *
 * Copyright (C) 2011-2012 Texas Instruments
 *
 * Author: Johan Hovold <jhovold@gmail.com>
 */

// C header guard: __LINUX_MFD_LM3533_H

// DEVICE_ATTR(_name, S_IRUGO, show_##_name, NULL)
// DEVICE_ATTR(_name, S_IRUGO | S_IWUSR, show_##_name, store_##_name)

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lm3533 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub hwen: *mut gpio_desc,
    pub irq: ::core::ffi::c_int,
    // C bit-fields: have_als:1, have_backlights:1, have_leds:1.
    pub have_als: u32,
    pub have_backlights: u32,
    pub have_leds: u32,
}

#[repr(C)]
pub struct lm3533_ctrlbank {
    pub lm3533: *mut lm3533,
    pub dev: *mut device,
    pub id: ::core::ffi::c_int,
}

#[repr(C)]
pub struct lm3533_als_platform_data {
    // C bit-field pwm_mode:1 (PWM input mode, default analog).
    pub pwm_mode: u32,
    pub r_select: u8, // 1 - 127 (ignored in PWM-mode)
}

#[repr(C)]
pub struct lm3533_bl_platform_data {
    pub name: *mut ::core::ffi::c_char,
    pub max_current: u16, // 5000 - 29800 uA (800 uA step)
    pub default_brightness: u8, // 0 - 255
    pub pwm: u8, // 0 - 0x3f
}

#[repr(C)]
pub struct lm3533_led_platform_data {
    pub name: *mut ::core::ffi::c_char,
    pub default_trigger: *const ::core::ffi::c_char,
    pub max_current: u16, // 5000 - 29800 uA (800 uA step)
    pub pwm: u8, // 0 - 0x3f
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum lm3533_boost_freq {
    LM3533_BOOST_FREQ_500KHZ,
    LM3533_BOOST_FREQ_1000KHZ,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum lm3533_boost_ovp {
    LM3533_BOOST_OVP_16V,
    LM3533_BOOST_OVP_24V,
    LM3533_BOOST_OVP_32V,
    LM3533_BOOST_OVP_40V,
}

#[repr(C)]
pub struct lm3533_platform_data {
    pub boost_ovp: lm3533_boost_ovp,
    pub boost_freq: lm3533_boost_freq,
    pub als: *mut lm3533_als_platform_data,
    pub backlights: *mut lm3533_bl_platform_data,
    pub num_backlights: ::core::ffi::c_int,
    pub leds: *mut lm3533_led_platform_data,
    pub num_leds: ::core::ffi::c_int,
}

extern "C" {
    pub fn lm3533_ctrlbank_enable(cb: *mut lm3533_ctrlbank) -> ::core::ffi::c_int;
    pub fn lm3533_ctrlbank_disable(cb: *mut lm3533_ctrlbank) -> ::core::ffi::c_int;

    pub fn lm3533_ctrlbank_set_brightness(
        cb: *mut lm3533_ctrlbank,
        val: u8,
    ) -> ::core::ffi::c_int;
    pub fn lm3533_ctrlbank_get_brightness(
        cb: *mut lm3533_ctrlbank,
        val: *mut u8,
    ) -> ::core::ffi::c_int;
    pub fn lm3533_ctrlbank_set_max_current(
        cb: *mut lm3533_ctrlbank,
        imax: u16,
    ) -> ::core::ffi::c_int;
    pub fn lm3533_ctrlbank_set_pwm(cb: *mut lm3533_ctrlbank, val: u8) -> ::core::ffi::c_int;
    pub fn lm3533_ctrlbank_get_pwm(cb: *mut lm3533_ctrlbank, val: *mut u8) -> ::core::ffi::c_int;

    pub fn lm3533_read(
        lm3533: *mut lm3533,
        reg: u8,
        val: *mut u8,
    ) -> ::core::ffi::c_int;
    pub fn lm3533_write(lm3533: *mut lm3533, reg: u8, val: u8) -> ::core::ffi::c_int;
    pub fn lm3533_update(
        lm3533: *mut lm3533,
        reg: u8,
        val: u8,
        mask: u8,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
