/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Mediated device definition
 *
 * Copyright (c) 2016, NVIDIA CORPORATION. All rights reserved.
 *     Author: Neo Jia <cjia@nvidia.com>
 *             Kirti Wankhede <kwankhede@nvidia.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
pub struct mdev_type {
    /* set by the driver before calling mdev_register parent: */
    pub sysfs_name: *const ::core::ffi::c_char,
    pub pretty_name: *const ::core::ffi::c_char,

    /* set by the core, can be used drivers */
    pub parent: *mut mdev_parent,

    /* internal only */
    pub kobj: kobject,
    pub devices_kobj: *mut kobject,
}

#[repr(C)]
pub struct mdev_device {
    pub dev: device,
    pub uuid: guid_t,
    pub next: list_head,
    pub type_: *mut mdev_type,
    pub active: bool,
}

/* embedded into the struct device that the mdev devices hang off */
#[repr(C)]
pub struct mdev_parent {
    pub dev: *mut device,
    pub mdev_driver: *mut mdev_driver,
    pub mdev_types_kset: *mut kset,
    /* Synchronize device creation/removal with parent unregistration */
    pub unreg_sem: rw_semaphore,
    pub types: *mut *mut mdev_type,
    pub nr_types: ::core::ffi::c_uint,
    pub available_instances: atomic_t,
}

/// struct mdev_driver - Mediated device driver
/// @device_api: string to return for the device_api sysfs
/// @max_instances: maximum number of instances supported (optional)
/// @probe: called when new device created
/// @remove: called when device removed
/// @get_available: Return the max number of instances that can be created
/// @show_description: Print a description of the mtype
/// @driver: device driver structure
#[repr(C)]
pub struct mdev_driver {
    pub device_api: *const ::core::ffi::c_char,
    pub max_instances: ::core::ffi::c_uint,
    pub probe: Option<unsafe extern "C" fn(dev: *mut mdev_device) -> ::core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(dev: *mut mdev_device)>,
    pub get_available:
        Option<unsafe extern "C" fn(mtype: *mut mdev_type) -> ::core::ffi::c_uint>,
    pub show_description: Option<
        unsafe extern "C" fn(
            mtype: *mut mdev_type,
            buf: *mut ::core::ffi::c_char,
        ) -> ssize_t,
    >,
    pub driver: device_driver,
}

pub type ssize_t = isize;

extern "C" {
    pub fn mdev_register_parent(
        parent: *mut mdev_parent,
        dev: *mut device,
        mdev_driver: *mut mdev_driver,
        types: *mut *mut mdev_type,
        nr_types: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn mdev_unregister_parent(parent: *mut mdev_parent);

    pub fn mdev_register_driver(drv: *mut mdev_driver) -> ::core::ffi::c_int;
    pub fn mdev_unregister_driver(drv: *mut mdev_driver);
}

pub unsafe fn to_mdev_device(dev: *mut device) -> *mut mdev_device {
    // Equivalent to Linux's container_of(dev, struct mdev_device, dev).
    (dev as *mut u8).sub(::core::mem::offset_of!(mdev_device, dev)) as *mut mdev_device
}

pub unsafe fn mdev_dev(mdev: *mut mdev_device) -> *mut device {
    &mut (*mdev).dev
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
