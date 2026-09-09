/* SPDX-License-Identifier: GPL-2.0+ */
/*
 *    Copyright (C) 2006 Benjamin Herrenschmidt, IBM Corp.
 *                       <benh@kernel.crashing.org>
 */

/* Dependencies supplied by the surrounding kernel translation. */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

/* `resource_size_t` is supplied by the translated kernel type definitions. */
pub type resource_size_t = usize;

/**
 * struct of_dev_auxdata - lookup table entry for device names & platform_data
 * @compatible: compatible value of node to match against node
 * @phys_addr: Start address of registers to match against node
 * @name: Name to assign for matching nodes
 * @platform_data: platform_data to assign for matching nodes
 *
 * This lookup table allows the caller of of_platform_populate() to override
 * the names of devices when creating devices from the device tree.  The table
 * should be terminated with an empty entry.  It also allows the platform_data
 * pointer to be set.
 *
 * The reason for this functionality is that some Linux infrastructure uses
 * the device name to look up a specific device, but the Linux-specific names
 * are not encoded into the device tree, so the kernel needs to provide specific
 * values.
 *
 * Note: Using an auxdata lookup table should be considered a last resort when
 * converting a platform to use the DT.  Normally the automatically generated
 * device name will not matter, and drivers should obtain data from the device
 * node instead of from an anonymous platform_data pointer.
 */
#[repr(C)]
pub struct of_dev_auxdata {
    pub compatible: *mut c_char,
    pub phys_addr: resource_size_t,
    pub name: *mut c_char,
    pub platform_data: *mut c_void,
}

/* Macro to simplify populating a lookup table */
#[macro_export]
macro_rules! OF_DEV_AUXDATA {
    ($compat:expr, $phys:expr, $name:expr, $pdata:expr) => {
        $crate::of_dev_auxdata {
            compatible: $compat,
            phys_addr: $phys,
            name: $name,
            platform_data: $pdata,
        }
    };
}

/* Platform drivers register/unregister */
extern "C" {
    pub fn of_device_alloc(
        np: *mut device_node,
        bus_id: *const c_char,
        parent: *mut device,
    ) -> *mut platform_device;

    pub fn of_device_add(pdev: *mut platform_device) -> c_int;
    pub fn of_device_register(ofdev: *mut platform_device) -> c_int;
    pub fn of_device_unregister(ofdev: *mut platform_device);
}

#[cfg(feature = "CONFIG_OF")]
extern "C" {
    pub fn of_find_device_by_node(np: *mut device_node) -> *mut platform_device;
}

#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub unsafe fn of_find_device_by_node(_np: *mut device_node) -> *mut platform_device {
    core::ptr::null_mut()
}

extern "C" {
    pub fn of_platform_bus_probe(
        root: *mut device_node,
        matches: *const of_device_id,
        parent: *mut device,
    ) -> c_int;
}

#[cfg(feature = "CONFIG_OF_ADDRESS")]
extern "C" {
    /* Platform devices and busses creation */
    pub fn of_platform_device_create(
        np: *mut device_node,
        bus_id: *const c_char,
        parent: *mut device,
    ) -> *mut platform_device;

    pub fn of_platform_device_destroy(dev: *mut device, data: *mut c_void) -> c_int;

    pub fn of_platform_populate(
        root: *mut device_node,
        matches: *const of_device_id,
        lookup: *const of_dev_auxdata,
        parent: *mut device,
    ) -> c_int;
    pub fn of_platform_default_populate(
        root: *mut device_node,
        lookup: *const of_dev_auxdata,
        parent: *mut device,
    ) -> c_int;
    pub fn of_platform_depopulate(parent: *mut device);

    pub fn devm_of_platform_populate(dev: *mut device) -> c_int;

    pub fn devm_of_platform_depopulate(dev: *mut device);
}

#[cfg(not(feature = "CONFIG_OF_ADDRESS"))]
mod without_config_of_address {
    use super::*;

    /* Platform devices and busses creation */
    #[inline]
    pub unsafe fn of_platform_device_create(
        _np: *mut device_node,
        _bus_id: *const c_char,
        _parent: *mut device,
    ) -> *mut platform_device {
        core::ptr::null_mut()
    }

    #[inline]
    pub unsafe fn of_platform_device_destroy(_dev: *mut device, _data: *mut c_void) -> c_int {
        -19
    }

    #[inline]
    pub unsafe fn of_platform_populate(
        _root: *mut device_node,
        _matches: *const of_device_id,
        _lookup: *const of_dev_auxdata,
        _parent: *mut device,
    ) -> c_int {
        -19
    }

    #[inline]
    pub unsafe fn of_platform_default_populate(
        _root: *mut device_node,
        _lookup: *const of_dev_auxdata,
        _parent: *mut device,
    ) -> c_int {
        -19
    }

    #[inline]
    pub unsafe fn of_platform_depopulate(_parent: *mut device) {}

    #[inline]
    pub unsafe fn devm_of_platform_populate(_dev: *mut device) -> c_int {
        -19
    }

    #[inline]
    pub unsafe fn devm_of_platform_depopulate(_dev: *mut device) {}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
