/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * External Connector (extcon) framework
 * - linux/include/linux/extcon-provider.h for extcon provider device driver.
 *
 * Copyright (C) 2017 Samsung Electronics
 * Author: Chanwoo Choi <cw00.choi@samsung.com>
 */

// C header dependency: <linux/extcon.h>

use core::ffi::c_int;

#[repr(C)]
pub struct extcon_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub union extcon_property_value {
    pub intval: c_int,
    pub intval64: i64,
    pub strval: *const core::ffi::c_char,
}

// When CONFIG_EXTCON is enabled, these are externally defined C functions.
#[cfg(feature = "CONFIG_EXTCON")]
extern "C" {
    pub fn extcon_dev_register(edev: *mut extcon_dev) -> c_int;
    pub fn extcon_dev_unregister(edev: *mut extcon_dev);
    pub fn devm_extcon_dev_register(dev: *mut device, edev: *mut extcon_dev) -> c_int;
    pub fn devm_extcon_dev_unregister(dev: *mut device, edev: *mut extcon_dev);

    pub fn extcon_dev_allocate(cable: *const u32) -> *mut extcon_dev;
    pub fn extcon_dev_free(edev: *mut extcon_dev);
    pub fn devm_extcon_dev_allocate(dev: *mut device, cable: *const u32) -> *mut extcon_dev;
    pub fn devm_extcon_dev_free(dev: *mut device, edev: *mut extcon_dev);

    pub fn extcon_sync(edev: *mut extcon_dev, id: u32) -> c_int;
    pub fn extcon_set_state(edev: *mut extcon_dev, id: u32, state: bool) -> c_int;
    pub fn extcon_set_state_sync(edev: *mut extcon_dev, id: u32, state: bool) -> c_int;
    pub fn extcon_set_property(
        edev: *mut extcon_dev,
        id: u32,
        prop: u32,
        prop_val: extcon_property_value,
    ) -> c_int;
    pub fn extcon_set_property_sync(
        edev: *mut extcon_dev,
        id: u32,
        prop: u32,
        prop_val: extcon_property_value,
    ) -> c_int;
    pub fn extcon_set_property_capability(edev: *mut extcon_dev, id: u32, prop: u32) -> c_int;
}

// CONFIG_EXTCON disabled: direct equivalents of the C inline stubs.
#[cfg(not(feature = "CONFIG_EXTCON"))]
pub unsafe fn extcon_dev_register(_edev: *mut extcon_dev) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_EXTCON"))]
pub unsafe fn extcon_dev_unregister(_edev: *mut extcon_dev) {}
#[cfg(not(feature = "CONFIG_EXTCON"))]
pub unsafe fn devm_extcon_dev_register(_dev: *mut device, _edev: *mut extcon_dev) -> c_int { -22 }
#[cfg(not(feature = "CONFIG_EXTCON"))]
pub unsafe fn devm_extcon_dev_unregister(_dev: *mut device, _edev: *mut extcon_dev) {}
#[cfg(not(feature = "CONFIG_EXTCON"))]
pub unsafe fn extcon_dev_allocate(_cable: *const u32) -> *mut extcon_dev { (-38isize) as *mut extcon_dev }
#[cfg(not(feature = "CONFIG_EXTCON"))]
pub unsafe fn extcon_dev_free(_edev: *mut extcon_dev) {}
#[cfg(not(feature = "CONFIG_EXTCON"))]
pub unsafe fn devm_extcon_dev_allocate(_dev: *mut device, _cable: *const u32) -> *mut extcon_dev { (-38isize) as *mut extcon_dev }
#[cfg(not(feature = "CONFIG_EXTCON"))]
pub unsafe fn devm_extcon_dev_free(_edev: *mut extcon_dev) {}
#[cfg(not(feature = "CONFIG_EXTCON"))]
pub unsafe fn extcon_set_state(_edev: *mut extcon_dev, _id: u32, _state: bool) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_EXTCON"))]
pub unsafe fn extcon_set_state_sync(_edev: *mut extcon_dev, _id: u32, _state: bool) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_EXTCON"))]
pub unsafe fn extcon_sync(_edev: *mut extcon_dev, _id: u32) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_EXTCON"))]
pub unsafe fn extcon_set_property(_edev: *mut extcon_dev, _id: u32, _prop: u32, _prop_val: extcon_property_value) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_EXTCON"))]
pub unsafe fn extcon_set_property_sync(_edev: *mut extcon_dev, _id: u32, _prop: u32, _prop_val: extcon_property_value) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_EXTCON"))]
pub unsafe fn extcon_set_property_capability(_edev: *mut extcon_dev, _id: u32, _prop: u32) -> c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
