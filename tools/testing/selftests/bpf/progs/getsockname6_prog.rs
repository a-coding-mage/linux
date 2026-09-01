// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Google LLC */

// Dependencies from the original C source:
// "vmlinux.h", <string.h>, <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>,
// <bpf/bpf_core_read.h>, and "bpf_kfuncs.h".

const REWRITE_ADDRESS_IP6_0: u32 = 0xfaceb00c;
const REWRITE_ADDRESS_IP6_1: u32 = 0x12345678;
const REWRITE_ADDRESS_IP6_2: u32 = 0x00000000;
const REWRITE_ADDRESS_IP6_3: u32 = 0x0000abcd;

const REWRITE_ADDRESS_PORT6: u16 = 6060;

extern "C" {
    fn bpf_htonl(x: u32) -> u32;
    fn bpf_htons(x: u16) -> u16;
}

#[repr(C)]
pub struct bpf_sock_addr {
    pub user_family: u32,
    pub user_ip4: u32,
    pub user_ip6: [u32; 4],
    pub user_port: u32,
}

#[no_mangle]
#[link_section = "cgroup/getsockname6"]
pub unsafe extern "C" fn getsockname_v6_prog(ctx: *mut bpf_sock_addr) -> i32 {
    (*ctx).user_ip6[0] = bpf_htonl(REWRITE_ADDRESS_IP6_0);
    (*ctx).user_ip6[1] = bpf_htonl(REWRITE_ADDRESS_IP6_1);
    (*ctx).user_ip6[2] = bpf_htonl(REWRITE_ADDRESS_IP6_2);
    (*ctx).user_ip6[3] = bpf_htonl(REWRITE_ADDRESS_IP6_3);
    (*ctx).user_port = bpf_htons(REWRITE_ADDRESS_PORT6) as u32;

    1
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
