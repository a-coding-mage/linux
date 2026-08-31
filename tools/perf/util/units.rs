// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/units.c. Original includes:
// "units.h", <inttypes.h>, <limits.h>, <stdlib.h>, <string.h>,
// <linux/kernel.h>, <linux/time64.h>

use core::ffi::{c_char, c_int, c_ulong};

pub type u64 = u64;

#[repr(C)]
pub struct parse_tag {
    pub tag: c_char,
    pub mult: c_ulong,
}

unsafe extern "C" {
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn parse_tag_value(
    str_: *const c_char,
    tags: *mut parse_tag,
) -> c_ulong {
    let mut i = tags;

    while unsafe { (*i).tag } != 0 {
        let s = unsafe { strchr(str_, (*i).tag as c_int) };

        if !s.is_null() {
            let mut value: c_ulong;
            let mut endptr: *mut c_char = core::ptr::null_mut();

            value = unsafe { strtoul(str_, &mut endptr, 10) };
            if s != endptr {
                break;
            }

            if value > c_ulong::MAX / unsafe { (*i).mult } {
                break;
            }
            value *= unsafe { (*i).mult };
            return value;
        }
        i = unsafe { i.add(1) };
    }

    -1isize as c_ulong
}

#[no_mangle]
pub unsafe extern "C" fn convert_unit_double(
    mut value: f64,
    unit: *mut c_char,
) -> f64 {
    unsafe {
        *unit = b' ' as c_char;
    }

    if value > 1000.0 {
        value /= 1000.0;
        unsafe {
            *unit = b'K' as c_char;
        }
    }

    if value > 1000.0 {
        value /= 1000.0;
        unsafe {
            *unit = b'M' as c_char;
        }
    }

    if value > 1000.0 {
        value /= 1000.0;
        unsafe {
            *unit = b'G' as c_char;
        }
    }

    value
}

#[no_mangle]
pub unsafe extern "C" fn convert_unit(
    value: c_ulong,
    unit: *mut c_char,
) -> c_ulong {
    let v = unsafe { convert_unit_double(value as f64, unit) };

    v as c_ulong
}

#[no_mangle]
pub unsafe extern "C" fn unit_number__scnprintf(
    buf: *mut c_char,
    size: usize,
    mut n: u64,
) -> c_int {
    let unit: [c_char; 5] = [
        b'B' as c_char,
        b'K' as c_char,
        b'M' as c_char,
        b'G' as c_char,
        0,
    ];
    let mut i: c_int = 0;

    while ((n / 1024) > 1) && (i < 3) {
        n /= 1024;
        i += 1;
    }

    unsafe {
        scnprintf(
            buf,
            size,
            b"%llu%c\0".as_ptr() as *const c_char,
            n,
            unit[i as usize] as c_int,
        )
    }
}
