/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2020 IBM Corporation
 *
 */

// C header dependencies: <sys/file.h>, <stdint.h>, <stdio.h>, <time.h>,
// and <pthread.h>. Their types and functions are expected to be supplied by
// the surrounding translated build.

use core::ffi::{c_char, c_int, c_uint};

unsafe extern "C" {
    pub static mut nx_gzip_log: *mut FILE;
    pub static mut nx_gzip_trace: c_int;
    pub static mut nx_gzip_inflate_impl: c_uint;
    pub static mut nx_gzip_deflate_impl: c_uint;
    pub static mut nx_gzip_inflate_flags: c_uint;
    pub static mut nx_gzip_deflate_flags: c_uint;

    pub static mut nx_dbg: c_int;
}

unsafe extern "C" {
    pub static mut mutex_log: pthread_mutex_t;
}

unsafe extern "C" {
    pub fn str_to_num(str: *mut c_char) -> u64;
    pub fn nx_lib_debug(onoff: c_int);
}

#[inline]
pub unsafe fn nx_gzip_trace_enabled() -> c_int {
    unsafe { nx_gzip_trace & 0x1 }
}

#[inline]
pub unsafe fn nx_gzip_hw_trace_enabled() -> c_int {
    unsafe { nx_gzip_trace & 0x2 }
}

#[inline]
pub unsafe fn nx_gzip_sw_trace_enabled() -> c_int {
    unsafe { nx_gzip_trace & 0x4 }
}

#[inline]
pub unsafe fn nx_gzip_gather_statistics() -> c_int {
    unsafe { nx_gzip_trace & 0x8 }
}

#[inline]
pub unsafe fn nx_gzip_per_stream_stat() -> c_int {
    unsafe { nx_gzip_trace & 0x10 }
}

#[macro_export]
macro_rules! prt {
    ($fmt:expr $(, $args:expr)* $(,)?) => {{
        unsafe {
            pthread_mutex_lock(&raw mut mutex_log);
            flock((*nx_gzip_log)._fileno, LOCK_EX);
            let mut t: time_t = 0;
            let mut m: *mut tm;
            time(&mut t);
            m = localtime(&mut t);
            fprintf(
                nx_gzip_log,
                concat!("[%04d/%02d/%02d %02d:%02d:%02d] pid %d: ", $fmt).as_ptr()
                    as *const core::ffi::c_char,
                (*m).tm_year as core::ffi::c_int + 1900,
                (*m).tm_mon as core::ffi::c_int + 1,
                (*m).tm_mday as core::ffi::c_int,
                (*m).tm_hour as core::ffi::c_int,
                (*m).tm_min as core::ffi::c_int,
                (*m).tm_sec as core::ffi::c_int,
                getpid() as core::ffi::c_int
                $(, $args)*
            );
            fflush(nx_gzip_log);
            flock((*nx_gzip_log)._fileno, LOCK_UN);
            pthread_mutex_unlock(&raw mut mutex_log);
        }
    }};
}

/* Use in case of an error */
#[macro_export]
macro_rules! prt_err {
    ($fmt:expr $(, $args:expr)* $(,)?) => {{
        unsafe {
            if nx_dbg >= 0 {
                prt!(
                    concat!("%s:%u: Error: ", $fmt),
                    file!().as_ptr() as *const core::ffi::c_char,
                    line!() as core::ffi::c_uint
                    $(, $args)*
                );
            }
        }
    }};
}

/* Use in case of an warning */
#[macro_export]
macro_rules! prt_warn {
    ($fmt:expr $(, $args:expr)* $(,)?) => {{
        unsafe {
            if nx_dbg >= 1 {
                prt!(
                    concat!("%s:%u: Warning: ", $fmt),
                    file!().as_ptr() as *const core::ffi::c_char,
                    line!() as core::ffi::c_uint
                    $(, $args)*
                );
            }
        }
    }};
}

/* Informational printouts */
#[macro_export]
macro_rules! prt_info {
    ($fmt:expr $(, $args:expr)* $(,)?) => {{
        unsafe {
            if nx_dbg >= 2 {
                prt!(concat!("Info: ", $fmt) $(, $args)*);
            }
        }
    }};
}

/* Trace zlib wrapper code */
#[macro_export]
macro_rules! prt_trace {
    ($fmt:expr $(, $args:expr)* $(,)?) => {{
        unsafe {
            if nx_gzip_trace_enabled() != 0 {
                prt!(concat!("### ", $fmt) $(, $args)*);
            }
        }
    }};
}

/* Trace statistics */
#[macro_export]
macro_rules! prt_stat {
    ($fmt:expr $(, $args:expr)* $(,)?) => {{
        unsafe {
            if nx_gzip_gather_statistics() != 0 {
                prt!(concat!("### ", $fmt) $(, $args)*);
            }
        }
    }};
}

/* Trace zlib hardware implementation */
#[macro_export]
macro_rules! hw_trace {
    ($fmt:expr $(, $args:expr)* $(,)?) => {{
        unsafe {
            if nx_gzip_hw_trace_enabled() != 0 {
                fprintf(
                    nx_gzip_log,
                    concat!("hhh ", $fmt).as_ptr() as *const core::ffi::c_char
                    $(, $args)*
                );
            }
        }
    }};
}

/* Trace zlib software implementation */
#[macro_export]
macro_rules! sw_trace {
    ($fmt:expr $(, $args:expr)* $(,)?) => {{
        unsafe {
            if nx_gzip_sw_trace_enabled() != 0 {
                fprintf(
                    nx_gzip_log,
                    concat!("sss ", $fmt).as_ptr() as *const core::ffi::c_char
                    $(, $args)*
                );
            }
        }
    }};
}

/**
 * str_to_num - Convert string into number and copy with endings like
 *              KiB for kilobyte
 *              MiB for megabyte
 *              GiB for gigabyte
 */
