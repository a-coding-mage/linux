/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * coreboot_table.h
 *
 * Internal header for coreboot table access.
 *
 * Copyright 2014 Gerd Hoffmann <kraxel@redhat.com>
 * Copyright 2017 Google Inc.
 * Copyright 2017 Samuel Holland <samuel@sholland.org>
 */

// C dependencies:
// #include <linux/coreboot.h>
// #include <linux/device.h>

/* A device, additionally with information from coreboot. */
#[repr(C)]
pub struct coreboot_device {
    pub dev: device,
    pub data: coreboot_device_data,
}

#[repr(C)]
pub union coreboot_device_data {
    pub entry: coreboot_table_entry,
    pub cbmem_ref: lb_cbmem_ref,
    pub cbmem_entry: lb_cbmem_entry,
    pub framebuffer: lb_framebuffer,
    pub raw: [u8; 0],
}

#[inline]
pub unsafe fn dev_to_coreboot_device(dev: *mut device) -> *mut coreboot_device {
    // `dev` is the first member, matching Linux's container_of() use here.
    dev as *mut coreboot_device
}

/* A driver for handling devices described in coreboot tables. */
#[repr(C)]
pub struct coreboot_driver {
    pub probe: Option<unsafe extern "C" fn(*mut coreboot_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut coreboot_device)>,
    pub drv: device_driver,
    pub id_table: *const coreboot_device_id,
}

/* use a macro to avoid include chaining to get THIS_MODULE */
#[macro_export]
macro_rules! coreboot_driver_register {
    ($driver:expr) => {
        $crate::__coreboot_driver_register($driver, THIS_MODULE)
    };
}

/* Register a driver that uses the data from a coreboot table. */
unsafe extern "C" {
    pub fn __coreboot_driver_register(
        driver: *mut coreboot_driver,
        owner: *mut module,
    ) -> i32;
}

/* Unregister a driver that uses the data from a coreboot table. */
unsafe extern "C" {
    pub fn coreboot_driver_unregister(driver: *mut coreboot_driver);
}

/* module_coreboot_driver() - Helper macro for drivers that don't do
 * anything special in module init/exit.  This eliminates a lot of
 * boilerplate.  Each module may only use this macro once, and
 * calling it replaces module_init() and module_exit()
 */
#[macro_export]
macro_rules! module_coreboot_driver {
    ($coreboot_driver:expr) => {
        module_driver!(
            $coreboot_driver,
            coreboot_driver_register,
            coreboot_driver_unregister
        )
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
