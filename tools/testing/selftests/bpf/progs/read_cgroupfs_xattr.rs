// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_core_read.h>
// #include "bpf_experimental.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

type pid_t = i32;
type u64 = u64;

const BPF_CGROUP_ITER_ANCESTORS_UP: u32 = 0;

#[repr(C)]
pub struct cgroup {
    pub self_: cgroup_subsys_state,
}

#[repr(C)]
pub struct cgroup_subsys_state {
    pub cgroup: *mut cgroup,
}

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_get_current_cgroup_id() -> u64;
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_rcu_read_lock();
    fn bpf_rcu_read_unlock();
    fn bpf_cgroup_from_id(cgrp_id: u64) -> *mut cgroup;
    fn bpf_cgroup_release(cgrp: *mut cgroup);
    fn bpf_dynptr_from_mem(
        data: *mut core::ffi::c_void,
        size: u32,
        flags: u64,
        ptr: *mut bpf_dynptr,
    ) -> i32;
    fn bpf_cgroup_read_xattr(
        cgrp: *mut cgroup,
        name: *const core::ffi::c_char,
        value: *mut bpf_dynptr,
    ) -> i32;
    fn bpf_strncmp(
        s1: *const core::ffi::c_char,
        s1_sz: u32,
        s2: *const core::ffi::c_char,
    ) -> i32;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [core::ffi::c_char; 4] = [
    b'G' as core::ffi::c_char,
    b'P' as core::ffi::c_char,
    b'L' as core::ffi::c_char,
    0,
];

#[unsafe(no_mangle)]
pub static mut target_pid: pid_t = 0;

#[unsafe(no_mangle)]
pub static mut xattr_value: [core::ffi::c_char; 64] = [0; 64];
static expected_value_a: [core::ffi::c_char; 21] = [
    b'b' as core::ffi::c_char,
    b'p' as core::ffi::c_char,
    b'f' as core::ffi::c_char,
    b'_' as core::ffi::c_char,
    b's' as core::ffi::c_char,
    b'e' as core::ffi::c_char,
    b'l' as core::ffi::c_char,
    b'f' as core::ffi::c_char,
    b't' as core::ffi::c_char,
    b'e' as core::ffi::c_char,
    b's' as core::ffi::c_char,
    b't' as core::ffi::c_char,
    b'_' as core::ffi::c_char,
    b'v' as core::ffi::c_char,
    b'a' as core::ffi::c_char,
    b'l' as core::ffi::c_char,
    b'u' as core::ffi::c_char,
    b'e' as core::ffi::c_char,
    b'_' as core::ffi::c_char,
    b'a' as core::ffi::c_char,
    0,
];
static expected_value_b: [core::ffi::c_char; 21] = [
    b'b' as core::ffi::c_char,
    b'p' as core::ffi::c_char,
    b'f' as core::ffi::c_char,
    b'_' as core::ffi::c_char,
    b's' as core::ffi::c_char,
    b'e' as core::ffi::c_char,
    b'l' as core::ffi::c_char,
    b'f' as core::ffi::c_char,
    b't' as core::ffi::c_char,
    b'e' as core::ffi::c_char,
    b's' as core::ffi::c_char,
    b't' as core::ffi::c_char,
    b'_' as core::ffi::c_char,
    b'v' as core::ffi::c_char,
    b'a' as core::ffi::c_char,
    b'l' as core::ffi::c_char,
    b'u' as core::ffi::c_char,
    b'e' as core::ffi::c_char,
    b'_' as core::ffi::c_char,
    b'b' as core::ffi::c_char,
    0,
];
#[unsafe(no_mangle)]
pub static mut found_value_a: bool = false;
#[unsafe(no_mangle)]
pub static mut found_value_b: bool = false;

#[unsafe(link_section = "lsm.s/file_open")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_file_open() -> i32 {
    let cgrp_id: u64 = unsafe { bpf_get_current_cgroup_id() };
    let mut css: *mut cgroup_subsys_state;
    let mut tmp: *mut cgroup_subsys_state;
    let mut value_ptr: bpf_dynptr = core::mem::zeroed();
    let cgrp: *mut cgroup;

    if ((unsafe { bpf_get_current_pid_tgid() } >> 32) as pid_t) != unsafe { target_pid } {
        return 0;
    }

    unsafe { bpf_rcu_read_lock() };
    cgrp = unsafe { bpf_cgroup_from_id(cgrp_id) };
    if cgrp.is_null() {
        unsafe { bpf_rcu_read_unlock() };
        return 0;
    }

    css = unsafe { &mut (*cgrp).self_ as *mut cgroup_subsys_state };
    unsafe {
        bpf_dynptr_from_mem(
            (&raw mut xattr_value).cast::<core::ffi::c_void>(),
            core::mem::size_of_val(&*(&raw const xattr_value)) as u32,
            0,
            &mut value_ptr,
        );
    }

    // C source uses:
    // bpf_for_each(css, tmp, css, BPF_CGROUP_ITER_ANCESTORS_UP) { ... }
    bpf_for_each!(css, tmp, css, BPF_CGROUP_ITER_ANCESTORS_UP, {
        let ret: i32;

        ret = unsafe {
            bpf_cgroup_read_xattr(
                unsafe { (*tmp).cgroup },
                c"user.bpf_test".as_ptr(),
                &mut value_ptr,
            )
        };
        if ret < 0 {
            continue;
        }

        if ret == core::mem::size_of_val(&expected_value_a) as i32
            && unsafe {
                bpf_strncmp(
                    (&raw const xattr_value).cast::<core::ffi::c_char>(),
                    core::mem::size_of_val(&expected_value_a) as u32,
                    expected_value_a.as_ptr(),
                )
            } == 0
        {
            unsafe { found_value_a = true };
        }
        if ret == core::mem::size_of_val(&expected_value_b) as i32
            && unsafe {
                bpf_strncmp(
                    (&raw const xattr_value).cast::<core::ffi::c_char>(),
                    core::mem::size_of_val(&expected_value_b) as u32,
                    expected_value_b.as_ptr(),
                )
            } == 0
        {
            unsafe { found_value_b = true };
        }
    });

    unsafe { bpf_rcu_read_unlock() };
    unsafe { bpf_cgroup_release(cgrp) };

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
