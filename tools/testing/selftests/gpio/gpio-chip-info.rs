// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * GPIO character device helper for reading chip information.
 *
 * Copyright (C) 2021 Bartosz Golaszewski <brgl@bgdev.pl>
 */

use std::ffi::c_void;
use std::mem;
use std::os::raw::{c_char, c_int, c_uint, c_ulong};

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const O_RDWR: c_int = 2;

// From <linux/gpio.h>; kept here because this file directly depends on the
// struct layout and ioctl number supplied by that header.
const GPIO_MAX_NAME_SIZE: usize = 32;
const GPIO_GET_CHIPINFO_IOCTL: c_ulong = 0x8044_b401;

#[repr(C)]
struct gpiochip_info {
    name: [c_char; GPIO_MAX_NAME_SIZE],
    label: [c_char; GPIO_MAX_NAME_SIZE],
    lines: c_uint,
}

unsafe extern "C" {
    static mut stderr: *mut c_void;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
}

unsafe fn print_usage() {
    unsafe {
        printf(c"usage:\n".as_ptr());
        printf(c"  gpio-chip-info <chip path> [name|label|num-lines]\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut info: gpiochip_info = unsafe { mem::zeroed() };
    let fd: c_int;
    let ret: c_int;

    if argc != 3 {
        unsafe {
            print_usage();
        }
        return EXIT_FAILURE;
    }

    fd = unsafe { open(*argv.add(1), O_RDWR) };
    if fd < 0 {
        unsafe {
            perror(c"unable to open the GPIO chip".as_ptr());
        }
        return EXIT_FAILURE;
    }

    unsafe {
        memset(
            &mut info as *mut gpiochip_info as *mut c_void,
            0,
            mem::size_of_val(&info),
        );
    }
    ret = unsafe {
        ioctl(
            fd,
            GPIO_GET_CHIPINFO_IOCTL,
            &mut info as *mut gpiochip_info,
        )
    };
    if ret != 0 {
        unsafe {
            perror(c"chip info ioctl failed".as_ptr());
        }
        return EXIT_FAILURE;
    }

    if unsafe { strcmp(*argv.add(2), c"name".as_ptr()) } == 0 {
        unsafe {
            printf(c"%s\n".as_ptr(), info.name.as_ptr());
        }
    } else if unsafe { strcmp(*argv.add(2), c"label".as_ptr()) } == 0 {
        unsafe {
            printf(c"%s\n".as_ptr(), info.label.as_ptr());
        }
    } else if unsafe { strcmp(*argv.add(2), c"num-lines".as_ptr()) } == 0 {
        unsafe {
            printf(c"%u\n".as_ptr(), info.lines);
        }
    } else {
        unsafe {
            fprintf(
                stderr,
                c"unknown command: %s\n".as_ptr(),
                *argv.add(2),
            );
        }
        return EXIT_FAILURE;
    }

    EXIT_SUCCESS
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
