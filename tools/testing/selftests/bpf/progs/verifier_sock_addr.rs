// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Google LLC */

// Dependencies from the original C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf_sockopt_helpers.h>
// #include "bpf_misc.h"

unsafe extern "C" {
    type bpf_sock_addr;
}

// SEC("cgroup/recvmsg4")
// __success
#[no_mangle]
pub unsafe extern "C" fn recvmsg4_good_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    1
}

// SEC("cgroup/recvmsg4")
// __failure __msg("At program exit the register R0 has smin=0 smax=0 should have been in [1, 1]")
#[no_mangle]
pub unsafe extern "C" fn recvmsg4_bad_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    0
}

// SEC("cgroup/recvmsg6")
// __success
#[no_mangle]
pub unsafe extern "C" fn recvmsg6_good_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    1
}

// SEC("cgroup/recvmsg6")
// __failure __msg("At program exit the register R0 has smin=0 smax=0 should have been in [1, 1]")
#[no_mangle]
pub unsafe extern "C" fn recvmsg6_bad_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    0
}

// SEC("cgroup/recvmsg_unix")
// __success
#[no_mangle]
pub unsafe extern "C" fn recvmsg_unix_good_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    1
}

// SEC("cgroup/recvmsg_unix")
// __failure __msg("At program exit the register R0 has smin=0 smax=0 should have been in [1, 1]")
#[no_mangle]
pub unsafe extern "C" fn recvmsg_unix_bad_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    0
}

// SEC("cgroup/sendmsg4")
// __success
#[no_mangle]
pub unsafe extern "C" fn sendmsg4_good_return_code_0(ctx: *mut bpf_sock_addr) -> i32 {
    0
}

// SEC("cgroup/sendmsg4")
// __success
#[no_mangle]
pub unsafe extern "C" fn sendmsg4_good_return_code_1(ctx: *mut bpf_sock_addr) -> i32 {
    1
}

// SEC("cgroup/sendmsg4")
// __failure __msg("At program exit the register R0 has smin=2 smax=2 should have been in [0, 1]")
#[no_mangle]
pub unsafe extern "C" fn sendmsg4_bad_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    2
}

// SEC("cgroup/sendmsg6")
// __success
#[no_mangle]
pub unsafe extern "C" fn sendmsg6_good_return_code_0(ctx: *mut bpf_sock_addr) -> i32 {
    0
}

// SEC("cgroup/sendmsg6")
// __success
#[no_mangle]
pub unsafe extern "C" fn sendmsg6_good_return_code_1(ctx: *mut bpf_sock_addr) -> i32 {
    1
}

// SEC("cgroup/sendmsg6")
// __failure __msg("At program exit the register R0 has smin=2 smax=2 should have been in [0, 1]")
#[no_mangle]
pub unsafe extern "C" fn sendmsg6_bad_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    2
}

// SEC("cgroup/sendmsg_unix")
// __success
#[no_mangle]
pub unsafe extern "C" fn sendmsg_unix_good_return_code_0(ctx: *mut bpf_sock_addr) -> i32 {
    0
}

// SEC("cgroup/sendmsg_unix")
// __success
#[no_mangle]
pub unsafe extern "C" fn sendmsg_unix_good_return_code_1(ctx: *mut bpf_sock_addr) -> i32 {
    1
}

// SEC("cgroup/sendmsg_unix")
// __failure __msg("At program exit the register R0 has smin=2 smax=2 should have been in [0, 1]")
#[no_mangle]
pub unsafe extern "C" fn sendmsg_unix_bad_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    2
}

// SEC("cgroup/getpeername4")
// __success
#[no_mangle]
pub unsafe extern "C" fn getpeername4_good_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    1
}

// SEC("cgroup/getpeername4")
// __failure __msg("At program exit the register R0 has smin=0 smax=0 should have been in [1, 1]")
#[no_mangle]
pub unsafe extern "C" fn getpeername4_bad_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    0
}

// SEC("cgroup/getpeername6")
// __success
#[no_mangle]
pub unsafe extern "C" fn getpeername6_good_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    1
}

// SEC("cgroup/getpeername6")
// __failure __msg("At program exit the register R0 has smin=0 smax=0 should have been in [1, 1]")
#[no_mangle]
pub unsafe extern "C" fn getpeername6_bad_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    0
}

// SEC("cgroup/getpeername_unix")
// __success
#[no_mangle]
pub unsafe extern "C" fn getpeername_unix_good_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    1
}

// SEC("cgroup/getpeername_unix")
// __failure __msg("At program exit the register R0 has smin=0 smax=0 should have been in [1, 1]")
#[no_mangle]
pub unsafe extern "C" fn getpeername_unix_bad_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    0
}

// SEC("cgroup/getsockname4")
// __success
#[no_mangle]
pub unsafe extern "C" fn getsockname4_good_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    1
}

