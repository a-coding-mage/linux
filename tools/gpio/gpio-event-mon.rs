// SPDX-License-Identifier: GPL-2.0-only
/*
 * gpio-event-mon - monitor GPIO line events from userspace
 *
 * Copyright (C) 2016 Linus Walleij
 *
 * Usage:
 *	gpio-event-mon -n <device-name> -o <offset>
 */

// C dependencies translated from:
// unistd.h, stdlib.h, stdbool.h, stdint.h, stdio.h, dirent.h, errno.h,
// string.h, poll.h, fcntl.h, getopt.h, inttypes.h, sys/ioctl.h, sys/types.h,
// linux/gpio.h, and "gpio-utils.h".

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    static mut optarg: *mut c_char;
    static mut errno: c_int;

    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn free(ptr: *mut c_void);
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;

    fn gpiotools_request_line(
        device_name: *const c_char,
        lines: *mut c_uint,
        num_lines: c_uint,
        config: *mut gpio_v2_line_config,
        consumer: *const c_char,
    ) -> c_int;
    fn gpiotools_set_bit(bits: *mut u64, bit: c_int);
    fn gpiotools_get_values(lfd: c_int, values: *mut gpio_v2_line_values) -> c_int;
    fn gpiotools_test_bit(bits: u64, bit: c_int) -> c_int;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

// Provided by linux/gpio.h in the original source.
#[repr(C)]
pub struct gpio_v2_line_values {
    pub bits: u64,
    pub mask: u64,
}

#[repr(C)]
pub struct gpio_v2_line_event {
    pub timestamp_ns: u64,
    pub id: u32,
    pub offset: u32,
    pub seqno: u32,
    pub line_seqno: u32,
    pub padding: [u32; 6],
}

#[repr(C)]
pub union gpio_v2_line_attribute_union {
    pub flags: u64,
    pub values: u64,
    pub debounce_period_us: u32,
}

#[repr(C)]
pub struct gpio_v2_line_attribute {
    pub id: u32,
    pub padding: u32,
    pub debounce_period_us: c_ulong,
}

#[repr(C)]
pub struct gpio_v2_line_config_attribute {
    pub attr: gpio_v2_line_attribute,
    pub mask: u64,
}

#[repr(C)]
pub struct gpio_v2_line_config {
    pub flags: u64,
    pub num_attrs: u32,
    pub padding: [u32; 5],
    pub attrs: [gpio_v2_line_config_attribute; GPIO_V2_LINE_NUM_ATTRS_MAX],
}

// External constants from linux/gpio.h and errno.h.
const ENOMEM: c_int = 12;
const EAGAIN: c_int = 11;
const EIO: c_int = 5;
const GPIO_V2_LINES_MAX: usize = 64;
const GPIO_V2_LINE_NUM_ATTRS_MAX: usize = 10;
const GPIO_V2_LINE_FLAG_INPUT: u64 = 1 << 0;
const GPIO_V2_LINE_FLAG_EDGE_RISING: u64 = 1 << 5;
const GPIO_V2_LINE_FLAG_EDGE_FALLING: u64 = 1 << 6;
const GPIO_V2_LINE_FLAG_OPEN_DRAIN: u64 = 1 << 2;
const GPIO_V2_LINE_FLAG_OPEN_SOURCE: u64 = 1 << 3;
const GPIO_V2_LINE_FLAG_EVENT_CLOCK_REALTIME: u64 = 1 << 8;
const GPIO_V2_LINE_FLAG_EVENT_CLOCK_HTE: u64 = 1 << 9;
const GPIO_V2_LINE_EVENT_RISING_EDGE: u32 = 1;
const GPIO_V2_LINE_EVENT_FALLING_EDGE: u32 = 2;
const GPIO_V2_LINE_ATTR_ID_DEBOUNCE: u32 = 3;

const EDGE_FLAGS: u64 = GPIO_V2_LINE_FLAG_EDGE_RISING | GPIO_V2_LINE_FLAG_EDGE_FALLING;

