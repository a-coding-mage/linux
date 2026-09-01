// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * GPIO character device helper for UAF tests.
 *
 * Copyright 2026 Google LLC
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::zeroed;
use core::ptr::null_mut;

const CONFIGFS_DIR: &[u8] = b"/sys/kernel/config/gpio-sim\0";
const PROCFS_DIR: &[u8] = b"/proc\0";

const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;
const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_RDWR: c_int = 2;
const POLLIN: c_short = 0x0001;
const POLLERR: c_short = 0x0008;
const POLLHUP: c_short = 0x0010;
const ENODEV: c_int = 19;

type c_short = i16;
type ssize_t = isize;
type size_t = usize;
type mode_t = u32;
type nfds_t = c_ulong;

// Constants supplied by <linux/gpio.h>.
extern "C" {
    static GPIO_GET_LINEHANDLE_IOCTL: c_ulong;
    static GPIO_GET_LINEEVENT_IOCTL: c_ulong;
    static GPIO_V2_GET_LINE_IOCTL: c_ulong;
}

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: c_short,
    revents: c_short,
}

#[repr(C)]
struct gpiohandle_request {
    lineoffsets: [u32; 64],
    flags: u32,
    default_values: [u8; 64],
    consumer_label: [c_char; 32],
    lines: u32,
    fd: c_int,
}

#[repr(C)]
struct gpioevent_request {
    lineoffset: u32,
    handleflags: u32,
    eventflags: u32,
    consumer_label: [c_char; 32],
    fd: c_int,
}

#[repr(C)]
union gpio_v2_line_attribute_union {
    id: u32,
    flags: u64,
    values: u64,
    debounce_period_us: u32,
}

#[repr(C)]
struct gpio_v2_line_attribute {
    attr: gpio_v2_line_attribute_union,
    mask: u64,
}

#[repr(C)]
struct gpio_v2_line_config {
    flags: u64,
    num_attrs: u32,
    padding: [u32; 5],
    attrs: [gpio_v2_line_attribute; 10],
}

#[repr(C)]
struct gpio_v2_line_request {
    offsets: [u32; 64],
    consumer: [c_char; 32],
    config: gpio_v2_line_config,
    num_lines: u32,
    event_buffer_size: u32,
    padding: [u32; 5],
    fd: c_int,
}

extern "C" {
    fn __errno_location() -> *mut c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn mkdir(pathname: *const c_char, mode: mode_t) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: c_int) -> c_int;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn print_usage() {
    printf(b"usage:\n\0".as_ptr() as *const c_char);
    printf(b"  gpio-cdev-uaf [chip|handle|event|req] [poll|read|ioctl]\n\0".as_ptr() as *const c_char);
}

unsafe fn _create_chip(name: *const c_char, create: c_int) -> c_int {
    let mut path = [0 as c_char; 64];

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        b"%s/%s\0".as_ptr() as *const c_char,
        CONFIGFS_DIR.as_ptr() as *const c_char,
        name,
    );

    if create != 0 {
        mkdir(path.as_ptr(), 0o755)
    } else {
        rmdir(path.as_ptr())
    }
}

unsafe fn create_chip(name: *const c_char) -> c_int {
    _create_chip(name, 1)
}

unsafe fn remove_chip(name: *const c_char) {
    _create_chip(name, 0);
}

unsafe fn _create_bank(chip_name: *const c_char, name: *const c_char, create: c_int) -> c_int {
    let mut path = [0 as c_char; 64];

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        b"%s/%s/%s\0".as_ptr() as *const c_char,
        CONFIGFS_DIR.as_ptr() as *const c_char,
        chip_name,
        name,
    );

    if create != 0 {
        mkdir(path.as_ptr(), 0o755)
    } else {
        rmdir(path.as_ptr())
    }
}

unsafe fn create_bank(chip_name: *const c_char, name: *const c_char) -> c_int {
    _create_bank(chip_name, name, 1)
}

unsafe fn remove_bank(chip_name: *const c_char, name: *const c_char) {
    _create_bank(chip_name, name, 0);
}

