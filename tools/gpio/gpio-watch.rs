// SPDX-License-Identifier: GPL-2.0-only
/*
 * gpio-watch - monitor unrequested lines for property changes using the
 *              character device
 *
 * Copyright (C) 2019 BayLibre SAS
 * Author: Bartosz Golaszewski <bgolaszewski@baylibre.com>
 */

use libc::{
    c_char, c_int, c_ulong, c_void, fprintf, memset, open, perror, poll, pollfd, printf, read,
    strtoul, ssize_t, stderr, O_CLOEXEC, O_RDWR, POLLIN, POLLPRI, EIO, EXIT_FAILURE,
};

extern "C" {
    static mut errno: c_int;
}

// From <linux/gpio.h>.
extern "C" {
    static GPIO_V2_GET_LINEINFO_WATCH_IOCTL: c_ulong;
    static GPIO_V2_LINE_CHANGED_REQUESTED: u32;
    static GPIO_V2_LINE_CHANGED_RELEASED: u32;
    static GPIO_V2_LINE_CHANGED_CONFIG: u32;
}

// From <linux/gpio.h>.
#[repr(C)]
pub struct gpio_v2_line_info {
    pub name: [c_char; 32],
    pub consumer: [c_char; 32],
    pub offset: u32,
    pub num_attrs: u32,
    pub flags: u64,
    pub attrs: [gpio_v2_line_attribute; 10],
    pub padding: [u32; 4],
}

// From <linux/gpio.h>.
#[repr(C)]
pub struct gpio_v2_line_info_changed {
    pub info: gpio_v2_line_info,
    pub timestamp_ns: u64,
    pub event_type: u32,
    pub padding: [u32; 5],
}

// From <linux/gpio.h>.
#[repr(C)]
pub struct gpio_v2_line_attribute {
    pub id: u32,
    pub padding: u32,
    pub attr: gpio_v2_line_attribute__bindgen_ty_1,
}

// From <linux/gpio.h>.
#[repr(C)]
pub union gpio_v2_line_attribute__bindgen_ty_1 {
    pub flags: u64,
    pub values: u64,
    pub debounce_period_us: u32,
}

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

unsafe fn main_0(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut chg: gpio_v2_line_info_changed = std::mem::zeroed();
    let mut req: gpio_v2_line_info = std::mem::zeroed();
    let mut pfd: pollfd = std::mem::zeroed();
    let mut fd: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let mut ret: c_int;
    let mut event: *const c_char;
    let mut end: *mut c_char = std::ptr::null_mut();
    let mut rd: ssize_t;

    if argc < 3 {
        return err_usage(argv);
    }

    fd = open(
        *argv.offset(1),
        O_RDWR | O_CLOEXEC,
    );
    if fd < 0 {
        perror(b"unable to open gpiochip\0".as_ptr() as *const c_char);
        return EXIT_FAILURE;
    }

    i = 0;
    j = 2;
    while i < argc - 2 {
        memset(
            &mut req as *mut gpio_v2_line_info as *mut c_void,
            0,
            std::mem::size_of::<gpio_v2_line_info>(),
        );

        req.offset = strtoul(*argv.offset(j as isize), &mut end, 0) as u32;
        if *end != b'\0' as c_char {
            return err_usage(argv);
        }

        ret = ioctl(
            fd,
            GPIO_V2_GET_LINEINFO_WATCH_IOCTL,
            &mut req as *mut gpio_v2_line_info,
        );
        if ret != 0 {
            perror(b"unable to set up line watch\0".as_ptr() as *const c_char);
            return EXIT_FAILURE;
        }

        i += 1;
        j += 1;
    }

    pfd.fd = fd;
    pfd.events = (POLLIN | POLLPRI) as i16;

    loop {
        ret = poll(&mut pfd as *mut pollfd, 1, 5000);
        if ret < 0 {
            perror(b"error polling the linechanged fd\0".as_ptr() as *const c_char);
            return EXIT_FAILURE;
        } else if ret > 0 {
            memset(
                &mut chg as *mut gpio_v2_line_info_changed as *mut c_void,
                0,
                std::mem::size_of::<gpio_v2_line_info_changed>(),
            );
            rd = read(
                pfd.fd,
                &mut chg as *mut gpio_v2_line_info_changed as *mut c_void,
                std::mem::size_of::<gpio_v2_line_info_changed>(),
            );
            if rd < 0 || rd as usize != std::mem::size_of::<gpio_v2_line_info_changed>() {
                if rd as usize != std::mem::size_of::<gpio_v2_line_info_changed>() {
                    errno = EIO;
                }

                perror(b"error reading line change event\0".as_ptr() as *const c_char);
                return EXIT_FAILURE;
            }

            if chg.event_type == GPIO_V2_LINE_CHANGED_REQUESTED {
                event = b"requested\0".as_ptr() as *const c_char;
            } else if chg.event_type == GPIO_V2_LINE_CHANGED_RELEASED {
                event = b"released\0".as_ptr() as *const c_char;
            } else if chg.event_type == GPIO_V2_LINE_CHANGED_CONFIG {
                event = b"config changed\0".as_ptr() as *const c_char;
            } else {
                fprintf(
                    stderr,
                    b"invalid event type received from the kernel\n\0".as_ptr() as *const c_char,
                );
                return EXIT_FAILURE;
            }

            printf(
                b"line %u: %s at %lu\n\0".as_ptr() as *const c_char,
                chg.info.offset,
                event,
                chg.timestamp_ns as u64,
            );
        }
    }
}

unsafe fn err_usage(argv: *mut *mut c_char) -> c_int {
    printf(
        b"%s: <gpiochip> <line0> <line1> ...\n\0".as_ptr() as *const c_char,
        *argv.offset(0),
    );
    EXIT_FAILURE
}

fn main() {
    let mut args: Vec<*mut c_char> = std::env::args()
        .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
        .collect();
    args.push(std::ptr::null_mut());

    let ret = unsafe { main_0((args.len() - 1) as c_int, args.as_mut_ptr()) };

    for arg in args.into_iter().filter(|arg| !arg.is_null()) {
        unsafe {
            let _ = std::ffi::CString::from_raw(arg);
        }
    }

    std::process::exit(ret);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
