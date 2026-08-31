/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * intel_pt_log.h: Intel Processor Trace support
 * Copyright (c) 2013-2014, Intel Corporation.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// C header dependencies:
// #include <linux/compiler.h>
// #include <stdint.h>
// #include <inttypes.h>

#[repr(C)]
pub struct intel_pt_pkt {
    _private: [u8; 0],
}

#[repr(C)]
pub struct intel_pt_insn {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn intel_pt_log_fp() -> *mut c_void;
    pub fn intel_pt_log_enable(dump_log_on_error: bool, log_on_error_size: c_uint);
    pub fn intel_pt_log_disable();
    pub fn intel_pt_log_set_name(name: *const c_char);
    pub fn intel_pt_log_dump_buf();

    pub fn __intel_pt_log_packet(
        packet: *const intel_pt_pkt,
        pkt_len: c_int,
        pos: u64,
        buf: *const u8,
    );

    pub fn __intel_pt_log_insn(intel_pt_insn: *mut intel_pt_insn, ip: u64);
    pub fn __intel_pt_log_insn_no_data(intel_pt_insn: *mut intel_pt_insn, ip: u64);

    // C declaration used __printf(1, 2).
    pub fn __intel_pt_log(fmt: *const c_char, ...);

    pub static mut intel_pt_enable_logging: bool;
}

#[macro_export]
macro_rules! intel_pt_log {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            if $crate::intel_pt_enable_logging {
                $crate::__intel_pt_log($fmt, $($arg),*);
            }
        }
    }};
}

#[macro_export]
macro_rules! intel_pt_log_packet {
    ($arg:expr $(, $rest:expr)* $(,)?) => {{
        unsafe {
            if $crate::intel_pt_enable_logging {
                $crate::__intel_pt_log_packet($arg, $($rest),*);
            }
        }
    }};
}

#[macro_export]
macro_rules! intel_pt_log_insn {
    ($arg:expr $(, $rest:expr)* $(,)?) => {{
        unsafe {
            if $crate::intel_pt_enable_logging {
                $crate::__intel_pt_log_insn($arg, $($rest),*);
            }
        }
    }};
}

#[macro_export]
macro_rules! intel_pt_log_insn_no_data {
    ($arg:expr $(, $rest:expr)* $(,)?) => {{
        unsafe {
            if $crate::intel_pt_enable_logging {
                $crate::__intel_pt_log_insn_no_data($arg, $($rest),*);
            }
        }
    }};
}

pub const x64_fmt: &[u8] = b"0x%lx\0";

#[inline]
pub unsafe fn intel_pt_log_at(msg: *const c_char, u: u64) {
    intel_pt_log!(b"%s at 0x%lx\n\0".as_ptr() as *const c_char, msg, u);
}

#[inline]
pub unsafe fn intel_pt_log_to(msg: *const c_char, u: u64) {
    intel_pt_log!(b"%s to 0x%lx\n\0".as_ptr() as *const c_char, msg, u);
}

#[macro_export]
macro_rules! intel_pt_log_var {
    ($var:expr, $fmt:expr) => {{
        $crate::intel_pt_log!(
            concat!("%s: ", stringify!($var), " ", $fmt, "\n\0").as_ptr()
                as *const core::ffi::c_char,
            core::ffi::c"".as_ptr(),
            $var
        );
    }};
}

#[macro_export]
macro_rules! intel_pt_log_x32 {
    ($var:expr) => {
        $crate::intel_pt_log_var!($var, "%#x")
    };
}

#[macro_export]
macro_rules! intel_pt_log_x64 {
    ($var:expr) => {
        $crate::intel_pt_log_var!($var, "%#lx")
    };
}
