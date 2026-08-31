// SPDX-License-Identifier: GPL-2.0-only
/*
 * gpio-hammer - example swiss army knife to shake GPIO lines on a system
 *
 * Copyright (C) 2016 Linus Walleij
 *
 * Usage:
 *	gpio-hammer -n <device-name> -o <offset1> -o <offset2>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::zeroed;
use core::ptr::{null, null_mut};

const GPIOHANDLES_MAX: usize = 64;
const GPIO_V2_LINE_NUM_ATTRS_MAX: usize = 10;
const GPIO_V2_LINE_FLAG_OUTPUT: u64 = 1 << 1;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union gpio_v2_line_attribute__bindgen_ty_1 {
    pub flags: u64,
    pub values: u64,
    pub debounce_period_us: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_v2_line_attribute {
    pub id: u32,
    pub padding: u32,
    pub __bindgen_anon_1: gpio_v2_line_attribute__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_v2_line_config_attribute {
    pub attr: gpio_v2_line_attribute,
    pub mask: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_v2_line_config {
    pub flags: u64,
    pub num_attrs: u32,
    pub padding: [u32; 5],
    pub attrs: [gpio_v2_line_config_attribute; GPIO_V2_LINE_NUM_ATTRS_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_v2_line_values {
    pub bits: u64,
    pub mask: u64,
}

unsafe extern "C" {
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut optarg: *mut c_char;

    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;

    fn gpiotools_request_line(
        device_name: *const c_char,
        lines: *mut c_uint,
        num_lines: c_int,
        config: *mut gpio_v2_line_config,
        consumer: *const c_char,
    ) -> c_int;
    fn gpiotools_release_line(fd: c_int);
    fn gpiotools_get_values(fd: c_int, values: *mut gpio_v2_line_values) -> c_int;
    fn gpiotools_set_values(fd: c_int, values: *mut gpio_v2_line_values) -> c_int;
    fn gpiotools_set_bit(bits: *mut u64, bit: c_int);
    fn gpiotools_change_bit(bits: *mut u64, bit: c_int);
    fn gpiotools_test_bit(bits: u64, bit: c_int) -> c_int;
}

pub unsafe fn hammer_device(
    device_name: *const c_char,
    lines: *mut c_uint,
    num_lines: c_int,
    loops: c_uint,
) -> c_int {
    let mut values: gpio_v2_line_values = unsafe { zeroed() };
    let mut config: gpio_v2_line_config = unsafe { zeroed() };
    let swirr: [c_char; 5] = [b'-' as c_char, b'\\' as c_char, b'|' as c_char, b'/' as c_char, 0];
    let fd: c_int;
    let mut ret: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let mut iteration: c_uint = 0;

    config.flags = GPIO_V2_LINE_FLAG_OUTPUT;

    ret = unsafe {
        gpiotools_request_line(
            device_name,
            lines,
            num_lines,
            &mut config,
            c"gpio-hammer".as_ptr(),
        )
    };
    if ret < 0 {
        return ret;
    } else {
        fd = ret;
    }

    values.mask = 0;
    values.bits = 0;
    i = 0;
    while i < num_lines {
        unsafe {
            gpiotools_set_bit(&mut values.mask, i);
        }
        i += 1;
    }

    ret = unsafe { gpiotools_get_values(fd, &mut values) };
    if ret < 0 {
        unsafe {
            gpiotools_release_line(fd);
        }
        return ret;
    }

    unsafe {
        fprintf(stdout, c"Hammer lines [".as_ptr());
    }
    i = 0;
    while i < num_lines {
        unsafe {
            fprintf(stdout, c"%u".as_ptr(), *lines.add(i as usize));
        }
        if i != num_lines - 1 {
            unsafe {
                fprintf(stdout, c", ".as_ptr());
            }
        }
        i += 1;
    }
    unsafe {
        fprintf(stdout, c"] on %s, initial states: [".as_ptr(), device_name);
    }
    i = 0;
    while i < num_lines {
        unsafe {
            fprintf(
                stdout,
                c"%d".as_ptr(),
                gpiotools_test_bit(values.bits, i),
            );
        }
        if i != num_lines - 1 {
            unsafe {
                fprintf(stdout, c", ".as_ptr());
            }
        }
        i += 1;
    }
    unsafe {
        fprintf(stdout, c"]\n".as_ptr());
    }

    /* Hammertime! */
    j = 0;
    loop {
        /* Invert all lines so we blink */
        i = 0;
        while i < num_lines {
            unsafe {
                gpiotools_change_bit(&mut values.bits, i);
            }
            i += 1;
        }

        ret = unsafe { gpiotools_set_values(fd, &mut values) };
        if ret < 0 {
            unsafe {
                gpiotools_release_line(fd);
            }
            return ret;
        }

        /* Re-read values to get status */
        ret = unsafe { gpiotools_get_values(fd, &mut values) };
        if ret < 0 {
            unsafe {
                gpiotools_release_line(fd);
            }
            return ret;
        }

        unsafe {
            fprintf(stdout, c"[%c] ".as_ptr(), swirr[j as usize] as c_int);
        }
        j += 1;
        if j == (swirr.len() - 1) as c_int {
            j = 0;
        }

        unsafe {
            fprintf(stdout, c"[".as_ptr());
        }
        i = 0;
        while i < num_lines {
            unsafe {
                fprintf(
                    stdout,
                    c"%u: %d".as_ptr(),
                    *lines.add(i as usize),
                    gpiotools_test_bit(values.bits, i),
                );
            }
            if i != num_lines - 1 {
                unsafe {
                    fprintf(stdout, c", ".as_ptr());
                }
            }
            i += 1;
        }
        unsafe {
            fprintf(stdout, c"]\r".as_ptr());
            fflush(stdout);
            sleep(1);
        }
        iteration = iteration.wrapping_add(1);
        if loops != 0 && iteration == loops {
            break;
        }
    }
    unsafe {
        fprintf(stdout, c"\n".as_ptr());
        gpiotools_release_line(fd);
    }
    ret = 0;

    ret
}

