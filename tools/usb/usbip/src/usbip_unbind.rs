// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2011 matt mooney <mfm@muteddisk.com>
 *               2005-2007 Takahiro Hirofuchi
 */

// C dependencies translated from:
// <libudev.h>, <errno.h>, <stdio.h>, <string.h>, <getopt.h>,
// "usbip_common.h", "utils.h", "usbip.h", "sysfs_utils.h"

use core::ffi::c_void;
use core::ptr;
use std::os::raw::{c_char, c_int};

#[repr(C)]
pub struct udev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct udev_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

unsafe extern "C" {
    static mut optarg: *mut c_char;

    static SYSFS_PATH_MAX: usize;
    static SYSFS_MNT_PATH: *const c_char;
    static SYSFS_BUS_NAME: *const c_char;
    static SYSFS_DRIVERS_NAME: *const c_char;
    static USBIP_HOST_DRV_NAME: *const c_char;

    fn printf(format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;

    fn udev_new() -> *mut udev;
    fn udev_unref(udev: *mut udev) -> *mut udev;
    fn udev_device_new_from_subsystem_sysname(
        udev: *mut udev,
        subsystem: *const c_char,
        sysname: *const c_char,
    ) -> *mut udev_device;
    fn udev_device_unref(udev_device: *mut udev_device) -> *mut udev_device;
    fn udev_device_get_driver(udev_device: *mut udev_device) -> *const c_char;

    fn write_sysfs_attribute(path: *const c_char, new_value: *const c_char, len: usize) -> c_int;
    fn modify_match_busid(busid: *mut c_char, add: c_int) -> c_int;

    fn err(format: *const c_char, ...);
    fn info(format: *const c_char, ...);
}

const REQUIRED_ARGUMENT: c_int = 1;

// Original C object was:
// static const char usbip_unbind_usage_string[] =
//      "usbip unbind <args>\n"
//      "    -b, --busid=<busid>    Unbind " USBIP_HOST_DRV_NAME ".ko from "
//      "device on <busid>\n";
// USBIP_HOST_DRV_NAME is provided by an included dependency, so the same text is
// emitted in pieces below while preserving its dependency on that external name.

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usbip_unbind_usage() {
    unsafe {
        printf(c"usage: usbip unbind <args>\n    -b, --busid=<busid>    Unbind ".as_ptr());
        printf(USBIP_HOST_DRV_NAME);
        printf(c".ko from device on <busid>\n".as_ptr());
    }
}

unsafe fn unbind_device(busid: *mut c_char) -> c_int {
    let bus_type = c"usb";
    let mut rc: c_int;
    let mut ret: c_int = -1;

    let unbind_attr_name = c"unbind";
    let mut unbind_attr_path = vec![0 as c_char; unsafe { SYSFS_PATH_MAX }];
    let rebind_attr_name = c"rebind";
    let mut rebind_attr_path = vec![0 as c_char; unsafe { SYSFS_PATH_MAX }];

    let udev: *mut udev;
    let dev: *mut udev_device;
    let driver: *const c_char;

    unsafe {
        /* Create libudev context. */
        udev = udev_new();

        /* Check whether the device with this bus ID exists. */
        dev = udev_device_new_from_subsystem_sysname(udev, c"usb".as_ptr(), busid);
        if dev.is_null() {
            err(c"device with the specified bus ID does not exist".as_ptr());
            udev_device_unref(dev);
            udev_unref(udev);
            return ret;
        }

        /* Check whether the device is using usbip-host driver. */
        driver = udev_device_get_driver(dev);
        if driver.is_null() || strcmp(driver, c"usbip-host".as_ptr()) != 0 {
            err(c"device is not bound to usbip-host driver".as_ptr());
            udev_device_unref(dev);
            udev_unref(udev);
            return ret;
        }

        /* Unbind device from driver. */
        snprintf(
            unbind_attr_path.as_mut_ptr(),
            unbind_attr_path.len(),
            c"%s/%s/%s/%s/%s/%s".as_ptr(),
            SYSFS_MNT_PATH,
            SYSFS_BUS_NAME,
            bus_type.as_ptr(),
            SYSFS_DRIVERS_NAME,
            USBIP_HOST_DRV_NAME,
            unbind_attr_name.as_ptr(),
        );

        rc = write_sysfs_attribute(unbind_attr_path.as_ptr(), busid, strlen(busid));
        if rc < 0 {
            err(c"error unbinding device %s from driver".as_ptr(), busid);
            udev_device_unref(dev);
            udev_unref(udev);
            return ret;
        }

        /* Notify driver of unbind. */
        rc = modify_match_busid(busid, 0);
        if rc < 0 {
            err(c"unable to unbind device on %s".as_ptr(), busid);
            udev_device_unref(dev);
            udev_unref(udev);
            return ret;
        }

        /* Trigger new probing. */
        snprintf(
            rebind_attr_path.as_mut_ptr(),
            unbind_attr_path.len(),
            c"%s/%s/%s/%s/%s/%s".as_ptr(),
            SYSFS_MNT_PATH,
            SYSFS_BUS_NAME,
            bus_type.as_ptr(),
            SYSFS_DRIVERS_NAME,
            USBIP_HOST_DRV_NAME,
            rebind_attr_name.as_ptr(),
        );

        rc = write_sysfs_attribute(rebind_attr_path.as_ptr(), busid, strlen(busid));
        if rc < 0 {
            err(c"error rebinding".as_ptr());
            udev_device_unref(dev);
            udev_unref(udev);
            return ret;
        }

        ret = 0;
        info(c"unbind device on busid %s: complete".as_ptr(), busid);

        udev_device_unref(dev);
        udev_unref(udev);
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usbip_unbind(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let opts = [
        option {
            name: c"busid".as_ptr(),
            has_arg: REQUIRED_ARGUMENT,
            flag: ptr::null_mut(),
            val: b'b' as c_int,
        },
        option {
            name: ptr::null(),
            has_arg: 0,
            flag: ptr::null_mut(),
            val: 0,
        },
    ];

    let mut opt: c_int;
    let mut ret: c_int = -1;

    unsafe {
        loop {
            opt = getopt_long(argc, argv, c"b:".as_ptr(), opts.as_ptr(), ptr::null_mut());

            if opt == -1 {
                break;
            }

            match opt {
                x if x == b'b' as c_int => {
                    ret = unbind_device(optarg);
                    return ret;
                }
                _ => {
                    usbip_unbind_usage();
                    return ret;
                }
            }
        }

        usbip_unbind_usage();
    }

    ret
}
