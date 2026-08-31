// SPDX-License-Identifier: GPL-2.0

/*
 * media_device_open.c - Media Controller Device Open Test
 *
 * Copyright (c) 2016 Shuah Khan <shuahkh@osg.samsung.com>
 * Copyright (c) 2016 Samsung Electronics Co., Ltd.
 *
 */

/*
 * This file adds a test for Media Controller API.
 * This test should be run as root and should not be
 * included in the Kselftest run. This test should be
 * run when hardware and driver that makes use Media
 * Controller API are present in the system.
 *
 * This test opens user specified Media Device and calls
 * MEDIA_IOC_DEVICE_INFO ioctl, closes the file, and exits.
 *
 * Usage:
 *	sudo ./media_device_open -d /dev/mediaX
 *
 *	Run this test is a loop and run bind/unbind on the driver.
*/

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// C dependencies: stdio.h, unistd.h, stdlib.h, errno.h, string.h, fcntl.h,
// sys/ioctl.h, sys/stat.h, linux/media.h, and "kselftest.h".

const O_RDWR: c_int = 0o00000002;

const MEDIA_IOC_MAGIC: c_uint = b'|' as c_uint;
const MEDIA_IOC_DEVICE_INFO_NR: c_uint = 0x00;

const IOC_NRBITS: c_uint = 8;
const IOC_TYPEBITS: c_uint = 8;
const IOC_SIZEBITS: c_uint = 14;
const IOC_DIRBITS: c_uint = 2;

const IOC_NRSHIFT: c_uint = 0;
const IOC_TYPESHIFT: c_uint = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: c_uint = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: c_uint = IOC_SIZESHIFT + IOC_SIZEBITS;

const IOC_READ: c_uint = 2;

const fn ioc(dir: c_uint, type_: c_uint, nr: c_uint, size: c_uint) -> c_ulong {
    ((dir << IOC_DIRSHIFT)
        | (type_ << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
        | (size << IOC_SIZESHIFT)) as c_ulong
}

const fn ior<T>(type_: c_uint, nr: c_uint) -> c_ulong {
    ioc(IOC_READ, type_, nr, core::mem::size_of::<T>() as c_uint)
}

const MEDIA_IOC_DEVICE_INFO: c_ulong =
    ior::<media_device_info>(MEDIA_IOC_MAGIC, MEDIA_IOC_DEVICE_INFO_NR);

#[repr(C)]
struct media_device_info {
    driver: [c_char; 16],
    model: [c_char; 32],
    serial: [c_char; 40],
    bus_info: [c_char; 32],
    media_version: u32,
    hw_revision: u32,
    driver_version: u32,
    reserved: [u32; 31],
}

unsafe extern "C" {
    static mut optarg: *mut c_char;

    fn printf(format: *const c_char, ...) -> c_int;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn getuid() -> c_uint;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn __errno_location() -> *mut c_int;

    fn ksft_exit_skip(msg: *const c_char, ...) -> !;
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut opt: c_int;
    let mut media_device: [c_char; 256] = [0; 256];
    let _count: c_int = 0;
    let mut mdi: media_device_info = unsafe { core::mem::zeroed() };
    let ret: c_int;
    let fd: c_int;

    if argc < 2 {
        unsafe {
            printf(
                b"Usage: %s [-d </dev/mediaX>]\n\0".as_ptr() as *const c_char,
                *argv.offset(0),
            );
        }
        std::process::exit(-1);
    }

    /* Process arguments */
    loop {
        opt = unsafe { getopt(argc, argv, b"d:\0".as_ptr() as *const c_char) };
        if opt == -1 {
            break;
        }

        match opt {
            x if x == b'd' as c_int => {
                unsafe {
                    strncpy(
                        media_device.as_mut_ptr(),
                        optarg,
                        core::mem::size_of_val(&media_device) - 1,
                    );
                }
                media_device[core::mem::size_of_val(&media_device) - 1] = b'\0' as c_char;
            }
            _ => {
                unsafe {
                    printf(
                        b"Usage: %s [-d </dev/mediaX>]\n\0".as_ptr() as *const c_char,
                        *argv.offset(0),
                    );
                }
                std::process::exit(-1);
            }
        }
    }

    if unsafe { getuid() } != 0 {
        unsafe {
            ksft_exit_skip(b"Please run the test as root - Exiting.\n\0".as_ptr() as *const c_char);
        }
    }

    /* Open Media device and keep it open */
    fd = unsafe { open(media_device.as_ptr(), O_RDWR) };
    if fd == -1 {
        unsafe {
            printf(
                b"Media Device open errno %s\n\0".as_ptr() as *const c_char,
                strerror(errno()),
            );
        }
        std::process::exit(-1);
    }

    ret = unsafe { ioctl(fd, MEDIA_IOC_DEVICE_INFO, &mut mdi as *mut media_device_info as *mut c_void) };
    if ret < 0 {
        unsafe {
            printf(
                b"Media Device Info errno %s\n\0".as_ptr() as *const c_char,
                strerror(errno()),
            );
        }
    } else {
        unsafe {
            printf(
                b"Media device model %s driver %s\n\0".as_ptr() as *const c_char,
                mdi.model.as_ptr(),
                mdi.driver.as_ptr(),
            );
        }
    }

    0
}
