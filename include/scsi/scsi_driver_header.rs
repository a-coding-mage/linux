/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <linux/blk_types.h>, <linux/device.h>, and
// <scsi/scsi_cmnd.h>; those declarations are supplied by other dependencies.

use core::ffi::c_int;

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct request {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub driver: *mut device_driver,
}

#[repr(C)]
pub struct class_interface {
    _private: [u8; 0],
}

pub type blk_status_t = u8;

#[repr(C)]
pub struct scsi_device {
    pub sdev_gendev: device,
}

#[repr(C)]
pub struct scsi_cmnd {
    pub device: *mut scsi_device,
}

#[repr(C)]
pub struct scsi_driver {
    pub gendrv: device_driver,

    pub probe: Option<unsafe extern "C" fn(*mut scsi_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut scsi_device)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut scsi_device)>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub rescan: Option<unsafe extern "C" fn(*mut device)>,
    pub init_command: Option<unsafe extern "C" fn(*mut scsi_cmnd) -> blk_status_t>,
    pub uninit_command: Option<unsafe extern "C" fn(*mut scsi_cmnd)>,
    pub done: Option<unsafe extern "C" fn(*mut scsi_cmnd) -> c_int>,
    pub eh_action: Option<unsafe extern "C" fn(*mut scsi_cmnd, c_int) -> c_int>,
    pub eh_reset: Option<unsafe extern "C" fn(*mut scsi_cmnd)>,
}

#[inline]
pub unsafe fn to_scsi_driver(drv: *mut device_driver) -> *mut scsi_driver {
    // container_of((drv), struct scsi_driver, gendrv)
    (drv as *mut u8).sub(core::mem::offset_of!(scsi_driver, gendrv)) as *mut scsi_driver
}

extern "C" {
    pub fn __scsi_register_driver(
        driver: *mut scsi_driver,
        owner: *mut module,
    ) -> c_int;
}

#[inline]
pub unsafe fn scsi_register_driver(drv: *mut scsi_driver) -> c_int {
    __scsi_register_driver(drv, THIS_MODULE)
}

#[inline]
pub unsafe fn scsi_unregister_driver(drv: *mut scsi_driver) {
    driver_unregister(&mut (*drv).gendrv);
}

extern "C" {
    pub fn scsi_register_interface(intf: *mut class_interface) -> c_int;
    pub fn class_interface_unregister(intf: *mut class_interface);
    pub fn driver_unregister(driver: *mut device_driver);
}

#[inline]
pub unsafe fn scsi_unregister_interface(intf: *mut class_interface) {
    class_interface_unregister(intf)
}

/* make sure not to use it with passthrough commands */
#[inline]
pub unsafe fn scsi_cmd_to_driver(cmd: *mut scsi_cmnd) -> *mut scsi_driver {
    to_scsi_driver((*(*cmd).device).sdev_gendev.driver)
}

// Build-time symbol supplied by the kernel module environment.
unsafe extern "C" {
    pub static mut THIS_MODULE: *mut module;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
