// SPDX-License-Identifier: GPL-2.0-only
/*
 * GPIO tools - helpers library for the GPIO tools
 *
 * Copyright (C) 2015 Linus Walleij
 * Copyright (C) 2016 Bamvor Jian Zhang
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const CONSUMER: &[u8] = b"gpio-utils\0";

const ENOMEM: c_int = 12;

// Constants and types supplied by linux/gpio.h and gpio-utils.h.
extern "C" {
    static GPIO_V2_GET_LINE_IOCTL: c_ulong;
    static GPIO_V2_LINE_SET_VALUES_IOCTL: c_ulong;
    static GPIO_V2_LINE_GET_VALUES_IOCTL: c_ulong;
    static GPIO_V2_LINE_FLAG_INPUT: u64;
    static GPIO_V2_LINE_FLAG_OUTPUT: u64;
    static GPIO_V2_LINE_ATTR_ID_OUTPUT_VALUES: u32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_v2_line_attribute {
    pub id: u32,
    pub padding: u32,
    pub values: u64,
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
    pub attrs: [gpio_v2_line_config_attribute; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_v2_line_values {
    pub bits: u64,
    pub mask: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_v2_line_request {
    pub offsets: [u32; 64],
    pub consumer: [c_char; 32],
    pub config: gpio_v2_line_config,
    pub num_lines: u32,
    pub event_buffer_size: u32,
    pub padding: [u32; 5],
    pub fd: i32,
}

extern "C" {
    static mut errno: c_int;

    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);

    static mut stderr: *mut c_void;
}

#[inline]
unsafe fn gpiotools_set_bit(bits: *mut u64, bit: c_int) {
    *bits |= 1u64 << bit;
}

#[inline]
unsafe fn gpiotools_test_bit(bits: u64, bit: c_int) -> c_uint {
    ((bits >> bit) & 1) as c_uint
}

#[inline]
unsafe fn gpiotools_assign_bit(bits: *mut u64, bit: c_int, value: c_uint) {
    if value != 0 {
        gpiotools_set_bit(bits, bit);
    } else {
        *bits &= !(1u64 << bit);
    }
}

/**
 * DOC: Operation of gpio
 *
 * Provide the api of gpiochip for chardev interface. There are two
 * types of api.  The first one provide as same function as each
 * ioctl, including request and release for lines of gpio, read/write
 * the value of gpio. If the user want to do lots of read and write of
 * lines of gpio, user should use this type of api.
 *
 * The second one provide the easy to use api for user. Each of the
 * following api will request gpio lines, do the operation and then
 * release these lines.
 */

/**
 * gpiotools_request_line() - request gpio lines in a gpiochip
 * @device_name:	The name of gpiochip without prefix "/dev/",
 *			such as "gpiochip0"
 * @lines:		An array desired lines, specified by offset
 *			index for the associated GPIO device.
 * @num_lines:		The number of lines to request.
 * @config:		The new config for requested gpio. Reference
 *			"linux/gpio.h" for config details.
 * @consumer:		The name of consumer, such as "sysfs",
 *			"powerkey". This is useful for other users to
 *			know who is using.
 *
 * Request gpio lines through the ioctl provided by chardev. User
 * could call gpiotools_set_values() and gpiotools_get_values() to
 * read and write respectively through the returned fd. Call
 * gpiotools_release_line() to release these lines after that.
 *
 * Return:		On success return the fd;
 *			On failure return the errno.
 */
