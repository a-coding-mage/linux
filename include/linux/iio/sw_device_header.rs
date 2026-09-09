/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Industrial I/O software device interface
 *
 * Copyright (c) 2016 Intel Corporation
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/module.h, linux/device.h, linux/iio/iio.h, linux/configfs.h

use core::ffi::c_char;

// `module_iio_sw_device_driver` expands to the kernel's module_driver macro
// with iio_register_sw_device_type and iio_unregister_sw_device_type.

#[repr(C)]
pub struct iio_sw_device_ops {
    pub probe: Option<unsafe extern "C" fn(*const c_char) -> *mut iio_sw_device>,
    pub remove: Option<unsafe extern "C" fn(*mut iio_sw_device) -> i32>,
}

#[repr(C)]
pub struct iio_sw_device_type {
    pub name: *const c_char,
    pub owner: *mut module,
    pub ops: *const iio_sw_device_ops,
    pub list: list_head,
    pub group: *mut config_group,
}

#[repr(C)]
pub struct iio_sw_device {
    pub device: *mut iio_dev,
    pub device_type: *mut iio_sw_device_type,
    pub group: config_group,
}

// Types supplied by the included kernel headers.
#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct config_group {
    _private: [u8; 0],
}

#[repr(C)]
pub struct config_item {
    _private: [u8; 0],
}

#[repr(C)]
pub struct config_item_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iio_dev {
    _private: [u8; 0],
}

extern "C" {
    pub fn iio_register_sw_device_type(dt: *mut iio_sw_device_type) -> i32;
    pub fn iio_unregister_sw_device_type(dt: *mut iio_sw_device_type);

    pub fn iio_sw_device_create(
        type_name: *const c_char,
        name: *const c_char,
    ) -> *mut iio_sw_device;
    pub fn iio_sw_device_destroy(d: *mut iio_sw_device);
}

// Equivalent of the C inline helper. `to_config_group` and `container_of`
// are supplied by the kernel configfs headers.
#[inline]
pub unsafe fn to_iio_sw_device(item: *mut config_item) -> *mut iio_sw_device {
    container_of(to_config_group(item), core::mem::offset_of!(iio_sw_device, group))
}

extern "C" {
    fn to_config_group(item: *mut config_item) -> *mut config_group;
    fn container_of<T>(ptr: *mut config_group, offset: usize) -> *mut T;
}

// When CONFIG_CONFIGFS_FS is enabled, this calls the kernel helper;
// otherwise it has no effect.
#[inline]
pub unsafe fn iio_swd_group_init_type_name(
    d: *mut iio_sw_device,
    name: *const c_char,
    type_: *const config_item_type,
) {
    // CONFIG_CONFIGFS_FS conditional from the original header.
    #[cfg(CONFIG_CONFIGFS_FS)]
    {
        config_group_init_type_name(core::ptr::addr_of_mut!((*d).group), name, type_);
    }
}

extern "C" {
    fn config_group_init_type_name(
        group: *mut config_group,
        name: *const c_char,
        type_: *const config_item_type,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
