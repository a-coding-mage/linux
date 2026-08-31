// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Google LLC */

// C dependencies: "vmlinux.h", <string.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_endian.h>, <bpf/bpf_core_read.h>, "bpf_kfuncs.h"

pub const REWRITE_ADDRESS_IP6_0: u32 = 0xfaceb00c;
pub const REWRITE_ADDRESS_IP6_1: u32 = 0x12345678;
pub const REWRITE_ADDRESS_IP6_2: u32 = 0x00000000;
pub const REWRITE_ADDRESS_IP6_3: u32 = 0x0000abcd;

pub const REWRITE_ADDRESS_PORT6: u16 = 6060;

#[repr(C)]
pub struct bpf_sock_addr {
    pub user_ip6: [u32; 4],
    pub user_port: u32,
}

#[inline]
pub const fn bpf_htonl(x: u32) -> u32 {
    x.to_be()
}

#[inline]
pub const fn bpf_htons(x: u16) -> u16 {
    x.to_be()
}

#[unsafe(link_section = "cgroup/getpeername6")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getpeername_v6_prog(ctx: *mut bpf_sock_addr) -> i32 {
    unsafe {
        (*ctx).user_ip6[0] = bpf_htonl(REWRITE_ADDRESS_IP6_0);
        (*ctx).user_ip6[1] = bpf_htonl(REWRITE_ADDRESS_IP6_1);
        (*ctx).user_ip6[2] = bpf_htonl(REWRITE_ADDRESS_IP6_2);
        (*ctx).user_ip6[3] = bpf_htonl(REWRITE_ADDRESS_IP6_3);
        (*ctx).user_port = bpf_htons(REWRITE_ADDRESS_PORT6) as u32;
    }

    1
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
