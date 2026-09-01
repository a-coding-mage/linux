// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Facebook */

// Dependencies from the original C source:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#[repr(C)]
pub struct bpf_testmod_btf_type_tag_1 {
    pub a: i32,
}

#[repr(C)]
pub struct bpf_testmod_btf_type_tag_2 {
    pub p: *mut bpf_testmod_btf_type_tag_1,
}

pub static mut g: i32 = 0;

#[no_mangle]
#[link_section = "fentry/bpf_testmod_test_btf_type_tag_user_1"]
pub unsafe extern "C" fn test_user1(arg: *mut bpf_testmod_btf_type_tag_1) -> i32 {
    unsafe {
        g = (*arg).a;
    }
    0
}

#[no_mangle]
#[link_section = "fentry/bpf_testmod_test_btf_type_tag_user_2"]
pub unsafe extern "C" fn test_user2(arg: *mut bpf_testmod_btf_type_tag_2) -> i32 {
    unsafe {
        g = (*(*arg).p).a;
    }
    0
}

/* int __sys_getsockname(int fd, struct sockaddr __user *usockaddr,
 *                       int __user *usockaddr_len);
 */
// `sockaddr` is supplied by the translated equivalent of vmlinux.h.
#[no_mangle]
#[link_section = "fentry/__sys_getsockname"]
pub unsafe extern "C" fn test_sys_getsockname(
    fd: i32,
    usockaddr: *mut sockaddr,
    usockaddr_len: *mut i32,
) -> i32 {
    let _ = fd;
    let _ = usockaddr_len;
    unsafe {
        g = (*usockaddr).sa_family as i32;
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
