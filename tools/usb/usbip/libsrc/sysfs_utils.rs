// SPDX-License-Identifier: GPL-2.0
//
// Translated from usb/usbip/libsrc/sysfs_utils.c.
// C includes referenced sys/types.h, sys/stat.h, fcntl.h, errno.h,
// sysfs_utils.h, and usbip_common.h.

#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_void};

pub type size_t = usize;
pub type ssize_t = isize;

pub const O_WRONLY: c_int = 1;

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn dbg(fmt: *const c_char, ...);
}

#[no_mangle]
pub unsafe extern "C" fn write_sysfs_attribute(
    attr_path: *const c_char,
    new_value: *const c_char,
    len: size_t,
) -> c_int {
    let fd: c_int;
    let length: c_int;

    fd = unsafe { open(attr_path, O_WRONLY) };
    if fd < 0 {
        unsafe { dbg(c"error opening attribute %s".as_ptr(), attr_path) };
        return -1;
    }

    length = unsafe { write(fd, new_value as *const c_void, len) as c_int };
    if length < 0 {
        unsafe { dbg(c"error writing to attribute %s".as_ptr(), attr_path) };
        unsafe {
            close(fd);
        }
        return -1;
    }

    unsafe {
        close(fd);
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
