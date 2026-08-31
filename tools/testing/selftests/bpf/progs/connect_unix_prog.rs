// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

/* Translated from:
 * #include "vmlinux.h"
 * #include <string.h>
 * #include <bpf/bpf_helpers.h>
 * #include <bpf/bpf_core_read.h>
 * #include "bpf_kfuncs.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_int, c_void};
use core::mem::size_of;

pub type __u8 = u8;
pub type __u32 = u32;
pub type sa_family_t = u16;

#[repr(C)]
pub struct bpf_sock_addr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_sock_addr_kern {
    pub uaddr: *mut c_void,
    pub uaddrlen: __u32,
}

#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [i8; 108],
}

unsafe extern "C" {
    fn bpf_cast_to_kern_ctx(ctx: *mut bpf_sock_addr) -> *mut bpf_sock_addr_kern;
    fn bpf_sock_addr_set_sun_path(
        sa_kern: *mut bpf_sock_addr_kern,
        sun_path: *mut __u8,
        sun_path_len: __u32,
    ) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
}

#[unsafe(no_mangle)]
pub static mut SERVUN_REWRITE_ADDRESS: [__u8; 30] = *b"\0bpf_cgroup_unix_test_rewrite\0";

const fn offsetof_sockaddr_un_sun_path() -> usize {
    size_of::<sa_family_t>()
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "cgroup/connect_unix")]
pub unsafe extern "C" fn connect_unix_prog(ctx: *mut bpf_sock_addr) -> c_int {
    let sa_kern: *mut bpf_sock_addr_kern = unsafe { bpf_cast_to_kern_ctx(ctx) };
    let mut sa_kern_unaddr: *mut sockaddr_un;
    let unaddrlen: __u32 = (offsetof_sockaddr_un_sun_path()
        + size_of::<[__u8; 30]>()
        - 1) as __u32;
    let ret: c_int;

    /* Rewrite destination. */
    ret = unsafe {
        bpf_sock_addr_set_sun_path(
            sa_kern,
            core::ptr::addr_of_mut!(SERVUN_REWRITE_ADDRESS) as *mut __u8,
            (size_of::<[__u8; 30]>() - 1) as __u32,
        )
    };
    if ret != 0 {
        return 0;
    }

    if unsafe { (*sa_kern).uaddrlen } != unaddrlen {
        return 0;
    }

    sa_kern_unaddr = unsafe { (*sa_kern).uaddr as *mut sockaddr_un };
    if unsafe {
        memcmp(
            core::ptr::addr_of!((*sa_kern_unaddr).sun_path) as *const c_void,
            core::ptr::addr_of!(SERVUN_REWRITE_ADDRESS) as *const c_void,
            size_of::<[__u8; 30]>() - 1,
        )
    } != 0
    {
        return 0;
    }

    1
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "cgroup/connect_unix")]
pub unsafe extern "C" fn connect_unix_deny_prog(_ctx: *mut bpf_sock_addr) -> c_int {
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";
