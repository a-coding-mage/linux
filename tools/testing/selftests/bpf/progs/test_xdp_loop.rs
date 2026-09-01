// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook
//
// Translated from C. Original dependencies:
// <stddef.h>, <string.h>, <linux/bpf.h>, <linux/if_ether.h>,
// <linux/if_packet.h>, <linux/ip.h>, <linux/ipv6.h>, <linux/in.h>,
// <linux/udp.h>, <linux/tcp.h>, <linux/pkt_cls.h>, <sys/socket.h>,
// <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>, "test_iptunnel_common.h",
// and "bpf_compiler.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type __be16 = __u16;

extern "C" {
    static mut rxcnt: c_void;
    static mut vip2tnl: c_void;

    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_xdp_adjust_head(xdp: *mut xdp_md, delta: i32) -> i32;
    fn bpf_ntohs(x: __u16) -> __u16;
    fn bpf_htons(x: __u16) -> __u16;
}

extern "C" {
    static MAX_IPTNL_ENTRIES: __u32;
    static BPF_MAP_TYPE_PERCPU_ARRAY: __u32;
    static BPF_MAP_TYPE_HASH: __u32;
    static IPPROTO_TCP: __u8;
    static IPPROTO_UDP: __u8;
    static IPPROTO_IPIP: __u8;
    static IPPROTO_IPV6: __u8;
    static ETH_P_IP: __u16;
    static ETH_P_IPV6: __u16;
    static AF_INET: __u16;
    static AF_INET6: __u16;
    static XDP_DROP: i32;
    static XDP_PASS: i32;
    static XDP_TX: i32;
}

#[repr(C)]
pub struct xdp_md {
    pub data: __u32,
    pub data_end: __u32,
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [__u8; 6],
    pub h_source: [__u8; 6],
    pub h_proto: __be16,
}

#[repr(C)]
pub struct tcphdr {
    pub source: __be16,
    pub dest: __be16,
}

#[repr(C)]
pub struct udphdr {
    pub source: __be16,
    pub dest: __be16,
}

#[repr(C)]
pub struct iphdr {
    pub ihl_version: __u8,
    pub tos: __u8,
    pub tot_len: __be16,
    pub id: __be16,
    pub frag_off: __be16,
    pub ttl: __u8,
    pub protocol: __u8,
    pub check: __be16,
    pub saddr: __u32,
    pub daddr: __u32,
}

impl iphdr {
    unsafe fn set_version(&mut self, version: __u8) {
        self.ihl_version = (self.ihl_version & 0x0f) | ((version & 0x0f) << 4);
    }

    unsafe fn set_ihl(&mut self, ihl: __u8) {
        self.ihl_version = (self.ihl_version & 0xf0) | (ihl & 0x0f);
    }
}

#[repr(C)]
pub struct in6_addr {
    pub s6_addr32: [__u32; 4],
}

#[repr(C)]
pub struct ipv6hdr {
    pub priority_version: __u8,
    pub flow_lbl: [__u8; 3],
    pub payload_len: __be16,
    pub nexthdr: __u8,
    pub hop_limit: __u8,
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

impl ipv6hdr {
    unsafe fn set_version(&mut self, version: __u8) {
        self.priority_version = (self.priority_version & 0x0f) | ((version & 0x0f) << 4);
    }

    unsafe fn set_priority(&mut self, priority: __u8) {
        self.priority_version = (self.priority_version & 0xf0) | (priority & 0x0f);
    }
}

#[repr(C)]
pub union vip_addr {
    pub v4: __u32,
    pub v6: [__u32; 4],
}

#[repr(C)]
pub struct vip {
    pub protocol: __u8,
    pub family: __u16,
    pub daddr: vip_addr,
    pub dport: i32,
}

#[repr(C)]
pub union iptnl_addr {
    pub v4: __u32,
    pub v6: [__u32; 4],
}

#[repr(C)]
pub struct iptnl_info {
    pub family: __u16,
    pub daddr: iptnl_addr,
    pub saddr: iptnl_addr,
    pub dmac: [__u8; 6],
}

// C map declarations translated from:
// struct {
//     __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
//     __uint(max_entries, 256);
//     __type(key, __u32);
//     __type(value, __u64);
// } rxcnt SEC(".maps");
//
// struct {
//     __uint(type, BPF_MAP_TYPE_HASH);
//     __uint(max_entries, MAX_IPTNL_ENTRIES);
//     __type(key, struct vip);
//     __type(value, struct iptnl_info);
// } vip2tnl SEC(".maps");

#[inline(always)]
unsafe fn count_tx(protocol: __u32) {
    let rxcnt_count: *mut __u64;

    rxcnt_count = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(rxcnt),
        (&protocol as *const __u32).cast::<c_void>(),
    )
    .cast::<__u64>();
    if !rxcnt_count.is_null() {
        *rxcnt_count = (*rxcnt_count).wrapping_add(1);
    }
}

