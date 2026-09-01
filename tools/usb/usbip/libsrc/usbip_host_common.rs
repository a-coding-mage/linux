// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2015-2016 Samsung Electronics
 *               Igor Kotrasinski <i.kotrasinsk@samsung.com>
 *               Krzysztof Opasiak <k.opasiak@samsung.com>
 *
 * Refactored from usbip_host_driver.c, which is:
 * Copyright (C) 2011 matt mooney <mfm@muteddisk.com>
 *               2005-2007 Takahiro Hirofuchi
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const O_RDONLY: c_int = 0;

unsafe extern "C" {
    static mut udev_context: *mut udev;

    fn snprintf(str: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;

    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn udev_new() -> *mut udev;
    fn udev_unref(udev: *mut udev) -> *mut udev;
    fn udev_device_new_from_syspath(udev: *mut udev, syspath: *const c_char) -> *mut udev_device;
    fn udev_device_unref(udev_device: *mut udev_device) -> *mut udev_device;
    fn udev_enumerate_new(udev: *mut udev) -> *mut udev_enumerate;
    fn udev_enumerate_add_match_subsystem(
        udev_enumerate: *mut udev_enumerate,
        subsystem: *const c_char,
    ) -> c_int;
    fn udev_enumerate_scan_devices(udev_enumerate: *mut udev_enumerate) -> c_int;
    fn udev_enumerate_get_list_entry(
        udev_enumerate: *mut udev_enumerate,
    ) -> *mut udev_list_entry;
    fn udev_list_entry_get_next(list_entry: *mut udev_list_entry) -> *mut udev_list_entry;
    fn udev_list_entry_get_name(list_entry: *mut udev_list_entry) -> *const c_char;

    fn write_sysfs_attribute(path: *const c_char, new_value: *const c_char, len: usize) -> c_int;

    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn INIT_LIST_HEAD(list: *mut list_head);

    fn err(fmt: *const c_char, ...);
    fn dbg(fmt: *const c_char, ...);
    fn info(fmt: *const c_char, ...);
}

#[repr(C)]
pub struct udev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct udev_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct udev_enumerate {
    _private: [u8; 0],
}

#[repr(C)]
pub struct udev_list_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct usbip_usb_device {
    pub path: [c_char; SYSFS_PATH_MAX],
    pub busid: [c_char; SYSFS_PATH_MAX],
    pub bNumInterfaces: c_int,
}

#[repr(C)]
pub struct usbip_usb_interface {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usbip_host_driver_ops {
    pub read_device:
        unsafe extern "C" fn(sudev: *mut udev_device, udev: *mut usbip_usb_device) -> c_int,
    pub read_interface: Option<
        unsafe extern "C" fn(
            udev: *mut usbip_usb_device,
            i: c_int,
            uinf: *mut usbip_usb_interface,
        ),
    >,
    pub is_my_device: unsafe extern "C" fn(dev: *mut udev_device) -> c_int,
}

#[repr(C)]
pub struct usbip_host_driver {
    pub ops: usbip_host_driver_ops,
    pub udev_subsystem: *const c_char,
    pub edev_list: list_head,
    pub ndevs: c_int,
}

#[repr(C)]
pub struct usbip_exported_device {
    pub node: list_head,
    pub sudev: *mut udev_device,
    pub udev: usbip_usb_device,
    pub status: c_int,
    pub uinf: [usbip_usb_interface; 0],
}

const SYSFS_PATH_MAX: usize = 256;
const SDEV_ST_AVAILABLE: c_int = 0x01;
const SDEV_ST_USED: c_int = 0x02;
const SDEV_ST_ERROR: c_int = 0x03;
const ST_DEV_ERR: c_int = -1;
const ST_DEV_BUSY: c_int = -2;

unsafe fn list_entry_usbip_exported_device(ptr: *mut list_head) -> *mut usbip_exported_device {
    ptr as *mut usbip_exported_device
}

unsafe fn read_attr_usbip_status(udev: *mut usbip_usb_device) -> i32 {
    let mut status_attr_path = [0 as c_char; SYSFS_PATH_MAX];
    let mut size: c_int;
    let fd: c_int;
    let length: c_int;
    let mut status = [0 as c_char; 2];
    let mut value: c_int = 0;

    size = snprintf(
        status_attr_path.as_mut_ptr(),
        size_of_val(&status_attr_path),
        c"%s/usbip_status".as_ptr(),
        (*udev).path.as_ptr(),
    );
    if size < 0 || size as c_uint >= size_of_val(&status_attr_path) as c_uint {
        err(
            c"usbip_status path length %i >= %lu or < 0".as_ptr(),
            size,
            size_of_val(&status_attr_path) as c_long,
        );
        return -1;
    }

    fd = open(status_attr_path.as_ptr(), O_RDONLY);
    if fd < 0 {
        err(c"error opening attribute %s".as_ptr(), status_attr_path.as_ptr());
        return -1;
    }

    length = read(fd, status.as_mut_ptr() as *mut c_void, 1) as c_int;
    if length < 0 {
        err(c"error reading attribute %s".as_ptr(), status_attr_path.as_ptr());
        close(fd);
        return -1;
    }

    value = atoi(status.as_ptr());
    close(fd);
    value
}

unsafe fn usbip_exported_device_new(
    hdriver: *mut usbip_host_driver,
    sdevpath: *const c_char,
) -> *mut usbip_exported_device {
    let mut edev: *mut usbip_exported_device = ptr::null_mut();
    let edev_old: *mut usbip_exported_device;
    let size: usize;
    let mut i: c_int;

    edev = calloc(1, size_of::<usbip_exported_device>()) as *mut usbip_exported_device;

    (*edev).sudev = udev_device_new_from_syspath(udev_context, sdevpath);
    if (*edev).sudev.is_null() {
        err(c"udev_device_new_from_syspath: %s".as_ptr(), sdevpath);
        goto_err(edev);
        return ptr::null_mut();
    }

    if ((*hdriver).ops.read_device)((*edev).sudev, &mut (*edev).udev) < 0 {
        goto_err(edev);
        return ptr::null_mut();
    }

    (*edev).status = read_attr_usbip_status(&mut (*edev).udev);
    if (*edev).status < 0 {
        goto_err(edev);
        return ptr::null_mut();
    }

    /* reallocate buffer to include usb interface data */
    size = size_of::<usbip_exported_device>()
        + ((*edev).udev.bNumInterfaces as usize) * size_of::<usbip_usb_interface>();

    edev_old = edev;
    edev = realloc(edev as *mut c_void, size) as *mut usbip_exported_device;
    if edev.is_null() {
        edev = edev_old;
        dbg(c"realloc failed".as_ptr());
        goto_err(edev);
        return ptr::null_mut();
    }

    i = 0;
    while i < (*edev).udev.bNumInterfaces {
        /* vudc does not support reading interfaces */
        if (*hdriver).ops.read_interface.is_none() {
            break;
        }
        ((*hdriver).ops.read_interface.unwrap())(
            &mut (*edev).udev,
            i,
            (*edev).uinf.as_mut_ptr().offset(i as isize),
        );
        i += 1;
    }

    edev
}

unsafe fn goto_err(edev: *mut usbip_exported_device) {
    if !(*edev).sudev.is_null() {
        udev_device_unref((*edev).sudev);
    }
    if !edev.is_null() {
        free(edev as *mut c_void);
    }
}

unsafe fn refresh_exported_devices(hdriver: *mut usbip_host_driver) -> c_int {
    let mut edev: *mut usbip_exported_device;
    let enumerate: *mut udev_enumerate;
    let devices: *mut udev_list_entry;
    let mut dev_list_entry: *mut udev_list_entry;
    let dev: *mut udev_device;
    let path: *const c_char;

    enumerate = udev_enumerate_new(udev_context);
    udev_enumerate_add_match_subsystem(enumerate, (*hdriver).udev_subsystem);
    udev_enumerate_scan_devices(enumerate);

    devices = udev_enumerate_get_list_entry(enumerate);

    dev_list_entry = devices;
    while !dev_list_entry.is_null() {
        path = udev_list_entry_get_name(dev_list_entry);
        dev = udev_device_new_from_syspath(udev_context, path);
        if dev.is_null() {
            dev_list_entry = udev_list_entry_get_next(dev_list_entry);
            continue;
        }

        /* Check whether device uses usbip driver. */
        if ((*hdriver).ops.is_my_device)(dev) != 0 {
            edev = usbip_exported_device_new(hdriver, path);
            if edev.is_null() {
                dbg(c"usbip_exported_device_new failed".as_ptr());
                dev_list_entry = udev_list_entry_get_next(dev_list_entry);
                continue;
            }

            list_add(&mut (*edev).node, &mut (*hdriver).edev_list);
            (*hdriver).ndevs += 1;
        }
        dev_list_entry = udev_list_entry_get_next(dev_list_entry);
    }

    if (*hdriver).ndevs == 0 {
        info(c"Please load appropriate modules or export devices.".as_ptr());
    }

    0
}

unsafe fn usbip_exported_device_destroy(devs: *mut list_head) {
    let mut i: *mut list_head;
    let mut tmp: *mut list_head;
    let edev: *mut usbip_exported_device;

    i = (*devs).next;
    while i != devs {
        tmp = (*i).next;
        edev = list_entry_usbip_exported_device(i);
        list_del(i);
        free(edev as *mut c_void);
        i = tmp;
    }
}

#[no_mangle]
pub unsafe extern "C" fn usbip_generic_driver_open(hdriver: *mut usbip_host_driver) -> c_int {
    let rc: c_int;

    udev_context = udev_new();
    if udev_context.is_null() {
        err(c"udev_new failed".as_ptr());
        return -1;
    }

    rc = refresh_exported_devices(hdriver);
    if rc < 0 {
        udev_unref(udev_context);
        return -1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn usbip_generic_driver_close(hdriver: *mut usbip_host_driver) {
    if hdriver.is_null() {
        return;
    }

    usbip_exported_device_destroy(&mut (*hdriver).edev_list);

    udev_unref(udev_context);
}

#[no_mangle]
pub unsafe extern "C" fn usbip_generic_refresh_device_list(
    hdriver: *mut usbip_host_driver,
) -> c_int {
    let rc: c_int;

    usbip_exported_device_destroy(&mut (*hdriver).edev_list);

    (*hdriver).ndevs = 0;
    INIT_LIST_HEAD(&mut (*hdriver).edev_list);

    rc = refresh_exported_devices(hdriver);
    if rc < 0 {
        return -1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn usbip_export_device(
    edev: *mut usbip_exported_device,
    sockfd: c_int,
) -> c_int {
    let attr_name = *b"usbip_sockfd\0";
    let mut sockfd_attr_path = [0 as c_char; SYSFS_PATH_MAX];
    let mut size: c_int;
    let mut sockfd_buff = [0 as c_char; 30];
    let ret: c_int;

    if (*edev).status != SDEV_ST_AVAILABLE {
        dbg(c"device not available: %s".as_ptr(), (*edev).udev.busid.as_ptr());
        match (*edev).status {
            SDEV_ST_ERROR => {
                dbg(c"status SDEV_ST_ERROR".as_ptr());
                return ST_DEV_ERR;
            }
            SDEV_ST_USED => {
                dbg(c"status SDEV_ST_USED".as_ptr());
                return ST_DEV_BUSY;
            }
            _ => {
                dbg(c"status unknown: 0x%x".as_ptr(), (*edev).status);
                return -1;
            }
        }
    }

    /* only the first interface is true */
    size = snprintf(
        sockfd_attr_path.as_mut_ptr(),
        size_of_val(&sockfd_attr_path),
        c"%s/%s".as_ptr(),
        (*edev).udev.path.as_ptr(),
        attr_name.as_ptr(),
    );
    if size < 0 || size as c_uint >= size_of_val(&sockfd_attr_path) as c_uint {
        err(
            c"exported device path length %i >= %lu or < 0".as_ptr(),
            size,
            size_of_val(&sockfd_attr_path) as c_long,
        );
        return -1;
    }

    size = snprintf(
        sockfd_buff.as_mut_ptr(),
        size_of_val(&sockfd_buff),
        c"%d\n".as_ptr(),
        sockfd,
    );
    if size < 0 || size as c_uint >= size_of_val(&sockfd_buff) as c_uint {
        err(
            c"socket length %i >= %lu or < 0".as_ptr(),
            size,
            size_of_val(&sockfd_buff) as c_long,
        );
        return -1;
    }

    ret = write_sysfs_attribute(
        sockfd_attr_path.as_ptr(),
        sockfd_buff.as_ptr(),
        strlen(sockfd_buff.as_ptr()),
    );
    if ret < 0 {
        err(
            c"write_sysfs_attribute failed: sockfd %s to %s".as_ptr(),
            sockfd_buff.as_ptr(),
            sockfd_attr_path.as_ptr(),
        );
        return ret;
    }

    info(c"connect: %s".as_ptr(), (*edev).udev.busid.as_ptr());

    ret
}

#[no_mangle]
pub unsafe extern "C" fn usbip_generic_get_device(
    hdriver: *mut usbip_host_driver,
    num: c_int,
) -> *mut usbip_exported_device {
    let mut i: *mut list_head;
    let edev: *mut usbip_exported_device;
    let mut cnt: c_int = 0;

    i = (*hdriver).edev_list.next;
    while i != &mut (*hdriver).edev_list {
        edev = list_entry_usbip_exported_device(i);
        if num == cnt {
            return edev;
        }
        cnt += 1;
        i = (*i).next;
    }

    ptr::null_mut()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
