// SPDX-License-Identifier: GPL-2.0
/*
 * GPIO mockup cdev test helper
 *
 * Copyright (C) 2020 Kent Gibson
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

const CONSUMER: &[u8] = b"gpio-mockup-cdev\0";

const GPIO_V2_LINES_MAX: usize = 64;
const GPIO_V2_LINE_NUM_ATTRS_MAX: usize = 10;
const GPIOHANDLES_MAX: usize = 64;
const GPIO_MAX_NAME_SIZE: usize = 32;

/* Constants supplied by <linux/gpio.h> in the original C source. */
unsafe extern "C" {
    static GPIO_V2_LINE_FLAG_INPUT: u64;
    static GPIO_V2_LINE_FLAG_OUTPUT: u64;
    static GPIO_V2_LINE_FLAG_ACTIVE_LOW: u64;
    static GPIO_V2_LINE_FLAG_BIAS_PULL_UP: u64;
    static GPIO_V2_LINE_FLAG_BIAS_PULL_DOWN: u64;
    static GPIO_V2_LINE_FLAG_BIAS_DISABLED: u64;
    static GPIO_V2_LINE_ATTR_ID_OUTPUT_VALUES: u32;
    static GPIOHANDLE_REQUEST_INPUT: u32;
    static GPIOHANDLE_REQUEST_OUTPUT: u32;
    static GPIOHANDLE_REQUEST_ACTIVE_LOW: u32;
    static GPIOHANDLE_REQUEST_BIAS_PULL_UP: u32;
    static GPIOHANDLE_REQUEST_BIAS_PULL_DOWN: u32;
    static GPIOHANDLE_REQUEST_BIAS_DISABLE: u32;
    static GPIO_V2_GET_LINE_IOCTL: c_ulong;
    static GPIO_V2_LINE_GET_VALUES_IOCTL: c_ulong;
    static GPIO_GET_LINEHANDLE_IOCTL: c_ulong;
    static GPIOHANDLE_GET_LINE_VALUES_IOCTL: c_ulong;
}

const SIGHUP: c_int = 1;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;

