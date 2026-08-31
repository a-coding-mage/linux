// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2015 Karol Kosik <karo9@interia.eu>
 *		 2015 Samsung Electronics
 * Author:	 Igor Kotrasinski <i.kotrasinsk@samsung.com>
 *
 * Based on tools/usb/usbip/libsrc/usbip_host_driver.c, which is:
 * Copyright (C) 2011 matt mooney <mfm@muteddisk.com>
 *               2005-2007 Takahiro Hirofuchi
 */

/* C dependencies:
 * <fcntl.h>, <string.h>, <linux/usb/ch9.h>, <unistd.h>
 * "usbip_host_common.h", "usbip_device_driver.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

const PROGNAME: &[u8] = b"libusbip\0";

macro_rules! copy_descr_attr16 {
    ($dev:expr, $descr:expr, $attr:ident) => {
        (*$dev).$attr = le16toh((*$descr).$attr)
    };
}

macro_rules! copy_descr_attr {
    ($dev:expr, $descr:expr, $attr:ident) => {
        (*$dev).$attr = (*$descr).$attr
    };
}

macro_rules! ARRAY_SIZE {
    ($arr:expr) => {
        $arr.len()
    };
}

#[repr(C)]
struct speed_name {
    speed: usb_device_speed,
    name: *const c_char,
}

static speed_names: [speed_name; 7] = [
    speed_name {
        speed: USB_SPEED_UNKNOWN,
        name: b"UNKNOWN\0".as_ptr() as *const c_char,
    },
    speed_name {
        speed: USB_SPEED_LOW,
        name: b"low-speed\0".as_ptr() as *const c_char,
    },
    speed_name {
        speed: USB_SPEED_FULL,
        name: b"full-speed\0".as_ptr() as *const c_char,
    },
    speed_name {
        speed: USB_SPEED_HIGH,
        name: b"high-speed\0".as_ptr() as *const c_char,
    },
    speed_name {
        speed: USB_SPEED_WIRELESS,
        name: b"wireless\0".as_ptr() as *const c_char,
    },
    speed_name {
        speed: USB_SPEED_SUPER,
        name: b"super-speed\0".as_ptr() as *const c_char,
    },
    speed_name {
        speed: USB_SPEED_SUPER_PLUS,
        name: b"super-speed-plus\0".as_ptr() as *const c_char,
    },
];

unsafe extern "C" {
    static mut errno: c_int;

    fn udev_device_get_parent(dev: *mut udev_device) -> *mut udev_device;
    fn udev_device_get_syspath(dev: *mut udev_device) -> *const c_char;
    fn udev_device_get_sysattr_value(
        dev: *mut udev_device,
        sysattr: *const c_char,
    ) -> *const c_char;
    fn udev_device_get_sysname(dev: *mut udev_device) -> *const c_char;
    fn udev_device_get_property_value(
        dev: *mut udev_device,
        key: *const c_char,
    ) -> *const c_char;

    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
    fn fclose(stream: *mut FILE) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;

    fn le16toh(value: u16) -> u16;
    fn err(format: *const c_char, ...) -> c_int;
    fn info(format: *const c_char, ...) -> c_int;

    fn INIT_LIST_HEAD(list: *mut list_head);
    fn usbip_generic_driver_open(hdriver: *mut usbip_host_driver) -> c_int;
    fn usbip_generic_driver_close(hdriver: *mut usbip_host_driver);
    fn usbip_generic_refresh_device_list(hdriver: *mut usbip_host_driver) -> c_int;
    fn usbip_generic_get_device(
        hdriver: *mut usbip_host_driver,
        busid: *const c_char,
    ) -> *mut usbip_exported_device;
}

unsafe extern "C" fn read_usb_vudc_device(
    sdev: *mut udev_device,
    dev: *mut usbip_usb_device,
) -> c_int {
        let mut path: *const c_char;
        let mut name: *const c_char;
        let mut filepath: [c_char; SYSFS_PATH_MAX] = [0; SYSFS_PATH_MAX];
        let mut descr: usb_device_descriptor = core::mem::zeroed();
        let mut i: c_uint;
        let mut fd: *mut FILE = core::ptr::null_mut();
        let mut plat: *mut udev_device;
        let mut speed: *const c_char;
        let mut ret: usize;

        plat = udev_device_get_parent(sdev);
        path = udev_device_get_syspath(plat);
        snprintf(
            filepath.as_mut_ptr(),
            SYSFS_PATH_MAX,
            b"%s/%s\0".as_ptr() as *const c_char,
            path,
            VUDC_DEVICE_DESCR_FILE,
        );
        fd = fopen(filepath.as_ptr(), b"r\0".as_ptr() as *const c_char);
        if fd.is_null() {
            return -1;
        }
        ret = fread(
            &mut descr as *mut usb_device_descriptor as *mut c_void,
            core::mem::size_of_val(&descr),
            1,
            fd,
        );
        if ret != 1 {
            err(
                b"Cannot read vudc device descr file: %s\0".as_ptr() as *const c_char,
                strerror(errno),
            );
            fclose(fd);
            return -1;
        }
        fclose(fd);

        copy_descr_attr!(dev, &mut descr as *mut usb_device_descriptor, bDeviceClass);
        copy_descr_attr!(dev, &mut descr as *mut usb_device_descriptor, bDeviceSubClass);
        copy_descr_attr!(dev, &mut descr as *mut usb_device_descriptor, bDeviceProtocol);
        copy_descr_attr!(dev, &mut descr as *mut usb_device_descriptor, bNumConfigurations);
        copy_descr_attr16!(dev, &mut descr as *mut usb_device_descriptor, idVendor);
        copy_descr_attr16!(dev, &mut descr as *mut usb_device_descriptor, idProduct);
        copy_descr_attr16!(dev, &mut descr as *mut usb_device_descriptor, bcdDevice);

        strncpy((*dev).path.as_mut_ptr(), path, SYSFS_PATH_MAX - 1);
        (*dev).path[SYSFS_PATH_MAX - 1] = b'\0' as c_char;

        (*dev).speed = USB_SPEED_UNKNOWN;
        speed = udev_device_get_sysattr_value(sdev, b"current_speed\0".as_ptr() as *const c_char);
        if !speed.is_null() {
            i = 0;
            while (i as usize) < ARRAY_SIZE!(speed_names) {
                if strcmp(speed_names[i as usize].name, speed) == 0 {
                    (*dev).speed = speed_names[i as usize].speed;
                    break;
                }
                i = i.wrapping_add(1);
            }
        }

        /* Only used for user output, little sense to output them in general */
        (*dev).bNumInterfaces = 0;
        (*dev).bConfigurationValue = 0;
        (*dev).busnum = 0;

        name = udev_device_get_sysname(plat);
        strncpy((*dev).busid.as_mut_ptr(), name, SYSFS_BUS_ID_SIZE - 1);
        (*dev).busid[SYSFS_BUS_ID_SIZE - 1] = b'\0' as c_char;
        0
}

