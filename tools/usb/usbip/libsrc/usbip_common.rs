// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2005-2007 Takahiro Hirofuchi
 */

// Translated from usbip_common.c. C includes:
// <libudev.h>, "usbip_common.h", "names.h"

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const PROGNAME: &[u8] = b"libusbip\0";

pub static mut usbip_use_syslog: c_int = 0;
pub static mut usbip_use_stderr: c_int = 0;
pub static mut usbip_use_debug: c_int = 0;

#[repr(C)]
pub struct udev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct udev_device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut udev_context: *mut udev;

    fn udev_device_get_sysattr_value(
        dev: *mut udev_device,
        sysattr: *const c_char,
    ) -> *const c_char;
    fn udev_device_get_syspath(dev: *mut udev_device) -> *const c_char;
    fn udev_device_get_sysname(dev: *mut udev_device) -> *const c_char;
    fn udev_device_new_from_subsystem_sysname(
        udev: *mut udev,
        subsystem: *const c_char,
        sysname: *const c_char,
    ) -> *mut udev_device;

    fn names_init(f: *mut c_char) -> c_int;
    fn names_free();
    fn names_product(vendor: u16, product: u16) -> *const c_char;
    fn names_vendor(vendor: u16) -> *const c_char;
    fn names_protocol(class: u8, subclass: u8, protocol: u8) -> *const c_char;
    fn names_subclass(class: u8, subclass: u8) -> *const c_char;
    fn names_class(class: u8) -> *const c_char;

    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;

    fn dbg(format: *const c_char, ...);
    fn err(format: *const c_char, ...);
}

unsafe extern "C" {
    static USB_SPEED_UNKNOWN: c_int;
    static USB_SPEED_LOW: c_int;
    static USB_SPEED_FULL: c_int;
    static USB_SPEED_HIGH: c_int;
    static USB_SPEED_WIRELESS: c_int;
    static USB_SPEED_SUPER: c_int;
    static USB_SPEED_SUPER_PLUS: c_int;

    static SDEV_ST_AVAILABLE: c_int;
    static SDEV_ST_USED: c_int;
    static SDEV_ST_ERROR: c_int;
    static VDEV_ST_NULL: c_int;
    static VDEV_ST_NOTASSIGNED: c_int;
    static VDEV_ST_USED: c_int;
    static VDEV_ST_ERROR: c_int;

    static ST_OK: c_int;
    static ST_NA: c_int;
    static ST_DEV_BUSY: c_int;
    static ST_DEV_ERR: c_int;
    static ST_NODEV: c_int;
    static ST_ERROR: c_int;
}

unsafe extern "C" {
    static SYSFS_PATH_MAX: usize;
    static SYSFS_BUS_ID_SIZE: usize;
}

#[repr(C)]
pub struct usbip_usb_interface {
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,
}

#[repr(C)]
pub struct usbip_usb_device {
    pub path: *mut c_char,
    pub busid: *mut c_char,
    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice: u16,
    pub bConfigurationValue: u8,
    pub bNumConfigurations: u8,
    pub bNumInterfaces: u8,
    pub devnum: u8,
    pub speed: c_int,
    pub busnum: u32,
}

#[repr(C)]
struct speed_string {
    num: c_int,
    speed: *const c_char,
    desc: *const c_char,
}

#[repr(C)]
struct portst_string {
    num: c_int,
    desc: *const c_char,
}

#[repr(C)]
struct op_common_status_string {
    num: c_int,
    desc: *const c_char,
}

unsafe impl Sync for speed_string {}
unsafe impl Sync for portst_string {}
unsafe impl Sync for op_common_status_string {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usbip_status_string(status: i32) -> *const c_char {
    let portst_strings = [
        portst_string { num: SDEV_ST_AVAILABLE, desc: b"Device Available\0".as_ptr() as *const c_char },
        portst_string { num: SDEV_ST_USED, desc: b"Device in Use\0".as_ptr() as *const c_char },
        portst_string { num: SDEV_ST_ERROR, desc: b"Device Error\0".as_ptr() as *const c_char },
        portst_string { num: VDEV_ST_NULL, desc: b"Port Available\0".as_ptr() as *const c_char },
        portst_string { num: VDEV_ST_NOTASSIGNED, desc: b"Port Initializing\0".as_ptr() as *const c_char },
        portst_string { num: VDEV_ST_USED, desc: b"Port in Use\0".as_ptr() as *const c_char },
        portst_string { num: VDEV_ST_ERROR, desc: b"Port Error\0".as_ptr() as *const c_char },
        portst_string { num: 0, desc: core::ptr::null() },
    ];

