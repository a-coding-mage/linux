// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2011 matt mooney <mfm@muteddisk.com>
 *               2005-2007 Takahiro Hirofuchi
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Dependencies supplied by the original C includes:
// <libudev.h>, <errno.h>, <stdio.h>, <stdlib.h>, <string.h>, <getopt.h>,
// "usbip_common.h", "utils.h", "usbip.h", and "sysfs_utils.h".

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

const REQUIRED_ARGUMENT: c_int = 1;

// Header-provided constants/macros expected from the surrounding repository.
const SYSFS_PATH_MAX: usize = 4096;

unsafe extern "C" {
    static mut errno: c_int;
    static mut optarg: *mut c_char;

    static SYSFS_MNT_PATH: [c_char; 0];
    static SYSFS_BUS_NAME: [c_char; 0];
    static SYSFS_BUS_TYPE: [c_char; 0];
    static SYSFS_DRIVERS_NAME: [c_char; 0];
    static USBIP_HOST_DRV_NAME: [c_char; 0];
    static USBIP_VHCI_DRV_NAME: [c_char; 0];

    fn printf(format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
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
    fn udev_device_get_sysattr_value(
        udev_device: *mut udev_device,
        sysattr: *const c_char,
    ) -> *const c_char;
    fn udev_device_get_driver(udev_device: *mut udev_device) -> *const c_char;
    fn udev_device_get_devpath(udev_device: *mut udev_device) -> *const c_char;

    fn write_sysfs_attribute(path: *const c_char, value: *const c_char, len: usize) -> c_int;
    fn modify_match_busid(busid: *mut c_char, add: c_int) -> c_int;

    fn err(format: *const c_char, ...);
    fn dbg(format: *const c_char, ...);
    fn info(format: *const c_char, ...);
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum unbind_status {
    UNBIND_ST_OK,
    UNBIND_ST_USBIP_HOST,
    UNBIND_ST_FAILED,
}

static USBIP_BIND_USAGE_STRING: &[u8] = b"usbip bind <args>\n    -b, --busid=<busid>    Bind usbip-host.ko to device on <busid>\n\0";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usbip_bind_usage() {
    unsafe {
        printf(
            b"usage: %s\0".as_ptr() as *const c_char,
            USBIP_BIND_USAGE_STRING.as_ptr() as *const c_char,
        );
    }
}

/* call at unbound state */
unsafe fn bind_usbip(busid: *mut c_char) -> c_int {
    let attr_name = b"bind\0";
    let mut bind_attr_path = [0 as c_char; SYSFS_PATH_MAX];
    let mut rc: c_int = -1;

    unsafe {
        snprintf(
            bind_attr_path.as_mut_ptr(),
            bind_attr_path.len(),
            b"%s/%s/%s/%s/%s/%s\0".as_ptr() as *const c_char,
            SYSFS_MNT_PATH.as_ptr(),
            SYSFS_BUS_NAME.as_ptr(),
            SYSFS_BUS_TYPE.as_ptr(),
            SYSFS_DRIVERS_NAME.as_ptr(),
            USBIP_HOST_DRV_NAME.as_ptr(),
            attr_name.as_ptr() as *const c_char,
        );

        rc = write_sysfs_attribute(bind_attr_path.as_ptr(), busid, strlen(busid));
        if rc < 0 {
            err(
                b"error binding device %s to driver: %s\0".as_ptr() as *const c_char,
                busid,
                strerror(errno),
            );
            return -1;
        }
    }

    0
}

/* buggy driver may cause dead lock */
unsafe fn unbind_other(busid: *mut c_char) -> c_int {
    let mut status = unbind_status::UNBIND_ST_OK;

    let attr_name = b"unbind\0";
    let mut unbind_attr_path = [0 as c_char; SYSFS_PATH_MAX];
    let mut rc: c_int = -1;

    let udev: *mut udev;
    let dev: *mut udev_device;
    let driver: *const c_char;
    let b_dev_class: *const c_char;

    unsafe {
        /* Create libudev context. */
        udev = udev_new();

        /* Get the device. */
        dev = udev_device_new_from_subsystem_sysname(udev, b"usb\0".as_ptr() as *const c_char, busid);
        if dev.is_null() {
            dbg(b"unable to find device with bus ID %s\0".as_ptr() as *const c_char, busid);
            status = unbind_status::UNBIND_ST_FAILED;
        } else {
            /* Check what kind of device it is. */
            b_dev_class = udev_device_get_sysattr_value(
                dev,
                b"bDeviceClass\0".as_ptr() as *const c_char,
            );
            if b_dev_class.is_null() {
                dbg(b"unable to get bDevClass device attribute\0".as_ptr() as *const c_char);
                status = unbind_status::UNBIND_ST_FAILED;
            } else if strncmp(
                b_dev_class,
                b"09\0".as_ptr() as *const c_char,
                strlen(b_dev_class),
            ) == 0
            {
                dbg(b"skip unbinding of hub\0".as_ptr() as *const c_char);
                status = unbind_status::UNBIND_ST_FAILED;
            } else {
                /* Get the device driver. */
                driver = udev_device_get_driver(dev);
                if !driver.is_null() {
                    if strncmp(
                        USBIP_HOST_DRV_NAME.as_ptr(),
                        driver,
                        strlen(USBIP_HOST_DRV_NAME.as_ptr()),
                    ) == 0
                    {
                        /* Already bound to usbip-host. */
                        status = unbind_status::UNBIND_ST_USBIP_HOST;
                    } else {
                        /* Unbind device from driver. */
                        snprintf(
                            unbind_attr_path.as_mut_ptr(),
                            unbind_attr_path.len(),
                            b"%s/%s/%s/%s/%s/%s\0".as_ptr() as *const c_char,
                            SYSFS_MNT_PATH.as_ptr(),
                            SYSFS_BUS_NAME.as_ptr(),
                            SYSFS_BUS_TYPE.as_ptr(),
                            SYSFS_DRIVERS_NAME.as_ptr(),
                            driver,
                            attr_name.as_ptr() as *const c_char,
                        );

                        rc = write_sysfs_attribute(unbind_attr_path.as_ptr(), busid, strlen(busid));
                        if rc < 0 {
                            err(
                                b"error unbinding device %s from driver\0".as_ptr() as *const c_char,
                                busid,
                            );
                            status = unbind_status::UNBIND_ST_FAILED;
                        }
                    }
                }
            }
        }

        udev_device_unref(dev);
        udev_unref(udev);
    }

    status as c_int
}

unsafe fn bind_device(busid: *mut c_char) -> c_int {
    let mut rc: c_int;
    let udev: *mut udev;
    let dev: *mut udev_device;
    let devpath: *const c_char;

    unsafe {
        /* Check whether the device with this bus ID exists. */
        udev = udev_new();
        dev = udev_device_new_from_subsystem_sysname(udev, b"usb\0".as_ptr() as *const c_char, busid);
        if dev.is_null() {
            err(b"device with the specified bus ID does not exist\0".as_ptr() as *const c_char);
            return -1;
        }
        devpath = udev_device_get_devpath(dev);
        udev_unref(udev);

        /* If the device is already attached to vhci_hcd - bail out */
        if !strstr(devpath, USBIP_VHCI_DRV_NAME.as_ptr()).is_null() {
            err(
                b"bind loop detected: device: %s is attached to %s\n\0".as_ptr() as *const c_char,
                devpath,
                USBIP_VHCI_DRV_NAME.as_ptr(),
            );
            return -1;
        }

        rc = unbind_other(busid);
        if rc == unbind_status::UNBIND_ST_FAILED as c_int {
            err(
                b"could not unbind driver from device on busid %s\0".as_ptr() as *const c_char,
                busid,
            );
            return -1;
        } else if rc == unbind_status::UNBIND_ST_USBIP_HOST as c_int {
            err(
                b"device on busid %s is already bound to %s\0".as_ptr() as *const c_char,
                busid,
                USBIP_HOST_DRV_NAME.as_ptr(),
            );
            return -1;
        }

        rc = modify_match_busid(busid, 1);
        if rc < 0 {
            err(b"unable to bind device on %s\0".as_ptr() as *const c_char, busid);
            return -1;
        }

        rc = bind_usbip(busid);
        if rc < 0 {
            err(
                b"could not bind device to %s\0".as_ptr() as *const c_char,
                USBIP_HOST_DRV_NAME.as_ptr(),
            );
            modify_match_busid(busid, 0);
            return -1;
        }

        info(b"bind device on busid %s: complete\0".as_ptr() as *const c_char, busid);
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usbip_bind(argc: c_int, argv: *mut *mut c_char) -> c_int {
    static OPTS: [option; 2] = [
        option {
            name: b"busid\0".as_ptr() as *const c_char,
            has_arg: REQUIRED_ARGUMENT,
            flag: core::ptr::null_mut(),
            val: b'b' as c_int,
        },
        option {
            name: core::ptr::null(),
            has_arg: 0,
            flag: core::ptr::null_mut(),
            val: 0,
        },
    ];

    let mut opt: c_int;
    let mut ret: c_int = -1;

    unsafe {
        loop {
            opt = getopt_long(
                argc,
                argv,
                b"b:\0".as_ptr() as *const c_char,
                OPTS.as_ptr(),
                core::ptr::null_mut(),
            );

            if opt == -1 {
                break;
            }

            match opt {
                x if x == b'b' as c_int => {
                    ret = bind_device(optarg);
                    return ret;
                }
                _ => {
                    break;
                }
            }
        }

        usbip_bind_usage();
    }

    ret
}
