/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2005-2007 Takahiro Hirofuchi
 */

/*
 * Rust translation of usbip_common.h.
 *
 * Original C dependencies included libudev.h, stdint.h, stdio.h, stdlib.h,
 * string.h, syslog.h, unistd.h, linux/usb/ch9.h, and linux/usbip.h.
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct udev_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut usbip_use_syslog: c_int;
    pub static mut usbip_use_stderr: c_int;
    pub static mut usbip_use_debug: c_int;

    pub static mut stderr: *mut FILE;

    pub fn syslog(priority: c_int, format: *const c_char, ...);
    pub fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    pub fn abort() -> !;
}

pub const USBIDS_FILE: &[u8] = b"/usr/share/hwdata/usb.ids\0";
pub const VHCI_STATE_PATH: &[u8] = b"/var/run/vhci_hcd\0";

pub const VUDC_DEVICE_DESCR_FILE: &[u8] = b"dev_desc\0";

/* kernel module names */
pub const USBIP_CORE_MOD_NAME: &[u8] = b"usbip-core\0";
pub const USBIP_HOST_DRV_NAME: &[u8] = b"usbip-host\0";
pub const USBIP_DEVICE_DRV_NAME: &[u8] = b"usbip-vudc\0";
pub const USBIP_VHCI_DRV_NAME: &[u8] = b"vhci_hcd\0";

/* sysfs constants */
pub const SYSFS_MNT_PATH: &[u8] = b"/sys\0";
pub const SYSFS_BUS_NAME: &[u8] = b"bus\0";
pub const SYSFS_BUS_TYPE: &[u8] = b"usb\0";
pub const SYSFS_DRIVERS_NAME: &[u8] = b"drivers\0";

pub const SYSFS_PATH_MAX: usize = 256;
pub const SYSFS_BUS_ID_SIZE: usize = 32;

/* Defines for op_code status in server/client op_common PDUs */
pub const ST_OK: c_int = 0x00;
pub const ST_NA: c_int = 0x01;
/* Device requested for import is not available */
pub const ST_DEV_BUSY: c_int = 0x02;
/* Device requested for import is in error state */
pub const ST_DEV_ERR: c_int = 0x03;
pub const ST_NODEV: c_int = 0x04;
pub const ST_ERROR: c_int = 0x05;

pub const PROGNAME: &[u8] = b"usbip\0";

pub const LOG_ERR: c_int = 3;
pub const LOG_INFO: c_int = 6;
pub const LOG_DEBUG: c_int = 7;

#[macro_export]
macro_rules! pr_fmt {
    ($fmt:literal) => {
        concat!("%s: %s: ", $fmt, "\n\0")
    };
}

#[macro_export]
macro_rules! dbg_fmt {
    ($fmt:literal) => {
        concat!("%s: %s: %s:%d:[%s] ", $fmt, "\n\0")
    };
}

#[macro_export]
macro_rules! err {
    ($fmt:literal $(, $args:expr)* $(,)?) => {{
        unsafe {
            if $crate::usbip_use_syslog != 0 {
                $crate::syslog(
                    $crate::LOG_ERR,
                    $crate::pr_fmt!($fmt).as_ptr() as *const core::ffi::c_char,
                    b"usbip\0".as_ptr() as *const core::ffi::c_char,
                    b"error\0".as_ptr() as *const core::ffi::c_char
                    $(, $args)*
                );
            }
            if $crate::usbip_use_stderr != 0 {
                $crate::fprintf(
                    $crate::stderr,
                    $crate::pr_fmt!($fmt).as_ptr() as *const core::ffi::c_char,
                    b"usbip\0".as_ptr() as *const core::ffi::c_char,
                    b"error\0".as_ptr() as *const core::ffi::c_char
                    $(, $args)*
                );
            }
        }
    }};
}

