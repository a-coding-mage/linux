/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation:
// #include <linux/device/driver.h>

use core::ffi::c_char;

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct of_device_id;
#[repr(C)]
pub struct kobj_uevent_env;
#[repr(C)]
pub struct device_node;
#[repr(C)]
pub struct device_driver;

// CONFIG_OF controls which implementation is selected at build time.
#[cfg(CONFIG_OF)]
extern "C" {
    pub fn of_match_device(
        matches: *const of_device_id,
        dev: *const device,
    ) -> *const of_device_id;

    pub fn of_device_modalias(dev: *mut device, str_: *mut c_char, len: isize) -> isize;

    pub fn of_device_uevent(dev: *const device, env: *mut kobj_uevent_env);
    pub fn of_device_uevent_modalias(
        dev: *const device,
        env: *mut kobj_uevent_env,
    ) -> i32;

    pub fn of_dma_configure_id(
        dev: *mut device,
        np: *mut device_node,
        force_dma: bool,
        id: *const u32,
    ) -> i32;

    pub fn of_device_make_bus_id(dev: *mut device);
}

#[cfg(CONFIG_OF)]
#[inline]
pub unsafe fn of_driver_match_device(dev: *mut device, drv: *const device_driver) -> i32 {
    // `drv->of_match_table` is supplied by the translated device-driver definition.
    (of_match_device((*drv).of_match_table, dev) != core::ptr::null()) as i32
}

#[cfg(CONFIG_OF)]
#[inline]
pub unsafe fn of_dma_configure(
    dev: *mut device,
    np: *mut device_node,
    force_dma: bool,
) -> i32 {
    of_dma_configure_id(dev, np, force_dma, core::ptr::null())
}

#[cfg(not(CONFIG_OF))]
#[inline]
pub unsafe fn of_driver_match_device(_dev: *mut device, _drv: *const device_driver) -> i32 {
    0
}

#[cfg(not(CONFIG_OF))]
#[inline]
pub unsafe fn of_device_uevent(_dev: *const device, _env: *mut kobj_uevent_env) {}

#[cfg(not(CONFIG_OF))]
#[inline]
pub unsafe fn of_device_modalias(_dev: *mut device, _str: *mut c_char, _len: isize) -> i32 {
    -19 // -ENODEV
}

#[cfg(not(CONFIG_OF))]
#[inline]
pub unsafe fn of_device_uevent_modalias(
    _dev: *const device,
    _env: *mut kobj_uevent_env,
) -> i32 {
    -19 // -ENODEV
}

#[cfg(not(CONFIG_OF))]
#[inline]
pub unsafe fn of_match_device(
    _matches: *const of_device_id,
    _dev: *const device,
) -> *const of_device_id {
    core::ptr::null()
}

#[cfg(not(CONFIG_OF))]
#[inline]
pub unsafe fn of_dma_configure_id(
    _dev: *mut device,
    _np: *mut device_node,
    _force_dma: bool,
    _id: *const u32,
) -> i32 {
    0
}

#[cfg(not(CONFIG_OF))]
#[inline]
pub unsafe fn of_dma_configure(
    _dev: *mut device,
    _np: *mut device_node,
    _force_dma: bool,
) -> i32 {
    0
}

#[cfg(not(CONFIG_OF))]
#[inline]
pub unsafe fn of_device_make_bus_id(_dev: *mut device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
