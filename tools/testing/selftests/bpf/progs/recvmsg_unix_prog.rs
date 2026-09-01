// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

/* Dependencies from the original C file:
 * vmlinux.h, string.h, bpf/bpf_helpers.h, bpf/bpf_core_read.h, bpf_kfuncs.h
 */

extern "C" {
    fn bpf_cast_to_kern_ctx(ctx: *mut bpf_sock_addr) -> *mut bpf_sock_addr_kern;
    fn bpf_sock_addr_set_sun_path(
        sa_kern: *mut bpf_sock_addr_kern,
        sun_path: *mut __u8,
        sun_path__sz: __u32,
    ) -> i32;
    fn memcmp(s1: *const core::ffi::c_void, s2: *const core::ffi::c_void, n: usize) -> i32;
}

type __u8 = u8;
type __u32 = u32;

#[repr(C)]
pub struct bpf_sock_addr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_sock_addr_kern {
    pub uaddr: *mut core::ffi::c_void,
    pub uaddrlen: __u32,
}

#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: u16,
    pub sun_path: [core::ffi::c_char; 108],
}

pub static mut SERVUN_ADDRESS: [__u8; 21] = *b"\0bpf_cgroup_unix_test";

const fn offsetof_sockaddr_un_sun_path() -> usize {
    core::mem::offset_of!(sockaddr_un, sun_path)
}

unsafe fn bpf_core_cast_sockaddr_un(ptr: *mut core::ffi::c_void) -> *mut sockaddr_un {
    ptr as *mut sockaddr_un
}

/* SEC("cgroup/recvmsg_unix") */
#[no_mangle]
pub unsafe extern "C" fn recvmsg_unix_prog(ctx: *mut bpf_sock_addr) -> i32 {
    let sa_kern: *mut bpf_sock_addr_kern = bpf_cast_to_kern_ctx(ctx);
    let mut sa_kern_unaddr: *mut sockaddr_un;
    let unaddrlen: __u32 =
        (offsetof_sockaddr_un_sun_path() + core::mem::size_of_val(&SERVUN_ADDRESS) - 1) as __u32;
    let ret: i32;

    ret = bpf_sock_addr_set_sun_path(
        sa_kern,
        SERVUN_ADDRESS.as_mut_ptr(),
        (core::mem::size_of_val(&SERVUN_ADDRESS) - 1) as __u32,
    );
    if ret != 0 {
        return 1;
    }

    if (*sa_kern).uaddrlen != unaddrlen {
        return 1;
    }

    sa_kern_unaddr = bpf_core_cast_sockaddr_un((*sa_kern).uaddr);
    if memcmp(
        (*sa_kern_unaddr).sun_path.as_ptr() as *const core::ffi::c_void,
        SERVUN_ADDRESS.as_ptr() as *const core::ffi::c_void,
        core::mem::size_of_val(&SERVUN_ADDRESS) - 1,
    ) != 0
    {
        return 1;
    }

    return 1;
}

/* char _license[] SEC("license") = "GPL"; */
#[no_mangle]
pub static mut _license: [core::ffi::c_char; 4] = [
    b'G' as core::ffi::c_char,
    b'P' as core::ffi::c_char,
    b'L' as core::ffi::c_char,
    0,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