#[macro_export]
macro_rules! info {
    ($fmt:literal $(, $args:expr)* $(,)?) => {{
        unsafe {
            if $crate::usbip_use_syslog != 0 {
                $crate::syslog(
                    $crate::LOG_INFO,
                    $crate::pr_fmt!($fmt).as_ptr() as *const core::ffi::c_char,
                    b"usbip\0".as_ptr() as *const core::ffi::c_char,
                    b"info\0".as_ptr() as *const core::ffi::c_char
                    $(, $args)*
                );
            }
            if $crate::usbip_use_stderr != 0 {
                $crate::fprintf(
                    $crate::stderr,
                    $crate::pr_fmt!($fmt).as_ptr() as *const core::ffi::c_char,
                    b"usbip\0".as_ptr() as *const core::ffi::c_char,
                    b"info\0".as_ptr() as *const core::ffi::c_char
                    $(, $args)*
                );
            }
        }
    }};
}

#[macro_export]
macro_rules! dbg {
    ($fmt:literal $(, $args:expr)* $(,)?) => {{
        unsafe {
            if $crate::usbip_use_debug != 0 {
                let file = ::std::ffi::CString::new(file!()).unwrap();
                let func = ::std::ffi::CString::new(module_path!()).unwrap();
                if $crate::usbip_use_syslog != 0 {
                    $crate::syslog(
                        $crate::LOG_DEBUG,
                        $crate::dbg_fmt!($fmt).as_ptr() as *const core::ffi::c_char,
                        b"usbip\0".as_ptr() as *const core::ffi::c_char,
                        b"debug\0".as_ptr() as *const core::ffi::c_char,
                        file.as_ptr(),
                        line!() as core::ffi::c_int,
                        func.as_ptr()
                        $(, $args)*
                    );
                }
                if $crate::usbip_use_stderr != 0 {
                    $crate::fprintf(
                        $crate::stderr,
                        $crate::dbg_fmt!($fmt).as_ptr() as *const core::ffi::c_char,
                        b"usbip\0".as_ptr() as *const core::ffi::c_char,
                        b"debug\0".as_ptr() as *const core::ffi::c_char,
                        file.as_ptr(),
                        line!() as core::ffi::c_int,
                        func.as_ptr()
                        $(, $args)*
                    );
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! BUG {
    () => {{
        $crate::err!("sorry, it's a bug!");
        unsafe {
            $crate::abort();
        }
    }};
}

#[repr(C, packed)]
pub struct usbip_usb_interface {
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,
    pub padding: u8, /* alignment */
}

#[repr(C, packed)]
pub struct usbip_usb_device {
    pub path: [c_char; SYSFS_PATH_MAX],
    pub busid: [c_char; SYSFS_BUS_ID_SIZE],

    pub busnum: u32,
    pub devnum: u32,
    pub speed: u32,

    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice: u16,

    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,
    pub bConfigurationValue: u8,
    pub bNumConfigurations: u8,
    pub bNumInterfaces: u8,
}

#[macro_export]
macro_rules! to_string {
    ($s:tt) => {
        stringify!($s)
    };
}

unsafe extern "C" {
    pub fn dump_usb_interface(interface: *mut usbip_usb_interface);
    pub fn dump_usb_device(device: *mut usbip_usb_device);
    pub fn read_usb_device(sdev: *mut udev_device, udev: *mut usbip_usb_device) -> c_int;
    pub fn read_attr_value(
        dev: *mut udev_device,
        name: *const c_char,
        format: *const c_char,
    ) -> c_int;
    pub fn read_usb_interface(
        udev: *mut usbip_usb_device,
        i: c_int,
        uinf: *mut usbip_usb_interface,
    ) -> c_int;

    pub fn usbip_speed_string(num: c_int) -> *const c_char;
    pub fn usbip_status_string(status: i32) -> *const c_char;
    pub fn usbip_op_common_status_string(status: c_int) -> *const c_char;

    pub fn usbip_names_init(path: *mut c_char) -> c_int;
    pub fn usbip_names_free();
    pub fn usbip_names_get_product(
        buff: *mut c_char,
        size: usize,
        vendor: u16,
        product: u16,
    );
    pub fn usbip_names_get_class(
        buff: *mut c_char,
        size: usize,
        class: u8,
        subclass: u8,
        protocol: u8,
    );
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
