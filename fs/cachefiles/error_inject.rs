// SPDX-License-Identifier: GPL-2.0-or-later
/* Error injection handling.
 *
 * Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Supplied by the Linux sysctl and cachefiles dependencies.
#[repr(C)]
pub struct ctl_table {
    pub procname: *const c_char,
    pub data: *mut c_void,
    pub maxlen: usize,
    pub mode: u16,
    pub proc_handler: *const c_void,
}

#[repr(C)]
pub struct ctl_table_header {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn proc_douintvec(
        table: *mut ctl_table,
        write: c_int,
        buffer: *mut c_void,
        length: *mut usize,
        ppos: *mut i64,
    ) -> c_int;
    fn register_sysctl(
        path: *const c_char,
        table: *const ctl_table,
    ) -> *mut ctl_table_header;
    fn unregister_sysctl_table(header: *mut ctl_table_header);
}

pub static mut cachefiles_error_injection_state: c_uint = 0;

static mut cachefiles_sysctl: *mut ctl_table_header = core::ptr::null_mut();

static CACHEFILES_SYSCTLS: [ctl_table; 1] = [ctl_table {
    procname: b"error_injection\0".as_ptr() as *const c_char,
    data: core::ptr::addr_of_mut!(cachefiles_error_injection_state) as *mut c_void,
    maxlen: core::mem::size_of::<c_uint>(),
    mode: 0o644,
    proc_handler: proc_douintvec as *const c_void,
}];

// C __init annotation is retained by the surrounding build system.
pub unsafe fn cachefiles_register_error_injection() -> c_int {
    cachefiles_sysctl = register_sysctl(
        b"cachefiles\0".as_ptr() as *const c_char,
        CACHEFILES_SYSCTLS.as_ptr(),
    );
    if cachefiles_sysctl.is_null() {
        return -12; // -ENOMEM
    }
    0
}

pub unsafe fn cachefiles_unregister_error_injection() {
    unregister_sysctl_table(cachefiles_sysctl);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
