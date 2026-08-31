// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * GPIO character device helper for reading line names.
 *
 * Copyright (C) 2021 Bartosz Golaszewski <brgl@bgdev.pl>
 */

// C dependencies: <fcntl.h>, <linux/gpio.h>, <stdio.h>, <stdlib.h>,
// <string.h>, <sys/ioctl.h>, <sys/types.h>

use core::ffi::{c_char, c_int, c_ulong, c_void};

const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;

extern "C" {
    static O_RDWR: c_int;
    static GPIO_V2_GET_LINEINFO_IOCTL: c_ulong;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

// Provided by <linux/gpio.h>.
#[repr(C)]
struct gpio_v2_line_info {
    name: [c_char; 32],
    consumer: [c_char; 32],
    offset: u32,
    num_attrs: u32,
    flags: u64,
    attrs: [gpio_v2_line_attribute; 10],
    padding: [u32; 4],
}

// Provided by <linux/gpio.h>.
#[repr(C)]
struct gpio_v2_line_attribute {
    id: u32,
    padding: u32,
    value: gpio_v2_line_attribute_value,
}

// Provided by <linux/gpio.h>.
#[repr(C)]
union gpio_v2_line_attribute_value {
    flags: u64,
    values: u64,
    debounce_period_us: u32,
}

unsafe fn print_usage() {
    printf(b"usage:\n\0".as_ptr() as *const c_char);
    printf(b"  gpio-line-name <chip path> <line offset>\n\0".as_ptr() as *const c_char);
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut info: gpio_v2_line_info;
    let fd: c_int;
    let ret: c_int;
    let mut endp: *mut c_char = core::ptr::null_mut();

    if argc != 3 {
        print_usage();
        return EXIT_FAILURE;
    }

    fd = open(*argv.add(1), O_RDWR);
    if fd < 0 {
        perror(b"unable to open the GPIO chip\0".as_ptr() as *const c_char);
        return EXIT_FAILURE;
    }

    info = core::mem::zeroed();
    info.offset = strtoul(*argv.add(2), &mut endp, 10) as u32;
    if *endp != b'\0' as c_char {
        print_usage();
        return EXIT_FAILURE;
    }

    ret = ioctl(fd, GPIO_V2_GET_LINEINFO_IOCTL, &mut info as *mut gpio_v2_line_info);
    if ret != 0 {
        perror(b"line info ioctl failed\0".as_ptr() as *const c_char);
        return EXIT_FAILURE;
    }

    printf(b"%s\n\0".as_ptr() as *const c_char, info.name.as_ptr());

    EXIT_SUCCESS
}
