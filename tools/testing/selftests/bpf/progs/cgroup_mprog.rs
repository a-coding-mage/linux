// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */
// C includes translated as external dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>

#[repr(C)]
pub struct bpf_sockopt {
    _private: [u8; 0],
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[link_section = "cgroup/getsockopt"]
#[no_mangle]
pub unsafe extern "C" fn getsockopt_1(ctx: *mut bpf_sockopt) -> i32 {
    1
}

#[link_section = "cgroup/getsockopt"]
#[no_mangle]
pub unsafe extern "C" fn getsockopt_2(ctx: *mut bpf_sockopt) -> i32 {
    1
}

#[link_section = "cgroup/getsockopt"]
#[no_mangle]
pub unsafe extern "C" fn getsockopt_3(ctx: *mut bpf_sockopt) -> i32 {
    1
}

#[link_section = "cgroup/getsockopt"]
#[no_mangle]
pub unsafe extern "C" fn getsockopt_4(ctx: *mut bpf_sockopt) -> i32 {
    1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