#[no_mangle]
pub unsafe extern "C" fn monitor_device(
    device_name: *const c_char,
    lines: *mut c_uint,
    num_lines: c_uint,
    config: *mut gpio_v2_line_config,
    loops: c_uint,
) -> c_int {
    let mut values: gpio_v2_line_values = mem::zeroed();
    let mut chrdev_name: *mut c_char = ptr::null_mut();
    let cfd: c_int;
    let lfd: c_int;
    let mut ret: c_int;
    let mut i: c_uint = 0;

    ret = asprintf(
        &mut chrdev_name as *mut *mut c_char,
        b"/dev/%s\0".as_ptr() as *const c_char,
        device_name,
    );
    if ret < 0 {
        return -ENOMEM;
    }

    cfd = open(chrdev_name, 0);
    if cfd == -1 {
        ret = -errno;
        fprintf(stderr, b"Failed to open %s\n\0".as_ptr() as *const c_char, chrdev_name);
        free(chrdev_name as *mut c_void);
        return ret;
    }

    ret = gpiotools_request_line(
        device_name,
        lines,
        num_lines,
        config,
        b"gpio-event-mon\0".as_ptr() as *const c_char,
    );
    if ret < 0 {
        if close(cfd) == -1 {
            perror(b"Failed to close GPIO character device file\0".as_ptr() as *const c_char);
        }
        free(chrdev_name as *mut c_void);
        return ret;
    } else {
        lfd = ret;
    }

    /* Read initial states */
    values.mask = 0;
    values.bits = 0;
    i = 0;
    while i < num_lines {
        gpiotools_set_bit(&mut values.mask as *mut u64, i as c_int);
        i = i.wrapping_add(1);
    }
    ret = gpiotools_get_values(lfd, &mut values as *mut gpio_v2_line_values);
    if ret < 0 {
        fprintf(
            stderr,
            b"Failed to issue GPIO LINE GET VALUES IOCTL (%d)\n\0".as_ptr() as *const c_char,
            ret,
        );
        if close(lfd) == -1 {
            perror(b"Failed to close line file\0".as_ptr() as *const c_char);
        }
        if close(cfd) == -1 {
            perror(b"Failed to close GPIO character device file\0".as_ptr() as *const c_char);
        }
        free(chrdev_name as *mut c_void);
        return ret;
    }

    if num_lines == 1 {
        fprintf(
            stdout,
            b"Monitoring line %u on %s\n\0".as_ptr() as *const c_char,
            *lines.add(0),
            device_name,
        );
        fprintf(
            stdout,
            b"Initial line value: %d\n\0".as_ptr() as *const c_char,
            gpiotools_test_bit(values.bits, 0),
        );
    } else {
        fprintf(
            stdout,
            b"Monitoring lines %u\0".as_ptr() as *const c_char,
            *lines.add(0),
        );
        i = 1;
        while i < num_lines.wrapping_sub(1) {
            fprintf(stdout, b", %u\0".as_ptr() as *const c_char, *lines.add(i as usize));
            i = i.wrapping_add(1);
        }
        fprintf(
            stdout,
            b" and %u on %s\n\0".as_ptr() as *const c_char,
            *lines.add(i as usize),
            device_name,
        );
        fprintf(
            stdout,
            b"Initial line values: %d\0".as_ptr() as *const c_char,
            gpiotools_test_bit(values.bits, 0),
        );
        i = 1;
        while i < num_lines.wrapping_sub(1) {
            fprintf(
                stdout,
                b", %d\0".as_ptr() as *const c_char,
                gpiotools_test_bit(values.bits, i as c_int),
            );
            i = i.wrapping_add(1);
        }
        fprintf(
            stdout,
            b" and %d\n\0".as_ptr() as *const c_char,
            gpiotools_test_bit(values.bits, i as c_int),
        );
    }

    i = 0;
    loop {
        let mut event: gpio_v2_line_event = mem::zeroed();

        ret = read(
            lfd,
            &mut event as *mut gpio_v2_line_event as *mut c_void,
            mem::size_of::<gpio_v2_line_event>(),
        ) as c_int;
        if ret == -1 {
            if errno == -EAGAIN {
                fprintf(stderr, b"nothing available\n\0".as_ptr() as *const c_char);
                continue;
            } else {
                ret = -errno;
                fprintf(
                    stderr,
                    b"Failed to read event (%d)\n\0".as_ptr() as *const c_char,
                    ret,
                );
                break;
            }
        }

        if ret as usize != mem::size_of::<gpio_v2_line_event>() {
            fprintf(stderr, b"Reading event failed\n\0".as_ptr() as *const c_char);
            ret = -EIO;
            break;
        }
        fprintf(
            stdout,
            b"GPIO EVENT at %lu on line %d (%d|%d) \0".as_ptr() as *const c_char,
            event.timestamp_ns as u64,
            event.offset as c_int,
            event.line_seqno as c_int,
            event.seqno as c_int,
        );
        match event.id {
            GPIO_V2_LINE_EVENT_RISING_EDGE => {
                fprintf(stdout, b"rising edge\0".as_ptr() as *const c_char);
            }
            GPIO_V2_LINE_EVENT_FALLING_EDGE => {
                fprintf(stdout, b"falling edge\0".as_ptr() as *const c_char);
            }
            _ => {
                fprintf(stdout, b"unknown event\0".as_ptr() as *const c_char);
            }
        }
        fprintf(stdout, b"\n\0".as_ptr() as *const c_char);

        i = i.wrapping_add(1);
        if i == loops {
            break;
        }
    }

    if close(lfd) == -1 {
        perror(b"Failed to close line file\0".as_ptr() as *const c_char);
    }
    if close(cfd) == -1 {
        perror(b"Failed to close GPIO character device file\0".as_ptr() as *const c_char);
    }
    free(chrdev_name as *mut c_void);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn print_usage() {
    fprintf(
        stderr,
        b"Usage: gpio-event-mon [options]...\n\
Listen to events on GPIO lines, 0->1 1->0\n\
  -n <name>  Listen on GPIOs on a named device (must be stated)\n\
  -o <n>     Offset of line to monitor (may be repeated)\n\
  -d         Set line as open drain\n\
  -s         Set line as open source\n\
  -r         Listen for rising edges\n\
  -f         Listen for falling edges\n\
  -w         Report the wall-clock time for events\n\
  -t         Report the hardware timestamp for events\n\
  -b <n>     Debounce the line with period n microseconds\n\
 [-c <n>]    Do <n> loops (optional, infinite loop if not stated)\n\
  -?         This helptext\n\
\n\
Example:\n\
gpio-event-mon -n gpiochip0 -o 4 -r -f -b 10000\n\0"
            .as_ptr() as *const c_char,
    );
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut device_name: *const c_char = ptr::null();
    let mut lines: [c_uint; GPIO_V2_LINES_MAX] = [0; GPIO_V2_LINES_MAX];
    let mut num_lines: c_uint = 0;
    let mut loops: c_uint = 0;
    let mut config: gpio_v2_line_config = mem::zeroed();
    let mut c: c_int;
    let mut attr: c_int;
    let mut i: c_int;
    let mut debounce_period_us: c_ulong = 0;

    ptr::write_bytes(
        &mut config as *mut gpio_v2_line_config as *mut u8,
        0,
        mem::size_of::<gpio_v2_line_config>(),
    );
    config.flags = GPIO_V2_LINE_FLAG_INPUT;
    loop {
        c = getopt(argc, argv, b"c:n:o:b:dsrfwt?\0".as_ptr() as *const c_char);
        if c == -1 {
            break;
        }
        match c {
            x if x == b'c' as c_int => {
                loops = strtoul(optarg, ptr::null_mut(), 10) as c_uint;
            }
            x if x == b'n' as c_int => {
                device_name = optarg;
            }
            x if x == b'o' as c_int => {
                if num_lines as usize >= GPIO_V2_LINES_MAX {
                    print_usage();
                    return -1;
                }
                lines[num_lines as usize] = strtoul(optarg, ptr::null_mut(), 10) as c_uint;
                num_lines = num_lines.wrapping_add(1);
            }
            x if x == b'b' as c_int => {
                debounce_period_us = strtoul(optarg, ptr::null_mut(), 10);
            }
            x if x == b'd' as c_int => {
                config.flags |= GPIO_V2_LINE_FLAG_OPEN_DRAIN;
            }
            x if x == b's' as c_int => {
                config.flags |= GPIO_V2_LINE_FLAG_OPEN_SOURCE;
            }
            x if x == b'r' as c_int => {
                config.flags |= GPIO_V2_LINE_FLAG_EDGE_RISING;
            }
            x if x == b'f' as c_int => {
                config.flags |= GPIO_V2_LINE_FLAG_EDGE_FALLING;
            }
            x if x == b'w' as c_int => {
                config.flags |= GPIO_V2_LINE_FLAG_EVENT_CLOCK_REALTIME;
            }
            x if x == b't' as c_int => {
                config.flags |= GPIO_V2_LINE_FLAG_EVENT_CLOCK_HTE;
            }
            x if x == b'?' as c_int => {
                print_usage();
                return -1;
            }
            _ => {}
        }
    }

    if debounce_period_us != 0 {
        attr = config.num_attrs as c_int;
        config.num_attrs = config.num_attrs.wrapping_add(1);
        i = 0;
        while i < num_lines as c_int {
            gpiotools_set_bit(&mut config.attrs[attr as usize].mask as *mut u64, i);
            i += 1;
        }
        config.attrs[attr as usize].attr.id = GPIO_V2_LINE_ATTR_ID_DEBOUNCE;
        config.attrs[attr as usize].attr.debounce_period_us = debounce_period_us;
    }

    if device_name.is_null() || num_lines == 0 {
        print_usage();
        return -1;
    }
    if (config.flags & EDGE_FLAGS) == 0 {
        printf(
            b"No flags specified, listening on both rising and falling edges\n\0".as_ptr()
                as *const c_char,
        );
        config.flags |= EDGE_FLAGS;
    }
    monitor_device(
        device_name,
        lines.as_mut_ptr(),
        num_lines,
        &mut config as *mut gpio_v2_line_config,
        loops,
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
