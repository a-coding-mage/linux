// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Isovalent */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;

const IPPROTO_TCP: __u32 = 6;
const IPPROTO_UDP: __u32 = 17;
const ETH_P_IP: __u16 = 0x0800;
const BPF_MAP_TYPE_SOCKMAP: __u32 = 15;
const SK_DROP: i32 = 0;
const SK_PASS: i32 = 1;
const TC_ACT_OK: i32 = 0;
const TC_ACT_SHOT: i32 = 2;

// Original C dependencies:
// #include <stdbool.h>
// #include <linux/bpf.h>
// #include <linux/if_ether.h>
// #include <linux/in.h>
// #include <linux/ip.h>
// #include <linux/ipv6.h>
// #include <linux/tcp.h>
// #include <linux/udp.h>
// #include <bpf/bpf_endian.h>
// #include <bpf/bpf_helpers.h>
// #include <linux/pkt_cls.h>

// SEC("license")
#[no_mangle]
pub static mut LICENSE: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut sk_cookie_seen: __u64 = 0;
#[no_mangle]
pub static mut reuseport_executed: __u64 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcphdr {
    pub source: __u16,
    pub dest: __u16,
    pub seq: __u32,
    pub ack_seq: __u32,
    pub doff_res_flags: __u16,
    pub window: __u16,
    pub check: __u16,
    pub urg_ptr: __u16,
}

impl tcphdr {
    unsafe fn syn(&self) -> bool_ {
        (self.doff_res_flags & bpf_htons(0x0002)) != 0
    }

    unsafe fn ack(&self) -> bool_ {
        (self.doff_res_flags & bpf_htons(0x0010)) != 0
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udphdr {
    pub source: __u16,
    pub dest: __u16,
    pub len: __u16,
    pub check: __u16,
}

#[repr(C)]
pub union headers_union {
    pub tcp: tcphdr,
    pub udp: udphdr,
}

#[no_mangle]
pub static mut headers: headers_union = headers_union {
    tcp: tcphdr {
        source: 0,
        dest: 0,
        seq: 0,
        ack_seq: 0,
        doff_res_flags: 0,
        window: 0,
        check: 0,
        urg_ptr: 0,
    },
};

#[no_mangle]
pub static dest_port: __u16 = 0;

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: __u16,
}

#[repr(C)]
pub struct iphdr {
    pub ihl_version: u8,
    pub tos: u8,
    pub tot_len: __u16,
    pub id: __u16,
    pub frag_off: __u16,
    pub ttl: u8,
    pub protocol: u8,
    pub check: __u16,
    pub saddr: __u32,
    pub daddr: __u32,
}

#[repr(C)]
pub struct ipv6hdr {
    pub priority_version: __u32,
    pub payload_len: __u16,
    pub nexthdr: u8,
    pub hop_limit: u8,
    pub saddr: [u8; 16],
    pub daddr: [u8; 16],
}

#[repr(C)]
pub struct __sk_buff {
    pub len: __u32,
    pub pkt_type: __u32,
    pub mark: __u32,
    pub queue_mapping: __u32,
    pub protocol: __u32,
    pub vlan_present: __u32,
    pub vlan_tci: __u32,
    pub vlan_proto: __u32,
    pub priority: __u32,
    pub ingress_ifindex: __u32,
    pub ifindex: __u32,
    pub tc_index: __u32,
    pub cb: [__u32; 5],
    pub hash: __u32,
    pub tc_classid: __u32,
    pub data: __u32,
    pub data_end: __u32,
}

#[repr(C)]
pub struct bpf_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_reuseport_md {
    pub data: *mut c_void,
    pub data_end: *mut c_void,
    pub len: __u32,
    pub eth_protocol: __u32,
    pub ip_protocol: __u32,
    pub bind_inany: __u32,
    pub hash: __u32,
    pub sk: *mut bpf_sock,
    pub migrating_sk: *mut bpf_sock,
}

#[repr(C)]
pub struct sk_map_def {
    pub type_: __u32,
    pub max_entries: __u32,
}

// struct { __uint(type, BPF_MAP_TYPE_SOCKMAP); __uint(max_entries, 1);
//          __type(key, __u32); __type(value, __u64); } sk_map SEC(".maps");
#[no_mangle]
pub static mut sk_map: sk_map_def = sk_map_def {
    type_: BPF_MAP_TYPE_SOCKMAP,
    max_entries: 1,
};

unsafe extern "C" {
    fn bpf_get_socket_cookie(sk: *mut bpf_sock) -> __u64;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut bpf_sock;
    fn bpf_sk_assign(skb: *mut __sk_buff, sk: *mut bpf_sock, flags: __u64) -> i32;
    fn bpf_sk_release(sk: *mut bpf_sock);
    fn bpf_htons(hostshort: __u16) -> __u16;
}

// SEC("sk_reuseport")
#[no_mangle]
pub unsafe extern "C" fn reuse_accept(ctx: *mut sk_reuseport_md) -> i32 {
    reuseport_executed = reuseport_executed.wrapping_add(1);

    if (*ctx).ip_protocol == IPPROTO_TCP {
        if ((*ctx).data as *mut u8).add(size_of::<tcphdr>()) > (*ctx).data_end as *mut u8 {
            return SK_DROP;
        }

        if core::intrinsics::memcmp(
            ptr::addr_of!(headers.tcp) as *const u8,
            (*ctx).data as *const u8,
            size_of::<tcphdr>(),
        ) != 0
        {
            return SK_DROP;
        }
    } else if (*ctx).ip_protocol == IPPROTO_UDP {
        if ((*ctx).data as *mut u8).add(size_of::<udphdr>()) > (*ctx).data_end as *mut u8 {
            return SK_DROP;
        }

        if core::intrinsics::memcmp(
            ptr::addr_of!(headers.udp) as *const u8,
            (*ctx).data as *const u8,
            size_of::<udphdr>(),
        ) != 0
        {
            return SK_DROP;
        }
    } else {
        return SK_DROP;
    }

    sk_cookie_seen = bpf_get_socket_cookie((*ctx).sk);
    SK_PASS
}

// SEC("sk_reuseport")
#[no_mangle]
pub unsafe extern "C" fn reuse_drop(_ctx: *mut sk_reuseport_md) -> i32 {
    reuseport_executed = reuseport_executed.wrapping_add(1);
    sk_cookie_seen = 0;
    SK_DROP
}

unsafe fn assign_sk(skb: *mut __sk_buff) -> i32 {
    let mut zero: i32 = 0;
    let mut ret: i32 = 0;
    let sk: *mut bpf_sock;

    sk = bpf_map_lookup_elem(
        ptr::addr_of_mut!(sk_map) as *mut c_void,
        ptr::addr_of_mut!(zero) as *const c_void,
    );
    if sk.is_null() {
        return TC_ACT_SHOT;
    }
    ret = bpf_sk_assign(skb, sk, 0);
    bpf_sk_release(sk);
    if ret != 0 {
        TC_ACT_SHOT
    } else {
        TC_ACT_OK
    }
}

unsafe fn maybe_assign_tcp(skb: *mut __sk_buff, th: *mut tcphdr) -> bool_ {
    if th.add(1) as *mut c_void > (*skb).data_end as usize as *mut c_void {
        return TC_ACT_SHOT != 0;
    }

    if !(*th).syn() || (*th).ack() || (*th).dest != bpf_htons(dest_port) {
        return TC_ACT_OK != 0;
    }

    ptr::copy_nonoverlapping(th, ptr::addr_of_mut!(headers.tcp), 1);
    assign_sk(skb) != 0
}

unsafe fn maybe_assign_udp(skb: *mut __sk_buff, uh: *mut udphdr) -> bool_ {
    if uh.add(1) as *mut c_void > (*skb).data_end as usize as *mut c_void {
        return TC_ACT_SHOT != 0;
    }

    if (*uh).dest != bpf_htons(dest_port) {
        return TC_ACT_OK != 0;
    }

    ptr::copy_nonoverlapping(uh, ptr::addr_of_mut!(headers.udp), 1);
    assign_sk(skb) != 0
}

// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_main(skb: *mut __sk_buff) -> i32 {
    let data_end: *mut c_void = (*skb).data_end as usize as *mut c_void;
    let data: *mut c_void = (*skb).data as usize as *mut c_void;
    let eth: *mut ethhdr;