#[no_mangle]
pub unsafe extern "C" fn gpiotools_request_line(
    device_name: *const c_char,
    lines: *mut c_uint,
    num_lines: c_uint,
    config: *mut gpio_v2_line_config,
    consumer: *const c_char,
) -> c_int {
    let mut req: gpio_v2_line_request = core::mem::zeroed();
    let mut chrdev_name: *mut c_char = core::ptr::null_mut();
    let fd: c_int;
    let mut i: c_int;
    let mut ret: c_int;

    ret = asprintf(&mut chrdev_name, b"/dev/%s\0".as_ptr() as *const c_char, device_name);
    if ret < 0 {
        return -ENOMEM;
    }

    fd = open(chrdev_name, 0);
    if fd == -1 {
        ret = -errno;
        fprintf(
            stderr,
            b"Failed to open %s, %s\n\0".as_ptr() as *const c_char,
            chrdev_name,
            strerror(errno),
        );
        free(chrdev_name as *mut c_void);
        return ret;
    }

    memset(
        &mut req as *mut gpio_v2_line_request as *mut c_void,
        0,
        core::mem::size_of::<gpio_v2_line_request>(),
    );
    i = 0;
    while i < num_lines as c_int {
        req.offsets[i as usize] = *lines.add(i as usize);
        i += 1;
    }

    req.config = *config;
    strcpy(req.consumer.as_mut_ptr(), consumer);
    req.num_lines = num_lines;

    ret = ioctl(fd, GPIO_V2_GET_LINE_IOCTL, &mut req);
    if ret == -1 {
        ret = -errno;
        fprintf(
            stderr,
            b"Failed to issue %s (%d), %s\n\0".as_ptr() as *const c_char,
            b"GPIO_GET_LINE_IOCTL\0".as_ptr() as *const c_char,
            ret,
            strerror(errno),
        );
    }

    if close(fd) == -1 {
        perror(b"Failed to close GPIO character device file\0".as_ptr() as *const c_char);
    }
    free(chrdev_name as *mut c_void);
    if ret < 0 {
        ret
    } else {
        req.fd
    }
}

/**
 * gpiotools_set_values() - Set the value of gpio(s)
 * @fd:			The fd returned by
 *			gpiotools_request_line().
 * @values:		The array of values want to set.
 *
 * Return:		On success return 0;
 *			On failure return the errno.
 */
#[no_mangle]
pub unsafe extern "C" fn gpiotools_set_values(
    fd: c_int,
    values: *mut gpio_v2_line_values,
) -> c_int {
    let mut ret: c_int;

    ret = ioctl(fd, GPIO_V2_LINE_SET_VALUES_IOCTL, values);
    if ret == -1 {
        ret = -errno;
        fprintf(
            stderr,
            b"Failed to issue %s (%d), %s\n\0".as_ptr() as *const c_char,
            b"GPIOHANDLE_SET_LINE_VALUES_IOCTL\0".as_ptr() as *const c_char,
            ret,
            strerror(errno),
        );
    }

    ret
}

/**
 * gpiotools_get_values() - Get the value of gpio(s)
 * @fd:			The fd returned by
 *			gpiotools_request_line().
 * @values:		The array of values get from hardware.
 *
 * Return:		On success return 0;
 *			On failure return the errno.
 */
#[no_mangle]
pub unsafe extern "C" fn gpiotools_get_values(
    fd: c_int,
    values: *mut gpio_v2_line_values,
) -> c_int {
    let mut ret: c_int;

    ret = ioctl(fd, GPIO_V2_LINE_GET_VALUES_IOCTL, values);
    if ret == -1 {
        ret = -errno;
        fprintf(
            stderr,
            b"Failed to issue %s (%d), %s\n\0".as_ptr() as *const c_char,
            b"GPIOHANDLE_GET_LINE_VALUES_IOCTL\0".as_ptr() as *const c_char,
            ret,
            strerror(errno),
        );
    }

    ret
}

/**
 * gpiotools_release_line() - Release the line(s) of gpiochip
 * @fd:			The fd returned by
 *			gpiotools_request_line().
 *
 * Return:		On success return 0;
 *			On failure return the errno.
 */
#[no_mangle]
pub unsafe extern "C" fn gpiotools_release_line(fd: c_int) -> c_int {
    let mut ret: c_int;

    ret = close(fd);
    if ret == -1 {
        perror(b"Failed to close GPIO LINE device file\0".as_ptr() as *const c_char);
        ret = -errno;
    }

    ret
}

/**
 * gpiotools_get() - Get value from specific line
 * @device_name:	The name of gpiochip without prefix "/dev/",
 *			such as "gpiochip0"
 * @line:		number of line, such as 2.
 *
 * Return:		On success return 0;
 *			On failure return the errno.
 */
#[no_mangle]
pub unsafe extern "C" fn gpiotools_get(device_name: *const c_char, line: c_uint) -> c_int {
    let mut ret: c_int;
    let mut value: c_uint = 0;
    let mut lines: [c_uint; 1] = [line];

    ret = gpiotools_gets(device_name, lines.as_mut_ptr(), 1, &mut value);
    if ret != 0 {
        return ret;
    }
    value as c_int
}