#[repr(C)]
struct sigset_t {
    __val: [c_ulong; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
union gpio_v2_line_attribute_data {
    values: u64,
    debounce_period_us: u32,
    event_clock: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct gpio_v2_line_attribute {
    id: u32,
    padding: u32,
    data: gpio_v2_line_attribute_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct gpio_v2_line_config_attribute {
    attr: gpio_v2_line_attribute,
    mask: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct gpio_v2_line_config {
    flags: u64,
    num_attrs: u32,
    padding: [u32; 5],
    attrs: [gpio_v2_line_config_attribute; GPIO_V2_LINE_NUM_ATTRS_MAX],
}

#[repr(C)]
struct gpio_v2_line_request {
    offsets: [u32; GPIO_V2_LINES_MAX],
    consumer: [c_char; GPIO_MAX_NAME_SIZE],
    config: gpio_v2_line_config,
    num_lines: u32,
    event_buffer_size: u32,
    padding: [u32; 5],
    fd: i32,
}

#[repr(C)]
struct gpio_v2_line_values {
    bits: u64,
    mask: u64,
}

#[repr(C)]
struct gpiohandle_request {
    lineoffsets: [u32; GPIOHANDLES_MAX],
    flags: u32,
    default_values: [u8; GPIOHANDLES_MAX],
    consumer_label: [c_char; GPIO_MAX_NAME_SIZE],
    lines: u32,
    fd: i32,
}

#[repr(C)]
struct gpiohandle_data {
    values: [u8; GPIOHANDLES_MAX],
}

unsafe extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int;
    fn sigwait(set: *const sigset_t, sig: *mut c_int) -> c_int;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn __errno_location() -> *mut c_int;

    static mut optarg: *mut c_char;
    static mut optind: c_int;
    static mut stderr: *mut c_void;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn request_line_v2(cfd: c_int, offset: c_uint, flags: u64, val: c_uint) -> c_int {
    let mut req: gpio_v2_line_request = mem::zeroed();
    let mut ret: c_int;

    memset(
        &mut req as *mut gpio_v2_line_request as *mut c_void,
        0,
        mem::size_of::<gpio_v2_line_request>(),
    );
    req.num_lines = 1;
    req.offsets[0] = offset;
    req.config.flags = flags;
    strcpy(req.consumer.as_mut_ptr(), CONSUMER.as_ptr() as *const c_char);
    if flags & GPIO_V2_LINE_FLAG_OUTPUT != 0 {
        req.config.num_attrs = 1;
        req.config.attrs[0].mask = 1;
        req.config.attrs[0].attr.id = GPIO_V2_LINE_ATTR_ID_OUTPUT_VALUES;
        if val != 0 {
            req.config.attrs[0].attr.data.values = 1;
        }
    }
    ret = ioctl(cfd, GPIO_V2_GET_LINE_IOCTL, &mut req);
    if ret == -1 {
        return -errno();
    }
    req.fd
}

unsafe fn get_value_v2(lfd: c_int) -> c_int {
    let mut vals: gpio_v2_line_values = mem::zeroed();
    let mut ret: c_int;

    memset(
        &mut vals as *mut gpio_v2_line_values as *mut c_void,
        0,
        mem::size_of::<gpio_v2_line_values>(),
    );
    vals.mask = 1;
    ret = ioctl(lfd, GPIO_V2_LINE_GET_VALUES_IOCTL, &mut vals);
    if ret == -1 {
        return -errno();
    }
    (vals.bits & 0x1) as c_int
}

unsafe fn request_line_v1(cfd: c_int, offset: c_uint, flags: u32, val: c_uint) -> c_int {
    let mut req: gpiohandle_request = mem::zeroed();
    let mut ret: c_int;

    memset(
        &mut req as *mut gpiohandle_request as *mut c_void,
        0,
        mem::size_of::<gpiohandle_request>(),
    );
    req.lines = 1;
    req.lineoffsets[0] = offset;
    req.flags = flags;
    strcpy(
        req.consumer_label.as_mut_ptr(),
        CONSUMER.as_ptr() as *const c_char,
    );
    if flags & GPIOHANDLE_REQUEST_OUTPUT != 0 {
        req.default_values[0] = val as u8;
    }

    ret = ioctl(cfd, GPIO_GET_LINEHANDLE_IOCTL, &mut req);
    if ret == -1 {
        return -errno();
    }
    req.fd
}

unsafe fn get_value_v1(lfd: c_int) -> c_int {
    let mut vals: gpiohandle_data = mem::zeroed();
    let mut ret: c_int;

    memset(
        &mut vals as *mut gpiohandle_data as *mut c_void,
        0,
        mem::size_of::<gpiohandle_data>(),
    );
    ret = ioctl(lfd, GPIOHANDLE_GET_LINE_VALUES_IOCTL, &mut vals);
    if ret == -1 {
        return -errno();
    }
    vals.values[0] as c_int
}

unsafe fn usage(prog: *mut c_char) {
    printf(
        b"Usage: %s [-l] [-b <bias>] [-s <value>] [-u <uAPI>] <gpiochip> <offset>\n\0".as_ptr()
            as *const c_char,
        prog,
    );
    printf(b"        -b: set line bias to one of pull-down, pull-up, disabled\n\0".as_ptr() as *const c_char);
    printf(b"               (default is to leave bias unchanged):\n\0".as_ptr() as *const c_char);
    printf(b"        -l: set line active low (default is active high)\n\0".as_ptr() as *const c_char);
    printf(b"        -s: set line value (default is to get line value)\n\0".as_ptr() as *const c_char);
    printf(b"        -u: uAPI version to use (default is 2)\n\0".as_ptr() as *const c_char);
    exit(-1);
}

unsafe fn wait_signal() -> c_int {
    let mut sig: c_int = 0;
    let mut wset: sigset_t = mem::zeroed();

    sigemptyset(&mut wset);
    sigaddset(&mut wset, SIGHUP);
    sigaddset(&mut wset, SIGINT);
    sigaddset(&mut wset, SIGTERM);
    sigwait(&wset, &mut sig);

    sig
}

unsafe fn c_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut chip: *mut c_char;
    let mut opt: c_int;
    let mut ret: c_int;
    let mut cfd: c_int;
    let mut lfd: c_int;
    let mut offset: c_uint;
    let mut val: c_uint = 0;
    let mut abiv: c_uint;
    let mut flags_v1: u32;
    let mut flags_v2: u64;

    abiv = 2;
    ret = 0;
    flags_v1 = GPIOHANDLE_REQUEST_INPUT;
    flags_v2 = GPIO_V2_LINE_FLAG_INPUT;

    loop {
        opt = getopt(argc, argv, b"lb:s:u:\0".as_ptr() as *const c_char);
        if opt == -1 {
            break;
        }
        match opt {
            x if x == b'l' as c_int => {
                flags_v1 |= GPIOHANDLE_REQUEST_ACTIVE_LOW;
                flags_v2 |= GPIO_V2_LINE_FLAG_ACTIVE_LOW;
            }
            x if x == b'b' as c_int => {
                if strcmp(b"pull-up\0".as_ptr() as *const c_char, optarg) == 0 {
                    flags_v1 |= GPIOHANDLE_REQUEST_BIAS_PULL_UP;
                    flags_v2 |= GPIO_V2_LINE_FLAG_BIAS_PULL_UP;
                } else if strcmp(b"pull-down\0".as_ptr() as *const c_char, optarg) == 0 {
                    flags_v1 |= GPIOHANDLE_REQUEST_BIAS_PULL_DOWN;
                    flags_v2 |= GPIO_V2_LINE_FLAG_BIAS_PULL_DOWN;
                } else if strcmp(b"disabled\0".as_ptr() as *const c_char, optarg) == 0 {
                    flags_v1 |= GPIOHANDLE_REQUEST_BIAS_DISABLE;
                    flags_v2 |= GPIO_V2_LINE_FLAG_BIAS_DISABLED;
                }
            }
            x if x == b's' as c_int => {
                val = atoi(optarg) as c_uint;
                flags_v1 &= !GPIOHANDLE_REQUEST_INPUT;
                flags_v1 |= GPIOHANDLE_REQUEST_OUTPUT;
                flags_v2 &= !GPIO_V2_LINE_FLAG_INPUT;
                flags_v2 |= GPIO_V2_LINE_FLAG_OUTPUT;
            }
            x if x == b'u' as c_int => {
                abiv = atoi(optarg) as c_uint;
            }
            _ => {
                usage(*argv);
            }
        }
    }

    if argc < optind + 2 {
        usage(*argv);
    }

    chip = *argv.offset(optind as isize);
    offset = atoi(*argv.offset((optind + 1) as isize)) as c_uint;

    cfd = open(chip, 0);
    if cfd == -1 {
        fprintf(
            stderr,
            b"Failed to open %s: %s\n\0".as_ptr() as *const c_char,
            chip,
            strerror(errno()),
        );
        return -errno();
    }

    if abiv == 1 {
        lfd = request_line_v1(cfd, offset, flags_v1, val);
    } else {
        lfd = request_line_v2(cfd, offset, flags_v2, val);
    }

    close(cfd);

    if lfd < 0 {
        fprintf(
            stderr,
            b"Failed to request %s:%d: %s\n\0".as_ptr() as *const c_char,
            chip,
            offset,
            strerror(-lfd),
        );
        return lfd;
    }

    if flags_v2 & GPIO_V2_LINE_FLAG_OUTPUT != 0 {
        wait_signal();
    } else if abiv == 1 {
        ret = get_value_v1(lfd);
    } else {
        ret = get_value_v2(lfd);
    }

    close(lfd);

    ret
}

fn main() {
    let mut args: Vec<*mut c_char> = std::env::args()
        .map(|arg| {
            let mut bytes = arg.into_bytes();
            bytes.push(0);
            Box::into_raw(bytes.into_boxed_slice()) as *mut c_char
        })
        .collect();
    args.push(ptr::null_mut());

    let ret = unsafe { c_main((args.len() - 1) as c_int, args.as_mut_ptr()) };
    std::process::exit(ret);
}