unsafe fn _enable_chip(name: *const c_char, enable: c_int) -> c_int {
    let mut path = [0 as c_char; 64];
    let fd: c_int;
    let ret: ssize_t;

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        b"%s/%s/live\0".as_ptr() as *const c_char,
        CONFIGFS_DIR.as_ptr() as *const c_char,
        name,
    );

    fd = open(path.as_ptr(), O_WRONLY);
    if fd == -1 {
        return fd;
    }

    if enable != 0 {
        ret = write(fd, b"1\0".as_ptr() as *const c_void, 1);
    } else {
        ret = write(fd, b"0\0".as_ptr() as *const c_void, 1);
    }

    close(fd);
    if ret == 1 { 0 } else { -1 }
}

unsafe fn enable_chip(name: *const c_char) -> c_int {
    _enable_chip(name, 1)
}

unsafe fn disable_chip(name: *const c_char) {
    _enable_chip(name, 0);
}

unsafe fn open_chip(chip_name: *const c_char, bank_name: *const c_char) -> c_int {
    let mut path = [0 as c_char; 64];
    let mut dev_name = [0 as c_char; 32];
    let mut ret: c_int;
    let mut fd: c_int;

    ret = create_chip(chip_name);
    if ret != 0 {
        fprintf(stderr, b"failed to create chip\n\0".as_ptr() as *const c_char);
        return ret;
    }

    ret = create_bank(chip_name, bank_name);
    if ret != 0 {
        fprintf(stderr, b"failed to create bank\n\0".as_ptr() as *const c_char);
        remove_chip(chip_name);
        return ret;
    }

    ret = enable_chip(chip_name);
    if ret != 0 {
        fprintf(stderr, b"failed to enable chip\n\0".as_ptr() as *const c_char);
        remove_bank(chip_name, bank_name);
        remove_chip(chip_name);
        return ret;
    }

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        b"%s/%s/%s/chip_name\0".as_ptr() as *const c_char,
        CONFIGFS_DIR.as_ptr() as *const c_char,
        chip_name,
        bank_name,
    );

    fd = open(path.as_ptr(), O_RDONLY);
    if fd == -1 {
        ret = fd;
        fprintf(stderr, b"failed to open %s\n\0".as_ptr() as *const c_char, path.as_ptr());
        disable_chip(chip_name);
        remove_bank(chip_name, bank_name);
        remove_chip(chip_name);
        return ret;
    }

    let read_ret = read(fd, dev_name.as_mut_ptr() as *mut c_void, dev_name.len() - 1);
    close(fd);
    if read_ret == -1 {
        fprintf(stderr, b"failed to read %s\n\0".as_ptr() as *const c_char, path.as_ptr());
        disable_chip(chip_name);
        remove_bank(chip_name, bank_name);
        remove_chip(chip_name);
        return ret;
    }
    ret = read_ret as c_int;
    dev_name[ret as usize] = 0;
    if ret != 0 && dev_name[(ret - 1) as usize] == b'\n' as c_char {
        dev_name[(ret - 1) as usize] = 0;
    }

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        b"/dev/%s\0".as_ptr() as *const c_char,
        dev_name.as_ptr(),
    );

    fd = open(path.as_ptr(), O_RDWR);
    if fd == -1 {
        ret = fd;
        fprintf(stderr, b"failed to open %s\n\0".as_ptr() as *const c_char, path.as_ptr());
        disable_chip(chip_name);
        remove_bank(chip_name, bank_name);
        remove_chip(chip_name);
        return ret;
    }

    fd
}

unsafe fn close_chip(chip_name: *const c_char, bank_name: *const c_char) {
    disable_chip(chip_name);
    remove_bank(chip_name, bank_name);
    remove_chip(chip_name);
}

unsafe fn test_poll(fd: c_int) -> c_int {
    let mut pfds = pollfd {
        fd,
        events: POLLIN,
        revents: 0,
    };

    if poll(&mut pfds, 1, 0) == -1 {
        return -1;
    }

    if (pfds.revents & !(POLLHUP | POLLERR)) != 0 { -1 } else { 0 }
}

unsafe fn test_read(fd: c_int) -> c_int {
    let mut data = 0 as c_char;

    if read(fd, &mut data as *mut c_char as *mut c_void, 1) == -1 && errno() == ENODEV {
        return 0;
    }
    -1
}

