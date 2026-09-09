/*
 * linux/include/kmsg_dump.h
 *
 * Copyright (C) 2009 Net Insight AB
 *
 * Author: Simon Kagstrom <simon.kagstrom@netinsight.net>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by other translated kernel headers.
use core::ffi::c_char;

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

pub type size_t = usize;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum kmsg_dump_reason {
    KMSG_DUMP_UNDEF,
    KMSG_DUMP_PANIC,
    KMSG_DUMP_OOPS,
    KMSG_DUMP_EMERG,
    KMSG_DUMP_SHUTDOWN,
    KMSG_DUMP_MAX,
}

#[repr(C)]
pub struct kmsg_dump_iter {
    pub cur_seq: u64,
    pub next_seq: u64,
}

#[repr(C)]
pub struct kmsg_dump_detail {
    pub reason: kmsg_dump_reason,
    pub description: *const c_char,
}

#[repr(C)]
pub struct kmsg_dumper {
    pub list: list_head,
    pub dump: Option<unsafe extern "C" fn(
        dumper: *mut kmsg_dumper,
        detail: *mut kmsg_dump_detail,
    )>,
    pub max_reason: kmsg_dump_reason,
    pub registered: bool,
}

// CONFIG_PRINTK selects the external implementations below; without it the
// header provides the inline disabled stubs.
#[cfg(CONFIG_PRINTK)]
extern "C" {
    pub fn kmsg_dump_desc(reason: kmsg_dump_reason, desc: *const c_char);

    pub fn kmsg_dump_get_line(
        iter: *mut kmsg_dump_iter,
        syslog: bool,
        line: *mut c_char,
        size: size_t,
        len: *mut size_t,
    ) -> bool;

    pub fn kmsg_dump_get_buffer(
        iter: *mut kmsg_dump_iter,
        syslog: bool,
        buf: *mut c_char,
        size: size_t,
        len_out: *mut size_t,
    ) -> bool;

    pub fn kmsg_dump_rewind(iter: *mut kmsg_dump_iter);

    pub fn kmsg_dump_register(dumper: *mut kmsg_dumper) -> i32;

    pub fn kmsg_dump_unregister(dumper: *mut kmsg_dumper) -> i32;

    pub fn kmsg_dump_reason_str(reason: kmsg_dump_reason) -> *const c_char;
}

#[cfg(not(CONFIG_PRINTK))]
#[inline]
pub unsafe fn kmsg_dump_desc(_reason: kmsg_dump_reason, _desc: *const c_char) {}

#[cfg(not(CONFIG_PRINTK))]
#[inline]
pub unsafe fn kmsg_dump_get_line(
    _iter: *mut kmsg_dump_iter,
    _syslog: bool,
    _line: *const c_char,
    _size: size_t,
    _len: *mut size_t,
) -> bool {
    false
}

#[cfg(not(CONFIG_PRINTK))]
#[inline]
pub unsafe fn kmsg_dump_get_buffer(
    _iter: *mut kmsg_dump_iter,
    _syslog: bool,
    _buf: *mut c_char,
    _size: size_t,
    _len: *mut size_t,
) -> bool {
    false
}

#[cfg(not(CONFIG_PRINTK))]
#[inline]
pub unsafe fn kmsg_dump_rewind(_iter: *mut kmsg_dump_iter) {}

#[cfg(not(CONFIG_PRINTK))]
#[inline]
pub unsafe fn kmsg_dump_register(_dumper: *mut kmsg_dumper) -> i32 {
    -22 // -EINVAL
}

#[cfg(not(CONFIG_PRINTK))]
#[inline]
pub unsafe fn kmsg_dump_unregister(_dumper: *mut kmsg_dumper) -> i32 {
    -22 // -EINVAL
}

#[cfg(not(CONFIG_PRINTK))]
#[inline]
pub unsafe fn kmsg_dump_reason_str(_reason: kmsg_dump_reason) -> *const c_char {
    b"Disabled\0".as_ptr() as *const c_char
}

#[inline]
pub unsafe fn kmsg_dump(reason: kmsg_dump_reason) {
    kmsg_dump_desc(reason, core::ptr::null());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
