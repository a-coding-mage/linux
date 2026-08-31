// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// Dependencies from the original C source:
// vmlinux.h, string.h, bpf/bpf_helpers.h, bpf/bpf_core_read.h, bpf_kfuncs.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_int, c_void};
use core::mem::size_of;

pub type __u8 = u8;
pub type __u32 = u32;

#[repr(C)]
pub struct bpf_sock_addr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_sock_addr_kern {
    pub uaddrlen: __u32,
    pub uaddr: *mut c_void,
}

#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: u16,
    pub sun_path: [u8; 108],
}

unsafe extern "C" {
    fn bpf_cast_to_kern_ctx(ctx: *mut bpf_sock_addr) -> *mut bpf_sock_addr_kern;
    fn bpf_sock_addr_set_sun_path(
        sa_kern: *mut bpf_sock_addr_kern,
        sun_path: *const __u8,
        sun_path__sz: __u32,
    ) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
}

#[inline(always)]
unsafe fn bpf_core_cast<T>(ptr: *mut c_void) -> *mut T {
    ptr as *mut T
}

const fn offsetof_sockaddr_un_sun_path() -> __u32 {
    2
}

#[unsafe(no_mangle)]
pub static mut SERVUN_REWRITE_ADDRESS: [__u8; 30] = *b"\0bpf_cgroup_unix_test_rewrite\0";

#[unsafe(link_section = "cgroup/getpeername_unix")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getpeername_unix_prog(ctx: *mut bpf_sock_addr) -> c_int {
    let sa_kern: *mut bpf_sock_addr_kern = unsafe { bpf_cast_to_kern_ctx(ctx) };
    let mut sa_kern_unaddr: *mut sockaddr_un;
    let unaddrlen: __u32 =
        offsetof_sockaddr_un_sun_path() + size_of::<[__u8; 30]>() as __u32 - 1;
    let ret: c_int;

    ret = unsafe {
        bpf_sock_addr_set_sun_path(
            sa_kern,
            SERVUN_REWRITE_ADDRESS.as_ptr(),
            size_of::<[__u8; 30]>() as __u32 - 1,
        )
    };
    if ret != 0 {
        return 1;
    }

    if unsafe { (*sa_kern).uaddrlen } != unaddrlen {
        return 1;
    }

    sa_kern_unaddr = unsafe { bpf_core_cast((*sa_kern).uaddr) };
    if unsafe {
        memcmp(
            (*sa_kern_unaddr).sun_path.as_ptr() as *const c_void,
            SERVUN_REWRITE_ADDRESS.as_ptr() as *const c_void,
            size_of::<[__u8; 30]>() - 1,
        )
    } != 0
    {
        return 1;
    }

    return 1;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";