unsafe fn test_ioctl(fd: c_int) -> c_int {
    if ioctl(fd, 0, null_mut::<c_void>()) == -1 && errno() == ENODEV {
        return 0;
    }
    -1
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let cfd: c_int;
    let fd: c_int;
    let ret: c_int;
    let test_func: unsafe fn(c_int) -> c_int;

    if argc != 3 {
        print_usage();
        return EXIT_FAILURE;
    }

    let arg1 = *argv.add(1);
    let arg2 = *argv.add(2);

    if strcmp(arg1, b"chip\0".as_ptr() as *const c_char) == 0
        || strcmp(arg1, b"event\0".as_ptr() as *const c_char) == 0
        || strcmp(arg1, b"req\0".as_ptr() as *const c_char) == 0
    {
        if strcmp(arg2, b"poll\0".as_ptr() as *const c_char) != 0
            && strcmp(arg2, b"read\0".as_ptr() as *const c_char) != 0
            && strcmp(arg2, b"ioctl\0".as_ptr() as *const c_char) != 0
        {
            fprintf(stderr, b"unknown command: %s\n\0".as_ptr() as *const c_char, arg2);
            return EXIT_FAILURE;
        }
    } else if strcmp(arg1, b"handle\0".as_ptr() as *const c_char) == 0 {
        if strcmp(arg2, b"ioctl\0".as_ptr() as *const c_char) != 0 {
            fprintf(stderr, b"unknown command: %s\n\0".as_ptr() as *const c_char, arg2);
            return EXIT_FAILURE;
        }
    } else {
        fprintf(stderr, b"unknown command: %s\n\0".as_ptr() as *const c_char, arg1);
        return EXIT_FAILURE;
    }

    if strcmp(arg2, b"poll\0".as_ptr() as *const c_char) == 0 {
        test_func = test_poll;
    } else if strcmp(arg2, b"read\0".as_ptr() as *const c_char) == 0 {
        test_func = test_read;
    } else {
        /* strcmp(argv[2], "ioctl") == 0 */
        test_func = test_ioctl;
    }

    cfd = open_chip(b"chip\0".as_ptr() as *const c_char, b"bank\0".as_ptr() as *const c_char);
    if cfd == -1 {
        fprintf(stderr, b"failed to open chip\n\0".as_ptr() as *const c_char);
        return EXIT_FAILURE;
    }

    /* Step 1: Hold a FD to the test target. */
    if strcmp(arg1, b"chip\0".as_ptr() as *const c_char) == 0 {
        fd = cfd;
    } else if strcmp(arg1, b"handle\0".as_ptr() as *const c_char) == 0 {
        let mut req: gpiohandle_request = zeroed();

        req.lines = 1;
        if ioctl(cfd, GPIO_GET_LINEHANDLE_IOCTL, &mut req as *mut gpiohandle_request) == -1 {
            fprintf(stderr, b"failed to get handle FD\n\0".as_ptr() as *const c_char);
            close(cfd);
            close_chip(b"chip\0".as_ptr() as *const c_char, b"bank\0".as_ptr() as *const c_char);
            return EXIT_FAILURE;
        }

        close(cfd);
        fd = req.fd;
    } else if strcmp(arg1, b"event\0".as_ptr() as *const c_char) == 0 {
        let mut req: gpioevent_request = zeroed();

        if ioctl(cfd, GPIO_GET_LINEEVENT_IOCTL, &mut req as *mut gpioevent_request) == -1 {
            fprintf(stderr, b"failed to get event FD\n\0".as_ptr() as *const c_char);
            close(cfd);
            close_chip(b"chip\0".as_ptr() as *const c_char, b"bank\0".as_ptr() as *const c_char);
            return EXIT_FAILURE;
        }

        close(cfd);
        fd = req.fd;
    } else {
        /* strcmp(argv[1], "req") == 0 */
        let mut req: gpio_v2_line_request = zeroed();

        req.num_lines = 1;
        if ioctl(cfd, GPIO_V2_GET_LINE_IOCTL, &mut req as *mut gpio_v2_line_request) == -1 {
            fprintf(stderr, b"failed to get req FD\n\0".as_ptr() as *const c_char);
            close(cfd);
            close_chip(b"chip\0".as_ptr() as *const c_char, b"bank\0".as_ptr() as *const c_char);
            return EXIT_FAILURE;
        }

        close(cfd);
        fd = req.fd;
    }

    /* Step 2: Free the chip. */
    close_chip(b"chip\0".as_ptr() as *const c_char, b"bank\0".as_ptr() as *const c_char);

    /* Step 3: Access the dangling FD to trigger UAF. */
    ret = test_func(fd);
    close(fd);
    if ret != 0 { EXIT_FAILURE } else { EXIT_SUCCESS }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
