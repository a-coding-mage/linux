/* SPDX-License-Identifier: GPL-2.0 */
// Copyright (c) 2018 Covalent IO, Inc. http://covalent.io

// C dependencies:
// <stddef.h>, <stdbool.h>, <string.h>, <linux/bpf.h>, <linux/if_ether.h>,
// <linux/in.h>, <linux/ip.h>, <linux/ipv6.h>, <linux/pkt_cls.h>,
// <linux/tcp.h>, <sys/socket.h>, <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type size_t = usize;

const ETH_P_IP: __u16 = 0x0800;
const ETH_P_IPV6: __u16 = 0x86DD;
const IPPROTO_TCP: __u8 = 6;
const TC_ACT_UNSPEC: i32 = -1;
const TC_ACT_OK: i32 = 0;
const TC_ACT_SHOT: i32 = 2;
const BPF_F_CURRENT_NETNS: __u64 = -1i32 as __u64;

#[repr(C)]
pub struct __sk_buff {
    pub data: __u32,
    pub data_end: __u32,
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [__u8; 6],
    pub h_source: [__u8; 6],
    pub h_proto: __u16,
}

#[repr(C)]
pub struct iphdr {
    pub ihl_version: __u8,
    pub tos: __u8,
    pub tot_len: __u16,
    pub id: __u16,
    pub frag_off: __u16,
    pub ttl: __u8,
    pub protocol: __u8,
    pub check: __u16,
    pub saddr: __u32,
    pub daddr: __u32,
}

impl iphdr {
    unsafe fn ihl(&self) -> __u8 {
        self.ihl_version & 0x0f
    }
}

#[repr(C)]
pub struct in6_addr {
    pub in6_u: [__u8; 16],
}

