// SPDX-License-Identifier: GPL-2.0
//
// Translated from test_tc_neigh.c.
// Original C dependencies:
// <stddef.h>, <stdint.h>, <stdbool.h>
// <linux/bpf.h>, <linux/stddef.h>, <linux/pkt_cls.h>, <linux/if_ether.h>,
// <linux/in.h>, <linux/ip.h>, <linux/ipv6.h>
// <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u8 = u8;
type __u32 = u32;
type __be32 = u32;

const ip4_src: __u32 = 0xac100164; /* 172.16.1.100 */
const ip4_dst: __u32 = 0xac100264; /* 172.16.2.100 */

const ip6_src: [__u8; 16] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xde, 0xad, 0xbe, 0xef, 0xca,
    0xfe,
];
const ip6_dst: [__u8; 16] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xde, 0xad, 0xbe, 0xef, 0xca,
    0xfe,
];

// volatile const __u32 in C; BPF loaders may rewrite these globals.
#[no_mangle]
pub static IFINDEX_SRC: __u32 = 0;
#[no_mangle]
pub static IFINDEX_DST: __u32 = 0;

extern "C" {
    fn bpf_skb_store_bytes(
        skb: *mut __sk_buff,
        offset: __u32,
        from: *const core::ffi::c_void,
        len: __u32,
        flags: __u64,
    ) -> i64;
    fn bpf_redirect_neigh(
        ifindex: __u32,
        params: *mut core::ffi::c_void,
        plen: i32,
        flags: __u64,
    ) -> i32;
}

type __u64 = u64;

// Types and constants are supplied by the translated kernel/BPF bindings.
extern "Rust" {
    type __sk_buff;
    type ethhdr;
    type iphdr;
    type ipv6hdr;
    type in6_addr;

    static TC_ACT_SHOT: i32;
    static TC_ACT_OK: i32;
    static ETH_ALEN: usize;
    static ETH_P_IP: u16;
    static ETH_P_IPV6: u16;
}

extern "Rust" {
    fn __bpf_constant_htons(x: u16) -> u16;
    fn __bpf_constant_htonl(x: __u32) -> __u32;
}

// Field accessors supplied by bindings for externally defined C structs.
extern "Rust" {
    fn skb_data(skb: *mut __sk_buff) -> usize;
    fn skb_data_end(skb: *mut __sk_buff) -> usize;
    fn skb_protocol(skb: *mut __sk_buff) -> u16;
    fn iphdr_daddr(ip4h: *mut iphdr) -> __be32;
    fn ipv6hdr_daddr(ip6h: *mut ipv6hdr) -> in6_addr;
    fn in6_addr_s6_addr32(addr: in6_addr, index: usize) -> __u32;
    fn in6_addr_from_bytes(bytes: [__u8; 16]) -> in6_addr;
}

#[inline(always)]
unsafe fn ctx_ptr(field: usize) -> *mut core::ffi::c_void {
    field as isize as *mut core::ffi::c_void
}

#[inline(always)]
unsafe fn v6_equal(a: in6_addr, b: in6_addr) -> bool {
    in6_addr_s6_addr32(a, 0) == in6_addr_s6_addr32(b, 0)
        && in6_addr_s6_addr32(a, 1) == in6_addr_s6_addr32(b, 1)
        && in6_addr_s6_addr32(a, 2) == in6_addr_s6_addr32(b, 2)
        && in6_addr_s6_addr32(a, 3) == in6_addr_s6_addr32(b, 3)
}

#[inline(always)]
unsafe fn is_remote_ep_v4(skb: *mut __sk_buff, addr: __be32) -> bool {
    let data_end = ctx_ptr(skb_data_end(skb)) as *mut __u8;
    let data = ctx_ptr(skb_data(skb)) as *mut __u8;
    let ip4h: *mut iphdr;

    if data.add(core::mem::size_of::<ethhdr>()) > data_end {
        return false;
    }

    ip4h = data.add(core::mem::size_of::<ethhdr>()) as *mut iphdr;
    if ip4h.add(1) as *mut core::ffi::c_void > data_end as *mut core::ffi::c_void {
        return false;
    }

    iphdr_daddr(ip4h) == addr
}

#[inline(always)]
unsafe fn is_remote_ep_v6(skb: *mut __sk_buff, addr: in6_addr) -> bool {
    let data_end = ctx_ptr(skb_data_end(skb)) as *mut __u8;
    let data = ctx_ptr(skb_data(skb)) as *mut __u8;
    let ip6h: *mut ipv6hdr;

    if data.add(core::mem::size_of::<ethhdr>()) > data_end {
        return false;
    }

    ip6h = data.add(core::mem::size_of::<ethhdr>()) as *mut ipv6hdr;
    if ip6h.add(1) as *mut core::ffi::c_void > data_end as *mut core::ffi::c_void {
        return false;
    }

    v6_equal(ipv6hdr_daddr(ip6h), addr)
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn tc_chk(skb: *mut __sk_buff) -> i32 {
    let data_end = ctx_ptr(skb_data_end(skb)) as *mut __u8;
    let data = ctx_ptr(skb_data(skb)) as *mut __u8;
    let raw = data as *mut __u32;

    if data.add(core::mem::size_of::<ethhdr>()) > data_end {
        return TC_ACT_SHOT;
    }

    if *raw.add(0) == 0 && *raw.add(1) == 0 && *raw.add(2) == 0 {
        TC_ACT_SHOT
    } else {
        TC_ACT_OK
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn tc_dst(skb: *mut __sk_buff) -> i32 {
    let mut zero: [__u8; 12] = [0; 12];
    let mut redirect = false;

    match skb_protocol(skb) {
        x if x == __bpf_constant_htons(ETH_P_IP) => {
            redirect = is_remote_ep_v4(skb, __bpf_constant_htonl(ip4_src));
        }
        x if x == __bpf_constant_htons(ETH_P_IPV6) => {
            redirect = is_remote_ep_v6(skb, in6_addr_from_bytes(ip6_src));
        }
        _ => {}
    }

    if !redirect {
        return TC_ACT_OK;
    }

    zero.fill(0);
    if bpf_skb_store_bytes(
        skb,
        0,
        zero.as_ptr() as *const core::ffi::c_void,
        core::mem::size_of_val(&zero) as __u32,
        0,
    ) < 0
    {
        return TC_ACT_SHOT;
    }

    bpf_redirect_neigh(IFINDEX_SRC, core::ptr::null_mut(), 0, 0)
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn tc_src(skb: *mut __sk_buff) -> i32 {
    let mut zero: [__u8; 12] = [0; 12];
    let mut redirect = false;

    match skb_protocol(skb) {
        x if x == __bpf_constant_htons(ETH_P_IP) => {
            redirect = is_remote_ep_v4(skb, __bpf_constant_htonl(ip4_dst));
        }
        x if x == __bpf_constant_htons(ETH_P_IPV6) => {
            redirect = is_remote_ep_v6(skb, in6_addr_from_bytes(ip6_dst));
        }
        _ => {}
    }

    if !redirect {
        return TC_ACT_OK;
    }

    zero.fill(0);
    if bpf_skb_store_bytes(
        skb,
        0,
        zero.as_ptr() as *const core::ffi::c_void,
        core::mem::size_of_val(&zero) as __u32,
        0,
    ) < 0
    {
        return TC_ACT_SHOT;
    }

    bpf_redirect_neigh(IFINDEX_DST, core::ptr::null_mut(), 0, 0)
}

#[no_mangle]
#[link_section = "license"]
pub static __license: [u8; 4] = *b"GPL\0";
