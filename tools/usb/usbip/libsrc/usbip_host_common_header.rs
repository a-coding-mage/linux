/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2015-2016 Samsung Electronics
 *               Igor Kotrasinski <i.kotrasinsk@samsung.com>
 *               Krzysztof Opasiak <k.opasiak@samsung.com>
 *
 * Refactored from usbip_host_driver.c, which is:
 * Copyright (C) 2011 matt mooney <mfm@muteddisk.com>
 *               2005-2007 Takahiro Hirofuchi
 */

// C header dependencies:
// <stdint.h>, <libudev.h>, <errno.h>, "list.h", "usbip_common.h",
// and "sysfs_utils.h".

pub const EOPNOTSUPP: ::std::os::raw::c_int = 95;

#[repr(C)]
pub struct udev_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usbip_usb_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usbip_usb_interface {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usbip_host_driver_ops {
    pub open: Option<
        unsafe extern "C" fn(hdriver: *mut usbip_host_driver) -> ::std::os::raw::c_int,
    >,
    pub close: Option<unsafe extern "C" fn(hdriver: *mut usbip_host_driver)>,
    pub refresh_device_list: Option<
        unsafe extern "C" fn(hdriver: *mut usbip_host_driver) -> ::std::os::raw::c_int,
    >,
    pub get_device: Option<
        unsafe extern "C" fn(
            hdriver: *mut usbip_host_driver,
            num: ::std::os::raw::c_int,
        ) -> *mut usbip_exported_device,
    >,

    pub read_device: Option<
        unsafe extern "C" fn(
            sdev: *mut udev_device,
            dev: *mut usbip_usb_device,
        ) -> ::std::os::raw::c_int,
    >,
    pub read_interface: Option<
        unsafe extern "C" fn(
            udev: *mut usbip_usb_device,
            i: ::std::os::raw::c_int,
            uinf: *mut usbip_usb_interface,
        ) -> ::std::os::raw::c_int,
    >,
    pub is_my_device:
        Option<unsafe extern "C" fn(udev: *mut udev_device) -> ::std::os::raw::c_int>,
}

#[repr(C)]
pub struct usbip_host_driver {
    pub ndevs: ::std::os::raw::c_int,
    /* list of exported device */
    pub edev_list: list_head,
    pub udev_subsystem: *const ::std::os::raw::c_char,
    pub ops: usbip_host_driver_ops,
}

#[repr(C)]
pub struct usbip_exported_device {
    pub sudev: *mut udev_device,
    pub status: i32,
    pub udev: usbip_usb_device,
    pub node: list_head,
    pub uinf: [usbip_usb_interface; 0],
}

/* External API to access the driver */
#[inline]
pub unsafe fn usbip_driver_open(hdriver: *mut usbip_host_driver) -> ::std::os::raw::c_int {
    if unsafe { (*hdriver).ops.open.is_none() } {
        return -EOPNOTSUPP;
    }
    unsafe { ((*hdriver).ops.open.unwrap_unchecked())(hdriver) }
}

#[inline]
pub unsafe fn usbip_driver_close(hdriver: *mut usbip_host_driver) {
    if unsafe { (*hdriver).ops.close.is_none() } {
        return;
    }
    unsafe { ((*hdriver).ops.close.unwrap_unchecked())(hdriver) };
}

#[inline]
pub unsafe fn usbip_refresh_device_list(
    hdriver: *mut usbip_host_driver,
) -> ::std::os::raw::c_int {
    if unsafe { (*hdriver).ops.refresh_device_list.is_none() } {
        return -EOPNOTSUPP;
    }
    unsafe { ((*hdriver).ops.refresh_device_list.unwrap_unchecked())(hdriver) }
}

#[inline]
pub unsafe fn usbip_get_device(
    hdriver: *mut usbip_host_driver,
    num: ::std::os::raw::c_int,
) -> *mut usbip_exported_device {
    if unsafe { (*hdriver).ops.get_device.is_none() } {
        return ::std::ptr::null_mut();
    }
    unsafe { ((*hdriver).ops.get_device.unwrap_unchecked())(hdriver, num) }
}

unsafe extern "C" {
    /* Helper functions for implementing driver backend */
    pub fn usbip_generic_driver_open(
        hdriver: *mut usbip_host_driver,
    ) -> ::std::os::raw::c_int;
    pub fn usbip_generic_driver_close(hdriver: *mut usbip_host_driver);
    pub fn usbip_generic_refresh_device_list(
        hdriver: *mut usbip_host_driver,
    ) -> ::std::os::raw::c_int;
    pub fn usbip_export_device(
        edev: *mut usbip_exported_device,
        sockfd: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn usbip_generic_get_device(
        hdriver: *mut usbip_host_driver,
        num: ::std::os::raw::c_int,
    ) -> *mut usbip_exported_device;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
