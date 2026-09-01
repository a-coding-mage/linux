// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2021 Intel Corporation. All rights reserved. */

// C dependencies:
// linux/platform_device.h
// linux/device.h
// linux/acpi.h
// cxl.h
// test/mock.h

use core::ffi::{c_char, c_int, c_void};
use core::mem::MaybeUninit;
use core::ptr;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_device {
    pub dev: device,
    pub handle: *mut c_void,
}

#[repr(C)]
pub struct cxl_mock_ops {
    pub is_mock_bridge: Option<unsafe extern "C" fn(dev: *mut device) -> bool>,
}

unsafe extern "C" {
    fn get_cxl_mock_ops(index: *mut c_int) -> *mut cxl_mock_ops;
    fn put_cxl_mock_ops(index: c_int);

    fn ACPI_COMPANION(dev: *mut device) -> *mut acpi_device;
    fn dev_is_platform(dev: *mut device) -> bool;
    fn to_acpi_device(dev: *mut device) -> *mut acpi_device;
    fn acpi_pci_find_root(handle: *mut c_void) -> *mut c_void;
    fn acpi_device_hid(adev: *mut acpi_device) -> *const c_char;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn to_cxl_host_bridge(
    host: *mut device,
    dev: *mut device,
) -> *mut acpi_device {
    let mut index = MaybeUninit::<c_int>::uninit();
    let mut found: *mut acpi_device = ptr::null_mut();
    let ops = unsafe { get_cxl_mock_ops(index.as_mut_ptr()) };

    if !ops.is_null()
        && unsafe {
            ((*ops).is_mock_bridge.expect("is_mock_bridge"))(dev)
        }
    {
        found = unsafe { ACPI_COMPANION(dev) };
        unsafe { put_cxl_mock_ops(index.assume_init()) };
        return found;
    }

    if unsafe { dev_is_platform(dev) } {
        unsafe { put_cxl_mock_ops(index.assume_init()) };
        return found;
    }

    let adev = unsafe { to_acpi_device(dev) };
    if unsafe { acpi_pci_find_root((*adev).handle) }.is_null() {
        unsafe { put_cxl_mock_ops(index.assume_init()) };
        return found;
    }

    if unsafe { strcmp(acpi_device_hid(adev), b"ACPI0016\0".as_ptr() as *const c_char) } == 0 {
        found = adev;
        unsafe {
            dev_dbg(
                host,
                b"found host bridge %s\n\0".as_ptr() as *const c_char,
                dev_name(&mut (*adev).dev),
            );
        }
    }

    unsafe { put_cxl_mock_ops(index.assume_init()) };
    found
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
