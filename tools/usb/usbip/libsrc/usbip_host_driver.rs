// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2011 matt mooney <mfm@muteddisk.com>
 *               2005-2007 Takahiro Hirofuchi
 * Copyright (C) 2015-2016 Samsung Electronics
 *               Igor Kotrasinski <i.kotrasinsk@samsung.com>
 *               Krzysztof Opasiak <k.opasiak@samsung.com>
 */

use core::ffi::{c_char, c_int};
use core::ptr;

/*
 * C dependencies:
 *   <unistd.h>
 *   <libudev.h>
 *   "usbip_host_common.h"
 *   "usbip_host_driver.h"
 *
 * #undef  PROGNAME
 * #define PROGNAME "libusbip"
 */
pub const PROGNAME: &[u8; 9] = b"libusbip\0";

#[repr(C)]
pub struct udev_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usbip_exported_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usb_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usb_interface {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct usbip_host_driver {
    pub edev_list: list_head,
    pub udev_subsystem: *const c_char,
    pub ndevs: c_int,
    pub ops: usbip_host_driver_ops,
}

#[repr(C)]
pub struct usbip_host_driver_ops {
    pub open: Option<unsafe extern "C" fn(*mut usbip_host_driver) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut usbip_host_driver)>,
    pub refresh_device_list: Option<unsafe extern "C" fn(*mut usbip_host_driver) -> c_int>,
    pub get_device:
        Option<unsafe extern "C" fn(*mut usbip_host_driver, *const c_char) -> *mut usbip_exported_device>,
    pub read_device: Option<unsafe extern "C" fn(*mut udev_device, *mut usb_device) -> c_int>,
    pub read_interface: Option<unsafe extern "C" fn(*mut udev_device, *mut usb_interface) -> c_int>,
    pub is_my_device: Option<unsafe extern "C" fn(*mut udev_device) -> c_int>,
}

unsafe extern "C" {
    static USBIP_HOST_DRV_NAME: *const c_char;
    static USBIP_CORE_MOD_NAME: *const c_char;

    fn udev_device_get_driver(dev: *mut udev_device) -> *const c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn INIT_LIST_HEAD(list: *mut list_head);

    fn usbip_generic_driver_open(hdriver: *mut usbip_host_driver) -> c_int;
    fn usbip_generic_driver_close(hdriver: *mut usbip_host_driver);
    fn usbip_generic_refresh_device_list(hdriver: *mut usbip_host_driver) -> c_int;
    fn usbip_generic_get_device(
        hdriver: *mut usbip_host_driver,
        busid: *const c_char,
    ) -> *mut usbip_exported_device;
    fn read_usb_device(dev: *mut udev_device, udev: *mut usb_device) -> c_int;
    fn read_usb_interface(dev: *mut udev_device, uinf: *mut usb_interface) -> c_int;

    fn info(fmt: *const c_char, ...);
}

unsafe extern "C" fn is_my_device(dev: *mut udev_device) -> c_int {
    let driver: *const c_char;

    driver = unsafe { udev_device_get_driver(dev) };
    (driver != ptr::null() && unsafe { strcmp(driver, USBIP_HOST_DRV_NAME) } == 0) as c_int
}

unsafe extern "C" fn usbip_host_driver_open(hdriver: *mut usbip_host_driver) -> c_int {
    let ret: c_int;

    unsafe {
        (*hdriver).ndevs = 0;
        INIT_LIST_HEAD(core::ptr::addr_of_mut!((*hdriver).edev_list));

        ret = usbip_generic_driver_open(hdriver);
        if ret != 0 || (*hdriver).ndevs == 0 {
            /*
             * C source:
             * info("please load " USBIP_CORE_MOD_NAME ".ko and "
             *      USBIP_HOST_DRV_NAME ".ko");
             *
             * The module-name tokens are header-provided string macros in C.
             */
            info(
                b"please load %s.ko and %s.ko\0".as_ptr() as *const c_char,
                USBIP_CORE_MOD_NAME,
                USBIP_HOST_DRV_NAME,
            );
        }
    }

    ret
}

#[unsafe(no_mangle)]
pub static mut host_driver: usbip_host_driver = usbip_host_driver {
    /*
     * C source uses LIST_HEAD_INIT(host_driver.edev_list), a self-referential
     * static initializer supplied by Linux list macros. The list is explicitly
     * initialized again in usbip_host_driver_open().
     */
    edev_list: list_head {
        next: ptr::null_mut(),
        prev: ptr::null_mut(),
    },
    udev_subsystem: b"usb\0".as_ptr() as *const c_char,
    ndevs: 0,
    ops: usbip_host_driver_ops {
        open: Some(usbip_host_driver_open),
        close: Some(usbip_generic_driver_close),
        refresh_device_list: Some(usbip_generic_refresh_device_list),
        get_device: Some(usbip_generic_get_device),
        read_device: Some(read_usb_device),
        read_interface: Some(read_usb_interface),
        is_my_device: Some(is_my_device),
    },
};
