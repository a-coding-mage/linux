/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2023 Ideas On Board Oy
 */

// C dependencies: linux/device.h and linux/device-id/vchiq.h.

use core::ffi::c_char;

// Supplied by the Linux/Rust compatibility layer.
use crate::{
    bus_type, container_of, device, device_driver, device_id, pm_message_t,
};

pub struct vchiq_drv_mgmt;

#[repr(C)]
pub struct vchiq_device {
    pub dev: device,
    pub drv_mgmt: *mut vchiq_drv_mgmt,
}

#[repr(C)]
pub struct vchiq_driver {
    pub probe: Option<unsafe extern "C" fn(device: *mut vchiq_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(device: *mut vchiq_device)>,
    pub resume: Option<unsafe extern "C" fn(device: *mut vchiq_device) -> i32>,
    pub suspend:
        Option<unsafe extern "C" fn(device: *mut vchiq_device, state: pm_message_t) -> i32>,

    pub id_table: *const device_id,
    pub driver: device_driver,
}

#[inline]
pub unsafe fn to_vchiq_device(d: *mut device) -> *mut vchiq_device {
    container_of!(d, vchiq_device, dev)
}

#[inline]
pub unsafe fn to_vchiq_driver(d: *mut device_driver) -> *mut vchiq_driver {
    container_of!(d, vchiq_driver, driver)
}

pub extern "C" {
    pub static vchiq_bus_type: bus_type;

    pub fn vchiq_device_register(
        parent: *mut device,
        name: *const c_char,
    ) -> *mut vchiq_device;
    pub fn vchiq_device_unregister(dev: *mut vchiq_device);

    pub fn vchiq_driver_register(vchiq_drv: *mut vchiq_driver) -> i32;
    pub fn vchiq_driver_unregister(vchiq_drv: *mut vchiq_driver);
}

/**
 * module_vchiq_driver() - Helper macro for registering a vchiq driver
 * @__vchiq_driver: vchiq driver struct
 *
 * Helper macro for vchiq drivers which do not do anything special in
 * module init/exit. This eliminates a lot of boilerplate. Each module may only
 * use this macro once, and calling it replaces module_init() and module_exit()
 */
// The module_driver! macro is supplied by the kernel module support layer.
#[macro_export]
macro_rules! module_vchiq_driver {
    ($vchiq_driver:expr) => {
        $crate::module_driver!(
            $vchiq_driver,
            $crate::vchiq_driver_register,
            $crate::vchiq_driver_unregister
        );
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