/**
 * gpiotools_gets() - Get values from specific lines.
 * @device_name:	The name of gpiochip without prefix "/dev/",
 *			such as "gpiochip0".
 * @lines:		An array desired lines, specified by offset
 *			index for the associated GPIO device.
 * @num_lines:		The number of lines to request.
 * @values:		The array of values get from gpiochip.
 *
 * Return:		On success return 0;
 *			On failure return the errno.
 */
#[no_mangle]
pub unsafe extern "C" fn gpiotools_gets(
    device_name: *const c_char,
    lines: *mut c_uint,
    num_lines: c_uint,
    values: *mut c_uint,
) -> c_int {
    let mut fd: c_int;
    let mut i: c_int;
    let mut ret: c_int;
    let ret_close: c_int;
    let mut config: gpio_v2_line_config = core::mem::zeroed();
    let mut lv: gpio_v2_line_values = core::mem::zeroed();

    memset(
        &mut config as *mut gpio_v2_line_config as *mut c_void,
        0,
        core::mem::size_of::<gpio_v2_line_config>(),
    );
    config.flags = GPIO_V2_LINE_FLAG_INPUT;
    ret = gpiotools_request_line(
        device_name,
        lines,
        num_lines,
        &mut config,
        CONSUMER.as_ptr() as *const c_char,
    );
    if ret < 0 {
        return ret;
    }

    fd = ret;
    i = 0;
    while i < num_lines as c_int {
        gpiotools_set_bit(&mut lv.mask, i);
        i += 1;
    }
    ret = gpiotools_get_values(fd, &mut lv);
    if ret == 0 {
        i = 0;
        while i < num_lines as c_int {
            *values.add(i as usize) = gpiotools_test_bit(lv.bits, i);
            i += 1;
        }
    }
    ret_close = gpiotools_release_line(fd);
    if ret < 0 {
        ret
    } else {
        ret_close
    }
}

/**
 * gpiotools_set() - Set value to specific line
 * @device_name:	The name of gpiochip without prefix "/dev/",
 *			such as "gpiochip0"
 * @line:		number of line, such as 2.
 * @value:		The value of gpio, must be 0(low) or 1(high).
 *
 * Return:		On success return 0;
 *			On failure return the errno.
 */
#[no_mangle]
pub unsafe extern "C" fn gpiotools_set(
    device_name: *const c_char,
    line: c_uint,
    value: c_uint,
) -> c_int {
    let mut lines: [c_uint; 1] = [line];
    let mut value = value;

    gpiotools_sets(device_name, lines.as_mut_ptr(), 1, &mut value)
}

/**
 * gpiotools_sets() - Set values to specific lines.
 * @device_name:	The name of gpiochip without prefix "/dev/",
 *			such as "gpiochip0".
 * @lines:		An array desired lines, specified by offset
 *			index for the associated GPIO device.
 * @num_lines:		The number of lines to request.
 * @values:		The array of values set to gpiochip, must be
 *			0(low) or 1(high).
 *
 * Return:		On success return 0;
 *			On failure return the errno.
 */
#[no_mangle]
pub unsafe extern "C" fn gpiotools_sets(
    device_name: *const c_char,
    lines: *mut c_uint,
    num_lines: c_uint,
    values: *mut c_uint,
) -> c_int {
    let mut ret: c_int;
    let mut i: c_int;
    let mut config: gpio_v2_line_config = core::mem::zeroed();

    memset(
        &mut config as *mut gpio_v2_line_config as *mut c_void,
        0,
        core::mem::size_of::<gpio_v2_line_config>(),
    );
    config.flags = GPIO_V2_LINE_FLAG_OUTPUT;
    config.num_attrs = 1;
    config.attrs[0].attr.id = GPIO_V2_LINE_ATTR_ID_OUTPUT_VALUES;
    i = 0;
    while i < num_lines as c_int {
        gpiotools_set_bit(&mut config.attrs[0].mask, i);
        gpiotools_assign_bit(&mut config.attrs[0].attr.values, i, *values.add(i as usize));
        i += 1;
    }
    ret = gpiotools_request_line(
        device_name,
        lines,
        num_lines,
        &mut config,
        CONSUMER.as_ptr() as *const c_char,
    );
    if ret < 0 {
        return ret;
    }

    gpiotools_release_line(ret)
}
