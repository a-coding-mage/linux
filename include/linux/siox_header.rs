/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2015 Pengutronix, Uwe Kleine-König <kernel@pengutronix.de>
 */

// Dependency equivalent of: #include <linux/device.h>

#[inline]
pub unsafe fn to_siox_device(_dev: *mut device) -> *mut siox_device {
    if !_dev.is_null() {
        container_of!(_dev, siox_device, dev)
    } else {
        core::ptr::null_mut()
    }
}

#[repr(C)]
pub struct siox_device {
    pub node: list_head, /* node in smaster->devices */
    pub smaster: *mut siox_master,
    pub dev: device,

    pub type_: *const core::ffi::c_char,
    pub inbytes: usize,
    pub outbytes: usize,
    pub statustype: u8,

    pub status_read_clean: u8,
    pub status_written: u8,
    pub status_written_lastcycle: u8,
    pub connected: bool,

    /* statistics */
    pub watchdog_errors: core::ffi::c_uint,
    pub status_errors: core::ffi::c_uint,

    pub status_errors_kn: *mut kernfs_node,
    pub watchdog_kn: *mut kernfs_node,
    pub watchdog_errors_kn: *mut kernfs_node,
    pub connected_kn: *mut kernfs_node,
}

extern "C" {
    pub fn siox_device_synced(sdevice: *mut siox_device) -> bool;
    pub fn siox_device_connected(sdevice: *mut siox_device) -> bool;
}

#[repr(C)]
pub struct siox_driver {
    pub probe: Option<unsafe extern "C" fn(sdevice: *mut siox_device) -> core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(sdevice: *mut siox_device)>,
    pub shutdown: Option<unsafe extern "C" fn(sdevice: *mut siox_device)>,

    /*
     * buf is big enough to hold sdev->inbytes - 1 bytes, the status byte
     * is in the scope of the framework.
     */
    pub set_data: Option<unsafe extern "C" fn(
        sdevice: *mut siox_device,
        status: u8,
        buf: *mut u8,
    ) -> core::ffi::c_int>,
    /*
     * buf is big enough to hold sdev->outbytes - 1 bytes, the status byte
     * is in the scope of the framework
     */
    pub get_data: Option<unsafe extern "C" fn(
        sdevice: *mut siox_device,
        buf: *const u8,
    ) -> core::ffi::c_int>,

    pub driver: device_driver,
}

#[inline]
pub unsafe fn to_siox_driver(driver: *mut device_driver) -> *mut siox_driver {
    if !driver.is_null() {
        container_of!(driver, siox_driver, driver)
    } else {
        core::ptr::null_mut()
    }
}

extern "C" {
    pub fn __siox_driver_register(
        sdriver: *mut siox_driver,
        owner: *mut module,
    ) -> core::ffi::c_int;
}

#[inline]
pub unsafe fn siox_driver_register(sdriver: *mut siox_driver) -> core::ffi::c_int {
    __siox_driver_register(sdriver, THIS_MODULE)
}

#[inline]
pub unsafe fn siox_driver_unregister(sdriver: *mut siox_driver) {
    driver_unregister(&mut (*sdriver).driver);
}

/*
 * module_siox_driver() - Helper macro for drivers that don't do
 * anything special in module init/exit.  This eliminates a lot of
 * boilerplate.  Each module may only use this macro once, and
 * calling it replaces module_init() and module_exit()
 */
// Equivalent macro expansion:
// module_driver(__siox_driver, siox_driver_register, siox_driver_unregister)
#[macro_export]
macro_rules! module_siox_driver {
    ($siox_driver:expr) => {
        module_driver!($siox_driver, siox_driver_register, siox_driver_unregister)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