unsafe extern "C" fn is_my_device(dev: *mut udev_device) -> c_int {
        let driver: *const c_char;

        driver = udev_device_get_property_value(dev, b"USB_UDC_NAME\0".as_ptr() as *const c_char);
        (!driver.is_null()
            && strcmp(
                driver,
                USBIP_DEVICE_DRV_NAME.as_ptr() as *const c_char,
            ) == 0) as c_int
}

unsafe extern "C" fn usbip_device_driver_open(hdriver: *mut usbip_host_driver) -> c_int {
        let ret: c_int;

        (*hdriver).ndevs = 0;
        INIT_LIST_HEAD(&mut (*hdriver).edev_list);

        ret = usbip_generic_driver_open(hdriver);
        if ret != 0 || (*hdriver).ndevs == 0 {
            /* C source used adjacent string literals and macro expansion:
             * "please load " USBIP_CORE_MOD_NAME ".ko and "
             * USBIP_DEVICE_DRV_NAME ".ko"
             */
            info(
                b"please load %s.ko and %s.ko\0".as_ptr() as *const c_char,
                USBIP_CORE_MOD_NAME.as_ptr() as *const c_char,
                USBIP_DEVICE_DRV_NAME.as_ptr() as *const c_char,
            );
        }

        ret
}

#[no_mangle]
pub static mut device_driver: usbip_host_driver = usbip_host_driver {
    edev_list: LIST_HEAD_INIT!((*core::ptr::addr_of!(device_driver)).edev_list),
    udev_subsystem: b"udc\0".as_ptr() as *const c_char,
    ops: usbip_host_driver_ops {
        open: Some(usbip_device_driver_open),
        close: Some(usbip_generic_driver_close),
        refresh_device_list: Some(usbip_generic_refresh_device_list),
        get_device: Some(usbip_generic_get_device),
        read_device: Some(read_usb_vudc_device),
        is_my_device: Some(is_my_device),
    },
};
