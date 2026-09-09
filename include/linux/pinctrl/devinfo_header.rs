/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Per-device information from the pin control system.
 * This is the stuff that get included into the device
 * core.
 *
 * Copyright (C) 2012 ST-Ericsson SA
 * Written on behalf of Linaro for ST-Ericsson
 * This interface is used in the core to keep track of pins.
 *
 * Author: Linus Walleij <linus.walleij@linaro.org>
 */

// C header guard: PINCTRL_DEVINFO_H

pub enum device {}

pub enum pinctrl {}

// CONFIG_PINCTRL
#[cfg(feature = "CONFIG_PINCTRL")]
pub enum pinctrl_state {}

/**
 * struct dev_pin_info - pin state container for devices
 * @p: pinctrl handle for the containing device
 * @default_state: the default state for the handle, if found
 * @init_state: the state at probe time, if found
 * @sleep_state: the state at suspend time, if found
 * @idle_state: the state at idle (runtime suspend) time, if found
 */
#[cfg(feature = "CONFIG_PINCTRL")]
#[repr(C)]
pub struct dev_pin_info {
    pub p: *mut pinctrl,
    pub default_state: *mut pinctrl_state,
    pub init_state: *mut pinctrl_state,
    // CONFIG_PM
    #[cfg(feature = "CONFIG_PM")]
    pub sleep_state: *mut pinctrl_state,
    #[cfg(feature = "CONFIG_PM")]
    pub idle_state: *mut pinctrl_state,
}

#[cfg(feature = "CONFIG_PINCTRL")]
unsafe extern "C" {
    pub fn pinctrl_init_done(dev: *mut device) -> i32;
}

#[cfg(feature = "CONFIG_PINCTRL")]
#[inline]
pub unsafe fn dev_pinctrl(dev: *mut device) -> *mut pinctrl {
    // The C implementation checks dev->pins. The containing device layout is
    // supplied by the external Linux device dependency.
    todo!("access device::pins from the external Linux device dependency")
}

// Stubs if we're not using pinctrl
#[cfg(not(feature = "CONFIG_PINCTRL"))]
#[inline]
pub unsafe fn pinctrl_init_done(_dev: *mut device) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_PINCTRL"))]
#[inline]
pub unsafe fn dev_pinctrl(_dev: *mut device) -> *mut pinctrl {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
