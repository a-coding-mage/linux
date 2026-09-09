/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding C headers.
use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

pub type __u64 = u64;

unsafe extern "C" {
    pub fn uml_kmalloc(size: c_int, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
    pub fn printk(fmt: *const c_char, ...);
    pub fn uml_strdup(str_: *mut c_char) -> *mut c_char;
    pub fn os_seek_file(fd: c_int, offset: __u64) -> c_int;
    pub fn os_file_size(file: *mut c_char, size_out: *mut c_ulonglong) -> c_int;
    pub fn os_write_file(fd: c_int, buf: *mut c_void, size: c_int) -> c_int;
}

pub unsafe fn cow_malloc(size: c_int) -> *mut c_void {
    unsafe { uml_kmalloc(size, UM_GFP_KERNEL) }
}

pub unsafe fn cow_free(ptr: *mut c_void) {
    unsafe { kfree(ptr) }
}

#[macro_export]
macro_rules! cow_printf {
    ($($arg:tt)*) => {
        unsafe { $crate::printk($($arg)*) }
    };
}

pub unsafe fn cow_strdup(str_: *mut c_char) -> *mut c_char {
    unsafe { uml_strdup(str_) }
}

pub unsafe fn cow_seek_file(fd: c_int, offset: __u64) -> c_int {
    unsafe { os_seek_file(fd, offset) }
}

pub unsafe fn cow_file_size(file: *mut c_char, size_out: *mut c_ulonglong) -> c_int {
    unsafe { os_file_size(file, size_out) }
}

pub unsafe fn cow_write_file(fd: c_int, buf: *mut c_void, size: c_int) -> c_int {
    unsafe { os_write_file(fd, buf, size) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
