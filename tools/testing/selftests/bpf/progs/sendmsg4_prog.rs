// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 Facebook

// Dependencies in the original C source:
// linux/stddef.h, linux/bpf.h, sys/socket.h, bpf/bpf_helpers.h,
// bpf/bpf_endian.h, and bpf_sockopt_helpers.h.

pub const SRC1_IP4: u32 = 0xAC100001; /* 172.16.0.1 */
pub const SRC2_IP4: u32 = 0x00000000;
pub const SRC_REWRITE_IP4: u32 = 0x7f000004;
pub const DST_IP4: u32 = 0xC0A801FE; /* 192.168.1.254 */
pub const DST_REWRITE_IP4: u32 = 0x7f000001;
pub const DST_PORT: u16 = 4040;
pub const DST_REWRITE_PORT4: u16 = 4444;

extern "C" {
    fn get_set_sk_priority(ctx: *mut bpf_sock_addr) -> i32;
}

extern "Rust" {
    fn bpf_htonl(hostlong: u32) -> u32;
    fn bpf_htons(hostshort: u16) -> u16;
}

extern "C" {
    static SOCK_DGRAM: i32;
}

#[repr(C)]
pub struct bpf_sock_addr {
    pub user_family: u32,
    pub user_ip4: u32,
    pub user_ip6: [u32; 4],
    pub user_port: u32,
    pub family: u32,
    pub type_: u32,
    pub protocol: u32,
    pub msg_src_ip4: u32,
    pub msg_src_ip6: [u32; 4],
}

#[no_mangle]
#[link_section = "cgroup/sendmsg4"]
pub unsafe extern "C" fn sendmsg_v4_prog(ctx: *mut bpf_sock_addr) -> i32 {
    if (*ctx).type_ != SOCK_DGRAM as u32 {
        return 0;
    }

    if get_set_sk_priority(ctx) == 0 {
        return 0;
    }

    /* Rewrite source. */
    if (*ctx).msg_src_ip4 == bpf_htonl(SRC1_IP4) || (*ctx).msg_src_ip4 == bpf_htonl(SRC2_IP4) {
        (*ctx).msg_src_ip4 = bpf_htonl(SRC_REWRITE_IP4);
    } else {
        /* Unexpected source. Reject sendmsg. */
        return 0;
    }

    /* Rewrite destination. */
    if ((*ctx).user_ip4 >> 24) == (bpf_htonl(DST_IP4) >> 24)
        && (*ctx).user_port == bpf_htons(DST_PORT) as u32
    {
        (*ctx).user_ip4 = bpf_htonl(DST_REWRITE_IP4);
        (*ctx).user_port = bpf_htons(DST_REWRITE_PORT4) as u32;
    } else {
        /* Unexpected source. Reject sendmsg. */
        return 0;
    }

    return 1;
}

#[no_mangle]
#[link_section = "cgroup/sendmsg4"]
pub unsafe extern "C" fn sendmsg_v4_deny_prog(_ctx: *mut bpf_sock_addr) -> i32 {
    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