#[inline(always)]
unsafe fn get_dport(trans_data: *mut c_void, data_end: *mut c_void, protocol: __u8) -> i32 {
    let th: *mut tcphdr;
    let uh: *mut udphdr;

    if protocol == IPPROTO_TCP {
        th = trans_data.cast::<tcphdr>();
        if th.add(1).cast::<c_void>() > data_end {
            return -1;
        }
        return (*th).dest as i32;
    } else if protocol == IPPROTO_UDP {
        uh = trans_data.cast::<udphdr>();
        if uh.add(1).cast::<c_void>() > data_end {
            return -1;
        }
        return (*uh).dest as i32;
    } else {
        return 0;
    }
}

#[inline(always)]
unsafe fn set_ethhdr(
    new_eth: *mut ethhdr,
    old_eth: *const ethhdr,
    tnl: *const iptnl_info,
    h_proto: __be16,
) {
    ptr::copy_nonoverlapping(
        (*old_eth).h_dest.as_ptr(),
        (*new_eth).h_source.as_mut_ptr(),
        size_of_val(&(*new_eth).h_source),
    );
    ptr::copy_nonoverlapping(
        (*tnl).dmac.as_ptr(),
        (*new_eth).h_dest.as_mut_ptr(),
        size_of_val(&(*new_eth).h_dest),
    );
    (*new_eth).h_proto = h_proto;
}

#[inline(always)]
unsafe fn handle_ipv4(xdp: *mut xdp_md) -> i32 {
    let mut data_end: *mut c_void = ((*xdp).data_end as usize) as *mut c_void;
    let mut data: *mut c_void = ((*xdp).data as usize) as *mut c_void;
    let mut tnl: *mut iptnl_info;
    let mut new_eth: *mut ethhdr;
    let mut old_eth: *mut ethhdr;
    let mut iph: *mut iphdr = data.cast::<u8>().add(size_of::<ethhdr>()).cast::<iphdr>();
    let mut next_iph: *mut __u16;
    let payload_len: __u16;
    let mut vip: vip = core::mem::zeroed();
    let dport: i32;
    let mut csum: __u32 = 0;
    let mut i: i32;

    if iph.add(1).cast::<c_void>() > data_end {
        return XDP_DROP;
    }

    dport = get_dport(iph.add(1).cast::<c_void>(), data_end, (*iph).protocol);
    if dport == -1 {
        return XDP_DROP;
    }

    vip.protocol = (*iph).protocol;
    vip.family = AF_INET;
    vip.daddr.v4 = (*iph).daddr;
    vip.dport = dport;
    payload_len = bpf_ntohs((*iph).tot_len);

    tnl = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(vip2tnl),
        (&vip as *const vip).cast::<c_void>(),
    )
    .cast::<iptnl_info>();
    /* It only does v4-in-v4 */
    if tnl.is_null() || (*tnl).family != AF_INET {
        return XDP_PASS;
    }

    if bpf_xdp_adjust_head(xdp, 0 - size_of::<iphdr>() as i32) != 0 {
        return XDP_DROP;
    }

    data = ((*xdp).data as usize) as *mut c_void;
    data_end = ((*xdp).data_end as usize) as *mut c_void;

    new_eth = data.cast::<ethhdr>();
    iph = data.cast::<u8>().add(size_of::<ethhdr>()).cast::<iphdr>();
    old_eth = data.cast::<u8>().add(size_of::<iphdr>()).cast::<ethhdr>();

    if new_eth.add(1).cast::<c_void>() > data_end
        || old_eth.add(1).cast::<c_void>() > data_end
        || iph.add(1).cast::<c_void>() > data_end
    {
        return XDP_DROP;
    }

    set_ethhdr(new_eth, old_eth, tnl, bpf_htons(ETH_P_IP));

    (*iph).set_version(4);
    (*iph).set_ihl((size_of::<iphdr>() >> 2) as __u8);
    (*iph).frag_off = 0;
    (*iph).protocol = IPPROTO_IPIP;
    (*iph).check = 0;
    (*iph).tos = 0;
    (*iph).tot_len = bpf_htons(payload_len.wrapping_add(size_of::<iphdr>() as __u16));
    (*iph).daddr = (*tnl).daddr.v4;
    (*iph).saddr = (*tnl).saddr.v4;
    (*iph).ttl = 8;

    next_iph = iph.cast::<__u16>();
    // __pragma_loop_no_unroll
    i = 0;
    while i < (size_of::<iphdr>() >> 1) as i32 {
        csum = csum.wrapping_add(*next_iph as __u32);
        next_iph = next_iph.add(1);
        i += 1;
    }

    (*iph).check = !((csum & 0xffff).wrapping_add(csum >> 16) as __u16);

    count_tx(vip.protocol as __u32);

    return XDP_TX;
}

