/* SPDX-License-Identifier: GPL-2.0-only */

/* IIO - useful set of util functionality
 *
 * Copyright (c) 2008 Jonathan Cameron
 */

use core::ffi::{c_char, c_float, c_int, c_uint};

/* Dependency intent from C header: <stdint.h> */

/* Made up value to limit allocation sizes */
pub const IIO_MAX_NAME_LENGTH: usize = 64;

pub const FORMAT_SCAN_ELEMENTS_DIR: &[u8; 12] = b"%s/buffer%d\0";
pub const FORMAT_EVENTS_DIR: &[u8; 10] = b"%s/events\0";
pub const FORMAT_TYPE_FILE: &[u8; 8] = b"%s_type\0";

pub const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

unsafe extern "C" {
    pub static iio_dir: *const c_char;

    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
}

/**
 * struct iio_channel_info - information about a given channel
 * @name: channel name
 * @generic_name: general name for channel type
 * @scale: scale factor to be applied for conversion to si units
 * @offset: offset to be applied for conversion to si units
 * @index: the channel index in the buffer output
 * @bytes: number of bytes occupied in buffer output
 * @bits_used: number of valid bits of data
 * @shift: amount of bits to shift right data before applying bit mask
 * @mask: a bit mask for the raw output
 * @be: flag if data is big endian
 * @format: format of the raw value
 * @location: data offset for this channel inside the buffer (in bytes)
 **/
#[repr(C)]
pub struct iio_channel_info {
    pub name: *mut c_char,
    pub generic_name: *mut c_char,
    pub scale: c_float,
    pub offset: c_float,
    pub index: c_uint,
    pub bytes: c_uint,
    pub bits_used: c_uint,
    pub shift: c_uint,
    pub mask: u64,
    pub be: c_uint,
    pub format: c_char,
    pub location: c_uint,
}

#[inline]
pub unsafe fn iioutils_check_suffix(str_: *const c_char, suffix: *const c_char) -> c_int {
    if unsafe { strlen(str_) } >= unsafe { strlen(suffix) }
        && unsafe {
            strncmp(
                str_.add(strlen(str_) - strlen(suffix)),
                suffix,
                strlen(suffix),
            )
        } == 0
    {
        1
    } else {
        0
    }
}

unsafe extern "C" {
    pub fn iioutils_break_up_name(
        full_name: *const c_char,
        generic_name: *mut *mut c_char,
    ) -> c_int;
    pub fn iioutils_get_param_float(
        output: *mut c_float,
        param_name: *const c_char,
        device_dir: *const c_char,
        name: *const c_char,
        generic_name: *const c_char,
    ) -> c_int;
    pub fn bsort_channel_array_by_index(ci_array: *mut iio_channel_info, cnt: c_int);
    pub fn build_channel_array(
        device_dir: *const c_char,
        buffer_idx: c_int,
        ci_array: *mut *mut iio_channel_info,
        counter: *mut c_int,
    ) -> c_int;
    pub fn find_type_by_name(name: *const c_char, type_: *const c_char) -> c_int;
    pub fn write_sysfs_int(filename: *const c_char, basedir: *const c_char, val: c_int) -> c_int;
    pub fn write_sysfs_int_and_verify(
        filename: *const c_char,
        basedir: *const c_char,
        val: c_int,
    ) -> c_int;
    pub fn write_sysfs_string_and_verify(
        filename: *const c_char,
        basedir: *const c_char,
        val: *const c_char,
    ) -> c_int;
    pub fn write_sysfs_string(
        filename: *const c_char,
        basedir: *const c_char,
        val: *const c_char,
    ) -> c_int;
    pub fn read_sysfs_posint(filename: *const c_char, basedir: *const c_char) -> c_int;
    pub fn read_sysfs_float(
        filename: *const c_char,
        basedir: *const c_char,
        val: *mut c_float,
    ) -> c_int;
    pub fn read_sysfs_string(
        filename: *const c_char,
        basedir: *const c_char,
        str_: *mut c_char,
    ) -> c_int;
}
