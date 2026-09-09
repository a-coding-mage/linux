/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Consumer interface the pin control subsystem
 *
 * Copyright (C) 2012 ST-Ericsson SA
 * Written on behalf of Linaro for ST-Ericsson
 * Based on bits of regulator core, gpio core and clk core
 *
 * Author: Linus Walleij <linus.walleij@linaro.org>
 */

use core::ffi::{c_char, c_ulong};

// Dependency declarations supplied by the surrounding kernel translation.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_chip {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pinctrl {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pinctrl_state {
    _private: [u8; 0],
}

// PINCTRL_STATE_DEFAULT is provided by pinctrl-state.h.
extern "C" {
    pub fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    pub fn ERR_CAST(ptr: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn ERR_PTR(error: i32) -> *mut core::ffi::c_void;
}

#[cfg(feature = "CONFIG_PINCTRL")]
extern "C" {
    pub fn pinctrl_gpio_can_use_line(gc: *mut gpio_chip, offset: u32) -> bool;
    pub fn pinctrl_gpio_request(gc: *mut gpio_chip, offset: u32) -> i32;
    pub fn pinctrl_gpio_free(gc: *mut gpio_chip, offset: u32);
    pub fn pinctrl_gpio_direction_input(gc: *mut gpio_chip, offset: u32) -> i32;
    pub fn pinctrl_gpio_direction_output(gc: *mut gpio_chip, offset: u32) -> i32;
    pub fn pinctrl_gpio_set_config(gc: *mut gpio_chip, offset: u32, config: c_ulong) -> i32;
    pub fn pinctrl_gpio_get_config(gc: *mut gpio_chip, offset: u32, config: *mut c_ulong) -> i32;
    pub fn pinctrl_get(dev: *mut device) -> *mut pinctrl;
    pub fn pinctrl_put(p: *mut pinctrl);
    pub fn pinctrl_lookup_state(p: *mut pinctrl, name: *const c_char) -> *mut pinctrl_state;
    pub fn pinctrl_select_state(p: *mut pinctrl, s: *mut pinctrl_state) -> i32;
    pub fn devm_pinctrl_get(dev: *mut device) -> *mut pinctrl;
    pub fn devm_pinctrl_put(p: *mut pinctrl);
    pub fn pinctrl_select_default_state(dev: *mut device) -> i32;
}

#[cfg(feature = "CONFIG_PINCTRL")]
#[cfg(feature = "CONFIG_PM")]
extern "C" {
    pub fn pinctrl_pm_select_default_state(dev: *mut device) -> i32;
    pub fn pinctrl_pm_select_init_state(dev: *mut device) -> i32;
    pub fn pinctrl_pm_select_sleep_state(dev: *mut device) -> i32;
    pub fn pinctrl_pm_select_idle_state(dev: *mut device) -> i32;
}

#[cfg(not(feature = "CONFIG_PM"))]
pub unsafe fn pinctrl_pm_select_default_state(_dev: *mut device) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_PM"))]
pub unsafe fn pinctrl_pm_select_init_state(_dev: *mut device) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_PM"))]
pub unsafe fn pinctrl_pm_select_sleep_state(_dev: *mut device) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_PM"))]
pub unsafe fn pinctrl_pm_select_idle_state(_dev: *mut device) -> i32 { 0 }

// !CONFIG_PINCTRL provides no-op inline implementations.
#[cfg(not(feature = "CONFIG_PINCTRL"))]
pub unsafe fn pinctrl_gpio_can_use_line(_gc: *mut gpio_chip, _offset: u32) -> bool { true }
#[cfg(not(feature = "CONFIG_PINCTRL"))]
pub unsafe fn pinctrl_gpio_request(_gc: *mut gpio_chip, _offset: u32) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_PINCTRL"))]
pub unsafe fn pinctrl_gpio_free(_gc: *mut gpio_chip, _offset: u32) {}
#[cfg(not(feature = "CONFIG_PINCTRL"))]
pub unsafe fn pinctrl_gpio_direction_input(_gc: *mut gpio_chip, _offset: u32) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_PINCTRL"))]
pub unsafe fn pinctrl_gpio_direction_output(_gc: *mut gpio_chip, _offset: u32) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_PINCTRL"))]
pub unsafe fn pinctrl_gpio_get_config(_gc: *mut gpio_chip, _offset: u32, _config: *mut c_ulong) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_PINCTRL"))]
pub unsafe fn pinctrl_gpio_set_config(_gc: *mut gpio_chip, _offset: u32, _config: c_ulong) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_PINCTRL"))]
pub unsafe fn pinctrl_get(_dev: *mut device) -> *mut pinctrl { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_PINCTRL"))]
pub unsafe fn pinctrl_put(_p: *mut pinctrl) {}
#[cfg(not(feature = "CONFIG_PINCTRL"))]
pub unsafe fn pinctrl_lookup_state(_p: *mut pinctrl, _name: *const c_char) -> *mut pinctrl_state { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_PINCTRL"))]
pub unsafe fn pinctrl_select_state(_p: *mut pinctrl, _s: *mut pinctrl_state) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_PINCTRL"))]
pub unsafe fn devm_pinctrl_get(_dev: *mut device) -> *mut pinctrl { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_PINCTRL"))]
pub unsafe fn devm_pinctrl_put(_p: *mut pinctrl) {}
#[cfg(not(feature = "CONFIG_PINCTRL"))]
pub unsafe fn pinctrl_select_default_state(_dev: *mut device) -> i32 { 0 }

pub const PINCTRL_STATE_DEFAULT: *const c_char = b"default\0".as_ptr() as *const c_char;

pub unsafe fn pinctrl_get_select(dev: *mut device, name: *const c_char) -> *mut pinctrl {
    let p = pinctrl_get(dev);
    if IS_ERR(p.cast()) { return p; }
    let s = pinctrl_lookup_state(p, name);
    if IS_ERR(s.cast()) {
        pinctrl_put(p);
        return ERR_CAST(s.cast()).cast();
    }
    let ret = pinctrl_select_state(p, s);
    if ret < 0 {
        pinctrl_put(p);
        return ERR_PTR(ret).cast();
    }
    p
}

pub unsafe fn pinctrl_get_select_default(dev: *mut device) -> *mut pinctrl {
    pinctrl_get_select(dev, PINCTRL_STATE_DEFAULT)
}

pub unsafe fn devm_pinctrl_get_select(dev: *mut device, name: *const c_char) -> *mut pinctrl {
    let p = devm_pinctrl_get(dev);
    if IS_ERR(p.cast()) { return p; }
    let s = pinctrl_lookup_state(p, name);
    if IS_ERR(s.cast()) {
        devm_pinctrl_put(p);
        return ERR_CAST(s.cast()).cast();
    }
    let ret = pinctrl_select_state(p, s);
    if ret < 0 {
        devm_pinctrl_put(p);
        return ERR_PTR(ret).cast();
    }
    p
}

pub unsafe fn devm_pinctrl_get_select_default(dev: *mut device) -> *mut pinctrl {
    devm_pinctrl_get_select(dev, PINCTRL_STATE_DEFAULT)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
