// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Google LLC */

// Dependencies from:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include "../test_kmods/bpf_testmod_kfunc.h"

extern "C" {
    fn bpf_kfunc_init_sock(args: *mut init_sock_args);
    fn bpf_kfunc_close_sock();
    fn bpf_kfunc_call_kernel_connect(args: *mut addr_args) -> i32;
    fn bpf_kfunc_call_kernel_bind(args: *mut addr_args) -> i32;
    fn bpf_kfunc_call_kernel_listen() -> i32;
    fn bpf_kfunc_call_kernel_sendmsg(args: *mut sendmsg_args) -> i32;
    fn bpf_kfunc_call_sock_sendmsg(args: *mut sendmsg_args) -> i32;
    fn bpf_kfunc_call_kernel_getsockname(args: *mut addr_args) -> i32;
    fn bpf_kfunc_call_kernel_getpeername(args: *mut addr_args) -> i32;
}

#[no_mangle]
#[link_section = "syscall"]
pub extern "C" fn init_sock(args: *mut init_sock_args) -> i32 {
    unsafe {
        bpf_kfunc_init_sock(args);
    }

    0
}

#[no_mangle]
#[link_section = "syscall"]
pub extern "C" fn close_sock(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    unsafe {
        bpf_kfunc_close_sock();
    }

    0
}

#[no_mangle]
#[link_section = "syscall"]
pub extern "C" fn kernel_connect(args: *mut addr_args) -> i32 {
    unsafe { bpf_kfunc_call_kernel_connect(args) }
}

#[no_mangle]
#[link_section = "syscall"]
pub extern "C" fn kernel_bind(args: *mut addr_args) -> i32 {
    unsafe { bpf_kfunc_call_kernel_bind(args) }
}

#[no_mangle]
#[link_section = "syscall"]
pub extern "C" fn kernel_listen(args: *mut addr_args) -> i32 {
    let _ = args;
    unsafe { bpf_kfunc_call_kernel_listen() }
}

#[no_mangle]
#[link_section = "syscall"]
pub extern "C" fn kernel_sendmsg(args: *mut sendmsg_args) -> i32 {
    unsafe { bpf_kfunc_call_kernel_sendmsg(args) }
}

#[no_mangle]
#[link_section = "syscall"]
pub extern "C" fn sock_sendmsg(args: *mut sendmsg_args) -> i32 {
    unsafe { bpf_kfunc_call_sock_sendmsg(args) }
}

#[no_mangle]
#[link_section = "syscall"]
pub extern "C" fn kernel_getsockname(args: *mut addr_args) -> i32 {
    unsafe { bpf_kfunc_call_kernel_getsockname(args) }
}

#[no_mangle]
#[link_section = "syscall"]
pub extern "C" fn kernel_getpeername(args: *mut addr_args) -> i32 {
    unsafe { bpf_kfunc_call_kernel_getpeername(args) }
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