pub unsafe fn print_usage() {
    unsafe {
        fprintf(
            stderr,
            c"Usage: gpio-hammer [options]...\nHammer GPIO lines, 0->1->0->1...\n  -n <name>  Hammer GPIOs on a named device (must be stated)\n  -o <n>     Offset[s] to hammer, at least one, several can be stated\n [-c <n>]    Do <n> loops (optional, infinite loop if not stated)\n  -?         This helptext\n\nExample:\ngpio-hammer -n gpiochip0 -o 4\n".as_ptr(),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut device_name: *const c_char = null();
    let mut lines: [c_uint; GPIOHANDLES_MAX] = [0; GPIOHANDLES_MAX];
    let mut loops: c_uint = 0;
    let num_lines: c_int;
    let mut c: c_int;
    let mut i: c_int;

    i = 0;
    loop {
        c = unsafe { getopt(argc, argv, c"c:n:o:?".as_ptr()) };
        if c == -1 {
            break;
        }
        match c {
            99 => {
                loops = unsafe { strtoul(optarg, null_mut(), 10) as c_uint };
            }
            110 => {
                device_name = unsafe { optarg };
            }
            111 => {
                /*
                 * Avoid overflow. Do not immediately error, we want to
                 * be able to accurately report on the amount of times
                 * '-o' was given to give an accurate error message
                 */
                if i < GPIOHANDLES_MAX as c_int {
                    lines[i as usize] = unsafe { strtoul(optarg, null_mut(), 10) as c_uint };
                }

                i += 1;
            }
            63 => {
                unsafe {
                    print_usage();
                }
                return -1;
            }
            _ => {}
        }
    }

    if i >= GPIOHANDLES_MAX as c_int {
        unsafe {
            fprintf(
                stderr,
                c"Only %d occurrences of '-o' are allowed, %d were found\n".as_ptr(),
                GPIOHANDLES_MAX as c_int,
                i + 1,
            );
        }
        return -1;
    }

    num_lines = i;

    if device_name.is_null() || num_lines == 0 {
        unsafe {
            print_usage();
        }
        return -1;
    }
    unsafe { hammer_device(device_name, lines.as_mut_ptr(), num_lines, loops) }
}
