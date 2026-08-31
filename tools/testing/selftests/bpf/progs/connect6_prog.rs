// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 Facebook

// C dependencies translated from:
// <string.h>, <linux/stddef.h>, <linux/bpf.h>, <linux/in.h>, <linux/in6.h>,
// <sys/socket.h>, <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>

const SRC_REWRITE_IP6_0: u32 = 0;
const SRC_REWRITE_IP6_1: u32 = 0;
const SRC_REWRITE_IP6_2: u32 = 0;
const SRC_REWRITE_IP6_3: u32 = 6;

const DST_REWRITE_IP6_0: u32 = 0;
const DST_REWRITE_IP6_1: u32 = 0;
const DST_REWRITE_IP6_2: u32 = 0;
const DST_REWRITE_IP6_3: u32 = 1;

const DST_REWRITE_PORT6: u16 = 6666;

const SOCK_STREAM: u32 = 1;
const SOCK_DGRAM: u32 = 2;
const AF_INET6: u16 = 10;
const BPF_F_CURRENT_NETNS: u64 = -1i32 as u64;

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

#[repr(C)]
pub struct bpf_sock_tuple_ipv6 {
    pub saddr: [u32; 4],
    pub daddr: [u32; 4],
    pub sport: u16,
    pub dport: u16,
}

#[repr(C)]
pub union bpf_sock_tuple {
    pub ipv6: bpf_sock_tuple_ipv6,
}

#[repr(C)]
pub struct bpf_sock {
    pub bound_dev_if: u32,
    pub family: u32,
    pub type_: u32,
    pub protocol: u32,
    pub mark: u32,
    pub priority: u32,
    pub src_ip4: u32,
    pub src_ip6: [u32; 4],
    pub src_port: u32,
    pub dst_port: u16,
    pub dst_ip4: u32,
    pub dst_ip6: [u32; 4],
    pub state: u32,
    pub rx_queue_mapping: i32,
}

#[repr(C)]
pub union in6_addr {
    pub s6_addr: [u8; 16],
    pub s6_addr16: [u16; 8],
    pub s6_addr32: [u32; 4],
}

#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [u8; 14],
}

extern "C" {
    fn bpf_sk_lookup_tcp(
        ctx: *mut bpf_sock_addr,
        tuple: *mut bpf_sock_tuple,
        tuple_size: u32,
        netns: u64,
        flags: u64,
    ) -> *mut bpf_sock;
    fn bpf_sk_lookup_udp(
        ctx: *mut bpf_sock_addr,
        tuple: *mut bpf_sock_tuple,
        tuple_size: u32,
        netns: u64,
        flags: u64,
    ) -> *mut bpf_sock;
    fn bpf_sk_release(sock: *mut bpf_sock);
    fn bpf_bind(ctx: *mut bpf_sock_addr, addr: *mut sockaddr, addr_len: u32) -> i64;
}

#[inline(always)]
fn bpf_htonl(x: u32) -> u32 {
    x.to_be()
}

#[inline(always)]
fn bpf_htons(x: u16) -> u16 {
    x.to_be()
}

#[no_mangle]
#[link_section = "cgroup/connect6"]
pub unsafe extern "C" fn connect_v6_prog(ctx: *mut bpf_sock_addr) -> i32 {
    let mut tuple: bpf_sock_tuple = core::mem::zeroed();
    let mut sa: sockaddr_in6;
    let sk: *mut bpf_sock;

    /* Verify that new destination is available. */
    core::ptr::write_bytes(
        core::ptr::addr_of_mut!(tuple.ipv6.saddr),
        0,
        core::mem::size_of_val(&tuple.ipv6.saddr),
    );
    core::ptr::write_bytes(
        core::ptr::addr_of_mut!(tuple.ipv6.sport),
        0,
        core::mem::size_of_val(&tuple.ipv6.sport),
    );

    tuple.ipv6.daddr[0] = bpf_htonl(DST_REWRITE_IP6_0);
    tuple.ipv6.daddr[1] = bpf_htonl(DST_REWRITE_IP6_1);
    tuple.ipv6.daddr[2] = bpf_htonl(DST_REWRITE_IP6_2);
    tuple.ipv6.daddr[3] = bpf_htonl(DST_REWRITE_IP6_3);

    tuple.ipv6.dport = bpf_htons(DST_REWRITE_PORT6);

    if (*ctx).type_ != SOCK_STREAM && (*ctx).type_ != SOCK_DGRAM {
        return 0;
    } else if (*ctx).type_ == SOCK_STREAM {
        sk = bpf_sk_lookup_tcp(
            ctx,
            &mut tuple,
            core::mem::size_of::<bpf_sock_tuple_ipv6>() as u32,
            BPF_F_CURRENT_NETNS,
            0,
        );
    } else {
        sk = bpf_sk_lookup_udp(
            ctx,
            &mut tuple,
            core::mem::size_of::<bpf_sock_tuple_ipv6>() as u32,
            BPF_F_CURRENT_NETNS,
            0,
        );
    }

    if sk.is_null() {
        return 0;
    }

    if (*sk).src_ip6[0] != tuple.ipv6.daddr[0]
        || (*sk).src_ip6[1] != tuple.ipv6.daddr[1]
        || (*sk).src_ip6[2] != tuple.ipv6.daddr[2]
        || (*sk).src_ip6[3] != tuple.ipv6.daddr[3]
        || (*sk).src_port != DST_REWRITE_PORT6 as u32
    {
        bpf_sk_release(sk);
        return 0;
    }

    bpf_sk_release(sk);

    /* Rewrite destination. */
    (*ctx).user_ip6[0] = bpf_htonl(DST_REWRITE_IP6_0);
    (*ctx).user_ip6[1] = bpf_htonl(DST_REWRITE_IP6_1);
    (*ctx).user_ip6[2] = bpf_htonl(DST_REWRITE_IP6_2);
    (*ctx).user_ip6[3] = bpf_htonl(DST_REWRITE_IP6_3);

    (*ctx).user_port = bpf_htons(DST_REWRITE_PORT6) as u32;

    /* Rewrite source. */
    sa = core::mem::zeroed();

    sa.sin6_family = AF_INET6;
    sa.sin6_port = bpf_htons(0);

    sa.sin6_addr.s6_addr32[0] = bpf_htonl(SRC_REWRITE_IP6_0);
    sa.sin6_addr.s6_addr32[1] = bpf_htonl(SRC_REWRITE_IP6_1);
    sa.sin6_addr.s6_addr32[2] = bpf_htonl(SRC_REWRITE_IP6_2);
    sa.sin6_addr.s6_addr32[3] = bpf_htonl(SRC_REWRITE_IP6_3);

    if bpf_bind(
        ctx,
        &mut sa as *mut sockaddr_in6 as *mut sockaddr,
        core::mem::size_of::<sockaddr_in6>() as u32,
    ) != 0
    {
        return 0;
    }

    return 1;
}

#[no_mangle]
#[link_section = "cgroup/connect6"]
pub unsafe extern "C" fn connect_v6_deny_prog(ctx: *mut bpf_sock_addr) -> i32 {
    let _ = ctx;
    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
