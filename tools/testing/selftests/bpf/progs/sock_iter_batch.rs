// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2024 Meta

// C dependencies translated as external Rust context:
// "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_core_read.h>,
// <bpf/bpf_endian.h>, "bpf_tracing_net.h", "bpf_kfuncs.h",
// and "test_jhash.h".

use core::ffi::{c_int, c_uint};
use core::mem::size_of;

const ATTR: &str = "__always_inline";

#[inline(always)]
unsafe fn ipv6_addr_loopback(a: *const in6_addr) -> bool {
    (((*a).s6_addr32[0] | (*a).s6_addr32[1] | (*a).s6_addr32[2]
        | ((*a).s6_addr32[3] ^ bpf_htonl(1))) == 0)
}

#[inline(always)]
unsafe fn ipv4_addr_loopback(a: __be32) -> bool {
    a == bpf_ntohl(0x7f000001)
}

// C volatile const globals are BPF configuration variables. Use volatile reads
// at each access to preserve the source-level access intent.
#[no_mangle]
pub static sf: c_uint = 0;
#[no_mangle]
pub static ss: c_uint = 0;
#[no_mangle]
pub static ports: [__u16; 2] = [0; 2];
#[no_mangle]
pub static mut bucket: [c_uint; 2] = [0; 2];

#[link_section = "iter/tcp"]
#[no_mangle]
pub unsafe extern "C" fn iter_tcp_soreuse(ctx: *mut bpf_iter__tcp) -> c_int {
    let mut sk: *mut sock = (*ctx).sk_common as *mut sock;
    let hinfo: *mut inet_hashinfo;
    let mut hash: c_uint;
    let sock_cookie: __u64;
    let net: *mut net;
    let idx: c_int;
    let sf_val = core::ptr::read_volatile(&sf);
    let ss_val = core::ptr::read_volatile(&ss);
    let port0 = core::ptr::read_volatile(ports.as_ptr().add(0));
    let port1 = core::ptr::read_volatile(ports.as_ptr().add(1));

    if sk.is_null() {
        return 0;
    }

    sock_cookie = bpf_get_socket_cookie(sk as *mut _);
    sk = sk as *mut sock;
    if (*sk).sk_family != sf_val
        || (ss_val != 0 && (*sk).sk_state != ss_val)
        || (if (*sk).sk_family == AF_INET6 {
            !ipv6_addr_loopback(&raw const (*sk).sk_v6_rcv_saddr)
        } else {
            !ipv4_addr_loopback((*sk).sk_rcv_saddr)
        })
    {
        return 0;
    }

    if (*sk).sk_num == port0 {
        idx = 0;
    } else if (*sk).sk_num == port1 {
        idx = 1;
    } else if port0 == 0 && port1 == 0 {
        idx = 0;
    } else {
        return 0;
    }

    /* bucket selection as in inet_lhash2_bucket_sk() */
    net = (*sk).sk_net.net;
    hash = jhash2((*sk).sk_v6_rcv_saddr.s6_addr32.as_ptr(), 4, (*net).hash_mix);
    hash ^= (*sk).sk_num as c_uint;
    hinfo = (*net).ipv4.tcp_death_row.hashinfo;
    bucket[idx as usize] = hash & (*hinfo).lhash2_mask;
    bpf_seq_write(
        (*(*ctx).meta).seq,
        &idx as *const _ as *const _,
        size_of::<c_int>() as __u32,
    );
    bpf_seq_write(
        (*(*ctx).meta).seq,
        &sock_cookie as *const _ as *const _,
        size_of::<__u64>() as __u32,
    );

    0
}

#[no_mangle]
pub static destroy_cookie: __u64 = 0;

#[link_section = "iter/tcp"]
#[no_mangle]
pub unsafe extern "C" fn iter_tcp_destroy(ctx: *mut bpf_iter__tcp) -> c_int {
    let sk_common: *mut sock_common = (*ctx).sk_common as *mut sock_common;
    let sock_cookie: __u64;
    let destroy_cookie_val = core::ptr::read_volatile(&destroy_cookie);

    if sk_common.is_null() {
        return 0;
    }

    sock_cookie = bpf_get_socket_cookie(sk_common as *mut _);
    if sock_cookie != destroy_cookie_val {
        return 0;
    }

    bpf_sock_destroy(sk_common);
    bpf_seq_write(
        (*(*ctx).meta).seq,
        &sock_cookie as *const _ as *const _,
        size_of::<__u64>() as __u32,
    );

    0
}

// C macro: #define udp_sk(ptr) container_of(ptr, struct udp_sock, inet.sk)
unsafe fn udp_sk(ptr: *mut sock) -> *mut udp_sock {
    container_of!(ptr, udp_sock, inet.sk)
}

#[link_section = "iter/udp"]
#[no_mangle]
pub unsafe extern "C" fn iter_udp_soreuse(ctx: *mut bpf_iter__udp) -> c_int {
    let mut sk: *mut sock = (*ctx).udp_sk as *mut sock;
    let udptable: *mut udp_table;
    let sock_cookie: __u64;
    let idx: c_int;
    let sf_val = core::ptr::read_volatile(&sf);
    let port0 = core::ptr::read_volatile(ports.as_ptr().add(0));
    let port1 = core::ptr::read_volatile(ports.as_ptr().add(1));

    if sk.is_null() {
        return 0;
    }

    sock_cookie = bpf_get_socket_cookie(sk as *mut _);
    sk = sk as *mut sock;
    if (*sk).sk_family != sf_val
        || (if (*sk).sk_family == AF_INET6 {
            !ipv6_addr_loopback(&raw const (*sk).sk_v6_rcv_saddr)
        } else {
            !ipv4_addr_loopback((*sk).sk_rcv_saddr)
        })
    {
        return 0;
    }

    if (*sk).sk_num == port0 {
        idx = 0;
    } else if (*sk).sk_num == port1 {
        idx = 1;
    } else if port0 == 0 && port1 == 0 {
        idx = 0;
    } else {
        return 0;
    }

    /* bucket selection as in udp_hashslot2() */
    udptable = (*(*sk).sk_net.net).ipv4.udp_table;
    bucket[idx as usize] = (*udp_sk(sk)).udp_portaddr_hash & (*udptable).mask;
    bpf_seq_write(
        (*(*ctx).meta).seq,
        &idx as *const _ as *const _,
        size_of::<c_int>() as __u32,
    );
    bpf_seq_write(
        (*(*ctx).meta).seq,
        &sock_cookie as *const _ as *const _,
        size_of::<__u64>() as __u32,
    );

    0
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
