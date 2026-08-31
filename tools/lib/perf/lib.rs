// SPDX-License-Identifier: GPL-2.0
// C dependencies: unistd.h, stdbool.h, errno.h, linux/kernel.h, internal/lib.h

use core::ffi::c_void;

pub type ssize_t = isize;
pub type size_t = usize;
pub type off_t = i64;

pub const EINTR: i32 = 4;

unsafe extern "C" {
    fn read(fd: i32, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: i32, buf: *const c_void, count: size_t) -> ssize_t;
    fn pread(fd: i32, buf: *mut c_void, count: size_t, offset: off_t) -> ssize_t;
    fn __errno_location() -> *mut i32;
}

#[no_mangle]
pub static mut page_size: u32 = 0;

#[inline]
unsafe fn errno() -> i32 {
    unsafe { *__errno_location() }
}

#[inline]
unsafe fn BUG_ON(condition: bool) {
    if condition {
        panic!("BUG_ON");
    }
}

unsafe fn ion(is_read: bool, fd: i32, mut buf: *mut c_void, n: size_t) -> ssize_t {
    let buf_start: *mut c_void = buf;
    let mut left: size_t = n;

    while left != 0 {
        /* buf must be treated as const if !is_read. */
        let ret: ssize_t = if is_read {
            unsafe { read(fd, buf, left) }
        } else {
            unsafe { write(fd, buf as *const c_void, left) }
        };

        if ret < 0 && unsafe { errno() } == EINTR {
            continue;
        }
        if ret <= 0 {
            return ret;
        }

        left -= ret as size_t;
        buf = unsafe { (buf as *mut u8).add(ret as usize) as *mut c_void };
    }

    unsafe {
        BUG_ON((buf as *mut u8).offset_from(buf_start as *mut u8) as size_t != n);
    }
    n as ssize_t
}

/*
 * Read exactly 'n' bytes or return an error.
 */
#[no_mangle]
pub unsafe extern "C" fn readn(fd: i32, buf: *mut c_void, n: size_t) -> ssize_t {
    unsafe { ion(true, fd, buf, n) }
}

#[no_mangle]
pub unsafe extern "C" fn preadn(
    fd: i32,
    mut buf: *mut c_void,
    n: size_t,
    mut offs: off_t,
) -> ssize_t {
    let mut left: size_t = n;

    while left != 0 {
        let ret: ssize_t = unsafe { pread(fd, buf, left, offs) };

        if ret < 0 && unsafe { errno() } == EINTR {
            continue;
        }
        if ret <= 0 {
            return ret;
        }

        left -= ret as size_t;
        buf = unsafe { (buf as *mut u8).add(ret as usize) as *mut c_void };
        offs += ret as off_t;
    }

    n as ssize_t
}

/*
 * Write exactly 'n' bytes or return an error.
 */
#[no_mangle]
pub unsafe extern "C" fn writen(fd: i32, buf: *const c_void, n: size_t) -> ssize_t {
    /* ion does not modify buf. */
    unsafe { ion(false, fd, buf as *mut c_void, n) }
}
