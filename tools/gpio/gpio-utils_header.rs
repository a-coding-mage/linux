/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * GPIO tools - utility helpers library for the GPIO tools
 *
 * Copyright (C) 2015 Linus Walleij
 *
 * Portions copied from iio_utils and lssio:
 * Copyright (c) 2010 Manuel Stahl <manuel.stahl@iis.fraunhofer.de>
 * Copyright (c) 2008 Jonathan Cameron
 * *
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

/*
 * C dependencies removed from executable Rust:
 * - <stdbool.h>
 * - <string.h>
 * - <linux/types.h>
 */

pub type __u64 = u64;

pub enum gpio_v2_line_config {}
pub enum gpio_v2_line_values {}

pub const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
}

#[inline]
pub unsafe fn check_prefix(str_: *const c_char, prefix: *const c_char) -> c_int {
    let str_len = unsafe { strlen(str_) };
    let prefix_len = unsafe { strlen(prefix) };

    (str_len > prefix_len && unsafe { strncmp(str_, prefix, prefix_len) } == 0) as c_int
}

unsafe extern "C" {
    pub fn gpiotools_request_line(
        device_name: *const c_char,
        lines: *mut c_uint,
        num_lines: c_uint,
        config: *mut gpio_v2_line_config,
        consumer: *const c_char,
    ) -> c_int;
    pub fn gpiotools_set_values(fd: c_int, values: *mut gpio_v2_line_values) -> c_int;
    pub fn gpiotools_get_values(fd: c_int, values: *mut gpio_v2_line_values) -> c_int;
    pub fn gpiotools_release_line(fd: c_int) -> c_int;

    pub fn gpiotools_get(device_name: *const c_char, line: c_uint) -> c_int;
    pub fn gpiotools_gets(
        device_name: *const c_char,
        lines: *mut c_uint,
        num_lines: c_uint,
        values: *mut c_uint,
    ) -> c_int;
    pub fn gpiotools_set(device_name: *const c_char, line: c_uint, value: c_uint) -> c_int;
    pub fn gpiotools_sets(
        device_name: *const c_char,
        lines: *mut c_uint,
        num_lines: c_uint,
        values: *mut c_uint,
    ) -> c_int;
}

#[inline]
pub const fn _BITULL(n: c_int) -> __u64 {
    1u64 << (n as u32)
}

/* helper functions for gpio_v2_line_values bits */
#[inline]
pub unsafe fn gpiotools_set_bit(b: *mut __u64, n: c_int) {
    unsafe {
        *b |= _BITULL(n);
    }
}

#[inline]
pub unsafe fn gpiotools_change_bit(b: *mut __u64, n: c_int) {
    unsafe {
        *b ^= _BITULL(n);
    }
}

#[inline]
pub unsafe fn gpiotools_clear_bit(b: *mut __u64, n: c_int) {
    unsafe {
        *b &= !_BITULL(n);
    }
}

#[inline]
pub fn gpiotools_test_bit(b: __u64, n: c_int) -> c_int {
    ((b & _BITULL(n)) != 0) as c_int
}

#[inline]
pub unsafe fn gpiotools_assign_bit(b: *mut __u64, n: c_int, value: bool) {
    if value {
        unsafe {
            gpiotools_set_bit(b, n);
        }
    } else {
        unsafe {
            gpiotools_clear_bit(b, n);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
