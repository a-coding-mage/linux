// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation */

// Translated from the Linux kernel implementation.  Types, constants, macros,
// and functions supplied by the included headers are external dependencies.

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

extern "C" {
    fn adf_devmgr_pci_to_accel_dev(dev: *mut pci_dev) -> *mut adf_accel_dev;
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn device_add_group(dev: *mut device, group: *const attribute_group) -> c_int;
    fn device_remove_group(dev: *mut device, group: *const attribute_group);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn GET_DEV(accel_dev: *mut adf_accel_dev) -> device;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_attribute {
    pub attr: attribute,
}

#[repr(C)]
pub struct attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute_group {
    pub name: *const c_char,
    pub attrs: *mut *mut attribute,
}

#[repr(C)]
pub struct adf_ras_errors {
    pub enabled: bool,
    pub sysfs_added: bool,
    _private: [u8; 0],
}

#[repr(C)]
pub struct adf_accel_dev {
    pub ras_errors: adf_ras_errors,
}

extern "C" {
    fn ADF_RAS_ERR_CTR_READ(errors: adf_ras_errors, counter: c_int) -> c_int;
    fn ADF_RAS_ERR_CTR_CLEAR(errors: adf_ras_errors);
}

extern "C" {
    static ADF_RAS_CORR: c_int;
    static ADF_RAS_UNCORR: c_int;
    static ADF_RAS_FATAL: c_int;
}

// DEVICE_ATTR_RO(errors_correctable), DEVICE_ATTR_RO(errors_nonfatal),
// DEVICE_ATTR_RO(errors_fatal), and DEVICE_ATTR_WO(reset_error_counters)
// generate the following kernel attribute objects and callbacks.

unsafe extern "C" {
    static mut dev_attr_errors_correctable: device_attribute;
    static mut dev_attr_errors_nonfatal: device_attribute;
    static mut dev_attr_errors_fatal: device_attribute;
    static mut dev_attr_reset_error_counters: device_attribute;
}

unsafe fn errors_correctable_show(
    dev: *mut device,
    _dev_attr: *mut device_attribute,
    buf: *mut c_char,
) -> isize {
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() {
        return -22;
    }

    let counter = ADF_RAS_ERR_CTR_READ((*accel_dev).ras_errors, ADF_RAS_CORR as c_int);
    sysfs_emit(buf, b"%d\0".as_ptr() as *const c_char, counter)
}

unsafe fn errors_nonfatal_show(
    dev: *mut device,
    _dev_attr: *mut device_attribute,
    buf: *mut c_char,
) -> isize {
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() {
        return -22;
    }

    let counter = ADF_RAS_ERR_CTR_READ((*accel_dev).ras_errors, ADF_RAS_UNCORR as c_int);
    sysfs_emit(buf, b"%d\0".as_ptr() as *const c_char, counter)
}

unsafe fn errors_fatal_show(
    dev: *mut device,
    _dev_attr: *mut device_attribute,
    buf: *mut c_char,
) -> isize {
    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() {
        return -22;
    }

    let counter = ADF_RAS_ERR_CTR_READ((*accel_dev).ras_errors, ADF_RAS_FATAL as c_int);
    sysfs_emit(buf, b"%d\0".as_ptr() as *const c_char, counter)
}

unsafe fn reset_error_counters_store(
    dev: *mut device,
    _dev_attr: *mut device_attribute,
    buf: *const c_char,
    count: usize,
) -> isize {
    if *buf != b'1' as c_char || count != 2 {
        return -22;
    }

    let accel_dev = adf_devmgr_pci_to_accel_dev(to_pci_dev(dev));
    if accel_dev.is_null() {
        return -22;
    }

    ADF_RAS_ERR_CTR_CLEAR((*accel_dev).ras_errors);
    count as isize
}

static mut qat_ras_attrs: [*mut attribute; 5] = [
    unsafe { &raw mut dev_attr_errors_correctable.attr },
    unsafe { &raw mut dev_attr_errors_nonfatal.attr },
    unsafe { &raw mut dev_attr_errors_fatal.attr },
    unsafe { &raw mut dev_attr_reset_error_counters.attr },
    ptr::null_mut(),
];

static mut qat_ras_group: attribute_group = attribute_group {
    name: b"qat_ras\0".as_ptr() as *const c_char,
    attrs: unsafe { qat_ras_attrs.as_mut_ptr() },
};

pub unsafe fn adf_sysfs_start_ras(accel_dev: *mut adf_accel_dev) {
    if !(*accel_dev).ras_errors.enabled {
        return;
    }

    ADF_RAS_ERR_CTR_CLEAR((*accel_dev).ras_errors);

    let mut dev = GET_DEV(accel_dev);
    if device_add_group(&mut dev, &qat_ras_group) != 0 {
        dev_err(&mut dev, b"Failed to create qat_ras attribute group.\n\0".as_ptr() as *const c_char);
    }

    (*accel_dev).ras_errors.sysfs_added = true;
}

pub unsafe fn adf_sysfs_stop_ras(accel_dev: *mut adf_accel_dev) {
    if !(*accel_dev).ras_errors.enabled {
        return;
    }

    if (*accel_dev).ras_errors.sysfs_added {
        let mut dev = GET_DEV(accel_dev);
        device_remove_group(&mut dev, &qat_ras_group);
        (*accel_dev).ras_errors.sysfs_added = false;
    }

    ADF_RAS_ERR_CTR_CLEAR((*accel_dev).ras_errors);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
