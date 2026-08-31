// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

/*
 * C dependencies removed from executable Rust:
 * - "vmlinux.h"
 * - <string.h>
 * - <bpf/bpf_helpers.h>
 * - <bpf/bpf_core_read.h>
 * - "bpf_kfuncs.h"
 */

use core::ffi::{c_int, c_void};

use crate::{bpf_sock_addr, bpf_sock_addr_kern, sockaddr_un};

extern "C" {
    fn bpf_cast_to_kern_ctx(ctx: *mut bpf_sock_addr) -> *mut bpf_sock_addr_kern;
    fn bpf_sock_addr_set_sun_path(
        sa_kern: *mut bpf_sock_addr_kern,
        path: *const u8,
        path__sz: u32,
    ) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
}

#[no_mangle]
pub static mut SERVUN_REWRITE_ADDRESS: [u8; 30] = *b"\0bpf_cgroup_unix_test_rewrite\0";

#[link_section = "cgroup/getsockname_unix"]
#[no_mangle]
pub unsafe extern "C" fn getsockname_unix_prog(ctx: *mut bpf_sock_addr) -> c_int {
    let sa_kern: *mut bpf_sock_addr_kern = bpf_cast_to_kern_ctx(ctx);
    let mut sa_kern_unaddr: *mut sockaddr_un;
    let unaddrlen: u32 = (core::mem::offset_of!(sockaddr_un, sun_path)
        + core::mem::size_of_val(&SERVUN_REWRITE_ADDRESS)
        - 1) as u32;
    let ret: c_int;

    ret = bpf_sock_addr_set_sun_path(
        sa_kern,
        SERVUN_REWRITE_ADDRESS.as_ptr(),
        (core::mem::size_of_val(&SERVUN_REWRITE_ADDRESS) - 1) as u32,
    );
    if ret != 0 {
        return 1;
    }

    if (*sa_kern).uaddrlen != unaddrlen {
        return 1;
    }

    sa_kern_unaddr = (*sa_kern).uaddr as *mut sockaddr_un;
    if memcmp(
        (*sa_kern_unaddr).sun_path.as_ptr() as *const c_void,
        SERVUN_REWRITE_ADDRESS.as_ptr() as *const c_void,
        core::mem::size_of_val(&SERVUN_REWRITE_ADDRESS) - 1,
    ) != 0
    {
        return 1;
    }

    return 1;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
