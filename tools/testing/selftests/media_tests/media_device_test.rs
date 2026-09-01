// SPDX-License-Identifier: GPL-2.0

/*
 * media_device_test.c - Media Controller Device ioctl loop Test
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
 * MEDIA_IOC_DEVICE_INFO ioctl in a loop once every 10
 * seconds.
 *
 * Usage:
 *	sudo ./media_device_test -d /dev/mediaX
 *
 *	While test is running, remove the device and
 *	ensure there are no use after free errors and
 *	other Oops in the dmesg. Enable KaSan kernel
 *	config option for use-after-free error detection.
*/

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const O_RDWR: c_int = 0o2;
const MEDIA_IOC_DEVICE_INFO: c_ulong = 0xc1007c00;

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
    fn exit(status: c_int) -> !;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn getuid() -> c_uint;
    fn srand(seed: c_uint);
    fn time(tloc: *mut c_long) -> c_long;
    fn rand() -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn __errno_location() -> *mut c_int;

    fn ksft_exit_skip(msg: *const c_char, ...) -> !;
}

unsafe fn strlen(mut s: *const c_char) -> usize {
    let mut len: usize = 0;

    while *s != 0 {
        len += 1;
        s = s.add(1);
    }

    len
}

unsafe fn puts_char_array(s: *const c_char) -> *const c_char {
    s
}

unsafe fn main_0(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut opt: c_int;
    let mut media_device: [c_char; 256] = [0; 256];
    let mut count: c_int;
    let mut mdi: media_device_info = core::mem::zeroed();
    let mut ret: c_int;
    let mut fd: c_int;

    if argc < 2 {
        printf(
            b"Usage: %s [-d </dev/mediaX>]\n\0".as_ptr() as *const c_char,
            *argv.add(0),
        );
        exit(-1);
    }

    /* Process arguments */
    loop {
        opt = getopt(argc, argv as *const *mut c_char, b"d:\0".as_ptr() as *const c_char);
        if opt == -1 {
            break;
        }

        match opt {
            100 => {
                strncpy(
                    media_device.as_mut_ptr(),
                    optarg,
                    core::mem::size_of_val(&media_device) - 1,
                );
                media_device[core::mem::size_of_val(&media_device) - 1] = '\0' as c_char;
            }
            _ => {
                printf(
                    b"Usage: %s [-d </dev/mediaX>]\n\0".as_ptr() as *const c_char,
                    *argv.add(0),
                );
                exit(-1);
            }
        }
    }

    if getuid() != 0 {
        ksft_exit_skip(b"Please run the test as root - Exiting.\n\0".as_ptr() as *const c_char);
    }

    /* Generate random number of interations */
    srand(time(core::ptr::null_mut()) as c_uint);
    count = rand();

    /* Open Media device and keep it open */
    fd = open(media_device.as_ptr(), O_RDWR);
    if fd == -1 {
        printf(
            b"Media Device open errno %s\n\0".as_ptr() as *const c_char,
            strerror(*__errno_location()),
        );
        exit(-1);
    }

    printf(
        b"\nNote:\nWhile test is running, remove the device and\nensure there are no use after free errors and\nother Oops in the dmesg. Enable KaSan kernel\nconfig option for use-after-free error detection.\n\n\0"
            .as_ptr() as *const c_char,
    );

    printf(
        b"Running test for %d iterations\n\0".as_ptr() as *const c_char,
        count,
    );

    while count > 0 {
        ret = ioctl(
            fd,
            MEDIA_IOC_DEVICE_INFO,
            &mut mdi as *mut media_device_info as *mut c_void,
        );
        if ret < 0 {
            printf(
                b"Media Device Info errno %s\n\0".as_ptr() as *const c_char,
                strerror(*__errno_location()),
            );
        } else {
            printf(
                b"Media device model %s driver %s - count %d\n\0".as_ptr() as *const c_char,
                puts_char_array(mdi.model.as_ptr()),
                puts_char_array(mdi.driver.as_ptr()),
                count,
            );
        }
        sleep(10);
        count -= 1;
    }

    0
}

fn main() {
    let mut argv_storage: Vec<*mut c_char> = std::env::args()
        .map(|arg| {
            let mut bytes = arg.into_bytes();
            bytes.push(0);
            Box::into_raw(bytes.into_boxed_slice()) as *mut c_char
        })
        .collect();

    unsafe {
        main_0(argv_storage.len() as c_int, argv_storage.as_mut_ptr());

        for arg in argv_storage {
            let len = strlen(arg as *const c_char) + 1;
            let slice = core::ptr::slice_from_raw_parts_mut(arg as *mut u8, len);
            drop(Box::from_raw(slice));
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