    let mut i = 0usize;
    while !portst_strings[i].desc.is_null() {
        if portst_strings[i].num == status {
            return portst_strings[i].desc;
        }
        i += 1;
    }

    b"Unknown Status\0".as_ptr() as *const c_char
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usbip_speed_string(num: c_int) -> *const c_char {
    let speed_strings = [
        speed_string { num: USB_SPEED_UNKNOWN, speed: b"unknown\0".as_ptr() as *const c_char, desc: b"Unknown Speed\0".as_ptr() as *const c_char },
        speed_string { num: USB_SPEED_LOW, speed: b"1.5\0".as_ptr() as *const c_char, desc: b"Low Speed(1.5Mbps)\0".as_ptr() as *const c_char },
        speed_string { num: USB_SPEED_FULL, speed: b"12\0".as_ptr() as *const c_char, desc: b"Full Speed(12Mbps)\0".as_ptr() as *const c_char },
        speed_string { num: USB_SPEED_HIGH, speed: b"480\0".as_ptr() as *const c_char, desc: b"High Speed(480Mbps)\0".as_ptr() as *const c_char },
        speed_string { num: USB_SPEED_WIRELESS, speed: b"53.3-480\0".as_ptr() as *const c_char, desc: b"Wireless\0".as_ptr() as *const c_char },
        speed_string { num: USB_SPEED_SUPER, speed: b"5000\0".as_ptr() as *const c_char, desc: b"Super Speed(5000Mbps)\0".as_ptr() as *const c_char },
        speed_string { num: USB_SPEED_SUPER_PLUS, speed: b"10000\0".as_ptr() as *const c_char, desc: b"Super Speed Plus(10000Mbps)\0".as_ptr() as *const c_char },
        speed_string { num: USB_SPEED_SUPER_PLUS, speed: b"20000\0".as_ptr() as *const c_char, desc: b"Super Speed Plus(20000Mbps)\0".as_ptr() as *const c_char },
        speed_string { num: 0, speed: core::ptr::null(), desc: core::ptr::null() },
    ];

    let mut i = 0usize;
    while !speed_strings[i].speed.is_null() {
        if speed_strings[i].num == num {
            return speed_strings[i].desc;
        }
        i += 1;
    }

    b"Unknown Speed\0".as_ptr() as *const c_char
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usbip_op_common_status_string(status: c_int) -> *const c_char {
    let op_common_status_strings = [
        op_common_status_string { num: ST_OK, desc: b"Request Completed Successfully\0".as_ptr() as *const c_char },
        op_common_status_string { num: ST_NA, desc: b"Request Failed\0".as_ptr() as *const c_char },
        op_common_status_string { num: ST_DEV_BUSY, desc: b"Device busy (exported)\0".as_ptr() as *const c_char },
        op_common_status_string { num: ST_DEV_ERR, desc: b"Device in error state\0".as_ptr() as *const c_char },
        op_common_status_string { num: ST_NODEV, desc: b"Device not found\0".as_ptr() as *const c_char },
        op_common_status_string { num: ST_ERROR, desc: b"Unexpected response\0".as_ptr() as *const c_char },
        op_common_status_string { num: 0, desc: core::ptr::null() },
    ];

    let mut i = 0usize;
    while !op_common_status_strings[i].desc.is_null() {
        if op_common_status_strings[i].num == status {
            return op_common_status_strings[i].desc;
        }
        i += 1;
    }

    b"Unknown Op Common Status\0".as_ptr() as *const c_char
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_usb_interface(uinf: *mut usbip_usb_interface) {
    let mut buff = [0 as c_char; 100];

    usbip_names_get_class(
        buff.as_mut_ptr(),
        buff.len(),
        (*uinf).bInterfaceClass,
        (*uinf).bInterfaceSubClass,
        (*uinf).bInterfaceProtocol,
    );
    dbg(b"%-20s = %s\0".as_ptr() as *const c_char, b"Interface(C/SC/P)\0".as_ptr() as *const c_char, buff.as_ptr());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_usb_device(udev: *mut usbip_usb_device) {
    let mut buff = [0 as c_char; 100];

    dbg(b"%-20s = %s\0".as_ptr() as *const c_char, b"path\0".as_ptr() as *const c_char, (*udev).path);
    dbg(b"%-20s = %s\0".as_ptr() as *const c_char, b"busid\0".as_ptr() as *const c_char, (*udev).busid);

    usbip_names_get_class(
        buff.as_mut_ptr(),
        buff.len(),
        (*udev).bDeviceClass,
        (*udev).bDeviceSubClass,
        (*udev).bDeviceProtocol,
    );
    dbg(b"%-20s = %s\0".as_ptr() as *const c_char, b"Device(C/SC/P)\0".as_ptr() as *const c_char, buff.as_ptr());

    dbg(b"%-20s = %x\0".as_ptr() as *const c_char, b"bcdDevice\0".as_ptr() as *const c_char, (*udev).bcdDevice as c_int);

    usbip_names_get_product(buff.as_mut_ptr(), buff.len(), (*udev).idVendor, (*udev).idProduct);
    dbg(b"%-20s = %s\0".as_ptr() as *const c_char, b"Vendor/Product\0".as_ptr() as *const c_char, buff.as_ptr());

    dbg(b"%-20s = %x\0".as_ptr() as *const c_char, b"bNumConfigurations\0".as_ptr() as *const c_char, (*udev).bNumConfigurations as c_int);
    dbg(b"%-20s = %x\0".as_ptr() as *const c_char, b"bNumInterfaces\0".as_ptr() as *const c_char, (*udev).bNumInterfaces as c_int);

    dbg(b"%-20s = %s\0".as_ptr() as *const c_char, b"speed\0".as_ptr() as *const c_char, usbip_speed_string((*udev).speed));

    dbg(b"%-20s = %x\0".as_ptr() as *const c_char, b"busnum\0".as_ptr() as *const c_char, (*udev).busnum as c_int);
    dbg(b"%-20s = %x\0".as_ptr() as *const c_char, b"devnum\0".as_ptr() as *const c_char, (*udev).devnum as c_int);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_attr_value(
    dev: *mut udev_device,
    name: *const c_char,
    format: *const c_char,
) -> c_int {
    let attr: *const c_char;
    let mut num: c_int = 0;
    let ret: c_int;

    attr = udev_device_get_sysattr_value(dev, name);
    if attr.is_null() {
        err(b"udev_device_get_sysattr_value failed\0".as_ptr() as *const c_char);
        return num;
    }

    /*
     * The client chooses the device configuration
     * when attaching it so right after being bound
     * to usbip-host on the server the device will
     * have no configuration.
     * Therefore, attributes such as bConfigurationValue
     * and bNumInterfaces will not exist and sscanf will
     * fail. Check for these cases and don't treat them
     * as errors.
     */

    ret = sscanf(attr, format, &mut num as *mut c_int);
    if ret < 1 {
        if strcmp(name, b"bConfigurationValue\0".as_ptr() as *const c_char) != 0
            && strcmp(name, b"bNumInterfaces\0".as_ptr() as *const c_char) != 0
        {
            err(b"sscanf failed for attribute %s\0".as_ptr() as *const c_char, name);
            return num;
        }
    }

    num
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_attr_speed(dev: *mut udev_device) -> c_int {
    let speed: *const c_char;

    speed = udev_device_get_sysattr_value(dev, b"speed\0".as_ptr() as *const c_char);
    if speed.is_null() {
        err(b"udev_device_get_sysattr_value failed\0".as_ptr() as *const c_char);
        return USB_SPEED_UNKNOWN;
    }

    let speed_strings = [
        speed_string { num: USB_SPEED_UNKNOWN, speed: b"unknown\0".as_ptr() as *const c_char, desc: b"Unknown Speed\0".as_ptr() as *const c_char },
        speed_string { num: USB_SPEED_LOW, speed: b"1.5\0".as_ptr() as *const c_char, desc: b"Low Speed(1.5Mbps)\0".as_ptr() as *const c_char },
        speed_string { num: USB_SPEED_FULL, speed: b"12\0".as_ptr() as *const c_char, desc: b"Full Speed(12Mbps)\0".as_ptr() as *const c_char },
        speed_string { num: USB_SPEED_HIGH, speed: b"480\0".as_ptr() as *const c_char, desc: b"High Speed(480Mbps)\0".as_ptr() as *const c_char },
        speed_string { num: USB_SPEED_WIRELESS, speed: b"53.3-480\0".as_ptr() as *const c_char, desc: b"Wireless\0".as_ptr() as *const c_char },
        speed_string { num: USB_SPEED_SUPER, speed: b"5000\0".as_ptr() as *const c_char, desc: b"Super Speed(5000Mbps)\0".as_ptr() as *const c_char },
        speed_string { num: USB_SPEED_SUPER_PLUS, speed: b"10000\0".as_ptr() as *const c_char, desc: b"Super Speed Plus(10000Mbps)\0".as_ptr() as *const c_char },
        speed_string { num: USB_SPEED_SUPER_PLUS, speed: b"20000\0".as_ptr() as *const c_char, desc: b"Super Speed Plus(20000Mbps)\0".as_ptr() as *const c_char },
        speed_string { num: 0, speed: core::ptr::null(), desc: core::ptr::null() },
    ];

    let mut i = 0usize;
    while !speed_strings[i].speed.is_null() {
        if strcmp(speed, speed_strings[i].speed) == 0 {
            return speed_strings[i].num;
        }
        i += 1;
    }

    USB_SPEED_UNKNOWN
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_usb_device(
    sdev: *mut udev_device,
    udev: *mut usbip_usb_device,
) -> c_int {
    let mut busnum: u32 = 0;
    let mut devnum: u32 = 0;
    let path: *const c_char;
    let name: *const c_char;

    (*udev).bDeviceClass = read_attr_value(sdev, b"bDeviceClass\0".as_ptr() as *const c_char, b"%02x\n\0".as_ptr() as *const c_char) as u8;
    (*udev).bDeviceSubClass = read_attr_value(sdev, b"bDeviceSubClass\0".as_ptr() as *const c_char, b"%02x\n\0".as_ptr() as *const c_char) as u8;
    (*udev).bDeviceProtocol = read_attr_value(sdev, b"bDeviceProtocol\0".as_ptr() as *const c_char, b"%02x\n\0".as_ptr() as *const c_char) as u8;

    (*udev).idVendor = read_attr_value(sdev, b"idVendor\0".as_ptr() as *const c_char, b"%04x\n\0".as_ptr() as *const c_char) as u16;
    (*udev).idProduct = read_attr_value(sdev, b"idProduct\0".as_ptr() as *const c_char, b"%04x\n\0".as_ptr() as *const c_char) as u16;
    (*udev).bcdDevice = read_attr_value(sdev, b"bcdDevice\0".as_ptr() as *const c_char, b"%04x\n\0".as_ptr() as *const c_char) as u16;

    (*udev).bConfigurationValue = read_attr_value(sdev, b"bConfigurationValue\0".as_ptr() as *const c_char, b"%02x\n\0".as_ptr() as *const c_char) as u8;
    (*udev).bNumConfigurations = read_attr_value(sdev, b"bNumConfigurations\0".as_ptr() as *const c_char, b"%02x\n\0".as_ptr() as *const c_char) as u8;
    (*udev).bNumInterfaces = read_attr_value(sdev, b"bNumInterfaces\0".as_ptr() as *const c_char, b"%02x\n\0".as_ptr() as *const c_char) as u8;

    (*udev).devnum = read_attr_value(sdev, b"devnum\0".as_ptr() as *const c_char, b"%d\n\0".as_ptr() as *const c_char) as u8;
    (*udev).speed = read_attr_speed(sdev);

    path = udev_device_get_syspath(sdev);
    name = udev_device_get_sysname(sdev);

    strncpy((*udev).path, path, SYSFS_PATH_MAX - 1);
    *((*udev).path.add(SYSFS_PATH_MAX - 1)) = b'\0' as c_char;
    strncpy((*udev).busid, name, SYSFS_BUS_ID_SIZE - 1);
    *((*udev).busid.add(SYSFS_BUS_ID_SIZE - 1)) = b'\0' as c_char;

    sscanf(
        name,
        b"%u-%u\0".as_ptr() as *const c_char,
        &mut busnum as *mut u32,
        &mut devnum as *mut u32,
    );
    (*udev).busnum = busnum;

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_usb_interface(
    udev: *mut usbip_usb_device,
    i: c_int,
    uinf: *mut usbip_usb_interface,
) -> c_int {
    let mut busid = [0 as c_char; 256];
    let size: c_int;
    let sif: *mut udev_device;

    size = snprintf(
        busid.as_mut_ptr(),
        busid.len(),
        b"%s:%d.%d\0".as_ptr() as *const c_char,
        (*udev).busid,
        (*udev).bConfigurationValue as c_int,
        i,
    );
    if size < 0 || size as c_uint >= busid.len() as c_uint {
        err(
            b"busid length %i >= %lu or < 0\0".as_ptr() as *const c_char,
            size,
            busid.len() as c_ulong,
        );
        return -1;
    }

    sif = udev_device_new_from_subsystem_sysname(
        udev_context,
        b"usb\0".as_ptr() as *const c_char,
        busid.as_ptr(),
    );
    if sif.is_null() {
        err(
            b"udev_device_new_from_subsystem_sysname %s failed\0".as_ptr() as *const c_char,
            busid.as_ptr(),
        );
        return -1;
    }

    (*uinf).bInterfaceClass = read_attr_value(sif, b"bInterfaceClass\0".as_ptr() as *const c_char, b"%02x\n\0".as_ptr() as *const c_char) as u8;
    (*uinf).bInterfaceSubClass = read_attr_value(sif, b"bInterfaceSubClass\0".as_ptr() as *const c_char, b"%02x\n\0".as_ptr() as *const c_char) as u8;
    (*uinf).bInterfaceProtocol = read_attr_value(sif, b"bInterfaceProtocol\0".as_ptr() as *const c_char, b"%02x\n\0".as_ptr() as *const c_char) as u8;

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usbip_names_init(f: *mut c_char) -> c_int {
    names_init(f)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usbip_names_free() {
    names_free();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usbip_names_get_product(
    buff: *mut c_char,
    size: usize,
    vendor: u16,
    product: u16,
) {
    let mut prod: *const c_char;
    let mut vend: *const c_char;

    prod = names_product(vendor, product);
    if prod.is_null() {
        prod = b"unknown product\0".as_ptr() as *const c_char;
    }

    vend = names_vendor(vendor);
    if vend.is_null() {
        vend = b"unknown vendor\0".as_ptr() as *const c_char;
    }

    snprintf(
        buff,
        size,
        b"%s : %s (%04x:%04x)\0".as_ptr() as *const c_char,
        vend,
        prod,
        vendor as c_int,
        product as c_int,
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usbip_names_get_class(
    buff: *mut c_char,
    size: usize,
    class: u8,
    subclass: u8,
    protocol: u8,
) {
    let mut c: *const c_char;
    let mut s: *const c_char;
    let mut p: *const c_char;

    if class == 0 && subclass == 0 && protocol == 0 {
        snprintf(
            buff,
            size,
            b"(Defined at Interface level) (%02x/%02x/%02x)\0".as_ptr() as *const c_char,
            class as c_int,
            subclass as c_int,
            protocol as c_int,
        );
        return;
    }

    p = names_protocol(class, subclass, protocol);
    if p.is_null() {
        p = b"unknown protocol\0".as_ptr() as *const c_char;
    }

    s = names_subclass(class, subclass);
    if s.is_null() {
        s = b"unknown subclass\0".as_ptr() as *const c_char;
    }

    c = names_class(class);
    if c.is_null() {
        c = b"unknown class\0".as_ptr() as *const c_char;
    }

    snprintf(
        buff,
        size,
        b"%s / %s / %s (%02x/%02x/%02x)\0".as_ptr() as *const c_char,
        c,
        s,
        p,
        class as c_int,
        subclass as c_int,
        protocol as c_int,
    );
}

