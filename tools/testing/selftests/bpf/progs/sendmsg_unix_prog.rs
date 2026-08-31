// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// C dependencies in the original source:
// #include "vmlinux.h"
// #include <string.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_core_read.h>
// #include "bpf_kfuncs.h"

use core::ffi::{c_int, c_void};
use core::mem::{offset_of, size_of};

extern "C" {
    fn bpf_cast_to_kern_ctx(ctx: *mut bpf_sock_addr) -> *mut bpf_sock_addr_kern;
    fn bpf_sock_addr_set_sun_path(
        sa_kern: *mut bpf_sock_addr_kern,
        sun_path: *const __u8,
        sun_path__sz: __u32,
    ) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
}

type __u8 = u8;
type __u32 = u32;

#[repr(C)]
pub struct bpf_sock_addr {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_sock_addr_kern {
    _unused_before_uaddrlen: [u8; 0],
    pub uaddrlen: __u32,
    pub uaddr: *mut c_void,
}

#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: u16,
    pub sun_path: [i8; 108],
}

#[no_mangle]
pub static mut SERVUN_REWRITE_ADDRESS: [__u8; 30] = *b"\0bpf_cgroup_unix_test_rewrite\0";

#[no_mangle]
#[link_section = "cgroup/sendmsg_unix"]
pub unsafe extern "C" fn sendmsg_unix_prog(ctx: *mut bpf_sock_addr) -> c_int {
    let sa_kern: *mut bpf_sock_addr_kern = bpf_cast_to_kern_ctx(ctx);
    let mut sa_kern_unaddr: *mut sockaddr_un;
    let unaddrlen: __u32 =
        (offset_of!(sockaddr_un, sun_path) + size_of::<[__u8; 30]>() - 1) as __u32;
    let ret: c_int;

    /* Rewrite destination. */
    ret = bpf_sock_addr_set_sun_path(
        sa_kern,
        SERVUN_REWRITE_ADDRESS.as_ptr(),
        (size_of::<[__u8; 30]>() - 1) as __u32,
    );
    if ret != 0 {
        return 0;
    }

    if (*sa_kern).uaddrlen != unaddrlen {
        return 0;
    }

    sa_kern_unaddr = (*sa_kern).uaddr as *mut sockaddr_un;
    if memcmp(
        (*sa_kern_unaddr).sun_path.as_ptr() as *const c_void,
        SERVUN_REWRITE_ADDRESS.as_ptr() as *const c_void,
        size_of::<[__u8; 30]>() - 1,
    ) != 0
    {
        return 0;
    }

    return 1;
}

#[no_mangle]
#[link_section = "cgroup/sendmsg_unix"]
pub unsafe extern "C" fn sendmsg_unix_deny_prog(_ctx: *mut bpf_sock_addr) -> c_int {
    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