// SEC("cgroup/getsockname4")
// __failure __msg("At program exit the register R0 has smin=0 smax=0 should have been in [1, 1]")
#[no_mangle]
pub unsafe extern "C" fn getsockname4_bad_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    0
}

// SEC("cgroup/getsockname6")
// __success
#[no_mangle]
pub unsafe extern "C" fn getsockname6_good_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    1
}

// SEC("cgroup/getsockname6")
// __failure __msg("At program exit the register R0 has smin=0 smax=0 should have been in [1, 1]")
#[no_mangle]
pub unsafe extern "C" fn getsockname6_bad_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    0
}

// SEC("cgroup/getsockname_unix")
// __success
#[no_mangle]
pub unsafe extern "C" fn getsockname_unix_good_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    1
}

// SEC("cgroup/getsockname_unix")
// __failure __msg("At program exit the register R0 has smin=0 smax=0 should have been in [1, 1]")
#[no_mangle]
pub unsafe extern "C" fn getsockname_unix_unix_bad_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    0
}

// SEC("cgroup/bind4")
// __success
#[no_mangle]
pub unsafe extern "C" fn bind4_good_return_code_0(ctx: *mut bpf_sock_addr) -> i32 {
    0
}

// SEC("cgroup/bind4")
// __success
#[no_mangle]
pub unsafe extern "C" fn bind4_good_return_code_1(ctx: *mut bpf_sock_addr) -> i32 {
    1
}

// SEC("cgroup/bind4")
// __success
#[no_mangle]
pub unsafe extern "C" fn bind4_good_return_code_2(ctx: *mut bpf_sock_addr) -> i32 {
    2
}

// SEC("cgroup/bind4")
// __success
#[no_mangle]
pub unsafe extern "C" fn bind4_good_return_code_3(ctx: *mut bpf_sock_addr) -> i32 {
    3
}

// SEC("cgroup/bind4")
// __failure __msg("At program exit the register R0 has smin=4 smax=4 should have been in [0, 3]")
#[no_mangle]
pub unsafe extern "C" fn bind4_bad_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    4
}

// SEC("cgroup/bind6")
// __success
#[no_mangle]
pub unsafe extern "C" fn bind6_good_return_code_1(ctx: *mut bpf_sock_addr) -> i32 {
    1
}

// SEC("cgroup/bind6")
// __success
#[no_mangle]
pub unsafe extern "C" fn bind6_good_return_code_2(ctx: *mut bpf_sock_addr) -> i32 {
    2
}

// SEC("cgroup/bind6")
// __success
#[no_mangle]
pub unsafe extern "C" fn bind6_good_return_code_3(ctx: *mut bpf_sock_addr) -> i32 {
    3
}

// SEC("cgroup/bind6")
// __failure __msg("At program exit the register R0 has smin=4 smax=4 should have been in [0, 3]")
#[no_mangle]
pub unsafe extern "C" fn bind6_bad_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    4
}

// SEC("cgroup/connect4")
// __success
#[no_mangle]
pub unsafe extern "C" fn connect4_good_return_code_0(ctx: *mut bpf_sock_addr) -> i32 {
    0
}

// SEC("cgroup/connect4")
// __success
#[no_mangle]
pub unsafe extern "C" fn connect4_good_return_code_1(ctx: *mut bpf_sock_addr) -> i32 {
    1
}

// SEC("cgroup/connect4")
// __failure __msg("At program exit the register R0 has smin=2 smax=2 should have been in [0, 1]")
#[no_mangle]
pub unsafe extern "C" fn connect4_bad_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    2
}

// SEC("cgroup/connect6")
// __success
#[no_mangle]
pub unsafe extern "C" fn connect6_good_return_code_0(ctx: *mut bpf_sock_addr) -> i32 {
    0
}

// SEC("cgroup/connect6")
// __success
#[no_mangle]
pub unsafe extern "C" fn connect6_good_return_code_1(ctx: *mut bpf_sock_addr) -> i32 {
    1
}

// SEC("cgroup/connect6")
// __failure __msg("At program exit the register R0 has smin=2 smax=2 should have been in [0, 1]")
#[no_mangle]
pub unsafe extern "C" fn connect6_bad_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    2
}

// SEC("cgroup/connect_unix")
// __success
#[no_mangle]
pub unsafe extern "C" fn connect_unix_good_return_code_0(ctx: *mut bpf_sock_addr) -> i32 {
    0
}

// SEC("cgroup/connect_unix")
// __success
#[no_mangle]
pub unsafe extern "C" fn connect_unix_good_return_code_1(ctx: *mut bpf_sock_addr) -> i32 {
    1
}

// SEC("cgroup/connect_unix")
// __failure __msg("At program exit the register R0 has smin=2 smax=2 should have been in [0, 1]")
#[no_mangle]
pub unsafe extern "C" fn connect_unix_bad_return_code(ctx: *mut bpf_sock_addr) -> i32 {
    2
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
