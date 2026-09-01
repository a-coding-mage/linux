// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 Facebook

// C dependencies:
// #include <linux/stddef.h>
// #include <linux/bpf.h>
// #include <sys/socket.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_endian.h>
// #include <bpf_sockopt_helpers.h>

const SRC_REWRITE_IP6_0: u32 = 0;
const SRC_REWRITE_IP6_1: u32 = 0;
const SRC_REWRITE_IP6_2: u32 = 0;
const SRC_REWRITE_IP6_3: u32 = 6;

const DST_REWRITE_IP6_0: u32 = 0;
const DST_REWRITE_IP6_1: u32 = 0;
const DST_REWRITE_IP6_2: u32 = 0;
const DST_REWRITE_IP6_3: u32 = 1;

const DST_REWRITE_IP6_V4_MAPPED_0: u32 = 0;
const DST_REWRITE_IP6_V4_MAPPED_1: u32 = 0;
const DST_REWRITE_IP6_V4_MAPPED_2: u32 = 0x0000FFFF;
const DST_REWRITE_IP6_V4_MAPPED_3: u32 = 0xc0a80004; // 192.168.0.4

const DST_REWRITE_PORT6: u16 = 6666;

extern "C" {
    static SOCK_DGRAM: i32;

    fn bpf_htonl(x: u32) -> u32;
    fn bpf_htons(x: u16) -> u16;
    fn get_set_sk_priority(ctx: *mut bpf_sock_addr) -> bool;
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
    pub sk: *mut core::ffi::c_void,
}

#[no_mangle]
#[link_section = "cgroup/sendmsg6"]
pub unsafe extern "C" fn sendmsg_v6_prog(ctx: *mut bpf_sock_addr) -> i32 {
    if (*ctx).type_ != SOCK_DGRAM as u32 {
        return 0;
    }

    if !get_set_sk_priority(ctx) {
        return 0;
    }

    /* Rewrite source. */
    if (*ctx).msg_src_ip6[3] == bpf_htonl(1) || (*ctx).msg_src_ip6[3] == bpf_htonl(0) {
        (*ctx).msg_src_ip6[0] = bpf_htonl(SRC_REWRITE_IP6_0);
        (*ctx).msg_src_ip6[1] = bpf_htonl(SRC_REWRITE_IP6_1);
        (*ctx).msg_src_ip6[2] = bpf_htonl(SRC_REWRITE_IP6_2);
        (*ctx).msg_src_ip6[3] = bpf_htonl(SRC_REWRITE_IP6_3);
    } else {
        /* Unexpected source. Reject sendmsg. */
        return 0;
    }

    /* Rewrite destination. */
    if (*ctx).user_ip6[0] == bpf_htonl(0xFACEB00C) {
        (*ctx).user_ip6[0] = bpf_htonl(DST_REWRITE_IP6_0);
        (*ctx).user_ip6[1] = bpf_htonl(DST_REWRITE_IP6_1);
        (*ctx).user_ip6[2] = bpf_htonl(DST_REWRITE_IP6_2);
        (*ctx).user_ip6[3] = bpf_htonl(DST_REWRITE_IP6_3);

        (*ctx).user_port = bpf_htons(DST_REWRITE_PORT6) as u32;
    } else {
        /* Unexpected destination. Reject sendmsg. */
        return 0;
    }

    return 1;
}

#[no_mangle]
#[link_section = "cgroup/sendmsg6"]
pub unsafe extern "C" fn sendmsg_v6_v4mapped_prog(ctx: *mut bpf_sock_addr) -> i32 {
    /* Rewrite source. */
    (*ctx).msg_src_ip6[0] = bpf_htonl(SRC_REWRITE_IP6_0);
    (*ctx).msg_src_ip6[1] = bpf_htonl(SRC_REWRITE_IP6_1);
    (*ctx).msg_src_ip6[2] = bpf_htonl(SRC_REWRITE_IP6_2);
    (*ctx).msg_src_ip6[3] = bpf_htonl(SRC_REWRITE_IP6_3);

    /* Rewrite destination. */
    (*ctx).user_ip6[0] = bpf_htonl(DST_REWRITE_IP6_V4_MAPPED_0);
    (*ctx).user_ip6[1] = bpf_htonl(DST_REWRITE_IP6_V4_MAPPED_1);
    (*ctx).user_ip6[2] = bpf_htonl(DST_REWRITE_IP6_V4_MAPPED_2);
    (*ctx).user_ip6[3] = bpf_htonl(DST_REWRITE_IP6_V4_MAPPED_3);

    (*ctx).user_port = bpf_htons(DST_REWRITE_PORT6) as u32;

    return 1;
}

#[no_mangle]
#[link_section = "cgroup/sendmsg6"]
pub unsafe extern "C" fn sendmsg_v6_wildcard_prog(ctx: *mut bpf_sock_addr) -> i32 {
    /* Rewrite source. */
    (*ctx).msg_src_ip6[0] = bpf_htonl(SRC_REWRITE_IP6_0);
    (*ctx).msg_src_ip6[1] = bpf_htonl(SRC_REWRITE_IP6_1);
    (*ctx).msg_src_ip6[2] = bpf_htonl(SRC_REWRITE_IP6_2);
    (*ctx).msg_src_ip6[3] = bpf_htonl(SRC_REWRITE_IP6_3);

    /* Rewrite destination. */
    (*ctx).user_ip6[0] = bpf_htonl(0);
    (*ctx).user_ip6[1] = bpf_htonl(0);
    (*ctx).user_ip6[2] = bpf_htonl(0);
    (*ctx).user_ip6[3] = bpf_htonl(0);

    (*ctx).user_port = bpf_htons(DST_REWRITE_PORT6) as u32;

    return 1;
}

#[no_mangle]
#[link_section = "cgroup/sendmsg6"]
pub unsafe extern "C" fn sendmsg_v6_preserve_dst_prog(_ctx: *mut bpf_sock_addr) -> i32 {
    return 1;
}

#[no_mangle]
#[link_section = "cgroup/sendmsg6"]
pub unsafe extern "C" fn sendmsg_v6_deny_prog(_ctx: *mut bpf_sock_addr) -> i32 {
    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