#[inline(always)]
unsafe fn handle_ipv6(xdp: *mut xdp_md) -> i32 {
    let mut data_end: *mut c_void = ((*xdp).data_end as usize) as *mut c_void;
    let mut data: *mut c_void = ((*xdp).data as usize) as *mut c_void;
    let mut tnl: *mut iptnl_info;
    let mut new_eth: *mut ethhdr;
    let mut old_eth: *mut ethhdr;
    let mut ip6h: *mut ipv6hdr = data.cast::<u8>().add(size_of::<ethhdr>()).cast::<ipv6hdr>();
    let payload_len: __u16;
    let mut vip: vip = core::mem::zeroed();
    let dport: i32;

    if ip6h.add(1).cast::<c_void>() > data_end {
        return XDP_DROP;
    }

    dport = get_dport(ip6h.add(1).cast::<c_void>(), data_end, (*ip6h).nexthdr);
    if dport == -1 {
        return XDP_DROP;
    }

    vip.protocol = (*ip6h).nexthdr;
    vip.family = AF_INET6;
    ptr::copy_nonoverlapping(
        (*ip6h).daddr.s6_addr32.as_ptr(),
        vip.daddr.v6.as_mut_ptr(),
        size_of_val(&vip.daddr),
    );
    vip.dport = dport;
    payload_len = (*ip6h).payload_len;

    tnl = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(vip2tnl),
        (&vip as *const vip).cast::<c_void>(),
    )
    .cast::<iptnl_info>();
    /* It only does v6-in-v6 */
    if tnl.is_null() || (*tnl).family != AF_INET6 {
        return XDP_PASS;
    }

    if bpf_xdp_adjust_head(xdp, 0 - size_of::<ipv6hdr>() as i32) != 0 {
        return XDP_DROP;
    }

    data = ((*xdp).data as usize) as *mut c_void;
    data_end = ((*xdp).data_end as usize) as *mut c_void;

    new_eth = data.cast::<ethhdr>();
    ip6h = data.cast::<u8>().add(size_of::<ethhdr>()).cast::<ipv6hdr>();
    old_eth = data.cast::<u8>().add(size_of::<ipv6hdr>()).cast::<ethhdr>();

    if new_eth.add(1).cast::<c_void>() > data_end
        || old_eth.add(1).cast::<c_void>() > data_end
        || ip6h.add(1).cast::<c_void>() > data_end
    {
        return XDP_DROP;
    }

    set_ethhdr(new_eth, old_eth, tnl, bpf_htons(ETH_P_IPV6));

    (*ip6h).set_version(6);
    (*ip6h).set_priority(0);
    ptr::write_bytes((*ip6h).flow_lbl.as_mut_ptr(), 0, size_of_val(&(*ip6h).flow_lbl));
    (*ip6h).payload_len = bpf_htons(bpf_ntohs(payload_len).wrapping_add(size_of::<ipv6hdr>() as __u16));
    (*ip6h).nexthdr = IPPROTO_IPV6;
    (*ip6h).hop_limit = 8;
    ptr::copy_nonoverlapping(
        (*tnl).saddr.v6.as_ptr(),
        (*ip6h).saddr.s6_addr32.as_mut_ptr(),
        size_of_val(&(*tnl).saddr.v6),
    );
    ptr::copy_nonoverlapping(
        (*tnl).daddr.v6.as_ptr(),
        (*ip6h).daddr.s6_addr32.as_mut_ptr(),
        size_of_val(&(*tnl).daddr.v6),
    );

    count_tx(vip.protocol as __u32);

    return XDP_TX;
}

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn _xdp_tx_iptunnel(xdp: *mut xdp_md) -> i32 {
    let data_end: *mut c_void = ((*xdp).data_end as usize) as *mut c_void;
    let data: *mut c_void = ((*xdp).data as usize) as *mut c_void;
    let eth: *mut ethhdr = data.cast::<ethhdr>();
    let h_proto: __u16;

    if eth.add(1).cast::<c_void>() > data_end {
        return XDP_DROP;
    }

    h_proto = (*eth).h_proto;

    if h_proto == bpf_htons(ETH_P_IP) {
        return handle_ipv4(xdp);
    } else if h_proto == bpf_htons(ETH_P_IPV6) {
        return handle_ipv6(xdp);
    } else {
        return XDP_DROP;
    }
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
