/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2005 Jeff Dike (jdike@karaya.com)
 */

// Original dependency: <linux/compiler_types.h>

pub const STUB_MAX_FDS: usize = 4;

#[repr(C)]
pub struct mm_id {
    pub pid: core::ffi::c_int,
    pub stack: core::ffi::c_ulong,
    pub syscall_data_len: core::ffi::c_int,

    /* Only used with SECCOMP mode */
    pub sock: core::ffi::c_int,
    pub syscall_fd_num: core::ffi::c_int,
    pub syscall_fd_map: [core::ffi::c_int; STUB_MAX_FDS],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn __get_turnstile(mm_id: *mut mm_id) -> *mut mutex;
    pub fn enter_turnstile(mm_id: *mut mm_id);
    pub fn exit_turnstile(mm_id: *mut mm_id);
    pub fn notify_mm_kill(pid: core::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