#[repr(C)]
pub struct ipv6hdr {
    pub priority_version: __u8,
    pub flow_lbl: [__u8; 3],
    pub payload_len: __u16,
    pub nexthdr: __u8,
    pub hop_limit: __u8,
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_sock_tuple_ipv4 {
    pub saddr: __u32,
    pub daddr: __u32,
    pub sport: __u16,
    pub dport: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_sock_tuple_ipv6 {
    pub saddr: [__u32; 4],
    pub daddr: [__u32; 4],
    pub sport: __u16,
    pub dport: __u16,
}

#[repr(C)]
pub union bpf_sock_tuple {
    pub ipv4: bpf_sock_tuple_ipv4,
    pub ipv6: bpf_sock_tuple_ipv6,
}

#[repr(C)]
pub struct bpf_sock {
    pub family: __u32,
}

unsafe extern "C" {
    fn bpf_sk_lookup_tcp(
        skb: *mut __sk_buff,
        tuple: *mut bpf_sock_tuple,
        tuple_size: size_t,
        netns: __u64,
        flags: __u64,
    ) -> *mut bpf_sock;
    fn bpf_sk_release(sk: *mut bpf_sock) -> i64;
    fn bpf_printk(fmt: *const u8, ...) -> i64;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[inline]
const fn bpf_htons(x: __u16) -> __u16 {
    x.to_be()
}

/* Fill 'tuple' with L3 info, and attempt to find L4. On fail, return NULL. */
unsafe fn get_tuple(
    data: *mut core::ffi::c_void,
    nh_off: __u64,
    data_end: *mut core::ffi::c_void,
    eth_proto: __u16,
    ipv4: *mut bool,
) -> *mut bpf_sock_tuple {
    let mut result: *mut bpf_sock_tuple = core::ptr::null_mut();
    let mut ihl_len: __u64 = 0;
    let mut proto: __u8 = 0;

    if eth_proto == bpf_htons(ETH_P_IP) {
        let iph = (data as *mut u8).add(nh_off as usize) as *mut iphdr;

        if iph.add(1) as *mut core::ffi::c_void > data_end {
            return core::ptr::null_mut();
        }
        ihl_len = ((*iph).ihl() as __u64).wrapping_mul(4);
        proto = (*iph).protocol;
        *ipv4 = true;
        result = core::ptr::addr_of_mut!((*iph).saddr) as *mut bpf_sock_tuple;
    } else if eth_proto == bpf_htons(ETH_P_IPV6) {
        let ip6h = (data as *mut u8).add(nh_off as usize) as *mut ipv6hdr;

        if ip6h.add(1) as *mut core::ffi::c_void > data_end {
            return core::ptr::null_mut();
        }
        ihl_len = core::mem::size_of_val(&*ip6h) as __u64;
        proto = (*ip6h).nexthdr;
        *ipv4 = true;
        result = core::ptr::addr_of_mut!((*ip6h).saddr) as *mut bpf_sock_tuple;
    }

    if (data as *mut u8).add(nh_off.wrapping_add(ihl_len) as usize) as *mut core::ffi::c_void
        > data_end
        || proto != IPPROTO_TCP
    {
        return core::ptr::null_mut();
    }

    result
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sk_lookup_success(skb: *mut __sk_buff) -> i32 {
    let data_end = (*skb).data_end as usize as *mut core::ffi::c_void;
    let data = (*skb).data as usize as *mut core::ffi::c_void;
    let eth = data as *mut ethhdr;
    let mut tuple: *mut bpf_sock_tuple;
    let sk: *mut bpf_sock;
    let tuple_len: size_t;
    let mut ipv4: bool = false;

    if eth.add(1) as *mut core::ffi::c_void > data_end {
        return TC_ACT_SHOT;
    }

    tuple = get_tuple(
        data,
        core::mem::size_of_val(&*eth) as __u64,
        data_end,
        (*eth).h_proto,
        &mut ipv4,
    );
    if tuple.is_null()
        || (tuple as *mut u8).add(core::mem::size_of::<bpf_sock_tuple>())
            > data_end as *mut u8
    {
        return TC_ACT_SHOT;
    }

    tuple_len = if ipv4 {
        core::mem::size_of::<bpf_sock_tuple_ipv4>()
    } else {
        core::mem::size_of::<bpf_sock_tuple_ipv6>()
    };
    sk = bpf_sk_lookup_tcp(skb, tuple, tuple_len, BPF_F_CURRENT_NETNS, 0);
    bpf_printk(c"sk=%d\n".as_ptr() as *const u8, if !sk.is_null() { 1 } else { 0 });
    if !sk.is_null() {
        bpf_sk_release(sk);
    }
    if !sk.is_null() {
        TC_ACT_OK
    } else {
        TC_ACT_UNSPEC
    }
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sk_lookup_success_simple(skb: *mut __sk_buff) -> i32 {
    let mut tuple: bpf_sock_tuple = core::mem::zeroed();
    let sk: *mut bpf_sock;

    sk = bpf_sk_lookup_tcp(
        skb,
        &mut tuple,
        core::mem::size_of_val(&tuple),
        BPF_F_CURRENT_NETNS,
        0,
    );
    if !sk.is_null() {
        bpf_sk_release(sk);
    }
    0
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn err_use_after_free(skb: *mut __sk_buff) -> i32 {
    let mut tuple: bpf_sock_tuple = core::mem::zeroed();
    let sk: *mut bpf_sock;
    let mut family: __u32 = 0;

    sk = bpf_sk_lookup_tcp(
        skb,
        &mut tuple,
        core::mem::size_of_val(&tuple),
        BPF_F_CURRENT_NETNS,
        0,
    );
    if !sk.is_null() {
        bpf_sk_release(sk);
        family = (*sk).family;
    }
    family as i32
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn err_modify_sk_pointer(skb: *mut __sk_buff) -> i32 {
    let mut tuple: bpf_sock_tuple = core::mem::zeroed();
    let mut sk: *mut bpf_sock;

    sk = bpf_sk_lookup_tcp(
        skb,
        &mut tuple,
        core::mem::size_of_val(&tuple),
        BPF_F_CURRENT_NETNS,
        0,
    );
    if !sk.is_null() {
        sk = sk.add(1);
        bpf_sk_release(sk);
    }
    0
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn err_modify_sk_or_null_pointer(skb: *mut __sk_buff) -> i32 {
    let mut tuple: bpf_sock_tuple = core::mem::zeroed();
    let mut sk: *mut bpf_sock;

    sk = bpf_sk_lookup_tcp(
        skb,
        &mut tuple,
        core::mem::size_of_val(&tuple),
        BPF_F_CURRENT_NETNS,
        0,
    );
    sk = sk.add(1);
    if !sk.is_null() {
        bpf_sk_release(sk);
    }
    0
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn err_no_release(skb: *mut __sk_buff) -> i32 {
    let mut tuple: bpf_sock_tuple = core::mem::zeroed();

    bpf_sk_lookup_tcp(
        skb,
        &mut tuple,
        core::mem::size_of_val(&tuple),
        BPF_F_CURRENT_NETNS,
        0,
    );
    0
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn err_release_twice(skb: *mut __sk_buff) -> i32 {
    let mut tuple: bpf_sock_tuple = core::mem::zeroed();
    let sk: *mut bpf_sock;

    sk = bpf_sk_lookup_tcp(
        skb,
        &mut tuple,
        core::mem::size_of_val(&tuple),
        BPF_F_CURRENT_NETNS,
        0,
    );
    bpf_sk_release(sk);
    bpf_sk_release(sk);
    0
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn err_release_unchecked(skb: *mut __sk_buff) -> i32 {
    let mut tuple: bpf_sock_tuple = core::mem::zeroed();
    let sk: *mut bpf_sock;

    sk = bpf_sk_lookup_tcp(
        skb,
        &mut tuple,
        core::mem::size_of_val(&tuple),
        BPF_F_CURRENT_NETNS,
        0,
    );
    bpf_sk_release(sk);
    0
}

pub unsafe extern "C" fn lookup_no_release(skb: *mut __sk_buff) {
    let mut tuple: bpf_sock_tuple = core::mem::zeroed();
    bpf_sk_lookup_tcp(
        skb,
        &mut tuple,
        core::mem::size_of_val(&tuple),
        BPF_F_CURRENT_NETNS,
        0,
    );
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn err_no_release_subcall(skb: *mut __sk_buff) -> i32 {
    lookup_no_release(skb);
    0
}
