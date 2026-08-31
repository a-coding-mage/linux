// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Google LLC */

// C dependencies:
// #include "vmlinux.h"
// #include <string.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_endian.h>
// #include <bpf/bpf_core_read.h>
// #include "bpf_kfuncs.h"

pub const REWRITE_ADDRESS_IP4: u32 = 0xc0a801fe; // 192.168.1.254
pub const REWRITE_ADDRESS_PORT4: u16 = 4040;

unsafe extern "C" {
    fn bpf_htonl(x: u32) -> u32;
    fn bpf_htons(x: u16) -> u16;
}

#[repr(C)]
pub struct bpf_sock_addr {
    // Full layout is supplied by vmlinux.h in the original C source.
    pub user_family: u32,
    pub user_ip4: u32,
    pub user_ip6: [u32; 4],
    pub user_port: u32,
}

// SEC("cgroup/getpeername4")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getpeername_v4_prog(ctx: *mut bpf_sock_addr) -> i32 {
    unsafe {
        (*ctx).user_ip4 = bpf_htonl(REWRITE_ADDRESS_IP4);
        (*ctx).user_port = bpf_htons(REWRITE_ADDRESS_PORT4) as u32;
    }

    1
}

// char _license[] SEC("license") = "GPL";
#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static _license: [u8; 4] = *b"GPL\0";
