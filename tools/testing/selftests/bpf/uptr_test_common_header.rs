/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

pub const MAGIC_VALUE: u32 = 0xabcd1234;
pub const PAGE_SIZE: usize = 4096;

/* C header dependency intent:
 * - When compiled for __BPF__, the original header declares dummy globals for
 *   forward BTF type generation avoidance.
 * - Otherwise, __uptr and __kptr are empty annotation macros.
 */
#[cfg(target_arch = "bpf")]
unsafe extern "C" {
    /* Avoid fwd btf type being generated for the following struct */
    pub static mut dummy_large_data: *mut large_data;
    pub static mut dummy_empty_data: *mut empty_data;
    pub static mut dummy_data: *mut user_data;
    pub static mut dummy_cgrp: *mut cgroup;
}

#[repr(C)]
pub struct user_data {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
    pub result: ::std::os::raw::c_int,
    pub nested_result: ::std::os::raw::c_int,
}

#[repr(C)]
pub struct nested_udata {
    pub udata: *mut user_data,
}

#[repr(C)]
pub struct value_type {
    pub udata: *mut user_data,
    pub cgrp: *mut cgroup,
    pub nested: nested_udata,
}

#[repr(C)]
pub struct value_lock_type {
    pub udata: *mut user_data,
    pub lock: bpf_spin_lock,
}

#[repr(C)]
pub struct large_data {
    pub one_page: [__u8; PAGE_SIZE],
    pub a: ::std::os::raw::c_int,
}

#[repr(C)]
pub struct large_uptr {
    pub udata: *mut large_data,
}

#[repr(C)]
pub struct empty_data {}

#[repr(C)]
pub struct empty_uptr {
    pub udata: *mut empty_data,
}

#[repr(C)]
pub struct kstruct_uptr {
    pub cgrp: *mut cgroup,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
