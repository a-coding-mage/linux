/* Copyright (c) 2016,2017 Facebook
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

// C dependencies removed from executable Rust:
// <stddef.h>, <string.h>, <linux/bpf.h>, <linux/if_ether.h>,
// <linux/if_packet.h>, <linux/ip.h>, <linux/ipv6.h>, <linux/in.h>,
// <linux/udp.h>, <linux/tcp.h>, <linux/pkt_cls.h>, <sys/socket.h>,
// <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>, "test_iptunnel_common.h",
// and "bpf_compiler.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::mem;
use core::ptr;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type __be16 = u16;

const BPF_MAP_TYPE_PERCPU_ARRAY: __u32 = 6;
const BPF_MAP_TYPE_HASH: __u32 = 1;
const MAX_IPTNL_ENTRIES: __u32 = 256;
const IPPROTO_TCP: __u8 = 6;
const IPPROTO_UDP: __u8 = 17;
const IPPROTO_IPIP: __u8 = 4;
const IPPROTO_IPV6: __u8 = 41;
const AF_INET: __u16 = 2;
const AF_INET6: __u16 = 10;
const ETH_P_IP: __u16 = 0x0800;
const ETH_P_IPV6: __u16 = 0x86DD;
const XDP_DROP: i32 = 1;
const XDP_PASS: i32 = 2;
const XDP_TX: i32 = 3;

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
pub struct iphdr {
    pub ihl_version: __u8,
    pub tos: __u8,
    pub tot_len: __be16,
    pub id: __u16,
    pub frag_off: __u16,
    pub ttl: __u8,
    pub protocol: __u8,
    pub check: __u16,
    pub saddr: __u32,
    pub daddr: __u32,
}

impl iphdr {
    unsafe fn set_version(&mut self, value: __u8) {
        self.ihl_version = (self.ihl_version & 0x0f) | ((value & 0x0f) << 4);
    }

    unsafe fn set_ihl(&mut self, value: __u8) {
        self.ihl_version = (self.ihl_version & 0xf0) | (value & 0x0f);
    }
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
    unsafe fn set_version(&mut self, value: __u8) {
        self.priority_version = (self.priority_version & 0x0f) | ((value & 0x0f) << 4);
    }

    unsafe fn set_priority(&mut self, value: __u8) {
        self.priority_version = (self.priority_version & 0xf0) | (value & 0x0f);
    }
}

#[repr(C)]
pub struct in6_addr {
    pub s6_addr32: [__u32; 4],
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
#[derive(Copy, Clone)]
pub union ipaddr {
    pub v4: __u32,
    pub v6: [__u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vip {
    pub protocol: __u8,
    pub family: __u16,
    pub daddr: ipaddr,
    pub dport: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iptnl_info {
    pub family: __u16,
    pub daddr: ipaddr,
    pub saddr: ipaddr,
    pub dmac: [__u8; 6],
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut rxcnt: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    max_entries: 256,
    key_size: mem::size_of::<__u32>() as __u32,
    value_size: mem::size_of::<__u64>() as __u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut vip2tnl: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: MAX_IPTNL_ENTRIES,
    key_size: mem::size_of::<vip>() as __u32,
    value_size: mem::size_of::<iptnl_info>() as __u32,
};

extern "C" {
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_xdp_adjust_head(xdp: *mut xdp_md, delta: i32) -> i32;
    fn bpf_htons(value: __u16) -> __u16;
    fn bpf_ntohs(value: __u16) -> __u16;
}

#[inline(always)]
unsafe fn count_tx(protocol: __u32) {
    let rxcnt_count: *mut __u64;

    rxcnt_count = bpf_map_lookup_elem(
        ptr::addr_of_mut!(rxcnt).cast::<c_void>(),
        ptr::addr_of!(protocol).cast::<c_void>(),
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

    match protocol {
        IPPROTO_TCP => {
            th = trans_data.cast::<tcphdr>();
            if th.add(1).cast::<c_void>() > data_end {
                return -1;
            }
            return (*th).dest as i32;
        }
        IPPROTO_UDP => {
            uh = trans_data.cast::<udphdr>();
            if uh.add(1).cast::<c_void>() > data_end {
                return -1;
            }
            return (*uh).dest as i32;
        }
        _ => {
            return 0;
        }
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
        (*new_eth).h_source.len(),
    );
    ptr::copy_nonoverlapping(
        (*tnl).dmac.as_ptr(),
        (*new_eth).h_dest.as_mut_ptr(),
        (*new_eth).h_dest.len(),
    );
    (*new_eth).h_proto = h_proto;
}

#[inline(always)]
unsafe fn handle_ipv4(xdp: *mut xdp_md) -> i32 {
    let mut data_end: *mut c_void = (*xdp).data_end as usize as *mut c_void;
    let mut data: *mut c_void = (*xdp).data as usize as *mut c_void;
    let mut tnl: *mut iptnl_info;
    let mut new_eth: *mut ethhdr;
    let mut old_eth: *mut ethhdr;
    let mut iph: *mut iphdr = data.cast::<u8>().add(mem::size_of::<ethhdr>()).cast::<iphdr>();
    let mut next_iph: *mut __u16;
    let payload_len: __u16;
    let mut vip: vip = mem::zeroed();
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
        ptr::addr_of_mut!(vip2tnl).cast::<c_void>(),
        ptr::addr_of!(vip).cast::<c_void>(),
    )
    .cast::<iptnl_info>();
    /* It only does v4-in-v4 */
    if tnl.is_null() || (*tnl).family != AF_INET {
        return XDP_PASS;
    }

    if bpf_xdp_adjust_head(xdp, 0 - mem::size_of::<iphdr>() as i32) != 0 {
        return XDP_DROP;
    }

    data = (*xdp).data as usize as *mut c_void;
    data_end = (*xdp).data_end as usize as *mut c_void;

    new_eth = data.cast::<ethhdr>();
    iph = data.cast::<u8>().add(mem::size_of_val(&*new_eth)).cast::<iphdr>();
    old_eth = data.cast::<u8>().add(mem::size_of_val(&*iph)).cast::<ethhdr>();

    if new_eth.add(1).cast::<c_void>() > data_end
        || old_eth.add(1).cast::<c_void>() > data_end
        || iph.add(1).cast::<c_void>() > data_end
    {
        return XDP_DROP;
    }

    set_ethhdr(new_eth, old_eth, tnl, bpf_htons(ETH_P_IP));

    (*iph).set_version(4);
    (*iph).set_ihl((mem::size_of_val(&*iph) >> 2) as __u8);
    (*iph).frag_off = 0;
    (*iph).protocol = IPPROTO_IPIP;
    (*iph).check = 0;
    (*iph).tos = 0;
    (*iph).tot_len = bpf_htons(payload_len.wrapping_add(mem::size_of_val(&*iph) as __u16));
    (*iph).daddr = (*tnl).daddr.v4;
    (*iph).saddr = (*tnl).saddr.v4;
    (*iph).ttl = 8;

    next_iph = iph.cast::<__u16>();
    // __pragma_loop_unroll_full
    i = 0;
    while i < (mem::size_of_val(&*iph) >> 1) as i32 {
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
    let mut data_end: *mut c_void = (*xdp).data_end as usize as *mut c_void;
    let mut data: *mut c_void = (*xdp).data as usize as *mut c_void;
    let mut tnl: *mut iptnl_info;
    let mut new_eth: *mut ethhdr;
    let mut old_eth: *mut ethhdr;
    let mut ip6h: *mut ipv6hdr = data.cast::<u8>().add(mem::size_of::<ethhdr>()).cast::<ipv6hdr>();
    let payload_len: __u16;
    let mut vip: vip = mem::zeroed();
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
        mem::size_of_val(&vip.daddr) / mem::size_of::<__u32>(),
    );
    vip.dport = dport;
    payload_len = (*ip6h).payload_len;

    tnl = bpf_map_lookup_elem(
        ptr::addr_of_mut!(vip2tnl).cast::<c_void>(),
        ptr::addr_of!(vip).cast::<c_void>(),
    )
    .cast::<iptnl_info>();
    /* It only does v6-in-v6 */
    if tnl.is_null() || (*tnl).family != AF_INET6 {
        return XDP_PASS;
    }

    if bpf_xdp_adjust_head(xdp, 0 - mem::size_of::<ipv6hdr>() as i32) != 0 {
        return XDP_DROP;
    }

    data = (*xdp).data as usize as *mut c_void;
    data_end = (*xdp).data_end as usize as *mut c_void;

    new_eth = data.cast::<ethhdr>();
    ip6h = data.cast::<u8>().add(mem::size_of_val(&*new_eth)).cast::<ipv6hdr>();
    old_eth = data.cast::<u8>().add(mem::size_of_val(&*ip6h)).cast::<ethhdr>();

    if new_eth.add(1).cast::<c_void>() > data_end
        || old_eth.add(1).cast::<c_void>() > data_end
        || ip6h.add(1).cast::<c_void>() > data_end
    {
        return XDP_DROP;
    }

    set_ethhdr(new_eth, old_eth, tnl, bpf_htons(ETH_P_IPV6));

    (*ip6h).set_version(6);
    (*ip6h).set_priority(0);
    ptr::write_bytes((*ip6h).flow_lbl.as_mut_ptr(), 0, (*ip6h).flow_lbl.len());
    (*ip6h).payload_len = bpf_htons(bpf_ntohs(payload_len).wrapping_add(mem::size_of_val(&*ip6h) as __u16));
    (*ip6h).nexthdr = IPPROTO_IPV6;
    (*ip6h).hop_limit = 8;
    ptr::copy_nonoverlapping(
        (*tnl).saddr.v6.as_ptr(),
        (*ip6h).saddr.s6_addr32.as_mut_ptr(),
        (*tnl).saddr.v6.len(),
    );
    ptr::copy_nonoverlapping(
        (*tnl).daddr.v6.as_ptr(),
        (*ip6h).daddr.s6_addr32.as_mut_ptr(),
        (*tnl).daddr.v6.len(),
    );

    count_tx(vip.protocol as __u32);

    return XDP_TX;
}

#[link_section = "xdp"]
#[no_mangle]
pub unsafe extern "C" fn _xdp_tx_iptunnel(xdp: *mut xdp_md) -> i32 {
    let data_end: *mut c_void = (*xdp).data_end as usize as *mut c_void;
    let data: *mut c_void = (*xdp).data as usize as *mut c_void;
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

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
