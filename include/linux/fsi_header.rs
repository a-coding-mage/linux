/* SPDX-License-Identifier: GPL-2.0-only */
/* FSI device & driver interfaces
 *
 * Copyright (C) IBM Corporation 2016
 */

use core::ffi::c_void;
use core::mem::offset_of;

pub type u8 = core::primitive::u8;
pub type uint32_t = core::primitive::u32;
pub type size_t = usize;
pub type dev_t = usize;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fsi_slave {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fsi_device {
    pub dev: device,
    pub engine_type: u8,
    pub version: u8,
    pub unit: u8,
    pub slave: *mut fsi_slave,
    pub addr: uint32_t,
    pub size: uint32_t,
}

extern "C" {
    pub fn dev_get_drvdata(dev: *const device) -> *mut c_void;
    pub fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
}

#[inline]
pub unsafe fn fsi_get_drvdata(fsi_dev: *mut fsi_device) -> *mut c_void {
    dev_get_drvdata(&(*fsi_dev).dev as *const device)
}

#[inline]
pub unsafe fn fsi_set_drvdata(fsi_dev: *mut fsi_device, data: *mut c_void) {
    dev_set_drvdata(&mut (*fsi_dev).dev as *mut device, data)
}

extern "C" {
    pub fn fsi_device_read(dev: *mut fsi_device, addr: uint32_t, val: *mut c_void, size: size_t) -> i32;
    pub fn fsi_device_write(dev: *mut fsi_device, addr: uint32_t, val: *const c_void, size: size_t) -> i32;
    pub fn fsi_device_peek(dev: *mut fsi_device, val: *mut c_void) -> i32;
}

#[repr(C)]
pub struct fsi_device_id {
    pub engine_type: u8,
    pub version: u8,
}

pub const FSI_VERSION_ANY: u8 = 0;

/* FSI_DEVICE(t): struct fsi_device_id initializer with engine_type = t and version = FSI_VERSION_ANY. */
/* FSI_DEVICE_VERSIONED(t, v): struct fsi_device_id initializer with engine_type = t and version = v. */

#[repr(C)]
pub struct fsi_driver {
    pub probe: Option<unsafe extern "C" fn(fsidev: *mut fsi_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(fsidev: *mut fsi_device)>,
    pub drv: device_driver,
    pub id_table: *const fsi_device_id,
}

#[inline]
pub unsafe fn to_fsi_dev(devp: *mut device) -> *mut fsi_device {
    (devp as *mut u8).sub(offset_of!(fsi_device, dev)) as *mut fsi_device
}

#[inline]
pub unsafe fn to_fsi_drv(drvp: *const device_driver) -> *const fsi_driver {
    (drvp as *const u8).sub(offset_of!(fsi_driver, drv)) as *const fsi_driver
}

extern "C" {
    pub fn fsi_driver_register(fsi_drv: *mut fsi_driver) -> i32;
    pub fn fsi_driver_unregister(fsi_drv: *mut fsi_driver);
}

/* module_fsi_driver(__fsi_driver) expands to module_driver(__fsi_driver,
 * fsi_driver_register, fsi_driver_unregister). */

/* direct slave API */
extern "C" {
    pub fn fsi_slave_claim_range(slave: *mut fsi_slave, addr: uint32_t, size: uint32_t) -> i32;
    pub fn fsi_slave_release_range(slave: *mut fsi_slave, addr: uint32_t, size: uint32_t);
    pub fn fsi_slave_read(slave: *mut fsi_slave, addr: uint32_t, val: *mut c_void, size: size_t) -> i32;
    pub fn fsi_slave_write(slave: *mut fsi_slave, addr: uint32_t, val: *const c_void, size: size_t) -> i32;
    pub static fsi_cdev_type: device_type;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum fsi_dev_type {
    fsi_dev_cfam,
    fsi_dev_sbefifo,
    fsi_dev_scom,
    fsi_dev_occ,
}

extern "C" {
    pub fn fsi_get_new_minor(
        fdev: *mut fsi_device,
        type_: fsi_dev_type,
        out_dev: *mut dev_t,
        out_index: *mut i32,
    ) -> i32;
    pub fn fsi_free_minor(dev: dev_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