    eth = data as *mut ethhdr;
    if eth.add(1) as *mut c_void > data_end {
        return TC_ACT_SHOT;
    }

    if (*eth).h_proto == bpf_htons(ETH_P_IP) {
        let iph: *mut iphdr = (data as *mut u8).add(size_of::<ethhdr>()) as *mut iphdr;

        if iph.add(1) as *mut c_void > data_end {
            return TC_ACT_SHOT;
        }

        if (*iph).protocol as __u32 == IPPROTO_TCP {
            if maybe_assign_tcp(skb, iph.add(1) as *mut tcphdr) {
                return 1;
            }
            return 0;
        } else if (*iph).protocol as __u32 == IPPROTO_UDP {
            if maybe_assign_udp(skb, iph.add(1) as *mut udphdr) {
                return 1;
            }
            return 0;
        } else {
            return TC_ACT_SHOT;
        }
    } else {
        let ip6h: *mut ipv6hdr = (data as *mut u8).add(size_of::<ethhdr>()) as *mut ipv6hdr;

        if ip6h.add(1) as *mut c_void > data_end {
            return TC_ACT_SHOT;
        }

        if (*ip6h).nexthdr as __u32 == IPPROTO_TCP {
            if maybe_assign_tcp(skb, ip6h.add(1) as *mut tcphdr) {
                return 1;
            }
            return 0;
        } else if (*ip6h).nexthdr as __u32 == IPPROTO_UDP {
            if maybe_assign_udp(skb, ip6h.add(1) as *mut udphdr) {
                return 1;
            }
            return 0;
        } else {
            return TC_ACT_SHOT;
        }
    }
}
